use defmt::{debug, error, info, Debug2Format};
use embassy_rp::peripherals::{PIN_16, PIN_17, PIN_18, PWM_SLICE0, PWM_SLICE1};
use embassy_rp::pwm::{Config, Pwm, PwmError, SetDutyCycle};
use embassy_rp::Peri;
use embassy_time::Timer;
use smart_leds::{colors, RGB8};

pub struct RGBLed<'d> {
    pwm_rg: Pwm<'d>,
    pwm_b: Pwm<'d>,
    top: u16,
    color: RGB8,
}

impl<'d> RGBLed<'d> {
    pub fn new(
        slice0: Peri<'static, PWM_SLICE0>,
        slice1: Peri<'static, PWM_SLICE1>,
        pin_r: Peri<'static, PIN_16>,
        pin_g: Peri<'static, PIN_17>,
        pin_b: Peri<'static, PIN_18>,
    ) -> Self {
        let desired_freq_hz = 1000;
        let clock_freq_hz = embassy_rp::clocks::clk_sys_freq();
        let divider = 16u8;
        let period = (clock_freq_hz / (desired_freq_hz * divider as u32)) as u16 - 1;

        let mut config = Config::default();
        config.top = period;
        config.divider = divider.into();

        let pwm_rg = Pwm::new_output_ab(slice0, pin_r, pin_g, config.clone());
        let pwm_b = Pwm::new_output_a(slice1, pin_b, config.clone());

        let mut rgb_led = RGBLed {
            pwm_rg,
            pwm_b,
            top: period,
            color: colors::BLACK,
        };
        let _ = rgb_led.set_color(rgb_led.color);
        rgb_led
    }

    fn set_color(&mut self, color: RGB8) -> Result<(), PwmError> {
        // Record the color
        info!("Setting color to {}", Debug2Format(&color));
        self.color = color;

        // Get the Red and Green PWM outputs from the RG slice
        let (r_opt, g_opt) = self.pwm_rg.split_by_ref();
        let mut pwm_r = r_opt.unwrap();
        let mut pwm_g = g_opt.unwrap();

        // LED is active low, the duty cycle is inverted
        let r_duty = (((255 - color.r) as u32 * self.top as u32) / 255) as u16;
        let g_duty = (((255 - color.g) as u32 * self.top as u32) / 255) as u16;
        let b_duty = (((255 - color.b) as u32 * self.top as u32) / 255) as u16;
        debug!(
            "r_duty: {}, g_duty: {}, b_duty: {}, top: {}",
            r_duty, g_duty, b_duty, self.top
        );

        pwm_r.set_duty_cycle(r_duty)?;
        pwm_g.set_duty_cycle(g_duty)?;
        self.pwm_b.set_duty_cycle(b_duty)?;

        Ok(())
    }

    fn blend_into(&mut self, target_color: RGB8, amount: u8) -> Result<(), PwmError> {
        let r: u8 = blend_u8(self.color.r, target_color.r, amount);
        let g: u8 = blend_u8(self.color.g, target_color.g, amount);
        let b: u8 = blend_u8(self.color.b, target_color.b, amount);
        info!("Blend color: {{ r: {}, g: {}, b: {} }}", r, g, b);
        self.set_color(RGB8 { r, g, b })
    }
}

fn blend_u8(color_a: u8, color_b: u8, amount_of_b: u8) -> u8 {
    let amount_of_a: u16 = 255 - amount_of_b as u16;
    let blend_a: u32 = (color_a as u16 * amount_of_a) as u32;
    let blend_b: u32 = (color_b as u16 * amount_of_b as u16) as u32;
    ((blend_a + blend_b + 128) >> 8) as u8
}

#[embassy_executor::task]
pub async fn led_task(mut rgb_led: RGBLed<'static>) {
    const BLEND_AMOUNT: u8 = 16;
    info!("Starting LED task");
    loop {
        while rgb_led.color != colors::INDIGO {
            info!(
                "Fading {} into INDIGO: {}",
                Debug2Format(&rgb_led.color),
                Debug2Format(&colors::INDIGO)
            );
            match rgb_led.blend_into(colors::INDIGO, BLEND_AMOUNT) {
                Ok(_) => {}
                Err(e) => {
                    error!(
                        "Failed to blend to INDIGO color (Duty Cycle Error): {:?}",
                        Debug2Format(&e)
                    );
                }
            }
            Timer::after_millis(50).await;
        }
        while rgb_led.color != colors::BLACK {
            info!(
                "Fading {} to BLACK: {}",
                Debug2Format(&rgb_led.color),
                Debug2Format(&colors::BLACK)
            );
            match rgb_led.blend_into(colors::BLACK, BLEND_AMOUNT) {
                Ok(_) => {}
                Err(e) => {
                    error!(
                        "Failed to blend to BLACK color (Duty Cycle Error): {:?}",
                        Debug2Format(&e)
                    );
                }
            }
            Timer::after_millis(50).await;
        }
    }
}
