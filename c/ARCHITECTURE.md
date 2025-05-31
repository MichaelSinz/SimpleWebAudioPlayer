# Waver C Architecture

This document explains the high-level architecture of the Waver C implementation
and the rationale behind key design decisions.

## Overview

Waver follows a data processing pipeline architecture with three main stages:

1. **Input Processing**: Parse and validate command-line arguments, identify audio files
2. **Audio Processing**: Stream and decode audio files, extract amplitude data
3. **Image Generation**: Create waveform visualizations, output PNG files

## Key Components

### CLI Module (`src/cli.c`)
Handles command-line argument parsing and validation.

- **waver_args_t**: Main arguments structure
- **process_file()**: Process individual files
- **waver_process_files()**: Entry point for file processing with parallel support

### Thread Pool Module (`src/threadpool.c`)
Manages parallel processing of multiple files.

- **threadpool_t**: Thread pool and task queue implementation
- **worker_thread()**: Thread function for processing files in parallel
- **waver_process_files_parallel()**: Parallel implementation of file processing

### Audio Module (`src/audio.c`)
Processes audio files in a streaming fashion to minimize memory usage.

- **waver_generate_waveform()**: Main entry point for waveform generation
- **process_audio_file()**: Streams audio data without buffering entire files
- Uses minimp3 library for MP3 decoding

### Image Module (`src/image.c`)
Manages waveform visualization and internal image representation.

- **waver_image_t**: Core structure for waveform generation
- **waver_image_draw_point/waver_image_draw_point_mono**: Render individual points of the waveform
- Uses 2-bit per pixel internal representation

### PNG Encoding Module (`src/optimized_png.c`)
Handles optimized PNG output with minimal file size.

- **waver_image_save_optimized_png()**: Creates optimized 2-bit indexed color PNG files
- Custom PNG encoder using zlib for compression
- Creates palette-based PNGs with transparency support

## Design Decisions

### Streaming vs. Buffering
Streaming audio data provides significant benefits over loading entire files:
- Reduces memory usage dramatically
- Allows processing of large audio files with constant memory usage
- Improves performance by avoiding large memory allocations

### 2-bit Internal Representation
We use a 2-bit per pixel representation internally for efficiency:
- Only need 3 colors: background, left channel, right channel
- Reduces memory usage for large images (4 pixels per byte)
- Simplifies generation of optimized PNG output

### Optimized PNG Format
For output, we chose a 2-bit indexed color PNG format:
- File sizes are dramatically smaller than 32-bit RGBA PNGs
- Support for transparency in the background color
- Direct mapping from our internal representation
- Efficiently compressible with zlib

### Parallel Processing
To improve performance with multiple files:
- Thread pool implementation for concurrent file processing
- Automatically scales to use all available CPU cores
- Significant speed improvement when processing many files

### Minimalist Dependencies
The implementation depends only on:
- Standard C library
- Math library
- zlib for PNG compression
- minimp3 header-only library (included)
- POSIX threads for parallel processing

This ensures the program can be compiled on virtually any platform with minimal setup.

## Performance Considerations

### Audio Processing
- **Streaming Approach**: Files are processed in a streaming fashion
- **Memory Efficiency**: Only keeps maximum amplitude values for the current pixel

### Image Generation
- **2-bit Color Depth**: Uses 2 bits per pixel internally
- **Pixel Packing**: Four 2-bit pixels are packed into each byte

### Parallel Processing
- **Thread Pool**: Uses multiple worker threads for file processing
- **CPU Scaling**: Auto-detection of CPU cores for optimal performance

## Future Improvements

1. **Additional Audio Formats**: Support for WAV, FLAC, and other formats
2. **Custom Scaling**: Options for logarithmic amplitude scaling
3. **Advanced Visualization Options**: Peak markers, grid lines, etc.
4. **Resource Management**: Dynamic thread creation based on system load