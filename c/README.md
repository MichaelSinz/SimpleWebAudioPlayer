# Waver (C)

Waver is a C program for generating waveform visualizations from audio files.  It creates optimized PNG images that represent the audio amplitude over time, similar to what you might see in an audio editing application.

> **NOTE:** This code was generated, for the most part, via an [AI coding tool in a box](https://github.com/MichaelSinz/Boxed) along with many hours of my giving it prompts and updates on what it did wrong or needed to address.  It was also given, as input, the original Swift and Rust versions that I had written.  I will say, that with the right set of prompts and careful knowledge of what is quality code, it was possible to make this AI tool actually produce reasonably good code.  The performance of the C version is almost as fast as the Rust version but it consumes significantly more memory!  It also required very specific prompting to help get it to not have buffer overflow errors that could happen.  (Like asking it to use strncpy and to check if truncation happened, etc).  I also had to fix the fractional sample management by hand but that was minor.  With that change, it produces exactly the same image (but slightly different png file) as the Rust version.  The beautiful thing about Rust is that as long as you stay in normal strict safe mode, most all of these errors are not possible and would be caught by the compiler.

## Features

- MP3 audio file support via minimp3 (header-only library included)
- Support for both mono and stereo audio visualization
- Customizable colors for left/right channels and background
- Transparency support for background color
- Parallel processing for fast batch processing of multiple files
- Directory traversal for batch processing
- Command-line options to control image dimensions and appearance
- Memory-efficient streaming audio processing (but many times worse than Rust!)
- Optimized 2-bit indexed color PNG output for small file sizes

## Building

### Requirements

- C compiler with C11 support
- Standard C library
- Math library (-lm)
- zlib development library (for PNG compression)
- POSIX threads library (for parallel processing)

### Build Instructions

```bash
# Building using make
make

# Clean and rebuild
make clean && make
```

## Running

Once built, you can run the program as follows:

```bash
# Basic usage - generate waveforms for all MP3 files in current directory
./waver *.mp3

# Process files in a directory
./waver music_directory/

# Customize output with options
./waver --width 1024 --height 256 --left-color ff0000 --right-color 0000ff song.mp3
```

## Usage

```
Usage: waver [options] audio_files...

Generate waveform visualizations from audio files.

Options:
  --width <pixels>          Width of the output image (default: 2048)
  --height <pixels>         Height of the output image, must be even (default: 128)
  --left-color <color>      Color for left channel (default: 00ff99)
  --right-color <color>     Color for right channel (default: 99ff00)
  --background-color <color> Background color (default: ffffff00)
  -o, --output-filename <file> Output file name (only in single-file mode)
  --file-extensions <ext>   Comma-separated list of audio file extensions (default: mp3)
  --threads <number>        Number of worker threads (default: auto)
  --dry-run                 Perform actions without generating files
  --overwrite               Overwrite existing output files
  --quiet                   Suppress most output
  --verbose                 Print additional information
  -h, --help                Display this help message

Colors can be specified in RGB, RRGGBB, or RRGGBBAA hex format.
```

## Examples

Generate a waveform for a single MP3 file:
```bash
./waver song.mp3
```

Generate waveforms for all MP3 files in a directory:
```bash
./waver music_directory/
```

Customize the waveform appearance:
```bash
./waver --width 1024 --height 256 --left-color ff0000 --right-color 0000ff song.mp3
```

Process files in parallel with a specific number of threads:
```bash
./waver --threads 4 music_directory/
```

## Implementation Notes

This implementation focuses on:
- Low memory usage with streaming audio processing
- Simple and clear C code structure
- Minimal dependencies
- Parallel processing for speed
- Optimized PNG output with minimal file size

The program uses an optimized 2-bit indexed color PNG format, which provides significantly smaller file sizes compared to standard RGB/RGBA formats, while still supporting transparency for the background.  This matches what the Rust version does.

## Performance

The implementation is designed to efficiently process audio files by:
- Streaming audio data rather than loading entire files into memory
- Optimizing memory usage for waveform generation
- Processing multiple files in parallel using a thread pool
- Using an optimized 2-bit indexed color format for PNG output

## Architecture

The program follows a data processing pipeline:
1. Parse and validate command-line arguments
2. Collect audio files to process
3. Create a thread pool for parallel processing
4. Process each file, streaming the audio and generating a waveform image
5. Save the output in optimized PNG format

## Credits

This implementation uses the following libraries:
- minimp3: https://github.com/lieff/minimp3 (for MP3 decoding)
- zlib: https://www.zlib.net/ (for PNG compression)

## License

This software is distributed under the MIT License.