macro_rules! escape {
    () => {
        "\x1b[L"
    };
}

pub const DISPLAY_ON: &str = concat!(escape!(), "D");
pub const DISPLAY_OFF: &str = concat!(escape!(), "d");
pub const CURSOR_ON: &str = concat!(escape!(), "C");
pub const CURSOR_OFF: &str = concat!(escape!(), "c");
pub const BLINK_ON: &str = concat!(escape!(), "B");
pub const BLINK_OFF: &str = concat!(escape!(), "b");
pub const BACKLIGHT_ON: &str = concat!(escape!(), "+");
pub const BACKLIGHT_OFF: &str = concat!(escape!(), "-");
pub const BACKLIGHT_FLASH: &str = concat!(escape!(), "*");
pub const SMALLFONT: &str = concat!(escape!(), "f");
pub const LARGEFONT: &str = concat!(escape!(), "F");
pub const ONELINE: &str = concat!(escape!(), "n");
pub const TWOLINES: &str = concat!(escape!(), "N");
pub const SHIFT_CURSOR_LEFT: &str = concat!(escape!(), "l");
pub const SHIFT_CURSOR_RIGHT: &str = concat!(escape!(), "r");
pub const SHIFT_DISPLAY_LEFT: &str = concat!(escape!(), "L");
pub const SHIFT_DISPLAY_RIGHT: &str = concat!(escape!(), "R");
pub const KILL_END_OF_LINE: &str = concat!(escape!(), "k");
pub const REINITIALISE: &str = concat!(escape!(), "I");

pub fn goto_x(x: usize) -> String {
    format!(concat!(escape!(), "x{};"), x)
}

pub fn goto_y(y: usize) -> String {
    format!(concat!(escape!(), "y{};"), y)
}

pub fn goto_xy(x: usize, y: usize) -> String {
    format!(concat!(escape!(), "x{}y{};"), x, y)
}

pub const BEGIN_OF_LINE: char = '\r';
pub const NEXT_LINE: char = '\n';
pub const CLEAR_DISPLAY: char = '\x0c'; // is the '\f' of the C/C++ lang
