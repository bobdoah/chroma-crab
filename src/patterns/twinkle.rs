use super::wheel;
use defmt::*;
use embassy_rp::peripherals::TRNG;
use embassy_rp::trng::Trng;
use embassy_time::Duration;
use smart_leds::RGB8;

const FADE_AMOUNT: u8 = 8;

pub const DURATION: Duration = Duration::from_millis(2);

pub async fn generate(data: &mut [RGB8], trng: &mut Trng<'static, TRNG>) {
    info!("generating twinkle frame");
    let mut random_bytes = [0u8; 10];
    trng.fill_bytes(&mut random_bytes).await;

    // Dim all the LEDs
    for i in 0..data.len() {
        let current_color = data[i];

        data[i] = RGB8 {
            r: current_color.r.saturating_sub(FADE_AMOUNT),
            g: current_color.g.saturating_sub(FADE_AMOUNT),
            b: current_color.b.saturating_sub(FADE_AMOUNT),
        };
    }

    // Pick some random LEDs to Twinkle
    for i in 0..(random_bytes.len() / 2) {
        let led_index = (random_bytes[i * 2] as usize) % data.len();
        data[led_index] = wheel(random_bytes[i * 2 + 1]);
    }
    info!("generated twinkle frame");
}
