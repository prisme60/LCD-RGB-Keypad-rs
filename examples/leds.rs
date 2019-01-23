use lcd_rgb_keypad::leds::{set, Leds};
use std::{thread::sleep, time::Duration};

fn main() {
    println!("BEGIN LEDS RGB SEQUENCE :");

    // Test leds
    set(Leds::RED);
    sleep(Duration::from_secs(1));
    set(Leds::GREEN);
    sleep(Duration::from_secs(1));
    set(Leds::BLUE);
    sleep(Duration::from_secs(1));
    set(Leds::WHITE);
    sleep(Duration::from_secs(1));
    set(Leds::VIOLET);
    sleep(Duration::from_secs(1));
    set(Leds::YELLOW);
    sleep(Duration::from_secs(1));
    set(Leds::TEAL);
    sleep(Duration::from_secs(1));
    set(Leds::empty());

    println!("END LEDS RGB SEQUENCE (LEDS OFF)");
}
