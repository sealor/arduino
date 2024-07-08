use core::{ptr::slice_from_raw_parts, usize};

use ufmt::uWrite;

#[derive(Default)]
pub struct String {
    text: [u8; 32],
    pos: usize,
}

impl uWrite for String {
    type Error = ();

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        for c in s.chars() {
            if self.pos >= self.text.len() {
                break;
            }
            self.text[self.pos] = c as u8;
            self.pos += 1;
        }

        Ok(())
    }
}

impl String {
    pub fn to_str(&self) -> &str {
        let a = slice_from_raw_parts(&self.text, self.pos);
        unsafe { &*(a as *const str) }
    }
}
