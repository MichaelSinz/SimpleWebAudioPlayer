//! Unit tests for audio decoding and waveform generation.
//!
//! Generated PCM WAV files keep sample values and frame counts predictable without
//! external audio fixtures or encoders.  The tests cover mono and stereo peak
//! aggregation, clips shorter than the image width, and truncated input.

use std::io::Write;
use tempfile::{tempdir, NamedTempFile};

use crate::audio::process_audio_file;
use crate::cli::{Height, Width};
use crate::color::Rgba;
use crate::image::WaveImage;

/// Creates a 16 kHz, signed 16-bit PCM WAV file with the supplied channel count.
///
/// Samples are interleaved for stereo input: left, right, left, right, etc.
/// The returned temporary file stays alive while the decoder opens its path.
fn write_wave(channels: u16, samples: &[i16]) -> NamedTempFile {
    let mut file = NamedTempFile::new().unwrap();
    let data_len = u32::try_from(samples.len() * 2).unwrap();

    // RIFF size excludes its own 8-byte ID and size fields.
    file.write_all(b"RIFF").unwrap();
    file.write_all(&(36 + data_len).to_le_bytes()).unwrap();
    file.write_all(b"WAVEfmt ").unwrap();

    // The 16-byte PCM format chunk describes the rate and interleaved frame layout.
    file.write_all(&16u32.to_le_bytes()).unwrap(); // Format chunk length
    file.write_all(&1u16.to_le_bytes()).unwrap(); // PCM format code
    file.write_all(&channels.to_le_bytes()).unwrap();
    file.write_all(&16000u32.to_le_bytes()).unwrap();
    file.write_all(&(16000 * u32::from(channels) * 2).to_le_bytes())
        .unwrap(); // Bytes per second
    file.write_all(&(channels * 2).to_le_bytes()).unwrap(); // Bytes per frame
    file.write_all(&16u16.to_le_bytes()).unwrap();

    // Declare the complete payload so truncation tests can remove data without
    // changing the frame count advertised by the header.
    file.write_all(b"data").unwrap();
    file.write_all(&data_len.to_le_bytes()).unwrap();
    for sample in samples {
        file.write_all(&sample.to_le_bytes()).unwrap();
    }
    file
}

/// Creates a minimum-width image whose 16-pixel half-height represents quarter
/// amplitudes exactly as 4, 8, and 12 pixels.
fn new_image() -> WaveImage {
    WaveImage::new(Width::new(16).unwrap(), Height::new(32).unwrap())
}

/// Compares complete waveform images using identical PNG encoding settings.
///
/// This checks both channel colors and untouched background pixels without
/// exposing the private packed pixel buffer in WaveImage.
fn assert_same_image(actual: &WaveImage, expected: &WaveImage) {
    let dir = tempdir().unwrap();
    let background = Rgba::new(0, 0, 0, 0);
    let left = Rgba::rgb(0, 255, 0);
    let right = Rgba::rgb(255, 0, 0);
    let actual_path = dir.path().join("actual.png");
    let expected_path = dir.path().join("expected.png");
    actual
        .save_png(&background, &left, &right, &actual_path)
        .unwrap();
    expected
        .save_png(&background, &left, &right, &expected_path)
        .unwrap();
    assert_eq!(
        std::fs::read(actual_path).unwrap(),
        std::fs::read(expected_path).unwrap(),
        "Decoded waveform should match the expected channel peaks"
    );
}

/// Verifies sample conversion, peak aggregation, and channel placement across packets.
///
/// Each 4096-frame input maps to 16 columns of 256 frames.  Sparse impulses at
/// opposite ends of each column distinguish peak detection from averaging and
/// catch dropped boundary samples.  Mono peaks must be mirrored around the center;
/// stereo peaks must remain in their separate upper and lower channel regions.
#[test]
fn test_mono_and_stereo_peaks_across_packets() {
    for channels in [1, 2] {
        let mut samples = Vec::new();
        let mut expected = new_image();
        for pixel in 0..16 {
            // Left/mono peaks cycle through 25%, 50%, and 75% amplitude.
            let peak = ((pixel % 3 + 1) * 8192) as i16;
            for frame in 0..256 {
                samples.push(if frame == 255 { peak } else { 0 });
                if channels == 2 {
                    // Negative right-channel peaks verify absolute-value handling.
                    samples.push(if frame == 0 { -8192 } else { 0 });
                }
            }
            // Signed 16-bit PCM converts to f32 by dividing by 32768.
            let amplitude = f32::from(peak) / 32768.0;
            if channels == 2 {
                expected.draw_point(pixel, amplitude, 0.25);
            } else {
                expected.draw_point_mono(pixel, amplitude);
            }
        }
        let file = write_wave(channels, &samples);
        let mut actual = new_image();
        process_audio_file(file.path(), &mut actual, 16).unwrap();

        // Compare every column, including the final one after the last packet.
        assert_same_image(&actual, &expected);
    }
}

/// Guards against a zero samples-per-pixel count when a clip has fewer frames
/// than image columns.  Each sample should occupy one column without underflow.
#[test]
fn test_audio_shorter_than_image_width() {
    // Three mono frames are fewer than the minimum image width of 16.
    let file = write_wave(1, &[16384, -8192, 24576]);
    let mut actual = new_image();
    process_audio_file(file.path(), &mut actual, 16).unwrap();
    let mut expected = new_image();

    // Absolute amplitudes are 50%, 25%, and 75%; columns 3 through 15 stay blank.
    expected.draw_point_mono(0, 0.5);
    expected.draw_point_mono(1, 0.25);
    expected.draw_point_mono(2, 0.75);
    assert_same_image(&actual, &expected);
}

/// Ensures packet-reading failures are propagated rather than mistaken for normal
/// end-of-stream, which would otherwise report success with an incomplete image.
#[test]
fn test_truncated_audio_error() {
    let file = write_wave(1, &[8192; 4096]);

    // Keep the 44-byte WAV header and one byte of an incomplete 16-bit sample.
    // The valid header still advertises 4096 frames, forcing a payload read error.
    file.as_file().set_len(45).unwrap();
    let mut image = new_image();
    assert!(
        process_audio_file(file.path(), &mut image, 16).is_err(),
        "Truncated audio should return an error instead of a partial waveform"
    );
}
