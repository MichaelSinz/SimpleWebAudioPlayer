/// Example demonstrating how to use the waver library programmatically.
///
/// This is a simple example showing how to generate a waveform image
/// from an audio file using the default settings.
///
/// # Example
///
/// ```no_run
/// use std::path::Path;
/// use waver::audio::generate_waveform;
/// use waver::cli::WaverArgs;
/// use clap::Parser;
///
/// fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Parse command line arguments or use defaults
///     let args = WaverArgs::parse_and_validate()?;
///
///     // Generate a waveform for a single file
///     generate_waveform(
///         "input.mp3",
///         "output.png",
///         &args,
///     )?;
///
///     println!("Generated waveform image: output.png");
///
///     Ok(())
/// }
/// ```
pub fn _doc_example() {
    // This function exists only for documentation purposes
}