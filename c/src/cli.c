/**
 * @file cli.c
 * @brief Command line interface implementation for waver
 */

#define _GNU_SOURCE
#include "waver.h"
#include <stdio.h>
#include <string.h>
#include <stdarg.h>
#include <ctype.h>
#include <dirent.h>
#include <sys/stat.h>
#include <libgen.h>
#include <stdlib.h> // For strdup
#include "thread_safe_console.h" // For console mutex

// Special return value for help display
#define WAVER_ARGS_HELP ((waver_args_t*)1)

// Default values
#define DEFAULT_WIDTH 2048
#define DEFAULT_HEIGHT 128
#define DEFAULT_LEFT_COLOR "00ff99"
#define DEFAULT_RIGHT_COLOR "99ff00"
#define DEFAULT_BG_COLOR "ffffff00"
#define DEFAULT_FILE_EXTENSION "mp3"

// For file processing
#define MAX_PATH_LENGTH 1024

// Help text
static const char *HELP_TEXT =
    "Usage: waver [options] audio_files...\n"
    "\n"
    "Generate waveform visualizations from audio files.\n"
    "\n"
    "Options:\n"
    "  --width <pixels>          Width of the output image (default: 2048)\n"
    "  --height <pixels>         Height of the output image, must be even (default: 128)\n"
    "  --left-color <color>      Color for left channel (default: 00ff99)\n"
    "  --right-color <color>     Color for right channel (default: 99ff00)\n"
    "  --background-color <color> Background color (default: ffffff00)\n"
    "  -o, --output-filename <file> Output PNG file name (only in single-file mode)\n"
    "  --file-extensions <ext>   Comma-separated list of audio file extensions (default: mp3)\n"
    "  --threads <number>        Number of worker threads (default: auto)\n"
    "  --dry-run                 Perform actions without generating files\n"
    "  --overwrite               Overwrite existing output files\n"
    "  --quiet                   Suppress most output\n"
    "  --verbose                 Print additional information\n"
    "  -h, --help                Display this help message\n"
    "\n"
    "Colors can be specified in RGB, RRGGBB, or RRGGBBAA hex format.\n";

/**
 * @brief Print the help message
 */
static void print_help(void) {
    printf("%s", HELP_TEXT);
}

/**
 * @brief Check if a file exists
 * 
 * @param path Path to check
 * @return true if the file exists, false otherwise
 */
static bool file_exists(const char *path) {
    FILE *file = fopen(path, "rb");
    if (file) {
        fclose(file);
        return true;
    }
    return false;
}

/**
 * @brief Check if a path is a directory
 * 
 * @param path Path to check
 * @return true if the path is a directory, false otherwise
 */
static bool is_directory(const char *path) {
    struct stat st;
    if (stat(path, &st) == 0) {
        return S_ISDIR(st.st_mode);
    }
    return false;
}

/**
 * @brief Check if a string has a specified file extension
 * 
 * @param filename The filename to check
 * @param extension The extension to match
 * @return true if the filename has the extension, false otherwise
 */
static bool has_extension(const char *filename, const char *extension) {
    size_t filename_len = strlen(filename);
    size_t extension_len = strlen(extension);
    
    if (filename_len <= extension_len) {
        return false;
    }
    
    const char *file_ext = filename + filename_len - extension_len;
    
    // Check for dot before extension
    if (*(file_ext - 1) != '.') {
        return false;
    }
    
    // Case-insensitive comparison
    for (size_t i = 0; i < extension_len; i++) {
        if (tolower((unsigned char)file_ext[i]) != tolower((unsigned char)extension[i])) {
            return false;
        }
    }
    
    return true;
}

/**
 * @brief Check if a file has any of the specified extensions
 * 
 * @param filename The filename to check
 * @param extensions Array of extensions to match
 * @param extension_count Number of extensions in the array
 * @return true if the file has any of the extensions, false otherwise
 */
static bool has_any_extension(const char *filename, char **extensions, unsigned int extension_count) {
    for (unsigned int i = 0; i < extension_count; i++) {
        if (has_extension(filename, extensions[i])) {
            return true;
        }
    }
    return false;
}

/**
 * @brief Print a message to stderr unless quiet mode is enabled
 * 
 * @param args Command-line arguments
 * @param format Format string
 * @param ... Additional arguments
 */
