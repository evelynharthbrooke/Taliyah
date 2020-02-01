//! Color Utilities
//! 
//! Various utilities designed to help with hexadecimal
//! color codes.

use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RGB {
    /// Takes a given hexadecimal color code (e.g. #1db954) and splits
    /// it off into individial integers matching the R, G, and B color
    /// specification.
    /// 
    /// Returns a Result containing said integers, or returns a 
    /// [ParseIntError] when an error is encountered.
    /// 
    /// [ParseIntError]: std::num::ParseIntError
    pub fn from_hex_code(code: &str) -> Result<Self, ParseIntError> {
        let r: u8 = u8::from_str_radix(&code[1..3], 16)?;
        let g: u8 = u8::from_str_radix(&code[3..5], 16)?;
        let b: u8 = u8::from_str_radix(&code[5..7], 16)?;
        Ok(RGB { r, g, b })
    }

    /// Takes the provided R, G, and B integers (e.g. 43, 116, 137), and 
    /// converts them to a hexadecimal color code.
    /// 
    /// # Example
    /// ```
    /// use ellie::utilities::RGB;
    /// use ellie::utilities::RGB::to_hex_code;
    /// 
    /// let rgb = RGB { 43, 116, 137 };
    /// let rgb_hex = to_hex_code(rgb);
    /// 
    /// assert_eq!(rgb_hex, "291e669".to_string());
    /// ```
    /// 
    pub fn to_hex_code(&self) -> String {
        let r = self.r;
        let g = self.g;
        let b = self.b;
        let rgb = format!("{}{}{}", r, g, b);
        let rgb_str = rgb.parse::<i32>().unwrap();
        let formatted_hex = format!("{:06X}", rgb_str);
        formatted_hex.to_lowercase()
    }
}
