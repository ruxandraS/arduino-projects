#![no_std]
#![no_main]

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // Input Pins
    let temperature_sensor = pins.a0.into_analog_input(&mut adc);

    // Output Pins
    let mut cold_led = pins.d3.into_output();
    let mut warm_led = pins.d4.into_output();
    let mut hot_led = pins.d5.into_output();

    // Definitions
    let mut sensor_value: u16;
    let mut voltage_value: f32;
    let mut temperature: f32;

    let temperature_baseline = 20.0;

    // Init setup
    cold_led.set_low();
    warm_led.set_low();
    hot_led.set_low();

    loop {
        sensor_value = temperature_sensor.analog_read(&mut adc);
        voltage_value = (sensor_value as f32 / 1024.0) * 5.0;
        temperature = (voltage_value - 0.5) * 100.0;

        match ufmt::uwriteln!(
            &mut serial,
            "Sensor val: {}, Voltage: {}, Temperature: {} deg C\r",
            (sensor_value * 100) as i32,
            (voltage_value * 100.0) as i32,
            (temperature * 100.0) as i32
        ) {
            Ok(_) => {}
            Err(_) => {
                cold_led.set_high();
                warm_led.set_high();
                hot_led.set_high();
            }
        }

        if temperature < temperature_baseline {
            cold_led.set_low();
            warm_led.set_low();
            hot_led.set_low();
        } else if (temperature_baseline..temperature_baseline + 2.0).contains(&temperature) {
            cold_led.set_high();
            warm_led.set_low();
            hot_led.set_low();
        } else if (temperature_baseline + 2.0..temperature_baseline + 4.0).contains(&temperature) {
            cold_led.set_low();
            warm_led.set_high();
            hot_led.set_low();
        } else {
            cold_led.set_low();
            warm_led.set_low();
            hot_led.set_high();
        }

        arduino_hal::delay_ms(1000);
    }
}