void waver_print_stderr(const waver_args_t *args, const char *format, ...) {
    if (!args || args->quiet) {
        return;
    }
    
    // Lock mutex for thread safety
    pthread_mutex_lock(&console_mutex);
    
    va_list ap;
    va_start(ap, format);
    vfprintf(stderr, format, ap);
    fprintf(stderr, "\n");
    va_end(ap);
    
    // Unlock mutex
    pthread_mutex_unlock(&console_mutex);
}

/**
 * @brief Print a message to stdout unless quiet mode is enabled
 * 
 * @param args Command-line arguments
 * @param format Format string
 * @param ... Additional arguments
 */
void waver_print_stdout(const waver_args_t *args, const char *format, ...) {
    if (!args || args->quiet) {
        return;
    }
    
    // Lock mutex for thread safety
    pthread_mutex_lock(&console_mutex);
    
    va_list ap;
    va_start(ap, format);
    vprintf(format, ap);
    printf("\n");
    va_end(ap);
    
    // Unlock mutex
    pthread_mutex_unlock(&console_mutex);
}

/**
 * @brief Print a verbose message if verbose mode is enabled
 * 
 * @param args Command-line arguments
 * @param format Format string
 * @param ... Additional arguments
 */
void waver_print_verbose(const waver_args_t *args, const char *format, ...) {
    if (!args || !args->verbose) {
        return;
    }
    
    // Lock mutex for thread safety
    pthread_mutex_lock(&console_mutex);
    
    va_list ap;
    va_start(ap, format);
    vprintf(format, ap);
    printf("\n");
    va_end(ap);
    
    // Unlock mutex
    pthread_mutex_unlock(&console_mutex);
}

/**
 * @brief Parse command-line arguments
 * 
 * @param argc Argument count
 * @param argv Argument vectors
 * @return Parsed arguments or NULL on error
 */
