use super::wheel;
use embassy_time::Duration;
use smart_leds::RGB8;

pub const DURATION: Duration = Duration::from_millis(10);

pub fn generate(data: &mut [RGB8], counter: &mut u16) {
    // Advance the animation counter
    *counter = counter.wrapping_add(1);
    let j = *counter;

    for i in 0..data.len() {
        let wheel_pos = ((i as u16 * (256 / data.len() as u16) + j) & 255) as u8;
        data[i] = wheel(wheel_pos);
    }
}
