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

use std::sync::Mutex;

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

    // Process files in parallel, collecting errors
    // PERFORMANCE: Parallel processing is critical for handling multiple files efficiently
    // This section uses Rayon's parallel iterator to process files concurrently
    // while safely collecting errors using a synchronized Mutex
    let errors = Mutex::new(Vec::<String>::new());

    // Convert PathBuf to AudioPath for processing
    audio_files.into_par_iter().for_each(|file_path| {
        // For each file, create a validated AudioPath
        match cli::AudioPath::new(&file_path) {
            Ok(audio_path) => {
                let output_file = args
                    .output_filename
                    .clone()
                    .unwrap_or_else(|| format!("{}.png", file_path.display()));

                if let Err(e) = generate_waveform(&audio_path, &output_file, &args) {
                    let error_msg = format!("{}: {}", file_path.display(), e);
                    args.print_to_stderr(&error_msg);
                    errors.lock().unwrap().push(error_msg);
                }
            },
            Err(e) => {
                let error_msg = format!("Invalid audio path {}: {}", file_path.display(), e);
                args.print_to_stderr(&error_msg);
                errors.lock().unwrap().push(error_msg);
            }
        }
    });

    // Report any errors
    let errors = errors.lock().unwrap();
    if !errors.is_empty() {
        return Err(Box::new(WaverError::generation_error(format!(
            "{} errors occurred while processing files",
            errors.len()
        ))));
    }

    Ok(())
}