use lazy_static::lazy_static;
use std::collections::HashMap;

type SpriteType = [u8; 8];

#[derive(Clone)]
pub struct Glyphe {
    original: char,
    replacement: char,
    sprite: Option<SpriteType>,
}

impl Glyphe {
    pub fn new(original: char, replacement: char, sprite: SpriteType) -> Self {
        Glyphe {
            original,
            replacement,
            sprite: Some(sprite),
        }
    }

    pub fn new_empty(original: char, replacement: char) -> Self {
        Glyphe {
            original,
            replacement,
            sprite: None,
        }
    }

    // printf "\e[LG0010101050D1F0C04;"  => 0 = [enter]
    // printf "\e[LG0 01 01 01 05 0D 1F 0C 04;"  => 0 = [enter]
    pub fn generate_glyph_code(&self, index: usize) -> String {
        match self.sprite {
            Some(sprite) => format!(
                "\x1b[LG{}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x};",
                index,
                sprite[0],
                sprite[1],
                sprite[2],
                sprite[3],
                sprite[4],
                sprite[5],
                sprite[6],
                sprite[7]
            ),
            None => String::new(),
        }
    }
}

static MAX_CUSTOM_CHAR: usize = 8;

lazy_static! {
    static ref GLYPHES: HashMap<char, Glyphe> = {
        let mut m = HashMap::new();
        m.insert('█', Glyphe::new('█', '#', [0x1f,0x1f,0x1f,0x1f,0x1f,0x1f,0x1f,0x1f])); // black square
        m.insert('☰', Glyphe::new('☰', 'E', [0x00,0x1f,0x00,0x1f,0x00,0x1f,0x00,0x00])); // 3 horizontal lines
        m.insert('♪', Glyphe::new('♪', 'd', [
                    0b00010, //music note
                    0b00011,
                    0b00010,
                    0b00010,
                    0b01110,
                    0b11110,
                    0b01100,
                    0b00000]));
        m.insert('🔔', Glyphe::new('🔔', 'O', [
                    0b00100, //bell
                    0b01110,
                    0b01110,
                    0b01110,
                    0b11111,
                    0b00000,
                    0b00100,
                    0b00000]));
        m.insert('⌛', Glyphe::new('⌛', 'X', [
                    0b11111, //hourglassFull
                    0b11111,
                    0b01110,
                    0b00100,
                    0b01010,
                    0b10001,
                    0b11111,
                    0b00000]));
        m.insert('⏳', Glyphe::new('⏳', 'x', [
                    0b11111, //hourglassMid
                    0b10001,
                    0b01110,
                    0b00100,
                    0b01010,
                    0b11111,
                    0b11111,
                    0b00000]));
        m.insert('é', Glyphe::new('é', 'e', [
                    0b00100,
                    0b01000,
                    0b01110,
                    0b10001,
                    0b11111,
                    0b10000,
                    0b01111,
                    0b00000]));
        m.insert('è', Glyphe::new('è', 'e', [
                    0b01000,
                    0b00100,
                    0b01110,
                    0b10001,
                    0b11111,
                    0b10000,
                    0b01111,
                    0b00000]));
        m.insert('ê', Glyphe::new('ê', 'e', [
                    0b00100,
                    0b01010,
                    0b01110,
                    0b10001,
                    0b11111,
                    0b10000,
                    0b01111,
                    0b00000]));
        m.insert('ë', Glyphe::new('ë', 'e', [
                    0b01010,
                    0b00000,
                    0b01110,
                    0b10001,
                    0b11111,
                    0b10000,
                    0b01111,
                    0b00000]));
        m.insert('à',  Glyphe::new('à', 'a', [
                    0b01000,
                    0b00100,
                    0b01110,
                    0b00001,
                    0b01111,
                    0b10001,
                    0b01111,
                    0b00000]));
        m.insert('ù', Glyphe::new('ù', 'u', [
                    0b01000,
                    0b00100,
                    0b10001,
                    0b10001,
                    0b10001,
                    0b10001,
                    0b01111,
                    0b00000]));
        m.insert('ô', Glyphe::new('ô', 'o', [
                    0b00100,
                    0b01010,
                    0b01110,
                    0b10001,
                    0b10001,
                    0b10001,
                    0b01110,
                    0b00000]));
        m
    };
}

// return message and update glyph list
pub fn convert_msg(message: &str, mut glyph_list: Vec<Glyphe>) -> String {
    let mut new_msg = String::new();

    for c in message.chars() {
        new_msg.push(match glyph_list.iter().position(|g| g.original == c) {
            // Glyph has already been added to the list, so use it!
            Some(g_pos) => g_pos as u8 as char,
            // Glyph not in the list!
            None => match GLYPHES.get(&c) {
                // Glyph is in the map
                Some(g) => {
                    if glyph_list.len() < MAX_CUSTOM_CHAR {
                        // Free place in the list!
                        glyph_list.push((*g).clone());
                        (glyph_list.len() - 1) as u8 as char
                    } else {
                        // No free place in the list! So use replacement character instead having problem to display original character.
                        g.replacement
                    }
                }
                // Glyph not in the map, used the character directly
                None => c,
            },
        });
    }
    new_msg
}

pub fn generate_glyphes_string(glyph_list: Vec<Glyphe>) -> String {
    let mut glyph_string = String::new();

    for (i, g) in glyph_list.iter().enumerate() {
        glyph_string.push_str(&g.generate_glyph_code(i));
    }

    glyph_string
}
