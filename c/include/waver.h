/**
 * @file waver.h
 * @brief Main header file for the waver application
 *
 * This file contains common definitions and function prototypes for the
 * waver application, which generates waveform visualizations from audio files.
 *
 * The program follows a data processing pipeline:
 * 1. Parse and validate command-line arguments
 * 2. Collect audio files to process
 * 3. Process each file, generating waveform images
 * 4. Report any errors that occurred during processing
 */

#ifndef WAVER_H
#define WAVER_H

#include <stdint.h>
#include <stdbool.h>
#include <stdlib.h>

// Forward declarations
typedef struct waver_args_t waver_args_t;
typedef struct waver_image_t waver_image_t;
typedef struct waver_color_t waver_color_t;

/**
 * @brief RGBA color representation
 */
struct waver_color_t {
    uint8_t red;    /**< Red component (0-255) */
    uint8_t green;  /**< Green component (0-255) */
    uint8_t blue;   /**< Blue component (0-255) */
    uint8_t alpha;  /**< Alpha component (0-255) */
};

/**
 * @brief Command-line argument structure
 */
struct waver_args_t {
    unsigned int width;           /**< Width of the output image in pixels */
    unsigned int height;          /**< Height of the output image in pixels */
    waver_color_t left_color;     /**< Color for left channel (and mono) */
    waver_color_t right_color;    /**< Color for right channel */
    waver_color_t bg_color;       /**< Background color */
    char *output_filename;        /**< Output PNG file name (only in single-file mode) */
    char **file_extensions;       /**< Audio file extensions to process */
    unsigned int extension_count; /**< Number of extensions in the extensions array */
    bool dry_run;                 /**< Perform actions without generating files */
    bool overwrite;               /**< Overwrite existing output files */
    bool quiet;                   /**< Suppress most output */
    bool verbose;                 /**< Print additional information */
    unsigned int threads;         /**< Number of threads to use (0 for auto) */
    char **audio_paths;           /**< Audio files or directories to process */
    unsigned int path_count;      /**< Number of paths in the audio_paths array */
};

/**
 * @brief Waveform image representation
 */
struct waver_image_t {
    unsigned int width;     /**< Width of the image in pixels */
    unsigned int height;    /**< Height of the image in pixels */
    unsigned int center;    /**< Vertical center line position */
    unsigned int line_width; /**< Width of a line in bytes (due to 2-bit pixels) */
    uint8_t *pixels;        /**< Pixel data stored as channel indices */
};

// Channel types for waveform images
enum waver_channel_t {
    WAVER_CHANNEL_BACKGROUND = 0,
    WAVER_CHANNEL_LEFT = 1,
    WAVER_CHANNEL_RIGHT = 2
};

// CLI functions
/**
 * @brief Parse command-line arguments
 *
 * @param argc Argument count
 * @param argv Argument vectors
 * @return Parsed arguments or NULL on error
 */
waver_args_t *waver_args_parse(int argc, char *argv[]);

/**
 * @brief Free memory allocated for arguments
 *
 * @param args Arguments to free
 */
void waver_args_free(waver_args_t *args);

/**
 * @brief Print a message to stderr unless quiet mode is enabled
 *
 * @param args Command-line arguments
 * @param format Format string
 * @param ... Additional arguments
 */
void waver_print_stderr(const waver_args_t *args, const char *format, ...);

/**
 * @brief Print a message to stdout unless quiet mode is enabled
 *
 * @param args Command-line arguments
 * @param format Format string
 * @param ... Additional arguments
 */
void waver_print_stdout(const waver_args_t *args, const char *format, ...);

/**
 * @brief Print a verbose message if verbose mode is enabled
 *
 * @param args Command-line arguments
 * @param format Format string
 * @param ... Additional arguments
 */
void waver_print_verbose(const waver_args_t *args, const char *format, ...);

// Color functions
/**
 * @brief Parse a color from a string
 *
 * @param color_str Color string (RGB, RRGGBB, or RRGGBBAA format)
 * @param color Output color
 * @return true if successful, false otherwise
 */
bool waver_color_parse(const char *color_str, waver_color_t *color);

// Image functions
/**
 * @brief Create a new waveform image
 *
 * @param width Width of the image in pixels
 * @param height Height of the image in pixels
 * @return New image or NULL on error
 */
waver_image_t *waver_image_new(unsigned int width, unsigned int height);

/**
 * @brief Free memory allocated for an image
 *
 * @param image Image to free
 */
void waver_image_free(waver_image_t *image);

/**
 * @brief Draw a single point (left and right channels) of the waveform
 *
 * @param image Image to draw to
 * @param x Horizontal position
 * @param left Max amplitude values for left (0-32767)
 * @param right Max amplitude values for right (0-32767)
 */
void waver_image_draw_point(waver_image_t *image, unsigned int x, const unsigned int left, unsigned int right);

/**
 * @brief Draw a single point for mono audio (symmetric around center)
 *
 * @param image Image to draw to
 * @param x Horizontal position
 * @param mono Max amplitude value (0-32767)
 */
void waver_image_draw_point_mono(waver_image_t *image, unsigned int x, const unsigned int mono);

/**
 * @brief Save the waveform image as a PNG file with 2-bit color depth
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
);

// Audio functions
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
);

/**
 * @brief Process audio files or directories
 *
 * @param args Command-line arguments
 * @return true if all files were processed successfully, false otherwise
 */
bool waver_process_files(const waver_args_t *args);

/**
 * @brief Process audio files or directories in parallel
 *
 * @param args Command-line arguments
 * @param num_threads Number of worker threads to use (0 for auto)
 * @return true if all files were processed successfully, false otherwise
 */
bool waver_process_files_parallel(const waver_args_t *args, size_t num_threads);

#endif /* WAVER_H */