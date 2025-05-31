/**
 * @file threadpool.c
 * @brief Thread pool implementation for parallel file processing
 */

#include "threadpool.h"
#include "waver.h"
#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <dirent.h>
#include <sys/stat.h>
#include <ctype.h>

// For strdup on some systems
#define _POSIX_C_SOURCE 200809L

// Default queue capacity
#define DEFAULT_QUEUE_CAPACITY 1024
#define MAX_PATH_LENGTH 1024

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
 * @brief Worker thread function that processes tasks from the queue
 * 
 * @param arg Thread pool pointer
 * @return NULL
 */
static void *worker_thread(void *arg) {
    threadpool_t *pool = (threadpool_t *)arg;
    task_t task;
    bool got_task = false;
    
    while (1) {
        // Lock the queue mutex
        pthread_mutex_lock(&pool->queue_mutex);
        
        // Wait for a task or stop signal
        while (pool->queue_size == 0 && !pool->stop) {
            pthread_cond_wait(&pool->queue_not_empty, &pool->queue_mutex);
        }
        
        // Check if we should stop
        if (pool->stop && pool->queue_size == 0) {
            pthread_mutex_unlock(&pool->queue_mutex);
            return NULL;
        }
        
        // Get a task from the queue
        if (pool->queue_size > 0) {
            task = pool->queue[pool->head];
            pool->head = (pool->head + 1) % pool->queue_capacity;
            pool->queue_size--;
            got_task = true;
            
            // Signal that the queue is not full
            pthread_cond_signal(&pool->queue_not_full);
        }
        
        // Unlock the queue mutex
        pthread_mutex_unlock(&pool->queue_mutex);
        
        // Process the task if we got one
        if (got_task) {
            // Determine output filename
            char output_file[MAX_PATH_LENGTH];
            if (task.args->output_filename) {
                // Initialize last character to detect truncation
                size_t output_filename_len = strlen(task.args->output_filename);
                if (output_filename_len >= sizeof(output_file)) {
                    waver_print_stderr(task.args, "Output filename too long, truncation would occur");
                    free(task.file_path);
                    // Update failed task count and return
                    pthread_mutex_lock(&pool->stats_mutex);
                    pool->completed_tasks++;
                    pool->failed_tasks++;
                    pthread_mutex_unlock(&pool->stats_mutex);
                    got_task = false;
                    continue;
                }
                
                // Safe to copy now
                strcpy(output_file, task.args->output_filename);
            } else {
                // Use .png extension for output
                int result = snprintf(output_file, sizeof(output_file), "%s.png", task.file_path);
                if (result < 0 || (size_t)result >= sizeof(output_file)) {
                    waver_print_stderr(task.args, "Output path too long or formatting error");
                    free(task.file_path);
                    // Update failed task count and return
                    pthread_mutex_lock(&pool->stats_mutex);
                    pool->completed_tasks++;
                    pool->failed_tasks++;
                    pthread_mutex_unlock(&pool->stats_mutex);
                    got_task = false;
                    continue;
                }
            }
            
            waver_print_verbose(task.args, "Input file: %s, Output file: %s", task.file_path, output_file);
            
            // Generate waveform
            bool success = waver_generate_waveform(task.file_path, output_file, task.args);
            
            // Update stats
            pthread_mutex_lock(&pool->stats_mutex);
            pool->completed_tasks++;
            if (!success) {
                pool->failed_tasks++;
            }
            pthread_mutex_unlock(&pool->stats_mutex);
            
            // Clean up task
            free(task.file_path);
            got_task = false;
        }
    }
    
    return NULL;
}

/**
 * @brief Initialize a thread pool with specified number of threads
 * 
 * @param num_threads Number of worker threads to create (0 for auto)
 * @return Pointer to thread pool or NULL on error
 */
