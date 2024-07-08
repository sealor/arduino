#![no_std]
#![no_main]

use arduino_hal::{
    hal::port::PB5,
    port::{mode::Output, Pin, Pins},
};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    sos(pins);
}

pub(crate) fn sos(pins: Pins) -> ! {
    let mut led = pins.d13.into_output();

    loop {
        for _ in 1..=3 {
            blink_short(&mut led);
        }
        for _ in 1..=3 {
            blink_long(&mut led);
        }
        for _ in 1..=3 {
            blink_short(&mut led);
        }

        arduino_hal::delay_ms(2000);
    }
}

const SHORT_MS: u16 = 400;
const LONG_MS: u16 = 1600;

fn blink_long(led: &mut Pin<Output, PB5>) {
    led.set_high();
    arduino_hal::delay_ms(LONG_MS);
    led.set_low();
    arduino_hal::delay_ms(SHORT_MS);
}

fn blink_short(led: &mut Pin<Output, PB5>) {
    led.set_high();
    arduino_hal::delay_ms(SHORT_MS);
    led.set_low();
    arduino_hal::delay_ms(SHORT_MS);
}
