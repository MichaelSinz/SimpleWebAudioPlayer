# Waver (Swift)

Waver is a high-performance macOS tool for generating waveform visualizations from audio files.  This Swift implementation provides efficient streaming-based audio processing to create PNG waveform representations with minimal memory usage.

## Features

- Generate PNG visualizations of audio waveforms
- Support for various audio formats (MP3, AAC, etc.)
- Customizable colors for left and right channels
- Adjustable output dimensions
- Parallel processing of multiple files
- Support for directory recursion to process many audio files at once
- Memory-efficient streaming audio processing

## Requirements

- macOS 10.15 or later
- Swift 6.0 or later

## Installation

```bash
# Build the project
swift build -c release

# Copy it to your personal bin directory
cp .build/release/waver ~/bin
```

## Usage

```bash
# Basic usage - generate waveform PNG from an audio file
waver my_audio_file.mp3

# Process multiple files or directories
waver music/*.mp3 podcasts/

# Customize colors and dimensions
waver --width 1024 --height 256 --left-color ff0000 --right-color 0000ff my_audio_file.mp3

# Process all MP3 and M4A files in a directory
waver --file-extensions mp3,m4a my_music_directory/
```

## Command Line Options

```
Options:
  --width <WIDTH>                    Width of the output image in pixels [default: 2048]
  --height <HEIGHT>                  Height of the output image in pixels (must be even) [default: 128]
  --left-color <LEFT_COLOR>          Color for left channel in RGB/RRGGBB/RRGGBBAA hex [default: 00ff99]
  --right-color <RIGHT_COLOR>        Color for right channel in RGB/RRGGBB/RRGGBBAA hex [default: 99ff00]
  --background-color <BACKGROUND_COLOR>  Background color in RGB/RRGGBB/RRGGBBAA hex [default: ffffff00]
  -o, --output-filename <OUTPUT_FILENAME>  Output PNG file name (only in single-file mode)
  --file-extensions <FILE_EXTENSIONS>  File extensions to process when given a directory [default: mp3,m4a]
  --buffer-size <BUFFER_SIZE>        Buffer size to use while streaming audio (in frames) [default: 65535]
  --dry-run                          Do not write anything
  --overwrite                         Overwrite existing output files
  --quiet                            Quieter output
  --verbose                          Verbose - show overwrite warnings
  -h, --help                         Print help information
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

Process all MP3 and M4A files in the current directory:

```bash
waver --file-extensions mp3,m4a .
```

## Technical Details

This Swift implementation uses several techniques to efficiently process audio files:

1. **Streaming Audio Processing**: Processes audio data in configurable chunks without loading entire files into memory, allowing for processing of large audio files with minimal memory usage.

2. **Parallel Processing**: Uses Grand Central Dispatch to process multiple files in parallel, maximizing throughput on multi-core systems.

3. **Optimized Buffer Size**: Default buffer size (65535 frames) has been tuned for optimal performance based on real-world testing.

4. **True-Color PNG Files**: The Swift implementation generates standard true-color PNG files (8 bits per component).  While the code comments note that a more optimized 2-bit color depth would be ideal for waveforms, macOS libraries no longer support this option.  Nonetheless, the files remain relatively compact for waveform visualizations.

## License

This software is distributed under the MIT License.