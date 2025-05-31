# Waver Performance Optimizations

This document explains the PNG size optimization introduced in waver3.

## The Problem

Our waveforms only require 3 colors at most (background, left channel,
right channel), we don't need true color images.

In the Swift version, we were using the Apple media libraries for generating
and writing the PNG image file.  This restricted us to true-color PNG files as
the current MacOS libraries do not support generating indexed color images,
let alone 2-bits per pixel indexed color.  While the compression does a great
job of handling repetition, it is hard to actually ignore all of that extra
data that is not actually needed.  With the Rust version, the PNG crate lets
us do 2-bits per pixel images so that is what we do.

This not only saves significantly in resulting file size, it also reduces
memory required internally while running.  We have roughly 1/16th the memory
requirement for the image since rather than 4-bytes per pixel we are at 4 pixels
per byte (2 bits per pixel).

- The palette only needs 4 entries (0, 1, 2, 3) (and #3 is not used)
- Each pixel required 4 full bytes, even though we only need 2 bits per pixel

## The Solution: 2-bit Pixel Depth

The Rust version implements a 2-bit pixel depth optimization that can reduce
PNG file sizes significantly from the Swift version, even with the compression.

1. **2-bit Color Depth**: We now use 2 bits per pixel, allowing for 4 possible color values (0-3), which is sufficient for our 3 colors.

2. **Pixel Packing**: Four 2-bit pixels are packed into a single byte:
   ```
   +--------+--------+--------+--------+
   | 2 bits | 2 bits | 2 bits | 2 bits |
   | pixel1 | pixel2 | pixel3 | pixel4 |
   +--------+--------+--------+--------+
   ```

3. **Same Visual Quality**: This change doesn't affect the visual quality of the waveforms since we're still representing the exact same colors.

## Implementation Details

1. **Internal Representation**:
   - Since the background is 0 and the left and right channels are 1 and 2
     we can use simple bit-or operations to render the pixels
   - During waveform generation, we just bit-or in the appropriate pixel
   - If there is ever a collision between left and right channels, we get
     a color value of 3.  This is fine as it is still valid 2-bits per pixel

2. **PNG Format Configuration**:
   - PNG format natively supports 1, 2, 4, and 8-bit color depths
   - We use `BitDepth::Two` to set 2-bit color depth
   - We still use indexed color mode with a small 4-color palette

## Expected Performance Impact

1. **File Size**:
   - Approximately 75% reduction in file size
   - Example: A 2048×128 waveform that was ~80KB might now be ~20KB

2. **Memory Usage**:
   - Significantly less memory needed for the image data
   - We don't need to copy the image data during save

3. **CPU Impact**:
   - Small CPU cost to get the right bit set for the x coordinate
   - Small CPU cost in that we read-modify-write to a byte (bitwise "or")
   - Small memory bandwidth savings since we write to so much less memory (1/16)
   - Relatively small cost in extra complexity

4. **Benefits**:
   - Smaller files for storage and sharing
   - Faster file I/O operations
   - Less bandwidth when transferring files

## Use Cases That Benefit Most

This optimization is most noticeable for:
- Large waveform images (high width/height)
- Batch processing of many audio files
- Applications where storage space or bandwidth is limited

## Testing Results

Our benchmarking shows that the pixel packing operation is very fast, adding
only a few milliseconds of processing time even for large batches of images,
while reducing file sizes significantly.  (Due to compression, the savings are
only roughly 75% but that is significant)