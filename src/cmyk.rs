//! Represents colors in the CMYK color space. u16 is used to represent each component internally.
//! Note that this results in potentially lossy conversions to RGB-255 color space.

use crate::rgb::RGB;
use crate::Color;
use std::fmt::Display;

/// A color in the CMYK color space, with u16 representing each component internally.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CMYK {
    /// Cyan component
    pub c: u16,
    /// Magenta component
    pub m: u16,
    /// Yellow component
    pub y: u16,
    /// Key (black) component
    pub k: u16,
}

impl Color for CMYK {
    fn as_rgb(&self) -> RGB {
        let [c, m, y, k] = self.as_parts();
        let r = 255.0 * (1.0 - c) * (1.0 - k);
        let g = 255.0 * (1.0 - m) * (1.0 - k);
        let b = 255.0 * (1.0 - y) * (1.0 - k);
        let r = r.round() as u8;
        let g = g.round() as u8;
        let b = b.round() as u8;
        RGB { r, g, b }
    }

    fn from_rgb(c: RGB) -> Self {
        if c.is_black() {
            return CMYK::BLACK;
        }

        let r_p = c.r as f64 / 255.0;
        let g_p = c.g as f64 / 255.0;
        let b_p = c.b as f64 / 255.0;
        let k = 1.0 - (*[c.r, c.g, c.b].iter().max().unwrap() as f64 / 255.0);
        let c = (1.0 - r_p - k) / (1.0 - k);
        let m = (1.0 - g_p - k) / (1.0 - k);
        let y = (1.0 - b_p - k) / (1.0 - k);
        CMYK::new(c, m, y, k)
    }
}

impl Display for CMYK {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "({:.2},{:.2},{:.2},{:.2})",
               CMYK::conv_to_float(self.c),
               CMYK::conv_to_float(self.m),
               CMYK::conv_to_float(self.y),
               CMYK::conv_to_float(self.k)
        )
    }
}

impl CMYK {

    /// The color black, as CMYK
    pub const BLACK: CMYK = CMYK::from_parts([0, 0, 0, std::u16::MAX]);

    /// The color white, as CMYK
    pub const WHITE: CMYK = CMYK::from_parts([0, 0, 0, 0]);

    /// Creates a CMYK object from the traditional floating point representation, i.e. (0, 0.23, 0.4, 1.0)
    /// # Panics
    /// c, m, y, and k must all be in the range `[0.0, 1.0]`
    pub fn new(c: f64, m: f64, y: f64, k: f64) -> Self {
        let parts = [
            CMYK::conv_to_int(c),
            CMYK::conv_to_int(m),
            CMYK::conv_to_int(y),
            CMYK::conv_to_int(k)
        ];
        CMYK::from_parts(parts)
    }

    /// Creates a CMYK object from the raw u16 components.
    /// This is used in [CMYK::new()]
    pub const fn from_parts(parts: [u16; 4]) -> Self {
        let [c, m, y, k] = parts;
        CMYK {c, m, y, k}
    }

    /// Converts a u16 into the traditional `[0.0, 1.0]` float representation
    pub(crate) fn conv_to_float(i: u16) -> f64 {
        (i as f64) / std::u16::MAX as f64
    }

    /// Returns whether or not the given float is in `[0.0, 1.0]`
    pub fn valid_cmyk_float(f: impl Into<f64>) -> bool {
        let f = f.into();
        0.0 <= f && f <= 1.0
    }

    /// Converts a float from `[0.0, 1.0]` into the CMYK u16 representation
    /// # Panics
    /// Panics if `f` is not in the aforementioned range.
    pub(crate) fn conv_to_int(f: f64) -> u16 {
        assert!(CMYK::valid_cmyk_float(f));
        (f * std::u16::MAX as f64).round() as u16
    }

    /// Returns the raw integer parts of the CMYK representation
    pub fn as_int_parts(&self) -> [u16; 4] {
        [self.c, self.m, self.y, self.k]
    }

    /// Returns the float representation of the CMYK object. Floats are as described elsewhere.
    pub fn as_parts(&self) -> [f64; 4] {
        [CMYK::conv_to_float(self.c),
            CMYK::conv_to_float(self.m),
            CMYK::conv_to_float(self.y),
            CMYK::conv_to_float(self.k)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmyk_conversions() {
        let black = CMYK::new(0., 0.,0., 1.);
        let rgb_black = RGB::BLACK;
        println!("{}", black);
        println!("{}", rgb_black);
        assert_eq!(black.as_rgb(), rgb_black);
        assert_eq!(rgb_black.into_color::<CMYK>(), black);

        let rgb_white = RGB::WHITE;
        let white = CMYK::new(0., 0., 0., 0.);
        assert_eq!(white.as_rgb(), rgb_white);
        assert_eq!(rgb_white.into_color::<CMYK>(), white);

        let sg = RGB::from_hex("#EDBBF3").unwrap();
        println!("{}", sg.into_color::<CMYK>());
        assert_eq!("(0.02,0.23,0.00,0.05)", sg.into_color::<CMYK>().to_string());
        assert_eq!(sg, sg.into_color::<CMYK>().into_rgb())
    }
}