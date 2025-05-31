/// Command-line interface module.
mod types;
#[cfg(test)]
mod tests;

use clap::Parser;

use crate::color::Rgba;
use crate::error::{Result, WaverError};

pub use types::*;

/// Command line arguments for waveform generation.
#[derive(Parser, Debug)]
#[command(
    name = "waver",
    about = "Generate waveform visualizations from audio files",
    version,
    author
)]
pub struct WaverArgs {
    /// Width of the output image in pixels
    #[arg(long = "width", default_value = "2048", value_parser = clap::value_parser!(Width))]
    pub width: Width,

    /// Height of the output image in pixels (must be even)
    #[arg(long = "height", default_value = "128", value_parser = clap::value_parser!(Height))]
    pub height: Height,

    /// Color for left channel (RGB, RRGGBB, or RRGGBBAA)
    #[arg(long = "left-color", default_value = "00ff99", value_parser = clap::value_parser!(Rgba))]
    pub left_color: Rgba,

    /// Color for right channel (RGB, RRGGBB, or RRGGBBAA)
    #[arg(long = "right-color", default_value = "99ff00", value_parser = clap::value_parser!(Rgba))]
    pub right_color: Rgba,

    /// Background color (RGB, RRGGBB, or RRGGBBAA)
    #[arg(long = "background-color", default_value = "ffffff00", value_parser = clap::value_parser!(Rgba))]
    pub background_color: Rgba,

    /// Output PNG file name (only in single-file mode)
    #[arg(short = 'o', long = "output-filename")]
    pub output_filename: Option<String>,

    /// Comma-separated list of audio file extensions
    #[arg(long = "file-extensions", default_value = "mp3", value_parser = clap::value_parser!(FileExtensions))]
    pub file_extensions: FileExtensions,

    /// Perform actions without generating files
    #[arg(long = "dry-run")]
    pub dry_run: bool,

    /// Overwrite existing output files
    #[arg(long = "overwrite")]
    pub overwrite: bool,

    /// Suppress most output
    #[arg(long = "quiet")]
    pub quiet: bool,

    /// Print additional information
    #[arg(long = "verbose")]
    pub verbose: bool,

    /// Audio files or directories to process
    #[arg(required = true, num_args = 1.., value_parser = clap::value_parser!(AudioPath))]
    pub audio_paths: Vec<AudioPath>,
}

impl WaverArgs {
    /// Parse command-line arguments and validate them.
    pub fn parse_and_validate() -> Result<Self> {
        let args = Self::parse();
        args.validate()?;
        Ok(args)
    }

    /// Validates inter-argument constraints that can't be handled by individual type validations.
    pub fn validate(&self) -> Result<()> {
        // Validate output filename constraints
        if self.audio_paths.len() > 1 && self.output_filename.is_some() {
            return Err(WaverError::argument_error(
                "Cannot specify --output-filename with multiple audio files",
            ));
        }

        // Check directory constraints
        if self.output_filename.is_some() {
            for path in &self.audio_paths {
                if path.is_dir() {
                    return Err(WaverError::argument_error(
                        "Cannot specify --output-filename with a directory",
                    ));
                }
            }
        }

        Ok(())
    }

    /// Prints messages to stderr unless quiet mode is enabled.
    pub fn print_to_stderr(&self, message: &str) {
        if !self.quiet {
            eprintln!("{message}");
        }
    }

    /// Prints messages to stdout (usually for successful operations).
    pub fn print_to_stdout(&self, message: &str) {
        if !self.quiet {
            println!("{message}");
        }
    }

    /// Prints verbose messages if verbose mode is enabled.
    pub fn print_verbose(&self, message: &str) {
        if self.verbose {
            println!("{message}");
        }
    }

    /// Returns the width value.
    pub fn width(&self) -> u32 {
        self.width.value()
    }

    /// Returns the height value.
    #[allow(dead_code)]
    pub fn height(&self) -> u32 {
        self.height.value()
    }

    /// Returns the center line position.
    #[allow(dead_code)]
    pub fn center(&self) -> u32 {
        self.height.center()
    }

    /// Returns the file extensions as strings.
    pub fn file_extensions(&self) -> Vec<String> {
        self.file_extensions.as_strings()
    }
}