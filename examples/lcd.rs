use lcd_rgb_keypad::lcd::{commands, Lcd};
use std::{thread::sleep, time::Duration};

fn main() {
    println!("Enter LCD test :");

    let mut l = Lcd::new("/dev/lcd");
    l.append_raw_str(commands::REINITIALISE);
    l.append_raw_str(commands::DISPLAY_ON);
    l.append_raw_str(commands::BACKLIGHT_ON);
    l.append_str("Curseur ON\nCursor ON\n");
    l.apply();
    sleep(Duration::from_millis(500));

    // Warning Cursor seems not disabled. Kernel issue ?
    l.append_raw_str(commands::CURSOR_OFF);
    l.append_str("Curseur OFF\nCursor OFF\n");
    l.apply();
    sleep(Duration::from_millis(2000));

    l.append_str("Première ligne\nSecond line\n"); //Look the stress 'è' it will be automatically replaced by a special character
    l.apply();
    sleep(Duration::from_secs(1));

    l.append_str("Lumière éteinte\nBacklight OFF\n");
    l.append_raw_str(commands::BACKLIGHT_OFF);
    l.apply();
    sleep(Duration::from_secs(1));

    l.append_raw_str(commands::BACKLIGHT_ON);
    l.append_str("Ceci est une longue phrase\nThis is a long sentence.\n");
    l.apply();
    sleep(Duration::from_millis(500));

    // BACKLIGHT_FLASH is not working on my device (not implemented in the kernel?)
    l.append_raw_str(commands::BACKLIGHT_FLASH);
    for _i in 0..24 {
        l.append_raw_str(commands::SHIFT_DISPLAY_LEFT);
        l.apply();
        sleep(Duration::from_millis(500));
    }
    // Be careful, backlight command is immediate (GPIO on expander), but others commands (data sequence send on GPIO expander to LCD chipset) are a little bit delayed
    sleep(Duration::from_secs(1));

    // Cleaning before exit
    l.append(commands::CLEAR_DISPLAY);
    l.append_raw_str(commands::DISPLAY_OFF);
    l.append_raw_str(commands::BACKLIGHT_OFF);
    l.apply();

    println!("Exit LCD test.");
}
