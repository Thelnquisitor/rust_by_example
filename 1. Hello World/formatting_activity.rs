use std::fmt::{self, Formatter, Display};

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let r = self.red as u32;
        let g = self.green as u32;
        let b = self.blue as u32;
        
        let rgb_calc = (r*65536) + (g*256) + b;
        write!(f, "RGB ({}, {}, {}) 0x{:X}", self.red, self.green, self.blue, rgb_calc)
    }
}

fn main() {
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ] {
        println!("{}", color);
    }
}
