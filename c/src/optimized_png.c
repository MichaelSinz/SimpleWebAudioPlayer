/**
 * @file optimized_png.c
 * @brief Optimized PNG encoding using 2-bit depth indexed color
 */

#include "waver.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <zlib.h>

// Indexed PNG format constants
#define PNG_SIGNATURE "\x89PNG\r\n\x1a\n"
#define CHUNK_TYPE_IHDR "IHDR"
#define CHUNK_TYPE_PLTE "PLTE"
#define CHUNK_TYPE_IDAT "IDAT"
#define CHUNK_TYPE_IEND "IEND"
#define PNG_COLOR_TYPE_INDEXED 3
#define PNG_COMPRESSION_TYPE_DEFAULT 0
#define PNG_FILTER_TYPE_DEFAULT 0
#define PNG_INTERLACE_NONE 0
#define PNG_COLOR_TYPE_MASK_COLOR 2
#define PNG_COLOR_TYPE_MASK_ALPHA 4

// PNG filter types
#define PNG_FILTER_NONE 0
#define PNG_FILTER_SUB  1
#define PNG_FILTER_UP   2
#define PNG_FILTER_AVG  3
#define PNG_FILTER_PAETH 4

// CRC table for PNG chunks
static uint32_t crc_table[256];
static int crc_table_computed = 0;

// Generate CRC table for PNG chunks
static void make_crc_table() {
    uint32_t c;
    int n, k;
    
    for (n = 0; n < 256; n++) {
        c = (uint32_t)n;
        for (k = 0; k < 8; k++) {
            if (c & 1)
                c = 0xedb88320L ^ (c >> 1);
            else
                c = c >> 1;
        }
        crc_table[n] = c;
    }
    crc_table_computed = 1;
}

// Update CRC calculation
static uint32_t update_crc(uint32_t crc, const unsigned char *buf, size_t len) {
    uint32_t c = crc;
    
    if (!crc_table_computed)
        make_crc_table();
    
    for (size_t n = 0; n < len; n++) {
        c = crc_table[(c ^ buf[n]) & 0xff] ^ (c >> 8);
    }
    return c;
}

// Calculate CRC for a chunk
static uint32_t crc(const unsigned char *buf, size_t len) {
    return update_crc(0xffffffffL, buf, len) ^ 0xffffffffL;
}

// Write a 4-byte unsigned integer in big-endian format
static bool write_uint32(FILE *fp, uint32_t value) {
    unsigned char bytes[4];
    bytes[0] = (value >> 24) & 0xFF;
    bytes[1] = (value >> 16) & 0xFF;
    bytes[2] = (value >> 8) & 0xFF;
    bytes[3] = value & 0xFF;
    return (fwrite(bytes, 1, 4, fp) == 4);
}

// Write a chunk to the PNG file
static int write_chunk(FILE *fp, const char *type, const unsigned char *data, size_t length) {
    // Write length
    if (!write_uint32(fp, length)) {
        return 0;
    }
    
    // Write type and data
    if (fwrite(type, 1, 4, fp) != 4) {
        return 0;
    }
    
    if (length > 0 && data != NULL) {
        if (fwrite(data, 1, length, fp) != length) {
            return 0;
        }
    }
    
    // Calculate and write CRC
    unsigned char *crc_buf = malloc(4 + length);
    if (!crc_buf) return 0;
    
    memcpy(crc_buf, type, 4);
    if (length > 0 && data != NULL) {
        memcpy(crc_buf + 4, data, length);
    }
    
    uint32_t crc_value = crc(crc_buf, 4 + length);
    free(crc_buf);
    
    if (!write_uint32(fp, crc_value)) {
        return 0;
    }
    
    return 1;
}

/**
 * @brief Save the waveform image as an optimized PNG file
 * 
 * This function creates a 2-bit indexed color PNG file with optimal compression
 * settings, directly using the internal 2-bit representation for maximum efficiency.
 * 
 * @param image Image to save
 * @param bg_color Background color
 * @param left_color Left channel color
 * @param right_color Right channel color
 * @param output_path Path to save the PNG to
 * @param use_up_filter Whether to use the UP filter (usually better compression)
 * @return true if successful, false otherwise
 */
