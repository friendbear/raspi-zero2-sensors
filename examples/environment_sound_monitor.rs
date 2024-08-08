use paho_mqtt as mqtt;
use rppal::gpio::{Gpio, InputPin};
use std::{thread::sleep, time::Duration};

const SOUND_PIN: u8 = 17; // GPIO 17

fn main() {
    let gpio = Gpio::new().expect("Failed to initialize GPIO");
    let sound: InputPin = gpio
        .setup(SOUND_PIN, rppal::gpio::InputPin::default())
        .expect("Failed to set up GPIO");

    let mqtt_client =
        mqtt::Client::new("tcp://broker.hivemq.com:1883").expect("Failed to create MQTT client");
    mqtt_client
        .connect(mqtt::ConnectOptions::default())
        .expect("Failed to connect to MQTT broker");

    loop {
        let sound_level = if sound.is_high() { "Loud" } else { "Quiet" };
        mqtt_client
            .publish(
                "sound/level",
                mqtt::Message::new("sound/level", sound_level, 1),
            )
            .expect("Failed to publish message");
        sleep(Duration::from_secs(1));
    }
}
