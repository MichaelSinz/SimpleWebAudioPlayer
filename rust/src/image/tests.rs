#[cfg(test)]
mod tests {
    use crate::cli::{Width, Height};
    use crate::image::{WaveImage, Channel};

    /// Helper extension trait for testing WaveImage
    trait WaveImageTest {
        /// Gets the pixel channel at the given coordinates
        fn get_pixel(&self, x: u32, y: u32) -> Channel;

        /// Verifies that a vertical line of pixels has the expected channel value
        fn assert_vertical_line(&self, x: u32, y_start: u32, y_end: u32, expected_channel: Channel);

        /// Verifies that a region of pixels is set to a specific channel
        fn assert_region(&self, x_start: u32, x_end: u32, y_start: u32, y_end: u32, expected_channel: Channel);

        /// Verifies that a region of pixels is set to the background channel
        fn assert_region_is_background(&self, x_start: u32, x_end: u32, y_start: u32, y_end: u32);
    }

    impl WaveImageTest for WaveImage {
        fn get_pixel(&self, x: u32, y: u32) -> Channel {
            // Return background for out-of-bounds coordinates
            if x >= self.width() || y >= self.height {
                return Channel::Background;
            }

            // Calculate the byte offset and bit position
            let byte_offset = x >> 2; // x / 4
            let bit_position = 2 * (x & 3); // 2 * (x % 4)
            let idx = (byte_offset + y * self.line_width) as usize;

            // Extract the 2-bit value from the byte
            let pixel_bits = (self.pixels[idx] >> bit_position) & 0x3;
            Channel::from(pixel_bits)
        }

        fn assert_vertical_line(&self, x: u32, y_start: u32, y_end: u32, expected_channel: Channel) {
            for y in y_start..y_end {
                assert_eq!(
                    self.get_pixel(x, y),
                    expected_channel,
                    "Pixel at ({}, {}) should be {:?}",
                    x, y, expected_channel
                );
            }
        }

        fn assert_region(&self, x_start: u32, x_end: u32, y_start: u32, y_end: u32, expected_channel: Channel) {
            for x in x_start..x_end {
                for y in y_start..y_end {
                    assert_eq!(
                        self.get_pixel(x, y),
                        expected_channel,
                        "Pixel at ({}, {}) should be {:?}",
                        x, y, expected_channel
                    );
                }
            }
        }

        fn assert_region_is_background(&self, x_start: u32, x_end: u32, y_start: u32, y_end: u32) {
            self.assert_region(x_start, x_end, y_start, y_end, Channel::Background);
        }
    }

    #[test]
    fn test_image_creation() {
        // Test different image sizes
        let test_sizes = [(16, 6), (100, 100), (2048, 128)];

        for (width_val, height_val) in test_sizes {
            let width = Width::new(width_val).unwrap();
            let height = Height::new(height_val).unwrap();

            let image = WaveImage::new(width, height);

            // Check dimensions
            assert_eq!(image.width(), width_val, "Image width should match requested width");
            assert_eq!(image.height, height_val, "Image height should match requested height");
            assert_eq!(image.center, height_val / 2, "Center should be half the height");

            // Check that all pixels are initialized to background
            image.assert_region_is_background(0, width_val, 0, height_val);
        }
    }

    #[test]
    fn test_mono_point_drawing() {
        let width = Width::new(100).unwrap();
        let height = Height::new(60).unwrap();
        let mut image = WaveImage::new(width, height);

        // Draw a simple point in the middle
        let x = 50;
        let amplitude = 0.5; // 50% amplitude
        image.draw_point_mono(x, amplitude);

        // Calculate expected affected region
        let center = 30; // For a height of 60, center is 30
        let wave_height = (center as f32 * amplitude + 0.5) as u32; // Should be about 15
        let y_start = center - wave_height; // Should be about 15
        let y_end = center + wave_height;   // Should be about 45

        // Verify the vertical line was drawn correctly
        image.assert_vertical_line(x, y_start, y_end, Channel::Left);

        // Verify areas outside the drawn line are still background
        if x > 0 {
            image.assert_vertical_line(x-1, 0, height.value(), Channel::Background);
        }
        image.assert_vertical_line(x+1, 0, height.value(), Channel::Background);
    }

