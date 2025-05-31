/// Waveform image generation functionality with optimized 2-bit PNG output.
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use png::{Encoder, FilterType};

use crate::cli::{Height, Width};
use crate::color::Rgba;
use crate::error::Result;

#[cfg(test)]
mod tests;

/// Represents the different channel types in a waveform image.
///
/// Using an enum instead of constants provides better type safety and
/// makes the code more self-documenting.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Channel {
    /// Background (transparent or base color)
    Background = 0,
    /// Left audio channel (typically drawn above center)
    Left = 1,
    /// Right audio channel (typically drawn below center)
    Right = 2,
}

impl From<Channel> for u8 {
    fn from(channel: Channel) -> Self {
        channel as u8
    }
}

impl From<u8> for Channel {
    fn from(value: u8) -> Self {
        match value {
            0 => Channel::Background,
            1 => Channel::Left,
            2 => Channel::Right,
            _ => Channel::Background, // Default to background for invalid values
        }
    }
}

/// Represents a waveform visualization image with 2-bit pixel depth optimization.
///
/// Contains the image dimensions and pixel data where each pixel is represented
/// by an index value (0 for background, 1 for left channel, 2 for right channel).
/// Uses 2 bits per pixel for significant space savings in the output PNG.
pub struct WaveImage {
    /// Width of the image in pixels.
    width: u32,

    /// Height of the image in pixels.
    height: u32,

    /// Vertical center line position.
    center: u32,

    /// Line size in bytes (due to 2 bits per pixel)
    line_width: u32,

    /// Pixel data stored as channel indices.
    /// During image generation, we use 1 byte per pixel for simplicity.
    pixels: Vec<u8>,
}

/// Convert a color index to the bit location based on the x coordinate
///
/// # Arguments
///
/// # `color` - The 2-bit color
/// # `x` - The horizontal position of the pixel
///
/// # Returns
///
/// The u8 with the color bits shifted to the correct location for 2-bpp
fn draw_bits(color: u8, x: u32) -> u8 {
    (color & 3) << (2 * (x & 3))
}

impl WaveImage {
    /// Creates a new waveform 2-bit per pixel image with the specified
    /// dimensions.  We render directly into the 2-bit per pixel form
    /// to reduce memory footprint and because we can do it efficiently.
    ///
    /// # Arguments
    ///
    /// * `width` - Width of the image in pixels
    /// * `height` - Height of the image in pixels (must be even)
    ///
    /// # Returns
    ///
    /// A new WaveImage instance initialized with background pixels
    ///
    /// # Notes
    ///
    /// This is a 2-bit per pixel image since we really only need
    /// at most 4 colors:
    ///   0:  Background color
    ///   1:  Left Channel  (or mono)
    ///   2:  Right Channel
    ///   3:  Background due to Left and Right collision
    ///
    /// ```
    /// use waver::cli::{Width, Height};
    /// use waver::image::WaveImage;
    ///
    /// // Create a 1024x128 waveform image
    /// let width = Width::new(1024).unwrap();
    /// let height = Height::new(128).unwrap();
    /// let image = WaveImage::new(width, height);
    /// ```
    pub fn new(width: Width, height: Height) -> Self {
        let width_val = width.value();
        let line_val = (width_val + 3) >> 2;
        let height_val = height.value();

        Self {
            width: width_val,
            height: height_val,
            line_width: line_val,
            center: height.center(),
            pixels: vec![0 as u8; (line_val * height_val) as usize],
        }
    }

