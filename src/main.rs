#![no_std]
#![no_main]

use crate::button::button_task;
use crate::led::{led_task, RGBLed};
use defmt::*;
use embassy_executor::Spawner;
use embassy_futures::select::{select, Either};
use embassy_rp::bind_interrupts;
use embassy_rp::gpio::{Input, Pull};
use embassy_rp::peripherals::{PIO0, TRNG, USB};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::pio_programs::ws2812::{PioWs2812, PioWs2812Program, Rgb};
use embassy_rp::Peri;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    channel::{Channel, Receiver, Sender},
};
use embassy_time::Ticker;
use patterns::{alternating, breathe, rainbow, rainbow_comet, twinkle, Pattern};
use smart_leds::RGB8;

use panic_probe as _;

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    TRNG_IRQ => embassy_rp::trng::InterruptHandler<TRNG>;
    USBCTRL_IRQ => embassy_rp::usb::InterruptHandler<USB>;
});

mod button;
mod led;
mod patterns;

static CHANNEL: Channel<CriticalSectionRawMutex, (), 1> = Channel::new();

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Start");
    let p = embassy_rp::init(Default::default());

    let Pio {
        mut common, sm0, ..
    } = Pio::new(p.PIO0, Irqs);

    const NUM_LEDS: usize = 50;
    let mut data = [RGB8::default(); NUM_LEDS];

    let program = PioWs2812Program::new(&mut common);
    let mut ws2812: PioWs2812<'_, _, 0, 50, Rgb> =
        PioWs2812::with_color_order(&mut common, sm0, p.DMA_CH0, p.PIN_15, &program);

    let mut trng_config = embassy_rp::trng::Config::default();
    trng_config.sample_count = 50;
    let mut trng = embassy_rp::trng::Trng::new(p.TRNG, Irqs, trng_config);
    let mut rainbow_counter: u16 = 0;
    let mut fading_state = breathe::FadingState::default();
    let mut alternating_state = alternating::AlternatingState::default();
    let mut rainbow_comet_state = rainbow_comet::CometState::default();

    let mut current_pattern = Pattern::Rainbow;
    let mut current_duration = rainbow::DURATION;

    let sender: Sender<'static, CriticalSectionRawMutex, (), 1> = CHANNEL.sender();
    let btn_a = Input::new(p.PIN_12, Pull::Up);
    let rgb_led = RGBLed::new(p.PWM_SLICE0, p.PWM_SLICE1, p.PIN_16, p.PIN_17, p.PIN_18);

    #[allow(clippy::unwrap_used)]
    spawner.spawn(button_task(sender, btn_a).unwrap());
    spawner.spawn(defmtusb_wrapper(p.USB).unwrap());
    spawner.spawn(led_task(rgb_led).unwrap());

    let receiver: Receiver<'static, CriticalSectionRawMutex, (), 1> = CHANNEL.receiver();
    let mut ticker = Ticker::every(current_duration);

    loop {
        match select(receiver.receive(), ticker.next()).await {
            Either::First(_) => {
                current_pattern = match current_pattern {
                    Pattern::Rainbow => {
                        info!("Switching pattern to Twinkle");
                        Pattern::Twinkle
                    }
                    Pattern::Twinkle => {
                        info!("Switching pattern to Breathe");
                        Pattern::Breathe
                    }
                    Pattern::Breathe => {
                        info!("Switching pattern to Rainbow");
                        Pattern::Alternating
                    }
                    Pattern::Alternating => {
                        info!("Switching pattern to Rainbow");
                        Pattern::RainbowComet
                    }
                    Pattern::RainbowComet => {
                        info!("Switching pattern to Rainbow");
                        Pattern::Rainbow
                    }
                };
                current_duration = match current_pattern {
                    Pattern::Rainbow => rainbow::DURATION,
                    Pattern::Twinkle => twinkle::DURATION,
                    Pattern::Breathe => breathe::DURATION,
                    Pattern::Alternating => alternating::DURATION,
                    Pattern::RainbowComet => rainbow_comet::DURATION,
                };
                ticker = Ticker::every(current_duration)
            }

            Either::Second(_) => {
                match current_pattern {
                    Pattern::Rainbow => {
                        rainbow::generate(&mut data, &mut rainbow_counter);
                    }
                    Pattern::Twinkle => {
                        twinkle::generate(&mut data, &mut trng).await;
                    }
                    Pattern::Breathe => {
                        breathe::generate(&mut data, &mut fading_state);
                    }
                    Pattern::Alternating => {
                        alternating::generate(&mut data, &mut alternating_state);
                    }
                    Pattern::RainbowComet => {
                        rainbow_comet::generate(&mut data, &mut rainbow_comet_state);
                    }
                }
                ws2812.write(&data).await;
            }
        }
    }
}

#[embassy_executor::task]
async fn defmtusb_wrapper(usb: Peri<'static, USB>) {
    let driver = embassy_rp::usb::Driver::new(usb, Irqs);
    let config = {
        let mut c = embassy_usb::Config::new(0x1234, 0x5678);
        c.serial_number = Some("defmt");
        c.max_packet_size_0 = 64;
        c.composite_with_iads = true;
        c.device_class = 0xEF;
        c.device_sub_class = 0x02;
        c.device_protocol = 0x01;
        c
    };
    defmt_embassy_usbserial::run(driver, config).await;
}
