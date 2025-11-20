use embassy_time::Duration;
use smart_leds::{brightness, colors::*, gamma, RGB8};

const FADE_AMOUNT: u8 = 1;
pub const DURATION: Duration = Duration::from_millis(8);

const COLOR_PALETTE: &[RGB8] = &[
    // --- Reds & Oranges ---
    RED,
    ORANGE_RED,
    TOMATO,
    DARK_ORANGE,
    // --- Yellows ---
    GOLD,
    YELLOW,
    // --- Greens ---
    CHARTREUSE,
    LIME,
    GREEN,
    // --- Teals & Cyans ---
    TEAL,
    DARK_TURQUOISE,
    CYAN,
    // --- Blues ---
    DEEP_SKY_BLUE,
    MEDIUM_BLUE,
    BLUE,
    // --- Purples ---
    INDIGO,
    DARK_VIOLET,
    DARK_MAGENTA,
    MEDIUM_VIOLET_RED,
    // --- Pinks ---
    FUCHSIA,
    DEEP_PINK,
    HOT_PINK,
    CRIMSON,
];

pub struct FadingState {
    direction: FadingDirection,
    target_color: RGB8,
    palette_index: usize,
    linear_step: u8,
}

impl Default for FadingState {
    fn default() -> Self {
        FadingState {
            direction: FadingDirection::FadingIn,
            target_color: COLOR_PALETTE[0],
            palette_index: 0,
            linear_step: 0,
        }
    }
}

pub enum FadingDirection {
    FadingIn,
    FadingOut,
}

pub fn generate(data: &mut [RGB8], state: &mut FadingState) {
    match state.direction {
        FadingDirection::FadingIn => {
            state.linear_step = state.linear_step.saturating_add(FADE_AMOUNT);
            if state.linear_step == 255 {
                state.direction = FadingDirection::FadingOut;
            }
        }
        FadingDirection::FadingOut => {
            state.linear_step = state.linear_step.saturating_sub(FADE_AMOUNT);
            if state.linear_step == 0 {
                state.palette_index = (state.palette_index + 1) % COLOR_PALETTE.len();
                state.target_color = COLOR_PALETTE[state.palette_index];
                state.direction = FadingDirection::FadingIn;
            }
        }
    }

    let new_colors = gamma(brightness(
        core::iter::repeat(state.target_color).take(data.len()),
        state.linear_step,
    ));

    data.iter_mut().zip(new_colors).for_each(|(pixel, color)| {
        *pixel = color;
    });
}