    /// Draws a single point (left and right channels) of the waveform.
    ///
    /// # Arguments
    ///
    /// * `x` - The horizontal position to draw at
    /// * `left` - Left channel maximum amplitude
    /// * `right` - Right channel maximum amplitude
    pub fn draw_point(&mut self, x: u32, left: f32, right: f32) {
        if x >= self.width {
            return;
        }

        // The byte offset where the 2-bit pixel will be
        let offset = x >> 2;

        // Draw left channel (above center, going up)
        // The bits for the left channel at this pixel offset
        let draw_left = draw_bits(Channel::Left as u8, x);
        let left_height = (self.center as f32 * left.max(0.0).min(1.0) + 0.5) as u32;
        for y in self.center.saturating_sub(left_height)..self.center {
            let idx = (offset + y * self.line_width) as usize;
            self.pixels[idx] |= draw_left;
        }

        // Draw right channel (below center, going down)
        // The bits for the right channel at this pixel offset
        let draw_right = draw_bits(Channel::Right as u8, x);
        let right_height = (self.center as f32 * right.max(0.0).min(1.0) + 0.5) as u32;
        let max_y = std::cmp::min(self.center + right_height, self.height);
        for y in self.center..max_y {
            let idx = (offset + y * self.line_width) as usize;
            self.pixels[idx] |= draw_right;
        }
    }

    /// Draws a single point for mono audio (symmetric around center).
    ///
    /// # Arguments
    ///
    /// * `x` - The horizontal position to draw at
    /// * `mono` - Mono channel maximum amplitude
    ///
    /// # Example
    ///
    /// ```
    /// use waver::cli::{Width, Height};
    /// use waver::image::WaveImage;
    ///
    /// let width = Width::new(1024).unwrap();
    /// let height = Height::new(128).unwrap();
    /// let mut image = WaveImage::new(width, height);
    ///
    /// // Draw a mono sample at x=100 with 50% amplitude
    /// image.draw_point_mono(100, 0.5);
    /// ```
    pub fn draw_point_mono(&mut self, x: u32, mono: f32) {
        if x >= self.width {
            return;
        }

        // The byte offset where the 2-bit pixel will be
        let offset = x >> 2;

        // Bit position for the pixel
        let draw = draw_bits(Channel::Left as u8, x);

        let wave_height = (self.center as f32 * mono.max(0.0).min(1.0) + 0.5) as u32;
        let y_start = self.center.saturating_sub(wave_height);
        let y_end = std::cmp::min(self.center + wave_height, self.height);

        for y in y_start..y_end {
            let idx = (offset + y * self.line_width) as usize;
            self.pixels[idx] |= draw;
        }
    }

    /// Returns the width of the image.
    #[allow(dead_code)]
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Saves the waveform image as a PNG file with 2-bit pixel depth optimization.
    ///
    /// # Performance
    ///
    /// This function implements several critical optimizations:
    /// - Uses 2-bit color depth instead of 8-bit (75% size reduction)
    /// - Uses indexed color mode with a minimal 3-color palette
    /// - Applies the Up filter which is optimal for waveform imagery
    /// - Uses maximum PNG compression for smallest possible files
    ///
    /// Changing these settings, especially the bit depth or filter type,
    /// would significantly impact file size or performance.
    ///
    /// # Arguments
    ///
    /// * `background` - Background color
    /// * `left` - Left channel color
    /// * `right` - Right channel color
    /// * `output_path` - Path where the PNG file will be saved
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or a WaverError on failure
    pub fn save_png(
        &self,
        background: &Rgba,
        left: &Rgba,
        right: &Rgba,
        output_path: impl AsRef<Path>,
    ) -> Result<()> {
        // Create palette for indexed color PNG
        let palette = [
            background.red, background.green, background.blue,
            left.red, left.green, left.blue,
            right.red, right.green, right.blue,
            background.red, background.green, background.blue,
        ];

        // Create transparency array
        let transparent = [background.alpha, left.alpha, right.alpha, background.alpha];

        // Create the output file and BufWriter
        let file = File::create(output_path)?;
        let mut encoder = Encoder::new(BufWriter::new(file), self.width, self.height);

        // Configure the PNG encoder - use 2-bit depth since we only need 3 colors
        encoder.set_color(png::ColorType::Indexed);
        encoder.set_depth(png::BitDepth::Two);
        encoder.set_palette(&palette);
        encoder.set_trns(&transparent);

        // Optimize for waveform imagery which typically has vertical runs
        encoder.set_filter(FilterType::Up);

        // Use maximum compression
        encoder.set_compression(png::Compression::Best);

        // Write the PNG data
        let mut writer = encoder.write_header()?;
        writer.write_image_data(&self.pixels)?;
        writer.finish()?;

        Ok(())
    }
}