    #[test]
    fn test_stereo_point_drawing() {
        let width = Width::new(100).unwrap();
        let height = Height::new(60).unwrap();
        let mut image = WaveImage::new(width, height);

        // Draw a stereo point in the middle
        let x = 50;
        let left_amplitude = 0.6;  // 60% amplitude for left channel
        let right_amplitude = 0.4; // 40% amplitude for right channel
        image.draw_point(x, left_amplitude, right_amplitude);

        // Calculate expected affected regions
        let center = 30; // For a height of 60, center is 30
        let left_height = (center as f32 * left_amplitude + 0.5) as u32; // Should be about 18
        let left_start = center - left_height; // Should be about 12

        let right_height = (center as f32 * right_amplitude + 0.5) as u32; // Should be about 12
        let right_end = center + right_height; // Should be about 42

        // Verify the left channel (above center) was drawn correctly
        image.assert_vertical_line(x, left_start, center, Channel::Left);

        // Verify the right channel (below center) was drawn correctly
        image.assert_vertical_line(x, center, right_end, Channel::Right);

        // Verify areas outside the drawn lines are still background
        if x > 0 {
            image.assert_vertical_line(x-1, 0, height.value(), Channel::Background);
        }
        image.assert_vertical_line(x+1, 0, height.value(), Channel::Background);

        // Top and bottom should still be background
        if left_start > 0 {
            image.assert_vertical_line(x, 0, left_start, Channel::Background);
        }
        image.assert_vertical_line(x, right_end, height.value(), Channel::Background);
    }

    #[test]
    fn test_boundary_conditions() {
        let width = Width::new(100).unwrap();
        let height = Height::new(60).unwrap();
        let mut image = WaveImage::new(width, height);

        // 1. Test drawing at x=0
        image.draw_point_mono(0, 0.5);
        image.assert_vertical_line(0, 15, 45, Channel::Left);

        // 2. Test drawing at x=width-1
        image.draw_point_mono(99, 0.5);
        image.assert_vertical_line(99, 15, 45, Channel::Left);

        // 3. Test drawing at out-of-bounds coordinates (should have no effect)
        let initial_pixels = image.pixels.clone(); // Save current state

        // Try drawing outside the image
        image.draw_point_mono(100, 0.5);
        image.draw_point(100, 0.5, 0.5);
        image.draw_point_mono(1000, 0.5);

        // Verify the image hasn't changed
        assert_eq!(
            image.pixels,
            initial_pixels,
            "Drawing outside image bounds should not modify the image"
        );
    }

    #[test]
    fn test_amplitude_clipping() {
        let width = Width::new(100).unwrap();
        let height = Height::new(60).unwrap();
        let mut image = WaveImage::new(width, height);

        // 1. Test with zero amplitude (should draw nothing since wave_height = 0)
        image.draw_point_mono(10, 0.0);
        image.assert_vertical_line(10, 0, 60, Channel::Background); // No pixels should be set

        // 2. Test with negative amplitude (should be treated the same as zero)
        image.draw_point_mono(11, -0.5);
        image.assert_vertical_line(11, 0, 60, Channel::Background); // No pixels should be set

        // 3. Test with very negative amplitude (should be treated the same as zero)
        image.draw_point_mono(12, -5.0);
        image.assert_vertical_line(12, 0, 60, Channel::Background); // No pixels should be set

        // 4. Test with very small but non-zero amplitude
        image.draw_point_mono(15, 0.01);
        // The calculation is wave_height = (30 * 0.01 + 0.5) as u32 = 0
        // So we still expect no pixels to be set
        image.assert_vertical_line(15, 0, 60, Channel::Background);

        // 5. Test with amplitude that should set pixels (0.1)
        // wave_height = (30 * 0.1 + 0.5) as u32 = 3
        image.draw_point_mono(16, 0.1);
        image.assert_vertical_line(16, 27, 33, Channel::Left); // Should have 6 pixels set

        // 6. Test with full amplitude (should fill from top to bottom)
        image.draw_point_mono(20, 1.0);
        image.assert_vertical_line(20, 0, 60, Channel::Left); // Entire column

        // 7. Test with slightly above full amplitude (should be clipped to image bounds)
        image.draw_point_mono(25, 1.1);
        image.assert_vertical_line(25, 0, 60, Channel::Left); // Entire column

        // 8. Test with oversized amplitude (should be clipped to image bounds)
        image.draw_point_mono(30, 2.0);
        image.assert_vertical_line(30, 0, 60, Channel::Left); // Entire column

        // 9. Test with extremely large amplitude (should be clipped to image bounds)
        image.draw_point_mono(31, 1000.0);
        image.assert_vertical_line(31, 0, 60, Channel::Left); // Entire column

        // 10. Test stereo drawing with zero and negative values
        image.draw_point(40, 0.0, -0.2);
        image.assert_vertical_line(40, 0, 60, Channel::Background); // Should draw nothing

        // 11. Test stereo drawing with small values
        image.draw_point(45, 0.1, 0.1);
        image.assert_vertical_line(45, 27, 30, Channel::Left);  // Left channel above center
        image.assert_vertical_line(45, 30, 33, Channel::Right); // Right channel below center

        // 12. Test stereo drawing with mixed normal and excessive values
        image.draw_point(48, 0.5, 2.0);
        image.assert_vertical_line(48, 15, 30, Channel::Left);   // Left channel (50% amplitude)
        image.assert_vertical_line(48, 30, 60, Channel::Right);  // Right channel (clipped to max)

        // 13. Test stereo drawing with full amplitude
        image.draw_point(50, 1.0, 1.0);
        image.assert_vertical_line(50, 0, 30, Channel::Left);   // Left from top to center
        image.assert_vertical_line(50, 30, 60, Channel::Right); // Right from center to bottom

        // 14. Test stereo drawing with negative values
        image.draw_point(55, -0.5, -0.3);
        image.assert_vertical_line(55, 0, 60, Channel::Background); // Should draw nothing

        // 15. Test stereo drawing with mixed negative and positive values
        image.draw_point(58, -0.2, 0.2);
        image.assert_vertical_line(58, 0, 30, Channel::Background); // Left channel (negative, clipped to zero)
        image.assert_vertical_line(58, 30, 36, Channel::Right);     // Right channel (20% amplitude)
    }

