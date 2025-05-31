# Waver (Rust)

Waver is a high-performance tool for generating waveform visualizations from audio files.

## Features

- Generate optimized PNG visualizations of audio waveforms
- Support for various audio formats (MP3, AAC, etc.)
- Customizable colors for left and right channels
- Adjustable output dimensions
- Parallel processing of multiple files
- Support for directory recursion to process many audio files at once
- Space-efficient 2-bit color depth for smaller file sizes

## Usage

```bash
# Basic usage - generate waveform PNG from an audio file
waver my_audio_file.mp3

# Process multiple files or directories
waver music/*.mp3 podcasts/

# Customize colors and dimensions
waver --width 1024 --height 256 --left-color ff0000 --right-color 0000ff my_audio_file.mp3

# Process all FLAC and MP3 files in a directory
waver --file-extensions mp3,flac my_music_directory/
```

## Command Line Options

```
Options:
  --width <WIDTH>                    Width of the output image in pixels [default: 2048]
  --height <HEIGHT>                  Height of the output image in pixels (must be even) [default: 128]
  --left-color <LEFT_COLOR>          Color for left channel (RGB, RRGGBB, or RRGGBBAA) [default: 00ff99]
  --right-color <RIGHT_COLOR>        Color for right channel (RGB, RRGGBB, or RRGGBBAA) [default: 99ff00]
  --background-color <BACKGROUND_COLOR>  Background color (RGB, RRGGBB, or RRGGBBAA) [default: ffffff00]
  --output-filename <OUTPUT_FILENAME>  Output PNG file name (only in single-file mode)
  --file-extensions <FILE_EXTENSIONS>  Comma-separated list of audio file extensions [default: mp3]
  --dry-run                          Perform actions without generating files
  --overwrite                        Overwrite existing output files
  --quiet                            Suppress most output
  --verbose                          Print additional information
  -h, --help                         Print help
  -V, --version                      Print version
```

## Examples

### Basic Waveform

Generate a standard waveform visualization:

```bash
waver input.mp3
```

This creates a file named `input.mp3.png` with default settings.

### Custom Colors

Create a waveform with red for the left channel and blue for the right channel:

```bash
waver --left-color FF0000 --right-color 0000FF input.mp3
```

### Custom Dimensions

Create a wide, short waveform:

```bash
waver --width 3000 --height 100 input.mp3
```

### Process All Files in a Directory

Process all MP3 files in the current directory:

```bash
waver --file-extensions mp3 .
```

## Technical Details

This version uses several optimizations to generate highly efficient PNG files:

1. **2-bit Pixel Depth**: Since waveforms only need 3 colors (background, left channel, right channel), we use 2-bit color depth to reduce file size.

2. **Pixel Packing**: Four 2-bit pixels are packed into each byte, optimizing memory usage and file size.

3. **Efficient PNG Encoding**: Uses optimal PNG filter types and maximum compression.

4. **Streaming Audio Processing**: Processes audio data on-the-fly without buffering entire files in memory.

## Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/waver.git
cd waver

# Build in release mode
cargo build --release

# Run the program
cargo run --release -- your_audio_file.mp3
```

## License

This software is distributed under the MIT License.