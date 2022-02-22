use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

trait RgbChannels {
    fn r(&self) -> u8;

    fn g(&self) -> u8;

    fn b(&self) -> u8;
}

impl RgbChannels for Rgb {
    fn r(&self) -> u8 {
        self.r
    }

    fn g(&self) -> u8 {
        self.g
    }

    fn b(&self) -> u8 {
        self.b
    }
}

#[allow(unused)]
#[derive(Debug)]
enum RgbHexError {
    Malformed,
    RedOutOfBounds,
    GreenOutOfBounds,
    BlueOutOfBounds,
}

impl FromStr for Rgb {
    type Err = RgbHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        /*
        if let Some(hex) = s.trim().strip_prefix('#') {
            let r = u8::from_str_radix(&hex[0..2], 16);
            let g = u8::from_str_radix(&hex[2..4], 16);
            let b = u8::from_str_radix(&hex[4..6], 16);
            Ok(Rgb {
                r: r.or_else(|_err| Err(RgbHexError::RedOutOfBounds))?,
                g: g.or_else(|_err| Err(RgbHexError::GreenOutOfBounds))?,
                b: b.or_else(|_err| Err(RgbHexError::BlueOutOfBounds))?,
            })
        } else {
            Err(RgbHexError::Malformed)
        }
        */

        if !s.starts_with("#") || !s.len() == 7 {
            return Err(RgbHexError::Malformed);
        }

        let mut colors = Vec::with_capacity(3);
        let mut i: usize = 1;
        while i < 6 {
            let range = i..=i + 1;
            let color = s.get(range).unwrap();

            let color = match u8::from_str_radix(color, 16) {
                Ok(color) => color,
                Err(_) => return Err(RgbHexError::Malformed),
            };

            colors.push(color);

            i += 2;
        }

        Ok(Rgb {
            r: colors[2],
            g: colors[1],
            b: colors[0],
        })
    }
}

impl Display for Rgb {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02x}{:02x}{:02x}", self.r(), self.g(), self.b())
    }
}

fn main() {
    //
}

#[test]
fn every_color() {
    let colors = (0_u8..255).zip(0_u8..255).zip(0_u8..255);

    for ((r, g), b) in colors {
        let hex = format!("#{:02x}{:02x}{:02x}", r, g, b);
        let color: Rgb = hex.parse().unwrap();
        assert_eq!(hex, format!("{}", color));
    }
}

#[test]
#[should_panic]
fn too_short() {
    let _: Rgb = "1234".parse().unwrap();
}

#[test]
#[should_panic]
fn not_a_hex_code() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn invalid_literals() {
    let _: Rgb = "?".parse().unwrap();
}

#[test]
#[should_panic]
fn no_leading_hash() {
    let _: Rgb = "aabbcc".parse().unwrap();
}

#[test]
#[should_panic]
fn out_of_bounds() {
    let _: Rgb = "00gg00".parse().unwrap();
}