    #[test]
    fn test_byte_alignment() {
        // Test pixels at different byte boundaries to ensure 2bpp packing works
        let width = Width::new(16).unwrap();
        let height = Height::new(6).unwrap();
        let mut image = WaveImage::new(width, height);

        // Draw at positions that cross byte boundaries
        for x in 0..16 {
            image.draw_point_mono(x, 0.5);

            // Verify the pixel was set correctly
            image.assert_vertical_line(x, 1, 5, Channel::Left);

            // Clear the image
            let width = Width::new(16).unwrap();
            let height = Height::new(6).unwrap();
            image = WaveImage::new(width, height);
        }
    }

    #[test]
    fn test_pattern_drawing() {
        // Test drawing a simple pattern and verifying it
        let width = Width::new(32).unwrap();
        let height = Height::new(32).unwrap();
        let mut image = WaveImage::new(width, height);

        // Draw a sawtooth wave pattern
        for x in 0..32 {
            let amplitude = (x as f32) / 32.0;
            image.draw_point_mono(x, amplitude);
        }

        // Verify a few key points in the pattern
        // For x=0, amplitude=0.0, wave_height=(16*0.0+0.5) as u32 = 0
        image.assert_vertical_line(0, 0, 32, Channel::Background);  // 0% amplitude draws nothing

        // For x=8, amplitude=0.25, wave_height=(16*0.25+0.5) as u32 = 4
        image.assert_vertical_line(8, 12, 20, Channel::Left);  // 25% amplitude

        // For x=16, amplitude=0.5, wave_height=(16*0.5+0.5) as u32 = 8
        image.assert_vertical_line(16, 8, 24, Channel::Left); // 50% amplitude

        // For x=24, amplitude=0.75, wave_height=(16*0.75+0.5) as u32 = 12
        image.assert_vertical_line(24, 4, 28, Channel::Left); // 75% amplitude

        // For x=31, amplitude=0.97, wave_height=(16*0.97+0.5) as u32 ~= 16
        image.assert_vertical_line(31, 0, 32, Channel::Left);  // ~97% amplitude

        // Test with negative amplitudes (should be treated as zero)
        let mut neg_image = WaveImage::new(width, height);
        // Draw a negative amplitude pattern
        for x in 0..32 {
            let amplitude = -((x as f32) / 32.0);  // Negative values from 0 to -1.0
            neg_image.draw_point_mono(x, amplitude);
        }

        // All pixels should remain background since negative amplitudes should be treated as zero
        neg_image.assert_region_is_background(0, 32, 0, 32);
    }

