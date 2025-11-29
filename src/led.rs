use embassy_rp::peripherals::{PIN_16, PIN_17, PIN_18, PWM_SLICE0, PWM_SLICE1};
use embassy_rp::pwm::{Config, Pwm, SetDutyCycle};
use embassy_rp::Peri;
use embassy_time::Timer;
use smart_leds::RGB8;

pub struct RGBLed<'d> {
    pwm_rg: Pwm<'d>,
    pwm_b: Pwm<'d>,
    top: u16,
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

        RGBLed {
            pwm_rg,
            pwm_b,
            top: period,
        }
    }

    fn set_color(&mut self, color: RGB8) {
        let (r_opt, g_opt) = self.pwm_rg.split_by_ref();

        let mut pwm_r = r_opt.unwrap();
        let mut pwm_g = g_opt.unwrap();

        // LED is active low
        let r_duty = ((255 - color.r) as u16 * self.top) / 255 as u16;
        let g_duty = ((255 - color.g) as u16 * self.top) / 255 as u16;
        let b_duty = ((255 - color.b) as u16 * self.top) / 255 as u16;

        pwm_r.set_duty_cycle(r_duty).unwrap();
        pwm_g.set_duty_cycle(g_duty).unwrap();
        self.pwm_b.set_duty_cycle(b_duty).unwrap();
    }
}

#[embassy_executor::task]
pub async fn led_task(mut rgb_led: RGBLed<'static>) {
    loop {
        rgb_led.set_color(smart_leds::colors::INDIGO);
        Timer::after_millis(500).await;
        rgb_led.set_color(smart_leds::colors::SNOW);
        Timer::after_millis(500).await;
    }
}
