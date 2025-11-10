#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_rp::bind_interrupts;
use embassy_rp::peripherals::{PIO0, TRNG};
use embassy_rp::pio::{InterruptHandler, Pio};
use embassy_rp::pio_programs::ws2812::{PioWs2812, PioWs2812Program, Rgb};
use embassy_time::{Duration, Ticker};
//use patterns::rainbow;
use patterns::twinkle;
use patterns::twinkle::TwinkleState;
use smart_leds::RGB8;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    PIO0_IRQ_0 => InterruptHandler<PIO0>;
    TRNG_IRQ => embassy_rp::trng::InterruptHandler<TRNG>;
});

mod patterns;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
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

    let mut randomness = [0u8; 2 * NUM_LEDS];
    let trng = embassy_rp::trng::Trng::new(p.TRNG, Irqs, Default::default());
    let mut twinkle_state = TwinkleState {
        trng,
        randomness: &mut randomness,
        rand_index: 0,
    };

    let mut ticker = Ticker::every(Duration::from_millis(100));
    //   let mut counter: u16 = 0;
    loop {
        twinkle::generate(&mut data, &mut twinkle_state).await;
        //        rainbow::generate(&mut data, &mut counter);
        ws2812.write(&data).await;
        ticker.next().await;
    }
}
