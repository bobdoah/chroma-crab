#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use {defmt_rtt as _, panic_probe as _};

// Program metadata for `picotool info`.
// This isn't needed, but it's recommended to have these minimal entries.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr 4] = [
    embassy_rp::binary_info::rp_program_name!(c"chroma crab"),
    embassy_rp::binary_info::rp_program_description!(
        c"This runs light patterns on a Pimoroni Plasma 2350 board"
    ),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];

#[embassy_executor::main]
async fn main(_spawner: Spawner) -> ! {
    let periphs = embassy_rp::init(Default::default());
    let mut led = Output::new(p.PIN_16, Level::Low);

    loop {
        defmt::info!("Blink on");
        led.set_high();
        Timer::after(Duration::from_millis(500)).await;

        defmt::info!("Blink off");
        led.set_low();
        Timer::after(Duration::from_millis(500)).await;
    }
}
