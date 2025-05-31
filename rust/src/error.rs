/// Error types for the waver application.
///
/// This module defines the error types used throughout the application and
/// provides a consistent approach to error handling and propagation.
///
/// # Error Handling Strategy
///
/// The waver application uses a structured error handling approach:
///
/// 1. **Custom Error Types**: All errors are consolidated into the WaverError enum
/// 2. **Context Preservation**: External errors (IO, etc.) are wrapped with context
/// 3. **Early Validation**: Most errors are caught at argument parsing time
/// 4. **Result Propagation**: Errors bubble up with the `?` operator
/// 5. **User-Friendly Messages**: Errors are formatted to be helpful to the user
///
/// This approach makes errors easier to handle, debug, and report to users.
use std::io;
use thiserror::Error;

/// Represents all possible errors that can occur in the waver application.
#[derive(Error, Debug)]
pub enum WaverError {
    /// Error when parsing or validating command line arguments.
    #[error("Invalid argument: {0}")]
    ArgumentError(String),

    /// Error during waveform generation process.
    #[error("Waveform generation error: {0}")]
    GenerationError(String),

    /// Error from the underlying IO operations.
    #[error("IO error: {0}")]
    IoError(#[from] io::Error),

    /// Error from the Symphonia audio decoding library.
    #[error("Audio decoding error: {0}")]
    SymphoniaError(#[from] symphonia::core::errors::Error),

    /// Error from the PNG encoding library.
    #[error("PNG encoding error: {0}")]
    PngError(#[from] png::EncodingError),
}

/// Type alias for Result with WaverError.
pub type Result<T> = std::result::Result<T, WaverError>;

impl WaverError {
    /// Create a new ArgumentError with the given message.
    ///
    /// Use this for errors related to command-line arguments or configuration.
    pub fn argument_error(msg: impl Into<String>) -> Self {
        WaverError::ArgumentError(msg.into())
    }

    /// Create a new GenerationError with the given message.
    ///
    /// Use this for errors that occur during the waveform generation process.
    pub fn generation_error(msg: impl Into<String>) -> Self {
        WaverError::GenerationError(msg.into())
    }
}