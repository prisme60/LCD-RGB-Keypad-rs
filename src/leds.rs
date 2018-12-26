use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use bitflags::bitflags;

const LED_RED: &str = "/sys/class/leds/red/brightness";
const LED_GREEN: &str = "/sys/class/leds/green/brightness";
const LED_BLUE: &str = "/sys/class/leds/blue/brightness";

bitflags! {
    pub struct Leds: u32 {
        const RED = 0b00000001;
        const GREEN = 0b00000010;
        const BLUE = 0b00000100;
        const YELLOW = Self::RED.bits | Self::GREEN.bits;
        const TEAL = Self::GREEN.bits | Self::BLUE.bits;
        const VIOLET = Self::RED.bits | Self::BLUE.bits;
        const WHITE = Self::RED.bits | Self::GREEN.bits | Self::BLUE.bits;
    }
}

pub fn set(leds: Leds) {
    write(LED_RED, leds & Leds::RED != Leds::empty());
    write(LED_GREEN, leds & Leds::GREEN != Leds::empty());
    write(LED_BLUE, leds & Leds::GREEN != Leds::empty());
}

pub fn set_some(leds: Leds, on: bool) {
    if leds & Leds::RED != Leds::empty() {
        write(LED_RED, on);
    }
    if leds & Leds::GREEN != Leds::empty() {
        write(LED_GREEN, on);
    }
    if leds & Leds::GREEN != Leds::empty() {
        write(LED_BLUE, on);
    }
}

fn write(led_path: &str, on: bool) {
    let mut file = match File::create(led_path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", led_path, why.description()),
        Ok(file) => file,
    };
    match file.write(if on { b"1\n" } else { b"0\n" }) {
        Err(why) => panic!("couldn't write {}: {}", led_path, why.description()),
        Ok(_) => {}
    }
}