static bool waver_image_save_optimized_png_with_filter(
    const waver_image_t *image,
    const waver_color_t *bg_color,
    const waver_color_t *left_color,
    const waver_color_t *right_color,
    const char *output_path,
    bool use_up_filter
) {
    if (!image || !bg_color || !left_color || !right_color || !output_path) {
        return false;
    }

    FILE *fp = fopen(output_path, "wb");
    if (!fp) {
        return false;
    }
    
    // Write PNG signature
    if (fwrite(PNG_SIGNATURE, 1, 8, fp) != 8) {
        fclose(fp);
        return false;
    }
    
    // Create IHDR chunk
    unsigned char ihdr_data[13];
    memset(ihdr_data, 0, 13);
    
    // Width and height (big-endian)
    ihdr_data[0] = (image->width >> 24) & 0xFF;
    ihdr_data[1] = (image->width >> 16) & 0xFF;
    ihdr_data[2] = (image->width >> 8) & 0xFF;
    ihdr_data[3] = image->width & 0xFF;
    
    ihdr_data[4] = (image->height >> 24) & 0xFF;
    ihdr_data[5] = (image->height >> 16) & 0xFF;
    ihdr_data[6] = (image->height >> 8) & 0xFF;
    ihdr_data[7] = image->height & 0xFF;
    
    ihdr_data[8] = 2;                      // Bit depth: 2 bits per pixel
    ihdr_data[9] = PNG_COLOR_TYPE_INDEXED; // Color type: indexed
    ihdr_data[10] = PNG_COMPRESSION_TYPE_DEFAULT;
    ihdr_data[11] = PNG_FILTER_TYPE_DEFAULT;
    ihdr_data[12] = PNG_INTERLACE_NONE;
    
    // Write IHDR chunk
    if (!write_chunk(fp, CHUNK_TYPE_IHDR, ihdr_data, 13)) {
        fclose(fp);
        return false;
    }
    
    // Create PLTE chunk (4 entries: background, left, right, unused)
    unsigned char plte_data[12]; // 3 bytes per color (RGB) * 4 colors
    
    // Background color (index 0)
    plte_data[0] = bg_color->red;
    plte_data[1] = bg_color->green;
    plte_data[2] = bg_color->blue;
    
    // Left channel color (index 1)
    plte_data[3] = left_color->red;
    plte_data[4] = left_color->green;
    plte_data[5] = left_color->blue;
    
    // Right channel color (index 2)
    plte_data[6] = right_color->red;
    plte_data[7] = right_color->green;
    plte_data[8] = right_color->blue;
    
    // Unused (index 3) - set to background color
    plte_data[9] = bg_color->red;
    plte_data[10] = bg_color->green;
    plte_data[11] = bg_color->blue;
    
    // Write PLTE chunk
    if (!write_chunk(fp, CHUNK_TYPE_PLTE, plte_data, 12)) {
        fclose(fp);
        return false;
    }
    
    // Add tRNS chunk if any color has alpha < 255
    if (bg_color->alpha < 255 || left_color->alpha < 255 || right_color->alpha < 255) {
        unsigned char trns_data[4];
        trns_data[0] = bg_color->alpha;
        trns_data[1] = left_color->alpha;
        trns_data[2] = right_color->alpha;
        trns_data[3] = bg_color->alpha;  // Unused entry alpha
        
        if (!write_chunk(fp, "tRNS", trns_data, 4)) {
            fclose(fp);
            return false;
        }
    }
    
    // Compress the image data with zlib
    uLongf compress_buf_size = image->height * (1 + image->line_width);  // +1 for filter byte per scanline
    unsigned char *scanlines = malloc(compress_buf_size);
    if (!scanlines) {
        fclose(fp);
        return false;
    }
    
    if (use_up_filter) {
        // Process scanlines with UP filter (filter type 2) for better compression
        // First scanline uses None filter (0) since there's no previous line
        scanlines[0] = PNG_FILTER_NONE;
        memcpy(scanlines + 1, image->pixels, image->line_width);
        
        // All other scanlines use UP filter
        for (unsigned int y = 1; y < image->height; y++) {
            scanlines[y * (1 + image->line_width)] = PNG_FILTER_UP;  // Up filter
            
            // Apply UP filter: each byte is difference from byte in previous scanline
            for (unsigned int x = 0; x < image->line_width; x++) {
                unsigned int current_idx = y * image->line_width + x;
                unsigned int prev_idx = (y - 1) * image->line_width + x;
                
                scanlines[y * (1 + image->line_width) + 1 + x] = 
                    image->pixels[current_idx] - image->pixels[prev_idx];
            }
        }
    } else {
        // Copy scanlines with filter byte (NONE) before each
        for (unsigned int y = 0; y < image->height; y++) {
            scanlines[y * (1 + image->line_width)] = PNG_FILTER_NONE;  // None filter
            memcpy(scanlines + y * (1 + image->line_width) + 1, 
                image->pixels + y * image->line_width, 
                image->line_width);
        }
    }
    
    // Compress the data
    uLongf compress_len = compress_buf_size * 1.1 + 12;  // Add some margin as required by zlib
    unsigned char *compressed = malloc(compress_len);
    if (!compressed) {
        free(scanlines);
        fclose(fp);
        return false;
    }
    
    int z_result = compress2(compressed, &compress_len, scanlines, compress_buf_size, 9);  // Max compression
    free(scanlines);
    
    if (z_result != Z_OK) {
        free(compressed);
        fclose(fp);
        return false;
    }
    
    // Write IDAT chunk
    if (!write_chunk(fp, CHUNK_TYPE_IDAT, compressed, compress_len)) {
        free(compressed);
        fclose(fp);
        return false;
    }
    
    free(compressed);
    
    // Write IEND chunk
    if (!write_chunk(fp, CHUNK_TYPE_IEND, NULL, 0)) {
        fclose(fp);
        return false;
    }
    
    fclose(fp);
    return true;
}

/**
 * @brief Save the waveform image as an optimized PNG file
 * 
 * This function creates a 2-bit indexed color PNG file with optimal compression
 * settings, directly using the internal 2-bit representation for maximum efficiency.
 * 
 * @param image Image to save
 * @param bg_color Background color
 * @param left_color Left channel color
 * @param right_color Right channel color
 * @param output_path Path to save the PNG to
 * @return true if successful, false otherwise
 */
bool waver_image_save_optimized_png(
    const waver_image_t *image,
    const waver_color_t *bg_color,
    const waver_color_t *left_color,
    const waver_color_t *right_color,
    const char *output_path
) {
    // Use the UP filter by default for better compression in most cases
    return waver_image_save_optimized_png_with_filter(
        image, bg_color, left_color, right_color, output_path, true);
}