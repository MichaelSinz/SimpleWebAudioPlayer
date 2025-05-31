# Waver Architecture (Swift)

This document explains the high-level architecture of the Swift implementation of Waver and the rationale behind key design decisions.

## Overview

Waver follows a streamlined, single-file architecture that handles the entire audio-to-waveform conversion pipeline:

1. **Argument Parsing**: Process and validate command-line arguments
2. **Audio Processing**: Stream and decode audio files, extract amplitude data
3. **Image Generation**: Create waveform visualizations, output PNG files
4. **File Management**: Handle multiple files or directories in parallel

The implementation is designed specifically for macOS, leveraging Apple frameworks for audio processing and image generation.

## Core Components

### Command Line Interface

The Swift implementation uses the `ArgumentParser` framework to define command-line arguments with built-in validation:

- **Custom Argument Validation**: Validates dimensions, colors, and other parameters
- **Error Types**: Uses `ArgumentError` for validation issues
- **Color Parsing**: Supports RGB, RRGGBB, and RRGGBBAA hex formats

### Audio Processing

The audio processing component uses AVFoundation to implement a streaming approach:

- **AVAudioFile**: Core component for streaming audio data without loading entire files
- **Buffer Management**: Configurable buffer size (default: 65535 frames) for optimal performance
- **Channel Processing**: Supports mono and stereo audio with separate visualization for each channel

### Image Generation

Image generation uses Core Graphics to create PNG visualizations:

- **CoreGraphics Context**: Used for drawing waveform visualization
- **Dynamic Rendering**: Maps audio samples to pixels dynamically based on audio length
- **Multi-Channel Support**: Different colors for left and right channels

### File Management

File handling supports both single files and recursive directory processing:

- **Directory Traversal**: Recursive handling of directory trees
- **Extension Filtering**: Process only files with specified extensions
- **Parallel Processing**: Uses Grand Central Dispatch for efficient multi-file processing

## Performance Considerations

### Audio Processing Efficiency

The Swift implementation focuses on processing efficiency in several ways:

- **Streaming Approach**: Files are processed in chunks, never loading the entire audio file into memory
- **Optimal Buffer Size**: Extensive performance testing determined that a buffer size of ~65KB provides the best balance between memory usage and processing speed
- **Memory Footprint**: Minimized by only keeping amplitude data for the current processing window

### Concurrency Model

The Swift implementation uses a sophisticated concurrency model for parallel file processing:

- **DispatchQueue**: Uses a concurrent dispatch queue for parallelism
- **DispatchGroup**: Coordinates completion of all processing tasks
- **Thread-Safe Error Handling**: Custom `ErrorList` class with locking mechanism to safely accumulate errors from parallel operations

## Key Design Decisions

### Single-File Architecture

Unlike the Rust version's modular structure, the Swift implementation uses a single-file design:

- **Pros**: Simplifies compilation, deployment, and code navigation
- **Cons**: Less separation of concerns, potentially harder to maintain as code grows

### Streaming vs. Buffering

The code includes detailed comments about buffer size selection with performance measurements:

```
Perf Tests - when processing a directory tree with 579 MP3 files:

Buffer Size:  4194303    131071     65535      32767      4095
----------------------------------- --------------------------------------
Real time:      9.35      8.79       8.64       8.77      9.07
User time:    139.18    137.43     135.36     136.81    136.76
Sys time:       4.12      0.77       0.75       1.02      5.11
Max memory:  2.8 GB    206 MB     162 MB     141 MB    123 MB
```

This data informed the selection of 65535 as the default buffer size, balancing:
- CPU cache efficiency (larger buffers cause cache misses)
- System call overhead (smaller buffers require more read operations)
- Memory usage (larger buffers consume more RAM)

### PNG Image Generation

The Swift implementation generates standard PNG files with 8-bit components.  The code includes a note about an optimization consideration:

```swift
// We were going to make the image and PNG as a 2-bit (4 color) image
// but MacOS libraries no longer seem to support that.  This results in
// PNG files that are a bit larger (about twice as large) as they would
// be if they were 2bpp.  But they are rather small anyway.
```

- **Limitation**: MacOS libraries no longer support the 2-bit (4 color) image format that would be ideal for waveforms
- **Impact**: Using 8-bit components results in larger file sizes than would be possible with a more optimized format
- **Mitigation**: For waveform visualizations, the files are still relatively small for most practical purposes

Since waveforms typically only need three colors (background, left channel, right channel), a more specialized image format could potentially reduce file sizes.  However, the standard PNG format provides good compatibility and sufficient quality for the intended purpose.

### Platform Considerations

The Swift implementation is specifically designed for macOS:

- **AVFoundation**: Uses Apple's audio framework, limiting cross-platform potential
- **Core Graphics**: Uses Apple's graphics library for PNG generation
- **Grand Central Dispatch**: Uses Apple's concurrency framework

## Error Handling Strategy

The implementation uses a structured error handling approach:

- **Custom Error Types**:
  - `ArgumentError`: For command-line argument validation issues
  - `GenerationError`: For runtime processing errors

- **Thread-Safe Error Collection**: Custom `ErrorList` class with locking mechanism for collecting errors from parallel operations

- **Proper Propagation**: Errors bubble up through the processing chain and are presented to the user

## Limitations and Considerations

- **Platform Dependency**: Limited to macOS due to reliance on Apple frameworks
- **Image Format**: No support for optimized 2-bit color depth due to macOS library limitations
- **AVAudioFile Length Inaccuracy**: Special handling for premature end-of-file conditions in certain audio formats (noted in code comments)