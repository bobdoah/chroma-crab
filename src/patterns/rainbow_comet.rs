use super::wheel;
use embassy_time::Duration;
use smart_leds::RGB8;

pub const DURATION: Duration = Duration::from_millis(50);

const FADE_AMOUNT: u8 = 32;

pub enum CometDirection {
    Forward,
    Reverse,
}

pub struct CometState {
    direction: CometDirection,
    comet_size: usize,
    comet_pos: usize,
    color_pos: u8,
    color_shift: u8,
}

impl Default for CometState {
    fn default() -> Self {
        let comet_size: usize = 10;
        CometState {
            direction: CometDirection::Forward,
            comet_size: comet_size,
            comet_pos: 1,
            color_pos: 0,
            color_shift: (255 / comet_size).max(1) as u8,
        }
    }
}

pub fn generate(data: &mut [RGB8], state: &mut CometState) {
    match state.comet_pos {
        0 => {
            state.direction = CometDirection::Forward;
        }
        val if val == data.len().saturating_sub(1) => {
            state.direction = CometDirection::Reverse;
        }
        _ => (),
    }

    match state.direction {
        CometDirection::Forward => {
            state.comet_pos = state.comet_pos.saturating_add(1);
        }
        CometDirection::Reverse => {
            state.comet_pos = state.comet_pos.saturating_sub(1);
        }
    }

    for led in data.iter_mut() {
        // Dim by about 25% on each channel (adjust R, G, B values)
        led.r = led.r.saturating_sub(FADE_AMOUNT);
        led.g = led.g.saturating_sub(FADE_AMOUNT);
        led.b = led.b.saturating_sub(FADE_AMOUNT);
    }

    state.color_pos = state.color_pos.wrapping_add(state.color_shift);

    for i in 0..state.comet_size {
        let index = match state.direction {
            CometDirection::Forward => state.comet_pos.saturating_sub(i),
            CometDirection::Reverse => state.comet_pos.saturating_add(i),
        };
        if index < data.len() {
            data[index] = wheel(state.color_pos)
        }
    }
}
