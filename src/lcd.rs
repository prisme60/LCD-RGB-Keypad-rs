#[macro_use]
pub mod commands;
pub mod glyphes;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

pub struct Lcd {
    device_file: File,
    text: String,
    glyphes: Vec<glyphes::Glyphe>,
}

impl Lcd {
    pub fn new(display_path: &str) -> Self {
        Lcd {
            device_file: match File::create(display_path) {
                // The `description` method of `io::Error` returns a string that
                // describes the error
                Err(why) => panic!("couldn't open {}: {}", display_path, why.description()),
                Ok(file) => file,
            },
            text: String::new(),
            glyphes: Vec::new(),
        }
    }

    pub fn append_str(&mut self, text: &str) {
        let s_text = glyphes::convert_msg(text, &mut self.glyphes);
        let g_text = glyphes::generate_glyphes_string(&self.glyphes);
        self.text += g_text.as_str();
        self.text += s_text.as_str();
    }

    pub fn append_raw_str(&mut self, text: &str) {
        self.text += text
    }

    pub fn append(&mut self, character: char) {
        self.text.push(character);
    }

    pub fn clear_glyphes(&mut self) {
        self.glyphes.clear();
    }

    pub fn apply(&mut self) {
        match self.device_file.write(self.text.as_bytes()) {
            Err(why) => panic!(
                "couldn't write to display: {} :  {}",
                why.description(),
                self.text
            ),
            Ok(_) => self.text.clear(),
        }
        if let Err(why) = self.device_file.flush() {
            panic!(
                "couldn't flush to display: {} :  {}",
                why.description(),
                self.text
            )
        }
    }
}