waver_args_t *waver_args_parse(int argc, char *argv[]) {
    if (argc < 2) {
        print_help();
        return (waver_args_t*)1; // Special value for help
    }

    // Create and initialize arguments structure with defaults
    waver_args_t *args = calloc(1, sizeof(waver_args_t));
    if (!args) {
        fprintf(stderr, "Memory allocation failed\n");
        return NULL;
    }

    args->width = DEFAULT_WIDTH;
    args->height = DEFAULT_HEIGHT;
    args->threads = 0;  // Auto-detect number of threads by default
    
    if (!waver_color_parse(DEFAULT_LEFT_COLOR, &args->left_color) ||
        !waver_color_parse(DEFAULT_RIGHT_COLOR, &args->right_color) ||
        !waver_color_parse(DEFAULT_BG_COLOR, &args->bg_color)) {
        fprintf(stderr, "Failed to parse default colors\n");
        free(args);
        return NULL;
    }

    // Default file extension
    args->extension_count = 1;
    args->file_extensions = calloc(1, sizeof(char*));
    if (!args->file_extensions) {
        fprintf(stderr, "Memory allocation failed\n");
        free(args);
        return NULL;
    }
    args->file_extensions[0] = strdup(DEFAULT_FILE_EXTENSION);
    if (!args->file_extensions[0]) {
        fprintf(stderr, "Memory allocation failed\n");
        free(args->file_extensions);
        free(args);
        return NULL;
    }

    // Parse arguments
    for (int i = 1; i < argc; i++) {
        const char *arg = argv[i];

        // Handle options
        if (arg[0] == '-') {
            // Help
            if (strcmp(arg, "-h") == 0 || strcmp(arg, "--help") == 0) {
                print_help();
                waver_args_free(args);
                return (waver_args_t*)1; // Special value for help
            }
            // Width
            else if (strcmp(arg, "--width") == 0) {
                if (i + 1 >= argc) {
                    fprintf(stderr, "Missing value for --width\n");
                    waver_args_free(args);
                    return NULL;
                }
                if (sscanf(argv[++i], "%u", &args->width) != 1 || args->width < 16) {
                    fprintf(stderr, "Width must be a number >= 16\n");
                    waver_args_free(args);
                    return NULL;
                }
            }
            // Height
            else if (strcmp(arg, "--height") == 0) {
                if (i + 1 >= argc) {
                    fprintf(stderr, "Missing value for --height\n");
                    waver_args_free(args);
                    return NULL;
                }
                if (sscanf(argv[++i], "%u", &args->height) != 1 || 
                    args->height < 6 || args->height % 2 != 0) {
                    fprintf(stderr, "Height must be an even number >= 6\n");
                    waver_args_free(args);
                    return NULL;
                }
            }
            // Left color
            else if (strcmp(arg, "--left-color") == 0) {
                if (i + 1 >= argc) {
                    fprintf(stderr, "Missing value for --left-color\n");
                    waver_args_free(args);
                    return NULL;
                }
                if (!waver_color_parse(argv[++i], &args->left_color)) {
                    fprintf(stderr, "Invalid color format for --left-color\n");
                    waver_args_free(args);
                    return NULL;
                }
            }
            // Right color
            else if (strcmp(arg, "--right-color") == 0) {
                if (i + 1 >= argc) {
                    fprintf(stderr, "Missing value for --right-color\n");
                    waver_args_free(args);
                    return NULL;
                }
                if (!waver_color_parse(argv[++i], &args->right_color)) {
                    fprintf(stderr, "Invalid color format for --right-color\n");
                    waver_args_free(args);
                    return NULL;
                }
            }
            // Background color
            else if (strcmp(arg, "--background-color") == 0) {
                if (i + 1 >= argc) {
                    fprintf(stderr, "Missing value for --background-color\n");
                    waver_args_free(args);
                    return NULL;
                }
                if (!waver_color_parse(argv[++i], &args->bg_color)) {
                    fprintf(stderr, "Invalid color format for --background-color\n");
                    waver_args_free(args);
                    return NULL;
                }
            }
            // Output filename
            else if (strcmp(arg, "-o") == 0 || strcmp(arg, "--output-filename") == 0) {
                if (i + 1 >= argc) {
                    fprintf(stderr, "Missing value for --output-filename\n");
                    waver_args_free(args);
                    return NULL;
                }
                args->output_filename = strdup(argv[++i]);
                if (!args->output_filename) {
                    fprintf(stderr, "Memory allocation failed\n");
                    waver_args_free(args);
                    return NULL;
                }
            }
            // File extensions
            else if (strcmp(arg, "--file-extensions") == 0) {
                if (i + 1 >= argc) {
                    fprintf(stderr, "Missing value for --file-extensions\n");
                    waver_args_free(args);
                    return NULL;
                }
                
                // Free default extension
                for (unsigned int j = 0; j < args->extension_count; j++) {
                    free(args->file_extensions[j]);
                }
                free(args->file_extensions);
                
                // Count extensions
                const char *ext_list = argv[++i];
                unsigned int count = 1;
                for (const char *p = ext_list; *p; p++) {
                    if (*p == ',') {
                        count++;
                    }
                }
                
                // Allocate extensions array
                args->file_extensions = calloc(count, sizeof(char*));
                if (!args->file_extensions) {
                    fprintf(stderr, "Memory allocation failed\n");
                    waver_args_free(args);
                    return NULL;
                }
                
                // Parse extensions
                args->extension_count = 0;
                char *ext_copy = strdup(ext_list);
                if (!ext_copy) {
                    fprintf(stderr, "Memory allocation failed\n");
                    waver_args_free(args);
                    return NULL;
                }
                
                char *token = strtok(ext_copy, ",");
                while (token) {
                    // Trim leading and trailing whitespace
                    while (isspace(*token)) {
                        token++;
                    }
                    
                    char *end = token + strlen(token) - 1;
                    while (end > token && isspace(*end)) {
                        *end-- = '\0';
                    }
                    
                    if (*token) {
                        args->file_extensions[args->extension_count] = strdup(token);
                        if (!args->file_extensions[args->extension_count]) {
                            fprintf(stderr, "Memory allocation failed\n");
                            free(ext_copy);
                            waver_args_free(args);
                            return NULL;
                        }
                        args->extension_count++;
                    }
                    
                    token = strtok(NULL, ",");
                }
                
                free(ext_copy);
                
                if (args->extension_count == 0) {
                    fprintf(stderr, "No valid file extensions specified\n");
                    waver_args_free(args);
                    return NULL;
                }
            }
            // Dry run
            else if (strcmp(arg, "--dry-run") == 0) {
                args->dry_run = true;
            }
            // Overwrite
            else if (strcmp(arg, "--overwrite") == 0) {
                args->overwrite = true;
            }
            // Quiet
            else if (strcmp(arg, "--quiet") == 0) {
                args->quiet = true;
            }
            // Verbose
            else if (strcmp(arg, "--verbose") == 0) {
                args->verbose = true;
            }
            // Threads
            else if (strcmp(arg, "--threads") == 0) {
                if (i + 1 >= argc) {
                    fprintf(stderr, "Missing value for --threads\n");
                    waver_args_free(args);
                    return NULL;
                }
                if (sscanf(argv[++i], "%u", &args->threads) != 1) {
                    fprintf(stderr, "Threads must be a number\n");
                    waver_args_free(args);
                    return NULL;
                }
            }
            // Unknown option
            else {
                fprintf(stderr, "Unknown option: %s\n", arg);
                waver_args_free(args);
                return NULL;
            }
        }
        // Audio paths
        else {
            // Allocate or expand the audio_paths array
            char **new_paths = realloc(args->audio_paths, (args->path_count + 1) * sizeof(char*));
            if (!new_paths) {
                fprintf(stderr, "Memory allocation failed\n");
                waver_args_free(args);
                return NULL;
            }
            args->audio_paths = new_paths;
            
            // Add the new path
            args->audio_paths[args->path_count] = strdup(arg);
            if (!args->audio_paths[args->path_count]) {
                fprintf(stderr, "Memory allocation failed\n");
                waver_args_free(args);
                return NULL;
            }
            args->path_count++;
        }
    }

    // Validate arguments
    if (args->path_count == 0) {
        fprintf(stderr, "No audio files specified\n");
        waver_args_free(args);
        return NULL;
    }

    // Check output filename constraints
    if (args->output_filename && args->path_count > 1) {
        fprintf(stderr, "Cannot specify --output-filename with multiple audio files\n");
        waver_args_free(args);
        return NULL;
    }

    // Check that paths exist
    for (unsigned int i = 0; i < args->path_count; i++) {
        if (!file_exists(args->audio_paths[i])) {
            fprintf(stderr, "File not found: %s\n", args->audio_paths[i]);
            waver_args_free(args);
            return NULL;
        }
        
        // Check directory constraints
        if (args->output_filename && is_directory(args->audio_paths[i])) {
            fprintf(stderr, "Cannot specify --output-filename with a directory\n");
            waver_args_free(args);
            return NULL;
        }
    }

    return args;
}

