use rppal::gpio::{Gpio, InputPin};
use serialport::prelude::*;
use std::{thread::sleep, time::Duration};

const SOUND_PIN: u8 = 17; // GPIO 17

fn main() {
    let gpio = Gpio::new().expect("Failed to initialize GPIO");
    let sound: InputPin = gpio
        .setup(SOUND_PIN, rppal::gpio::InputPin::default())
        .expect("Failed to set up GPIO");

    let mut port = SerialPort::new("/dev/ttyS0", 9600).expect("Failed to open serial port");
    port.set_timeout(Duration::from_secs(1))
        .expect("Failed to set timeout");

    loop {
        let sound_level = if sound.is_high() { "Loud" } else { "Quiet" };
        port.write(sound_level.as_bytes())
            .expect("Failed to write to serial port");
        sleep(Duration::from_secs(1));
    }
}
