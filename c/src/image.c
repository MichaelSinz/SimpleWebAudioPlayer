/**
 * @file image.c
 * @brief Waveform image generation functionality with optimized PNG output
 */

#include "waver.h"
#include <stdio.h>
#include <string.h>
#include <math.h>
#include <stdlib.h>

/**
 * @brief Convert a color index to the bit location based on the x coordinate
 *
 * @param color The 2-bit color
 * @param x The horizontal position of the pixel
 * @return The u8 with the color bits shifted to the correct location for 2-bpp
 */
static inline uint8_t draw_bits(uint8_t color, unsigned int x) {
    return (color & 3) << (2 * (x & 3));
}

/**
 * @brief Create a new waveform image
 *
 * @param width Width of the image in pixels
 * @param height Height of the image in pixels
 * @return New image or NULL on error
 */
waver_image_t *waver_image_new(unsigned int width, unsigned int height) {
    if (width < 16 || height < 6 || height % 2 != 0) {
        return NULL;
    }

    waver_image_t *image = calloc(1, sizeof(waver_image_t));
    if (!image) {
        return NULL;
    }

    // Calculate line width in bytes (for 2-bit pixels)
    unsigned int line_width = (width + 3) >> 2;  // Divide by 4, rounding up

    image->width = width;
    image->height = height;
    image->center = height / 2;
    image->line_width = line_width;

    // Allocate memory for pixels
    image->pixels = calloc(line_width * height, sizeof(uint8_t));
    if (!image->pixels) {
        free(image);
        return NULL;
    }

    return image;
}

/**
 * @brief Free memory allocated for an image
 *
 * @param image Image to free
 */
void waver_image_free(waver_image_t *image) {
    if (!image) {
        return;
    }

    free(image->pixels);
    free(image);
}

/**
 * @brief Draw a single point (left and right channels) of the waveform
 *
 * @param image Image to draw to
 * @param x Horizontal position
 * @param left Max amplitude values for left (0-32767)
 * @param right Max amplitude values for right (0-32767)
 */
void waver_image_draw_point(waver_image_t *image, unsigned int x, const unsigned int left, unsigned int right) {
    if (!image || x >= image->width) {
        return;
    }

    // The byte offset where the 2-bit pixel will be
    unsigned int offset = x >> 2;

    // Draw left channel (above center, going up)
    uint8_t draw_left = draw_bits(WAVER_CHANNEL_LEFT, x);
    // Scale the 16-bit value (0-32767) to image height
    unsigned int left_height = (left * image->center + 16384) >> 15;

    for (unsigned int y = (left_height > image->center) ? 0 : image->center - left_height;
         y < image->center;
         y++) {
        unsigned int idx = offset + y * image->line_width;
        image->pixels[idx] |= draw_left;
    }

    // Draw right channel (below center, going down)
    uint8_t draw_right = draw_bits(WAVER_CHANNEL_RIGHT, x);
    // Scale the 16-bit value (0-32767) to image height
    unsigned int right_height = (right * image->center + 16384) >> 15;
    unsigned int max_y = (image->center + right_height < image->height) ?
                          image->center + right_height :
                          image->height;

    for (unsigned int y = image->center; y < max_y; y++) {
        unsigned int idx = offset + y * image->line_width;
        image->pixels[idx] |= draw_right;
    }
}

/**
 * @brief Draw a single point for mono audio (symmetric around center)
 *
 * @param image Image to draw to
 * @param x Horizontal position
 * @param mono Max amplitude value (0-32767)
 */
void waver_image_draw_point_mono(waver_image_t *image, unsigned int x, const unsigned int mono) {
    if (!image || x >= image->width) {
        return;
    }

    // The byte offset where the 2-bit pixel will be
    unsigned int offset = x >> 2;

    // Bit position for the pixel
    uint8_t draw = draw_bits(WAVER_CHANNEL_LEFT, x);

    // Calculate wave height - scale 16-bit value (0-32767) to image height
    unsigned int wave_height = (mono * image->center + 16384) >> 15;
    unsigned int y_start = (wave_height > image->center) ? 0 : image->center - wave_height;
    unsigned int y_end = (image->center + wave_height < image->height) ?
                          image->center + wave_height :
                          image->height;

    for (unsigned int y = y_start; y < y_end; y++) {
        unsigned int idx = offset + y * image->line_width;
        image->pixels[idx] |= draw;
    }
}