threadpool_t *threadpool_init(size_t num_threads) {
    // Allocate memory for thread pool
    threadpool_t *pool = (threadpool_t *)calloc(1, sizeof(threadpool_t));
    if (!pool) {
        return NULL;
    }
    
    // Determine number of threads
    if (num_threads == 0) {
        // Auto-detect number of threads (get number of processors)
        long nprocs = sysconf(_SC_NPROCESSORS_ONLN);
        if (nprocs <= 0) {
            nprocs = 2; // Default to 2 threads
        }
        pool->num_threads = (size_t)nprocs;
    } else {
        pool->num_threads = num_threads;
    }
    
    // Initialize queue
    pool->queue_capacity = DEFAULT_QUEUE_CAPACITY;
    pool->queue = (task_t *)calloc(pool->queue_capacity, sizeof(task_t));
    if (!pool->queue) {
        free(pool);
        return NULL;
    }
    
    // Initialize worker threads
    pool->threads = (pthread_t *)calloc(pool->num_threads, sizeof(pthread_t));
    if (!pool->threads) {
        free(pool->queue);
        free(pool);
        return NULL;
    }
    
    // Initialize mutexes and condition variables
    if (pthread_mutex_init(&pool->queue_mutex, NULL) != 0 ||
        pthread_mutex_init(&pool->stats_mutex, NULL) != 0 ||
        pthread_cond_init(&pool->queue_not_empty, NULL) != 0 ||
        pthread_cond_init(&pool->queue_not_full, NULL) != 0) {
        
        if (pool->threads) free(pool->threads);
        if (pool->queue) free(pool->queue);
        free(pool);
        return NULL;
    }
    
    // Create worker threads
    for (size_t i = 0; i < pool->num_threads; i++) {
        if (pthread_create(&pool->threads[i], NULL, worker_thread, (void *)pool) != 0) {
            // Clean up any threads that were created
            pool->stop = true;
            pthread_cond_broadcast(&pool->queue_not_empty);
            for (size_t j = 0; j < i; j++) {
                pthread_join(pool->threads[j], NULL);
            }
            
            pthread_cond_destroy(&pool->queue_not_full);
            pthread_cond_destroy(&pool->queue_not_empty);
            pthread_mutex_destroy(&pool->stats_mutex);
            pthread_mutex_destroy(&pool->queue_mutex);
            free(pool->threads);
            free(pool->queue);
            free(pool);
            return NULL;
        }
    }
    
    return pool;
}

/**
 * @brief Add a task to the thread pool
 * 
 * @param pool Thread pool
 * @param file_path Path to the audio file to process
 * @param args Command-line arguments (shared)
 * @return true if successful, false otherwise
 */
bool threadpool_add_task(threadpool_t *pool, const char *file_path, const waver_args_t *args) {
    if (!pool || !file_path || !args) {
        return false;
    }
    
    // Lock the queue mutex
    pthread_mutex_lock(&pool->queue_mutex);
    
    // Wait for the queue to have space
    while (pool->queue_size == pool->queue_capacity && !pool->stop) {
        pthread_cond_wait(&pool->queue_not_full, &pool->queue_mutex);
    }
    
    // Check if we should stop
    if (pool->stop) {
        pthread_mutex_unlock(&pool->queue_mutex);
        return false;
    }
    
    // Create a task
    task_t task;
    
    // Duplicate the file path
    size_t path_len = strlen(file_path);
    task.file_path = malloc(path_len + 1);
    if (!task.file_path) {
        pthread_mutex_unlock(&pool->queue_mutex);
        return false;
    }
    memcpy(task.file_path, file_path, path_len);
    task.file_path[path_len] = '\0';
    
    task.args = args;
    
    // Add the task to the queue
    pool->queue[pool->tail] = task;
    pool->tail = (pool->tail + 1) % pool->queue_capacity;
    pool->queue_size++;
    
    // Signal that the queue is not empty
    pthread_cond_signal(&pool->queue_not_empty);
    
    // Unlock the queue mutex
    pthread_mutex_unlock(&pool->queue_mutex);
    
    return true;
}

/**
 * @brief Wait for all tasks to complete and destroy the thread pool
 * 
 * @param pool Thread pool
 * @return true if all tasks completed successfully, false otherwise
 */