/**
 * @brief Free memory allocated for arguments
 * 
 * @param args Arguments to free
 */
void waver_args_free(waver_args_t *args) {
    if (!args) {
        return;
    }

    free(args->output_filename);
    
    if (args->file_extensions) {
        for (unsigned int i = 0; i < args->extension_count; i++) {
            free(args->file_extensions[i]);
        }
        free(args->file_extensions);
    }
    
    if (args->audio_paths) {
        for (unsigned int i = 0; i < args->path_count; i++) {
            free(args->audio_paths[i]);
        }
        free(args->audio_paths);
    }
    
    free(args);
}

/**
 * @brief Process a single file
 * 
 * @param file_path Path to the file
 * @param args Command-line arguments
 * @return true if successful, false otherwise
 */
static bool process_file(const char *file_path, const waver_args_t *args) {
    // Skip files that don't match extensions
    if (!has_any_extension(file_path, args->file_extensions, args->extension_count)) {
        return true; // Not an error, just not processed
    }
    
    // Determine output filename
    char output_file[MAX_PATH_LENGTH];
    if (args->output_filename) {
        // Check if the output filename will fit
        size_t output_filename_len = strlen(args->output_filename);
        if (output_filename_len >= sizeof(output_file)) {
            waver_print_stderr(args, "Output filename too long, truncation would occur");
            return false;
        }
        
        // Safe to copy now
        strcpy(output_file, args->output_filename);
    } else {
        // Use .png extension for output
        int result = snprintf(output_file, sizeof(output_file), "%s.png", file_path);
        if (result < 0 || (size_t)result >= sizeof(output_file)) {
            waver_print_stderr(args, "Output path too long or formatting error");
            return false;
        }
    }
    
    waver_print_verbose(args, "Input file: %s, Output file: %s", file_path, output_file);
    
    // Generate waveform
    return waver_generate_waveform(file_path, output_file, args);
}

/**
 * @brief Process audio files or directories
 * 
 * @param args Command-line arguments
 * @return true if all files were processed successfully, false otherwise
 */
bool waver_process_files(const waver_args_t *args) {
    if (!args) {
        return false;
    }
    
    // Use parallel processing if there are multiple files or directories
    if (args->path_count > 1 || is_directory(args->audio_paths[0])) {
        return waver_process_files_parallel(args, args->threads);
    }
    
    // For a single file, process it directly
    if (!is_directory(args->audio_paths[0])) {
        return process_file(args->audio_paths[0], args);
    }
    
    // For a single directory, use parallel processing
    return waver_process_files_parallel(args, args->threads);
}