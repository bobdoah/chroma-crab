use embassy_time::{Duration, Instant};
use smart_leds::{colors::*, RGB8};

struct ColorPair(RGB8, RGB8);

const COLOR_PALETTE: &[ColorPair] = &[
    ColorPair(RED, GREEN),
    ColorPair(BLUE, RED),
    ColorPair(DARK_VIOLET, BLUE),
    ColorPair(GREEN, DARK_VIOLET),
    ColorPair(ORANGE, GREEN),
    ColorPair(BLUE, ORANGE),
    ColorPair(GREEN, BLUE),
];

pub const DURATION: Duration = Duration::from_millis(400);
pub const COLOR_CYCLE_DURATION: Duration = Duration::from_secs(30);

pub enum AlternatingOrder {
    Forward,
    Reverse,
}

pub struct AlternatingState {
    palette_index: usize,
    order: AlternatingOrder,
    last_cycle_time: Instant,
}

impl Default for AlternatingState {
    fn default() -> Self {
        AlternatingState {
            palette_index: 0,
            order: AlternatingOrder::Forward,
            last_cycle_time: Instant::now(),
        }
    }
}

pub fn generate(data: &mut [RGB8], state: &mut AlternatingState) {
    if state.last_cycle_time.elapsed() >= COLOR_CYCLE_DURATION {
        state.palette_index = (state.palette_index + 1) % COLOR_PALETTE.len();
        state.last_cycle_time = Instant::now();
        defmt::info!("alternating color: next pattern");
    }

    let (even_pixels, odd_pixels): (RGB8, RGB8);

    match state.order {
        AlternatingOrder::Forward => {
            ColorPair(odd_pixels, even_pixels) = COLOR_PALETTE[state.palette_index];
            state.order = AlternatingOrder::Reverse;
        }
        AlternatingOrder::Reverse => {
            ColorPair(even_pixels, odd_pixels) = COLOR_PALETTE[state.palette_index];
            state.order = AlternatingOrder::Forward;
        }
    }

    for pixel in data.iter_mut().step_by(2) {
        *pixel = odd_pixels;
    }

    for pixel in data.iter_mut().skip(1).step_by(2) {
        *pixel = even_pixels;
    }
}