bool threadpool_destroy(threadpool_t *pool) {
    if (!pool) {
        return false;
    }
    
    // Signal threads to stop
    pthread_mutex_lock(&pool->queue_mutex);
    pool->stop = true;
    pthread_cond_broadcast(&pool->queue_not_empty);
    pthread_mutex_unlock(&pool->queue_mutex);
    
    // Wait for threads to finish
    for (size_t i = 0; i < pool->num_threads; i++) {
        pthread_join(pool->threads[i], NULL);
    }
    
    // Get stats
    size_t completed_tasks = pool->completed_tasks;
    size_t failed_tasks = pool->failed_tasks;
    
    // Clean up resources
    pthread_mutex_destroy(&pool->queue_mutex);
    pthread_mutex_destroy(&pool->stats_mutex);
    pthread_cond_destroy(&pool->queue_not_empty);
    pthread_cond_destroy(&pool->queue_not_full);
    
    // Clean up queue
    for (size_t i = 0; i < pool->queue_size; i++) {
        size_t idx = (pool->head + i) % pool->queue_capacity;
        free(pool->queue[idx].file_path);
    }
    
    free(pool->threads);
    free(pool->queue);
    free(pool);
    
    // Return true if all tasks completed successfully
    return (failed_tasks == 0 && completed_tasks > 0);
}

/**
 * @brief Scan directory recursively and add all matching files to the thread pool
 * 
 * @param pool Thread pool
 * @param dir_path Path to the directory
 * @param args Command-line arguments
 * @return true if all files were added to the pool, false otherwise
 */
static bool scan_directory(threadpool_t *pool, const char *dir_path, const waver_args_t *args) {
    DIR *dir = opendir(dir_path);
    if (!dir) {
        waver_print_stderr(args, "Failed to open directory: %s", dir_path);
        return false;
    }
    
    bool success = true;
    struct dirent *entry;
    
    while ((entry = readdir(dir)) != NULL) {
        // Skip "." and ".."
        if (strcmp(entry->d_name, ".") == 0 || strcmp(entry->d_name, "..") == 0) {
            continue;
        }
        
        // Build full path
        char full_path[MAX_PATH_LENGTH];
        int result = snprintf(full_path, sizeof(full_path), "%s/%s", dir_path, entry->d_name);
        if (result < 0 || (size_t)result >= sizeof(full_path)) {
            waver_print_stderr(args, "Path too long or formatting error: %s/%s", dir_path, entry->d_name);
            continue; // Skip this entry
        }
        
        // Process subdirectories recursively
        if (is_directory(full_path)) {
            if (!scan_directory(pool, full_path, args)) {
                success = false;
            }
        } 
        // Add audio files to the thread pool
        else if (has_any_extension(entry->d_name, args->file_extensions, args->extension_count)) {
            if (!threadpool_add_task(pool, full_path, args)) {
                success = false;
            }
        }
    }
    
    closedir(dir);
    return success;
}

/**
 * @brief Process audio files or directories in parallel
 * 
 * @param args Command-line arguments
 * @param num_threads Number of worker threads to use (0 for auto)
 * @return true if all files were processed successfully, false otherwise
 */
bool waver_process_files_parallel(const waver_args_t *args, size_t num_threads) {
    if (!args) {
        return false;
    }
    
    // Create thread pool
    threadpool_t *pool = threadpool_init(num_threads);
    if (!pool) {
        waver_print_stderr(args, "Failed to create thread pool");
        return false;
    }
    
    waver_print_verbose(args, "Processing files using %zu threads", pool->num_threads);
    
    // Collect all audio files to process
    bool task_added = false;
    
    // Process each path
    for (unsigned int i = 0; i < args->path_count; i++) {
        if (is_directory(args->audio_paths[i])) {
            // Scan directory and add files to the thread pool
            if (scan_directory(pool, args->audio_paths[i], args)) {
                task_added = true;
            }
        } else if (has_any_extension(args->audio_paths[i], args->file_extensions, args->extension_count)) {
            // Add file to the thread pool
            if (threadpool_add_task(pool, args->audio_paths[i], args)) {
                task_added = true;
            }
        }
    }
    
    // Return false if no tasks were added
    if (!task_added) {
        waver_print_stderr(args, "No files to process");
        threadpool_destroy(pool);
        return false;
    }
    
    // Wait for all tasks to complete
    bool success = threadpool_destroy(pool);
    
    return success;
}