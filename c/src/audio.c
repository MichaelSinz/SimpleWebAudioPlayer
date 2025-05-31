/**
 * @file audio.c
 * @brief Audio processing functionality for waveform generation
 */

// MINIMP3_IMPLEMENTATION and MINIMP3_NO_SIMD are defined in the Makefile

#include "waver.h"
#include "minimp3.h"
#include "minimp3_ex.h"

#include <stdio.h>
#include <string.h>
#include <stdint.h>

// Define INT16_MIN if not defined
#ifndef INT16_MIN
#define INT16_MIN (-32768)
#endif

/**
 * @brief Absolute value of a PCM sample
 *
 * This function converts a PCM sample into its positive only value
 * It handles the special case of sample being INT16_MIN value.
 *
 * @param sample The signed PCM sample
 * @return The absolue value of the signed PCM sample
 */
static unsigned int abs_pcm(int16_t sample) {
    if (sample == INT16_MIN) {
        return 32767;
    }
    return (sample < 0) ? -sample : sample;
}

/**
 * @brief Process an audio file and generate a waveform visualization
 *
 * This function decodes the audio file frame by frame and immediately processes
 * each frame to generate the waveform image, without storing all audio data in memory.
 *
 * @param input_path Path to the input audio file
 * @param image The waveform image to draw into
 * @param width Width of the output image in pixels
 * @return true if successful, false otherwise
 */
static bool process_audio_file(const char *input_path, waver_image_t *image, unsigned int width) {
    mp3dec_ex_t mp3d;
    if (mp3dec_ex_open(&mp3d, input_path, MP3D_SEEK_TO_SAMPLE)) {
        return false;
    }

    // Get channel information
    unsigned int channel_count = mp3d.info.channels;
    if (channel_count > 2) {
        channel_count = 2; // Limit to stereo
    }

    // Get total number of frames for scaling calculation
    uint64_t total_samples = mp3d.samples / mp3d.info.channels;
    if (total_samples == 0) {
        mp3dec_ex_close(&mp3d);
        return false;
    }

    // Calculate samples per pixel ratio for even distribution
    uint64_t samples_per_pixel = total_samples / width;
    uint64_t remainder = total_samples % width;

    // Use a reasonably sized buffer for streaming
    const size_t BUFFER_SIZE = 4096;
    mp3d_sample_t pcm[BUFFER_SIZE];

    // Initialize tracking variables
    uint64_t sample_count = samples_per_pixel;
    uint64_t sample_remainder = 0;

    unsigned int current_pixel = 0;  // The pixel we're currently collecting samples for
    unsigned int left = 0;
    unsigned int right = 0;


    // Process audio stream packet by packet
    size_t samples_read;
    while ((samples_read = mp3dec_ex_read(&mp3d, pcm, BUFFER_SIZE)) > 0) {
        // Process each frame in the current packet
        for (size_t i = 0; i < samples_read; i += channel_count) {
            // Process each channel's sample
            unsigned int sample = abs_pcm(pcm[i]);
            if (sample > left) {
                left = sample;
            }

            if (channel_count > 1) {
                sample = abs_pcm(pcm[i + 1]);
                if (sample > right) {
                    right = sample;
                }
            }

            sample_count--;
            if (sample_count == 0) {
                // Draw the current pixel with its max values
                if (channel_count > 1) {
                    waver_image_draw_point(image, current_pixel, left, right);
                } else {
                    waver_image_draw_point_mono(image, current_pixel, left);
                }

                // Reset max values for the new pixel
                left = 0;
                right = 0;

                // Update current pixel
                current_pixel++;
                sample_count = samples_per_pixel;
                sample_remainder += remainder;
                if (sample_remainder >= width) {
                    sample_remainder -= width;
                    sample_count++;
                }
            }
        }
    }

    // Draw the final pixel if we've collected any samples for it
    if (sample_count > 0) {
        if (channel_count > 1) {
            waver_image_draw_point(image, current_pixel, left, right);
        } else {
            waver_image_draw_point_mono(image, current_pixel, left);
        }
    }

    mp3dec_ex_close(&mp3d);
    return true;
}

/**
 * @brief Generate a waveform from an audio file
 *
 * @param input_path Path to the input audio file
 * @param output_path Path to save the output PNG
 * @param args Command-line arguments
 * @return true if successful, false otherwise
 */
bool waver_generate_waveform(
    const char *input_path,
    const char *output_path,
    const waver_args_t *args
) {
    if (!input_path || !output_path || !args) {
        return false;
    }

    // Skip if output exists and overwrite isn't allowed
    if (!args->overwrite) {
        FILE *test_file = fopen(output_path, "rb");
        if (test_file) {
            fclose(test_file);
            if (args->verbose) {
                waver_print_stderr(args, "Output file '%s' already exists - use --overwrite", output_path);
            }
            return true; // Not an error, but skipped
        }
    }

    // Generate the image buffer
    waver_image_t *image = waver_image_new(args->width, args->height);
    if (!image) {
        waver_print_stderr(args, "Failed to create image buffer");
        return false;
    }

    // Process audio file and generate waveform
    bool success = process_audio_file(input_path, image, args->width);
    if (!success) {
        waver_print_stderr(args, "Failed to process audio file: %s", input_path);
        waver_image_free(image);
        return false;
    }

    // Save or log the result
    if (!args->dry_run) {
        success = waver_image_save_optimized_png(
            image,
            &args->bg_color,
            &args->left_color,
            &args->right_color,
            output_path
        );

        if (success) {
            waver_print_stdout(args, "Created %s", output_path);
        } else {
            waver_print_stderr(args, "Failed to save PNG file: %s", output_path);
        }
    } else if (args->verbose) {
        waver_print_verbose(args, "DryRun %s", output_path);
        success = true;
    }

    waver_image_free(image);
    return success;
}