//! Contains datatypes and functions for manipulation and creation of RGB-255 colors

use crate::Color;
use std::fmt::Display;

/// Represents an RGB-255 color.
#[derive(Copy, Debug, Clone, Eq, PartialEq)]
pub struct RGB {
    /// The red component
    pub r: u8,
    /// The green component
    pub g: u8,
    /// The blue component
    pub b: u8,
}

impl Display for RGB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "{:02X}{:02X}{:02X}",
               self.r,
               self.g,
               self.b)
    }
}

impl Color for RGB {
    fn into_rgb(self) -> RGB {
        self
    }

    fn as_rgb(&self) -> RGB {
        self.clone()
    }

    fn from_rgb(c: RGB) -> Self {
        c
    }
}


impl RGB {
    /// Black (`#000000`)
    pub const BLACK: RGB = RGB::new(0, 0,0);
    /// White (`FFFFFF`)
    pub const WHITE: RGB = RGB::new(0xFF, 0xFF, 0xFF);

    /// Creates an RGB-255 color from raw components
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        RGB { r, g, b }
    }

    /// Returns the raw bytes of the RGB color as an array, in order RGB
    /// # Examples
    /// ```
    /// use color_changer::rgb::RGB;
    /// let black = RGB::new(0, 0, 0);
    /// let [r, g, b] = black.as_parts();
    /// assert_eq!(r, g);
    /// assert_eq!(g, b);
    /// assert_eq!(b, 0);
    /// ```
    pub fn as_parts(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }

    /// Returns whether or not this is equivalent to [RGB::BLACK]
    pub fn is_black(&self) -> bool {
        self.as_parts().iter().all(|&x| x == 0)
    }
}