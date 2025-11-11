use defmt::info;
use embassy_rp::gpio::Input; // Pin trait import is essential here
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex; // Needed for the Sender type
use embassy_sync::channel::Sender;
use embassy_time::{Duration, Timer};

#[embassy_executor::task]
pub async fn button_task(
    sender: Sender<'static, CriticalSectionRawMutex, (), 1>,
    input: Input<'static>,
) {
    info!("Button Task started on PIN_2");

    // GPIO 2 is set up as an input with an internal pull-up resistor.
    // The button should connect PIN_2 to Ground (GND).
    let mut button = input;

    loop {
        // 1. Wait efficiently for the button to be pressed (falling edge: High -> Low)
        button.wait_for_falling_edge().await;

        // 2. Software debounce: Wait 50ms to filter out switch bounce
        Timer::after(Duration::from_millis(50)).await;

        // 3. Check the state again after debounce
        if button.is_low() {
            info!("Button pressed!");

            // 4. Send the signal to the main task to change the pattern
            // try_send is used as we only care if the signal gets through eventually.
            let _ = sender.try_send(());

            // 5. Wait for the button to be released before looking for the next press
            button.wait_for_rising_edge().await;
            info!("Button released. Ready for next press.");
        }
    }
}
