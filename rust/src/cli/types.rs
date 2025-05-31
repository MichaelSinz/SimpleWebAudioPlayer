/// Custom types for command-line argument validation.
///
/// This module provides strongly-typed representations of command-line arguments
/// with built-in validation.  These types make invalid states unrepresentable and
/// push validation to the earliest possible point - during argument parsing.
///
/// # Design Philosophy
///
/// Rather than validate arguments after parsing, we use Rust's type system to:
/// 1. Ensure values meet constraints (e.g., minimum width/height)
/// 2. Provide clear, targeted error messages directly during parsing
/// 3. Allow functions to assume arguments are already valid
/// 4. Make the code more self-documenting
///
/// # Usage
///
/// These types implement FromStr and can be used with clap's value_parser:
/// ```
/// #[arg(value_parser = clap::value_parser!(Width))]
/// pub width: Width,
/// ```
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::error::{Result, WaverError};

/// A validated width value for the waveform image.
///
/// Ensures the width is at least 16 pixels.
#[derive(Debug, Clone, Copy)]
pub struct Width(u32);

impl Width {
    /// The minimum allowed width in pixels.
    pub const MIN_WIDTH: u32 = 16;

    /// Creates a new validated width.
    pub fn new(width: u32) -> Result<Self> {
        if width < Self::MIN_WIDTH {
            return Err(WaverError::argument_error(
                format!("Width must be at least {} pixels", Self::MIN_WIDTH)
            ));
        }
        Ok(Self(width))
    }

    /// Returns the width value.
    pub fn value(&self) -> u32 {
        self.0
    }
}

impl FromStr for Width {
    type Err = WaverError;

    fn from_str(s: &str) -> Result<Self> {
        let width = s.parse::<u32>()
            .map_err(|_| WaverError::argument_error("Width must be a positive integer"))?;
        Self::new(width)
    }
}

/// A validated height value for the waveform image.
///
/// Ensures the height is at least 6 pixels and even.
#[derive(Debug, Clone, Copy)]
pub struct Height(u32);

impl Height {
    /// The minimum allowed height in pixels.
    pub const MIN_HEIGHT: u32 = 6;

    /// Creates a new validated height.
    pub fn new(height: u32) -> Result<Self> {
        if height < Self::MIN_HEIGHT {
            return Err(WaverError::argument_error(
                format!("Height must be at least {} pixels", Self::MIN_HEIGHT)
            ));
        }
        if height % 2 != 0 {
            return Err(WaverError::argument_error("Height must be an even number"));
        }
        Ok(Self(height))
    }

    /// Returns the height value.
    pub fn value(&self) -> u32 {
        self.0
    }

    /// Returns the vertical center line position.
    pub fn center(&self) -> u32 {
        self.0 / 2
    }
}

impl FromStr for Height {
    type Err = WaverError;

    fn from_str(s: &str) -> Result<Self> {
        let height = s.parse::<u32>()
            .map_err(|_| WaverError::argument_error("Height must be a positive integer"))?;
        Self::new(height)
    }
}

/// A validated audio file path.
///
/// Ensures the path exists and is a file.
#[derive(Debug, Clone)]
pub struct AudioPath(PathBuf);

impl AudioPath {
    /// Creates a new validated audio path.
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        if !path.exists() {
            return Err(WaverError::argument_error(
                format!("Path does not exist: {}", path.display())
            ));
        }
        Ok(Self(path.to_path_buf()))
    }

    /// Returns whether this path points to a directory.
    pub fn is_dir(&self) -> bool {
        self.0.is_dir()
    }

    /// Returns the path.
    pub fn path(&self) -> &Path {
        &self.0
    }
}

impl FromStr for AudioPath {
    type Err = WaverError;

    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

/// A validated audio file extension.
///
/// Ensures the extension is valid.
#[derive(Debug, Clone)]
pub struct FileExtension(String);

impl FileExtension {
    /// Creates a new validated file extension.
    pub fn new(extension: impl AsRef<str>) -> Result<Self> {
        let extension = extension.as_ref().trim().to_lowercase();
        if extension.is_empty() {
            return Err(WaverError::argument_error("File extension cannot be empty"));
        }
        Ok(Self(extension))
    }

    /// Returns the extension string.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for FileExtension {
    type Err = WaverError;

    fn from_str(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

/// A collection of validated file extensions.
#[derive(Debug, Clone)]
pub struct FileExtensions(Vec<FileExtension>);

impl FileExtensions {
    /// Creates a new collection of validated file extensions.
    pub fn new(extensions: Vec<impl AsRef<str>>) -> Result<Self> {
        let mut validated_extensions = Vec::with_capacity(extensions.len());

        for ext in extensions {
            validated_extensions.push(FileExtension::new(ext)?);
        }

        Ok(Self(validated_extensions))
    }

    /// Returns an iterator over the file extensions.
    #[allow(dead_code)]
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.0.iter().map(|e| e.as_str())
    }

    /// Returns the file extensions as a vector of strings.
    pub fn as_strings(&self) -> Vec<String> {
        self.0.iter().map(|e| e.as_str().to_string()).collect()
    }
}

impl FromStr for FileExtensions {
    type Err = WaverError;

    fn from_str(s: &str) -> Result<Self> {
        let extensions = s.split(',')
            .map(|part| part.trim())
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>();

        if extensions.is_empty() {
            return Err(WaverError::argument_error("No file extensions specified"));
        }

        Self::new(extensions)
    }
}