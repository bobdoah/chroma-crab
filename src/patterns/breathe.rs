use super::wheel;
use embassy_time::Duration;
use smart_leds::{colors::BLACK, RGB8};

const FADE_AMOUNT: u8 = 8;

pub const DURATION: Duration = Duration::from_millis(10);

pub struct FadingState {
    direction: FadingDirection,
    target: RGB8,
    wheel_pos: u8,
}

impl Default for FadingState {
    fn default() -> Self {
        let initial_wheel_pos = 0; // Start at the first color (position 0)
        FadingState {
            direction: FadingDirection::FadingIn,
            target: wheel(initial_wheel_pos),
            wheel_pos: initial_wheel_pos,
        }
    }
}

pub enum FadingDirection {
    FadingIn,
    FadingOut,
}

pub fn generate(data: &mut [RGB8], state: &mut FadingState) {
    // All pixels are the same color
    let current_color = data[0];
    let new_color: RGB8;

    match state.direction {
        FadingDirection::FadingIn => {
            new_color = RGB8 {
                r: current_color
                    .r
                    .saturating_add(FADE_AMOUNT)
                    .min(state.target.r),
                g: current_color
                    .g
                    .saturating_add(FADE_AMOUNT)
                    .min(state.target.g),
                b: current_color
                    .b
                    .saturating_add(FADE_AMOUNT)
                    .min(state.target.b),
            };
            if new_color == state.target {
                *state = FadingState {
                    direction: FadingDirection::FadingOut,
                    target: BLACK,
                    wheel_pos: state.wheel_pos,
                }
            }
        }
        FadingDirection::FadingOut => {
            new_color = RGB8 {
                r: current_color.r.saturating_sub(FADE_AMOUNT),
                g: current_color.g.saturating_sub(FADE_AMOUNT),
                b: current_color.b.saturating_sub(FADE_AMOUNT),
            };
            if new_color == BLACK {
                let wheel_pos = state.wheel_pos.wrapping_add(1);
                *state = FadingState {
                    direction: FadingDirection::FadingIn,
                    target: wheel(wheel_pos),
                    wheel_pos: wheel_pos,
                }
            }
        }
    }
    for pixel in data.iter_mut() {
        *pixel = new_color;
    }
}
