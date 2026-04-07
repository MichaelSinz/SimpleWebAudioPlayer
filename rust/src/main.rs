/// Waver: Generate waveform visualizations from audio files.
///
/// This tool creates PNG visualizations of audio waveforms from various audio
/// file formats.  It supports multiple audio file processing, customizable
/// colors, and various output options.
///
/// # Architecture
///
/// The program follows a data processing pipeline:
/// 1. Parse and validate command-line arguments
/// 2. Collect audio files to process
/// 3. Process each file in parallel, generating waveform images
/// 4. Report any errors that occurred during processing
///
/// # Performance
///
/// Key performance optimizations:
/// - Parallel processing of audio files using rayon
/// - Streaming audio decoding rather than buffering
/// - 2-bit pixel depth in PNG output for smaller files
///
/// See ARCHITECTURE.md and OPTIMIZATIONS.md for more details.
mod audio;
mod cli;
mod color;
mod error;
mod image;
mod examples;

use rayon::prelude::*;
use walkdir::WalkDir;

use audio::generate_waveform;
use cli::WaverArgs;
use error::WaverError;

/// Main entry point for the waver application.
fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Parse and validate command-line arguments
    let args = WaverArgs::parse_and_validate()?;

    // Collect all audio files to process
    let mut audio_files = Vec::new();
    for audio_path in &args.audio_paths {
        let path = audio_path.path();
        if path.is_file() {
            // Directly entered file names are just used as is
            // We don't filter it to the extensions
            audio_files.push(path.to_path_buf());
        } else if path.is_dir() {
            // We use WalkDir such that the complexity of loops/etc are handled
            // for us rather than getting us stuck
            for entry in WalkDir::new(path)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|entry| entry.file_type().is_file())
                .filter(|entry| {
                    entry
                        .path()
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .map(|ext_str| {
                            args.file_extensions()
                                .iter()
                                .any(|e| e.eq(ext_str))
                            })
                        .unwrap_or(false)
                    })
                .map(|entry| entry.into_path()) {
                    audio_files.push(entry);
            }
        }
    }

    if audio_files.is_empty() {
        return Err(Box::new(WaverError::argument_error(
            "No matching audio files found",
        )));
    }

    if args.verbose {
        args.print_verbose(&format!("Found {} audio files to process", audio_files.len()));
    }

    // If there is exactly one file and an output filename was provided,
    // we don't even need Rayon or a loop. We just run it.
    if audio_files.len() == 1 && args.output_filename.is_some() {
        let file_path = &audio_files[0];
        let output_path = args.output_filename.as_ref().unwrap();

        // We wrap it in AudioPath for the API, but there's no loop overhead.
        let audio_path = cli::AudioPath::new(file_path)?;

        if let Err(e) = generate_waveform(&audio_path, output_path, &args) {
            args.print_to_stderr(&format!("{}: {}", file_path.display(), e));
            return Err(e.into());
        }

        args.print_to_stdout(&format!("Created {}", output_path));
        return Ok(());
    }

    // Parallel processing in order to handle multiple files efficiently
    // This section uses Rayon's parallel map to process files concurrently
    // while safely collecting any errors.
    // We transform each PathBuf into a Result<(), String>
    // and then filter any errors from the operation
    // so we end up with just the errors in a Vec<String>
    let errors: Vec<String> = audio_files
        .into_par_iter()
        .map(|file_path| {
            // Attempt to create the AudioPath
            let audio_path = match cli::AudioPath::new(&file_path) {
                Ok(p) => p,
                Err(e) => return Err(format!("Invalid audio path {}: {}", file_path.display(), e)),
            };

            // In this path, we ALWAYS generate a new string. No clones, no Arcs, no Cow.
            let output_file = format!("{}.png", file_path.display());

            // Attempt to generate the waveform
            if let Err(e) = generate_waveform(&audio_path, &output_file, &args) {
                let error_msg = format!("{}: {}", file_path.display(), e);
                args.print_to_stderr(&error_msg);
                Err(error_msg)
            } else {
                Ok(())
            }
        })
        .filter_map(|result| result.err()) // Only keep the errors
        .collect();

    // Report any errors
    if !errors.is_empty() {
        return Err(Box::new(WaverError::generation_error(format!(
            "{} errors occurred while processing files",
            errors.len()
        ))));
    }

    Ok(())
}