    #[test]
    fn test_memory_efficiency() {
        // Test the memory efficiency of the 2-bit-per-pixel format
        // Focusing on both boundary conditions and padding behavior

        // Constants for the test
        const STD_HEIGHT: u32 = 8;  // Standard height for width tests

        // Part 1: Test specific width boundaries around multiples of 4
        // Using Vec instead of fixed-size arrays to allow different sizes
        let test_boundaries: Vec<Vec<u32>> = vec![
            // Around 16 (multiple of 4)
            vec![16, 17, 18, 19, 20],
            // Around 32 (multiple of 4)
            vec![31, 32, 33, 34],
            // Around 64 (multiple of 4)
            vec![63, 64, 65, 66],
            // Around 120 (multiple of 4)
            vec![119, 120, 121, 122, 123, 124],
        ];

        // Test each boundary group
        for width_group in &test_boundaries {
            let mut line_widths = Vec::new();
            let mut memories = Vec::new();

            for &width in width_group {
                let image = WaveImage::new(
                    Width::new(width).unwrap(),
                    Height::new(STD_HEIGHT).unwrap()
                );

                // Calculate expected bytes per row: ceiling(width / 4)
                let expected_line_width = (width + 3) / 4;

                // Verify line width calculation
                assert_eq!(
                    image.line_width, expected_line_width,
                    "Width {} should use {} bytes per row", width, expected_line_width
                );

                // Total memory should be line_width * height
                assert_eq!(
                    image.pixels.len(), (image.line_width * STD_HEIGHT) as usize,
                    "Total memory for width {} should be line_width * height", width
                );

                line_widths.push((width, image.line_width));
                memories.push((width, image.pixels.len()));
            }

            // Check that all non-multiple-of-4 widths up to the next multiple of 4
            // use the same amount of memory as the next multiple of 4
            for i in 1..width_group.len() {
                let width = width_group[i];
                let prev_width = width_group[i-1];

                // Compare expected line widths - they should be based on ceiling(width/4)
                let expected_current = (width + 3) / 4;
                let expected_previous = (prev_width + 3) / 4;

                // Verify that line widths match our expectation
                assert_eq!(
                    line_widths[i].1, expected_current,
                    "Width {} should have line width {}", width, expected_current
                );

                // If both widths are in the same 4-pixel group, they should have the same memory usage
                if width / 4 == prev_width / 4 && expected_current == expected_previous {
                    assert_eq!(
                        memories[i].1, memories[i-1].1,
                        "Width {} should use same total memory as width {}", width, prev_width
                    );
                }
            }
        }

        // Part 2: Test various image dimensions to ensure memory calculation is correct
        let test_sizes = [
            // Minimum size
            (16, 6),
            // Various width and height combinations
            (17, 6), (20, 6), (23, 10), (32, 16), (100, 60),
            // Larger sizes
            (640, 480), (1024, 64),
        ];

        for (width_val, height_val) in test_sizes {
            let width = Width::new(width_val).unwrap();
            let height = Height::new(height_val).unwrap();

            let image = WaveImage::new(width, height);

            // Expected line width in bytes: ceiling(width / 4)
            let expected_line_width = (width_val + 3) / 4;

            // Expected total bytes: line_width * height
            let expected_bytes = expected_line_width as usize * height_val as usize;

            // Check that line width matches expected
            assert_eq!(
                image.line_width, expected_line_width,
                "Line width for {}x{} should be {} bytes",
                width_val, height_val, expected_line_width
            );

            // Check that total allocated memory matches expected
            assert_eq!(
                image.pixels.len(), expected_bytes,
                "Memory allocation for {}x{} should be exactly {} bytes",
                width_val, height_val, expected_bytes
            );

            // The only thing that matters is the correct calculation of bytes:
            // - line_width should be ceiling(width/4)
            // - total memory should be line_width * height

            // We've already asserted these core requirements above, so no additional
            // assertions are needed here
        }

        // Part 3: Test specific padding behavior for consecutive widths
        // Test that width of 17, 18, 19, and 20 all use the same memory
        let memories_for_range = (17..=20).map(|w| {
            let image = WaveImage::new(
                Width::new(w).unwrap(),
                Height::new(STD_HEIGHT).unwrap()
            );
            (w, image.pixels.len())
        }).collect::<Vec<_>>();

        // They should all use the same amount of memory (5 bytes per row * height)
        let first_memory = memories_for_range[0].1;
        for (width, memory) in &memories_for_range[1..] {
            assert_eq!(
                *memory, first_memory,
                "Width {} should use the same memory as width 17", width
            );
        }
    }
}

