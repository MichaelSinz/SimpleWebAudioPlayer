/// Color handling functionality for waveform visualization.
use std::str::FromStr;

use crate::error::{Result, WaverError};

#[cfg(test)]
mod tests;

/// Represents an RGBA color.
#[derive(Clone, Debug)]
pub struct Rgba {
    /// Red component (0-255)
    pub red: u8,
    /// Green component (0-255)
    pub green: u8,
    /// Blue component (0-255)
    pub blue: u8,
    /// Alpha component (0-255)
    pub alpha: u8,
}

impl Rgba {
    /// Creates a new RGBA color with the given components.
    #[allow(dead_code)]
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    /// Creates a new opaque RGB color with the given components and alpha=255.
    #[allow(dead_code)]
    pub fn rgb(red: u8, green: u8, blue: u8) -> Self {
        Self::new(red, green, blue, 255)
    }
}

impl FromStr for Rgba {
    type Err = WaverError;

    /// Parses a color from a string in the following formats:
    /// - RGB (3-digit hex): e.g. "F00" for bright red
    /// - RRGGBB (6-digit hex): e.g. "FF0000" for bright red
    /// - RRGGBBAA (8-digit hex): e.g. "FF0000FF" for opaque bright red
    fn from_str(color: &str) -> Result<Self> {
        let hex = color.trim();
        let value = u32::from_str_radix(hex, 16)
            .map_err(|e| WaverError::argument_error(format!("Invalid color format: {}", e)))?;

        match hex.len() {
            3 => Ok(Rgba {
                red: ((value & 0xF00) >> 8) as u8 * 17,
                green: ((value & 0x0F0) >> 4) as u8 * 17,
                blue: (value & 0x00F) as u8 * 17,
                alpha: 255,
            }),
            6 => Ok(Rgba {
                red: ((value & 0xFF0000) >> 16) as u8,
                green: ((value & 0x00FF00) >> 8) as u8,
                blue: (value & 0x0000FF) as u8,
                alpha: 255,
            }),
            8 => Ok(Rgba {
                red: ((value & 0xFF000000) >> 24) as u8,
                green: ((value & 0x00FF0000) >> 16) as u8,
                blue: ((value & 0x0000FF00) >> 8) as u8,
                alpha: (value & 0x000000FF) as u8,
            }),
            _ => Err(WaverError::argument_error(
                "Color must be in RGB, RRGGBB, or RRGGBBAA format",
            )),
        }
    }
}