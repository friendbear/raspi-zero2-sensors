use rppal::gpio::{Gpio, OutputPin};
use std::{thread::sleep, time::Duration};

const LED_PIN: u8 = 18; // GPIO 18

fn main() {
    let gpio = Gpio::new().expect("Failed to initialize GPIO");
    let mut led: OutputPin = gpio.setup(LED_PIN, rppal::gpio::OutputPin::default()).expect("Failed to set up GPIO");

    loop {
        led.set_high();
        sleep(Duration::from_millis(100));
        led.set_low();
        sleep(Duration::from_millis(100));
    }
}