#[cfg(test)]
mod benchmarks {
    /// # Running Benchmarks
    ///
    /// ```bash
    /// cargo test --release -- --nocapture
    /// ```
    ///
    /// The `--release` flag ensures benchmarks run with optimizations.
    /// The `--nocapture` flag ensures output is displayed.

    use crate::cli::{Width, Height};
    use crate::image::WaveImage;
    use std::time::Instant;

    /// Run a benchmark for the given operation and return the duration and per-op time
    fn run_benchmark<F>(op: F) -> std::time::Duration
    where
        F: Fn(),
    {
        // Run the benchmark
        let start = Instant::now();
        op();
        let duration = start.elapsed();

        duration
    }

    #[test]
    fn benchmark_draw_point_performance() {
        const WIDTH_VAL: u32 = 2048;
        const HEIGHT_VAL: u32 = 128;
        const ITERATIONS: u32 = 1_000_000;
        const REPEAT_COUNT: usize = 10; // Run multiple times for more reliable results

        // Create Width and Height instances from raw values
        let width = Width::new(WIDTH_VAL).unwrap();
        let height = Height::new(HEIGHT_VAL).unwrap();

        println!("\n\nRUNNING IMAGE RENDERING BENCHMARKS\n");

        let mut mono_times = Vec::with_capacity(REPEAT_COUNT);
        let mut stereo_times = Vec::with_capacity(REPEAT_COUNT);

        // Run multiple iterations for more stable results
        for i in 0..=REPEAT_COUNT {
            // Test mono point drawing
            let mono_duration = run_benchmark(|| {
                let mut image = WaveImage::new(width, height);
                for i in 0..ITERATIONS {
                    let x = i % WIDTH_VAL;
                    let h = ((i % 9) + 1) as f32 / 10.0;
                    image.draw_point_mono(x, h);
                }
            });
            // Throw out the first run
            if i > 0 {
                mono_times.push(mono_duration);
            }

            // Test stereo point drawing
            let stereo_duration = run_benchmark(|| {
                let mut image = WaveImage::new(width, height);
                for i in 0..ITERATIONS {
                    let x = i % WIDTH_VAL;
                    let h = ((i % 9) + 1) as f32 / 10.0;
                    image.draw_point(x, h, h);
                }
            });
            // Throw out the first run
            if i > 0 {
                stereo_times.push(stereo_duration);
            }
        }

        // Calculate average times
        let avg_mono = mono_times.iter().sum::<std::time::Duration>() / REPEAT_COUNT as u32;
        let avg_stereo = stereo_times.iter().sum::<std::time::Duration>() / REPEAT_COUNT as u32;

        // Calculate min and max times
        let min_mono = mono_times.iter().min().unwrap();
        let max_mono = mono_times.iter().max().unwrap();
        let min_stereo = stereo_times.iter().min().unwrap();
        let max_stereo = stereo_times.iter().max().unwrap();

        println!("\n==================================================");
        println!("BENCHMARK SUMMARY (Average of {} runs)", REPEAT_COUNT);
        println!("==================================================");
        println!("Mono drawing:   {:.2?} per {} ops ({:.2?} per op)",
                 avg_mono, ITERATIONS, avg_mono / ITERATIONS);
        println!("  Range: {:.2?} to {:.2?}", min_mono, max_mono);
        println!("Stereo drawing: {:.2?} per {} ops ({:.2?} per op)",
                 avg_stereo, ITERATIONS, avg_stereo / ITERATIONS);
        println!("  Range: {:.2?} to {:.2?}", min_stereo, max_stereo);
        println!("==================================================");

        // Compare the performance
        println!("\n==================================================");
        println!("PERFORMANCE COMPARISON");
        println!("==================================================");
        let ratio = avg_stereo.as_secs_f64() / avg_mono.as_secs_f64();

        if ratio < 1.0 {
            // If ratio < 1.0, stereo is actually faster
            println!("Stereo is {:.2}x faster than mono", 1.0 / ratio);
        } else {
            println!("Stereo is {:.2}x slower than mono", ratio);
        }
        println!("==================================================\n");

    }


}