#![no_std]
#![no_main]

mod string;

use arduino_hal::{
    hal::port::{PB0, PB1, PD4, PD5, PD6, PD7},
    port::{mode::Output, Pin},
    Delay,
};
use hd44780_driver::{bus::FourBitBus, Cursor, CursorBlink, Display, DisplayMode, HD44780};
use panic_halt as _;
use string::String;
use ufmt::uwrite;

type MyDisplay = HD44780<
    FourBitBus<
        Pin<Output, PB0>,
        Pin<Output, PB1>,
        Pin<Output, PD4>,
        Pin<Output, PD5>,
        Pin<Output, PD6>,
        Pin<Output, PD7>,
    >,
>;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    let a0 = pins.a0.into_analog_input(&mut adc);

    let mut delay = Delay::new();

    let mut lcd: MyDisplay = HD44780::new_4bit(
        pins.d8.into_output(),
        pins.d9.into_output(),
        //
        pins.d4.into_output(),
        pins.d5.into_output(),
        pins.d6.into_output(),
        pins.d7.into_output(),
        &mut delay,
    )
    .unwrap();

    // Unshift display and set cursor to 0
    lcd.reset(&mut delay).ok();

    // Clear existing characters
    lcd.clear(&mut delay).ok();

    // Display the following string
    write_slow_typing_text(&mut lcd, &mut delay, "Hallo Timo!");

    lcd.set_cursor_pos(40, &mut delay).ok();
    write_slow_typing_text(&mut lcd, &mut delay, "Voltage: ");

    lcd.set_display_mode(
        DisplayMode {
            cursor_visibility: Cursor::Invisible,
            cursor_blink: CursorBlink::Off,
            display: Display::On,
        },
        &mut delay,
    )
    .ok();

    loop {
        arduino_hal::delay_ms(100);
        //// Move the cursor to the second line
        lcd.set_cursor_pos(40, &mut delay).ok();

        lcd.write_str("Voltage: ", &mut delay).ok();
        let voltage = a0.analog_read(&mut adc);
        let mut text = String::default();
        uwrite!(&mut text, "{}U     ", voltage).ok();
        lcd.write_str(text.to_str(), &mut delay).ok();
    }
}

fn write_slow_typing_text(lcd: &mut MyDisplay, delay: &mut Delay, text: &str) {
    for char in text.chars() {
        lcd.write_char(char, delay).ok();
        arduino_hal::delay_ms(150);
    }
}
