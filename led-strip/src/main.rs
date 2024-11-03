#![no_std]
#![no_main]

use core::ops::Shl;

use arduino_hal::{
    hal::port::PB0,
    hal::port::PD7,
    port::{mode::Output, Pin},
    Delay,
};
use embedded_hal::delay::DelayNs;
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let pin = pins.d8.into_output();
    let fake_pin = pins.d7.into_output();
    let delay = Delay::new();

    let mut strip = LedStrip::new(pin, fake_pin, delay);

    loop {
        strip.reset();

        strip.send_led(16, 0, 0, 0);
        strip.send_led(0, 16, 0, 0);
        strip.send_led(0, 0, 16, 0);
        strip.send_led(0, 0, 0, 16);
        strip.send_led(0, 0, 0, 0);
        strip.send_led(0, 0, 0, 0);

        arduino_hal::delay_ms(1000);
    }
}

const FACTOR: u32 = 2;

struct LedStrip {
    pin: Pin<Output, PB0>,
    fake_pin: Pin<Output, PD7>,
    delay: Delay,
}

impl LedStrip {
    pub fn new(pin: Pin<Output, PB0>, fake_pin: Pin<Output, PD7>, delay: Delay) -> Self {
        Self {
            pin,
            fake_pin,
            delay,
        }
    }

    pub fn reset(&mut self) {
        self.pin.set_low();
        self.delay.delay_us(90)
    }

    pub fn tx_0(&mut self) {
        self.pin.set_high();
        for _ in 0..FACTOR {
            self.fake_pin.toggle();
        }
        self.pin.set_low();
        for _ in 0..3 * FACTOR {
            self.fake_pin.toggle();
        }
    }

    pub fn tx_1(&mut self) {
        self.pin.set_high();
        for _ in 0..2 * FACTOR {
            self.fake_pin.toggle();
        }
        self.pin.set_low();
        for _ in 0..2 * FACTOR {
            self.fake_pin.toggle();
        }
    }

    pub fn send_led(&mut self, r: u8, g: u8, b: u8, w: u8) {
        self.send_byte(g);
        self.send_byte(r);
        self.send_byte(b);
        self.send_byte(w);
    }

    pub fn send_byte(&mut self, byte: u8) {
        for i in 0..8 {
            let bit = byte.shl(i) & 128u8;
            if bit > 0u8 {
                self.tx_1();
            } else {
                self.tx_0();
            }
        }
    }
}
