#![no_std]
#![no_main]
#![feature(asm_experimental_arch)]

use core::arch::asm;

use arduino_hal::{
    delay_us,
    hal::port::PB0,
    port::{mode::Output, Pin},
};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let pin = pins.d8.into_output();

    let mut strip = LedStrip::new(pin);

    loop {
        strip.reset();

        strip.send_led(16, 0, 0, 0);
        strip.send_led(0, 16, 0, 0);
        strip.send_led(0, 0, 16, 0);
        strip.send_led(0, 0, 0, 16);

        for _ in 0..60 - 4 {
            strip.send_led(0, 0, 0, 0);
        }

        arduino_hal::delay_ms(1000);
    }
}

/// SK6812 RGBW LED strip for 16Mhz Arduino Uno ATmega328P
///
/// see: https://cdn-shop.adafruit.com/product-files/2757/p2757_SK6812RGBW_REV01.pdf
/// see: https://gingerlabs.de/uploads/2022-05-29-addressable-led-comparison/SK6812_RGBW.pdf
struct LedStrip {
    pin: Pin<Output, PB0>,
}

impl LedStrip {
    pub fn new(pin: Pin<Output, PB0>) -> Self {
        Self { pin }
    }

    pub fn reset(&mut self) {
        self.pin.set_low();
        delay_us(81);
    }

    pub fn send_led(&mut self, r: u8, g: u8, b: u8, w: u8) {
        self.send_byte(g);
        self.send_byte(r);
        self.send_byte(b);
        self.send_byte(w);
    }

    #[inline(always)]
    fn send_byte(&mut self, byte: u8) {
        for i in 0..8 {
            let bit = byte << i & 128u8;
            if bit > 0u8 {
                self.tx_1();
            } else {
                self.tx_0();
            }
        }
    }

    #[inline(always)]
    fn tx_0(&mut self) {
        self.pin.set_high();
        unsafe {
            asm!("nop");
        }
        self.pin.set_low();
        unsafe {
            asm!("nop");
        }
    }

    #[inline(always)]
    fn tx_1(&mut self) {
        self.pin.set_high();
        unsafe {
            asm!("nop", "nop", "nop", "nop", "nop");
        }
        self.pin.set_low();
        unsafe {
            asm!("nop");
        }
    }
}
