#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    // Input Pins
    let engage_button = pins.d2.into_floating_input();

    // Output Pins
    let mut ready_led = pins.d3.into_output();
    let mut standby_led_1 = pins.d4.into_output();
    let mut standby_led_2 = pins.d5.into_output();

    loop {
        // Button is not pressed
        if engage_button.is_low() {
            ready_led.set_high();
            standby_led_1.set_low();
            standby_led_2.set_low();
        }
        else {
            // Button is not pressed
            ready_led.set_low();
            standby_led_1.set_low();
            standby_led_2.set_high();

            arduino_hal::delay_ms(250);

            ready_led.set_low();
            standby_led_1.set_high();
            standby_led_2.set_low();

            arduino_hal::delay_ms(250);
        }
    }
}
