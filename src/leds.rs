use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

use bitflags::bitflags;

const LED_RED: &str = "/sys/class/leds/red/brightness";
const LED_GREEN: &str = "/sys/class/leds/green/brightness";
const LED_BLUE: &str = "/sys/class/leds/blue/brightness";

bitflags! {
    pub struct Leds: u32 {
        const RED = 0b0000_0001;
        const GREEN = 0b0000_0010;
        const BLUE = 0b0000_0100;
        const YELLOW = Self::RED.bits | Self::GREEN.bits;
        const TEAL = Self::GREEN.bits | Self::BLUE.bits;
        const VIOLET = Self::RED.bits | Self::BLUE.bits;
        const WHITE = Self::RED.bits | Self::GREEN.bits | Self::BLUE.bits;
    }
}

pub fn set(leds: Leds) {
    write(LED_RED, leds.contains(Leds::RED));
    write(LED_GREEN, leds.contains(Leds::GREEN));
    write(LED_BLUE, leds.contains(Leds::BLUE));
}

pub fn set_some(leds: Leds, on: bool) {
    if leds.contains(Leds::RED) {
        write(LED_RED, on);
    }
    if leds.contains(Leds::GREEN) {
        write(LED_GREEN, on);
    }
    if leds.contains(Leds::BLUE) {
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
    if let Err(why) = file.write(if on { b"1\n" } else { b"0\n" }) {
        panic!("couldn't write {}: {}", led_path, why.description())
    }
}
