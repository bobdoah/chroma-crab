use embassy_rp::peripherals::{PIN_16, PIN_17, PIN_18, PWM_SLICE0, PWM_SLICE1};
use embassy_rp::pwm::{Config, Pwm};
use embassy_rp::Peri;

pub struct RGBLed<'d> {
    pwm_rg: Pwm<'d>,
    pwm_b: Pwm<'d>,
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

        RGBLed { pwm_rg, pwm_b }
    }
}

#[embassy_executor::task]
pub async fn led_task(rgb_led: RGBLed<'static>) {}
