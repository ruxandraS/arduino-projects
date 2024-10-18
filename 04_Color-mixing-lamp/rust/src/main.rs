#![no_std]
#![no_main]

use arduino_hal::simple_pwm::{IntoPwmPin, Prescaler, Timer1Pwm, Timer2Pwm};
use embedded_hal::pwm::SetDutyCycle;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // Input Pins
    let red_sensor = pins.a0.into_analog_input(&mut adc);
    let green_sensor = pins.a1.into_analog_input(&mut adc);
    let blue_sensor = pins.a2.into_analog_input(&mut adc);

    // Output Pins

    // Pulse Width Modulations pins for Arduino Uno
    //  3	Timer2	8-bit timer, PWM on OC2B
    //  5	Timer0	8-bit timer, PWM on OC0B
    //  6	Timer0	8-bit timer, PWM on OC0A
    //  9	Timer1	16-bit timer, PWM on OC1A
    // 10	Timer1	16-bit timer, PWM on OC1B
    // 11	Timer2	8-bit timer, PWM on OC2A

    let mut timer1 = Timer1Pwm::new(dp.TC1, Prescaler::Prescale64);
    let mut timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);

    let mut red_led_pin = pins.d10.into_output().into_pwm(&mut timer1); //10
    let mut green_led_pin = pins.d11.into_output().into_pwm(&mut timer2); // 11
    let mut blue_led_pin = pins.d9.into_output().into_pwm(&mut timer1); //9

    // Definitions
    let mut red_sensor_value: u16 = 0; //10
    let mut green_sensor_value: u16 = 0; //11
    let mut blue_sensor_value: u16 = 0; //9

    // Init Setup
    red_led_pin.set_duty(0);
    green_led_pin.set_duty(0);
    blue_led_pin.set_duty(0);

    red_led_pin.enable();
    green_led_pin.enable();
    blue_led_pin.enable();

    loop {
        red_sensor_value = red_sensor.analog_read(&mut adc);
        arduino_hal::delay_ms(5);

        green_sensor_value = green_sensor.analog_read(&mut adc);
        arduino_hal::delay_ms(5);

        blue_sensor_value = blue_sensor.analog_read(&mut adc);
        arduino_hal::delay_ms(5);

        red_led_pin.set_duty((red_sensor_value / 4).try_into().unwrap());
        green_led_pin.set_duty((green_sensor_value / 4).try_into().unwrap());
        blue_led_pin.set_duty((blue_sensor_value / 4).try_into().unwrap());

        arduino_hal::delay_ms(250);
    }
}
