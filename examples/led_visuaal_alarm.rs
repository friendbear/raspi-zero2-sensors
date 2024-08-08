use rppal::gpio::{Gpio, InputPin, OutputPin};
use std::{thread::sleep, time::Duration};

const HALL_SENSOR_PIN: u8 = 22; // GPIO 22
const LED_PIN: u8 = 18; // GPIO 18

fn main() {
    let gpio = Gpio::new().expect("Failed to initialize GPIO");
    let hall_sensor: InputPin = gpio
        .setup(HALL_SENSOR_PIN, rppal::gpio::InputPin::default())
        .expect("Failed to set up GPIO");
    let mut led: OutputPin = gpio
        .setup(LED_PIN, rppal::gpio::OutputPin::default())
        .expect("Failed to set up GPIO");

    loop {
        if hall_sensor.is_high() {
            led.set_high();
        } else {
            led.set_low();
        }
        sleep(Duration::from_millis(100));
    }
}
