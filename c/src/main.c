/**
 * @file main.c
 * @brief Main entry point for the waver application
 *
 * Waver: Generate waveform visualizations from audio files.
 *
 * This tool creates PNG visualizations of audio waveforms from MP3 audio
 * files.  It supports multiple audio file processing, customizable colors,
 * and various output options.
 *
 * The program follows a data processing pipeline:
 * 1. Parse and validate command-line arguments
 * 2. Collect audio files to process
 * 3. Process each file, generating waveform images
 * 4. Report any errors that occurred during processing
 */

#include "waver.h"
#include <stdio.h>

// Special return value for help display
#define WAVER_ARGS_HELP ((waver_args_t*)1)

/**
 * @brief Main entry point for the waver application
 *
 * @param argc Number of command-line arguments
 * @param argv Array of command-line argument strings
 * @return 0 on success, non-zero on error
 */
int main(int argc, char *argv[]) {
    // Parse and validate command-line arguments
    waver_args_t *args = waver_args_parse(argc, argv);

    // Handle special help case
    if (args == WAVER_ARGS_HELP) {
        return 0; // Help was shown, exit with success
    }

    if (!args) {
        return 1;
    }

    // Process files
    bool success = waver_process_files(args);

    // Clean up and return
    waver_args_free(args);
    return success ? 0 : 1;
}