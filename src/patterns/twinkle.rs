use super::wheel;
use embassy_rp::peripherals::TRNG;
use embassy_rp::trng::Trng;
use smart_leds::RGB8;

const TWINKLE_THRESHOLD: u8 = 4;
const FADE_AMOUNT: u8 = 8;

pub struct TwinkleState<'a> {
    pub trng: Trng<'a, TRNG>,
    pub randomness: &'a mut [u8],
    pub rand_index: usize,
}

impl<'a> TwinkleState<'a> {
    /// Refills the random buffer with fresh TRNG data.
    pub async fn replenish_randomness(&mut self) {
        self.trng.fill_bytes(self.randomness).await;
        self.rand_index = 0;
    }
}

pub async fn generate(data: &mut [RGB8], state: &mut TwinkleState<'_>) {
    state.replenish_randomness().await;
    let randomness = &state.randomness;

    for i in 0..data.len() {
        let current_color = data[i];

        // Dim the LED
        data[i] = RGB8 {
            r: current_color.r.saturating_sub(FADE_AMOUNT),
            g: current_color.g.saturating_sub(FADE_AMOUNT),
            b: current_color.b.saturating_sub(FADE_AMOUNT),
        };

        let twinkle_check = randomness[state.rand_index];
        state.rand_index += 1;

        // Or Twinkle
        if twinkle_check < (256 / TWINKLE_THRESHOLD as u16) as u8 {
            let random_wheel_pos = randomness[state.rand_index];
            state.rand_index += 1;

            data[i] = wheel(random_wheel_pos);
        }
    }
}
