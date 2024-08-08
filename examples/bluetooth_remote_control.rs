use rppal::gpio::{Gpio, OutputPin};
use serialport::prelude::*;
use std::{thread::sleep, time::Duration};

const LED_PIN: u8 = 18; // GPIO 18

fn main() {
    let gpio = Gpio::new().expect("Failed to initialize GPIO");
    let mut led: OutputPin = gpio
        .setup(LED_PIN, rppal::gpio::OutputPin::default())
        .expect("Failed to set up GPIO");

    let mut port = SerialPort::new("/dev/ttyS0", 9600).expect("Failed to open serial port");
    port.set_timeout(Duration::from_secs(1))
        .expect("Failed to set timeout");

    loop {
        let mut buffer: [u8; 1] = [0];
        if port.read(&mut buffer).is_ok() {
            if buffer[0] == b'1' {
                led.set_high();
            } else {
                led.set_low();
            }
        }
        sleep(Duration::from_millis(100));
    }
}
