#![forbid(unsafe_code)]
#![deny(unused_imports)]
#![deny(missing_docs)]

//! color_changer is a crate designed for conversions between color representations, i.e. `RGB <-> CMYK`

use std::fmt::{Display};
use regex::Regex;
use once_cell::sync::Lazy;
use std::num::ParseIntError;
use crate::rgb::RGB;
use thiserror::Error as ThisErr;
use crate::ColorParseError::BadInput;

pub mod rgb;
pub mod cmyk;

static HEX_RE: Lazy<Regex> = Lazy::new(
    || Regex::new(r#"#?([0-9a-fA-F]{2})([0-9a-fA-F]{2})([0-9a-fA-F]{2})"#).unwrap()
);

/// Represents the potential reasons parsing a hex string into a color could fail
#[derive(ThisErr, Clone, Debug)]
pub enum ColorParseError {
    /// Occurs when the input does not match the hex color regex, like `"#FFABCD"` or `"ABFFED"`
    #[error("The input wasn't a valid hex color, e.g. #FFABCD or ABFFED")]
    BadInput,
    /// Occurs when the input breaks u8's parse method.
    #[error("A component of the hex string didn't parse: {0}")]
    ParseFailure(#[from] ParseIntError)
}

/// Represents a color, with RGB-255 as the "common" format for conversions
pub trait Color: Display + Sized {

    /// Converts this color into RGB-255
    fn into_rgb(self) -> RGB {
        self.as_rgb()
    }

    /// Converts this color into RGB-255, but without consuming `self`
    fn as_rgb(&self) -> RGB;

    /// Creates a color from the RGB representation.
    fn from_rgb(c: RGB) -> Self;

    /// Generic conversion into any other color type.
    fn into_color<U: Color>(self) -> U {
        U::from_rgb(self.into_rgb())
    }

    /// Generic conversion into any other color type without consuming `self`
    fn as_color<U: Color>(&self) -> U {
        U::from_rgb(self.as_rgb())
    }

    /// Generic conversion from any other color type without consuming `self`
    fn from_color<U: Color>(c: U) -> Self {
        Self::from_rgb(c.into_rgb())
    }

    /// Converts this color into the hex string of the RGB-255 representation.
    /// May be lossy.
    fn as_hex(&self) -> String {
        self.as_rgb().to_string()
    }

    /// Converts a hex string into whichever color representation is appropriate.
    /// # Examples
    /// ```
    /// use color_changer::rgb::RGB;
    /// use color_changer::Color;
    /// let black = RGB::from_hex("#000000").unwrap();
    /// assert_eq!(black, RGB::BLACK);
    /// ```
    fn from_hex(s: impl AsRef<str>) -> Result<Self, ColorParseError> {
        let matches = HEX_RE.captures(s.as_ref()).ok_or(BadInput)?;
        let rgb: Result<Vec<u8>, ColorParseError> = matches.iter()
            .skip(1)
            .map(|x| x.unwrap())
            .map(|m|m.as_str())
            .map(|i| u8::from_str_radix(i, 16))
            .try_fold(Vec::new(), |mut acc, i| {
                acc.push(i?);
                Ok(acc)
            });
        let rgb = rgb?;

        if let [r, g, b] = &rgb[..] {
            Ok(Self::from_rgb(RGB::new(*r, *g, *b)))
        } else {
            unreachable!()
        }
    }
}

