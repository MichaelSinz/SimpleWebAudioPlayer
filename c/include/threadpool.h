/**
 * @file threadpool.h
 * @brief Thread pool implementation for parallel file processing
 */

#ifndef THREADPOOL_H
#define THREADPOOL_H

#include <pthread.h>
#include <stdbool.h>
#include <stddef.h>

// Forward declarations for waver structures
typedef struct waver_args_t waver_args_t;

// The task structure represents a file to be processed
typedef struct {
    char *file_path;           // Path to the audio file
    const waver_args_t *args;  // Command-line arguments (shared)
} task_t;

// Thread pool structure
typedef struct {
    task_t *queue;             // Task queue
    size_t queue_size;         // Size of the queue
    size_t queue_capacity;     // Capacity of the queue
    size_t head;               // Head of the queue
    size_t tail;               // Tail of the queue
    
    pthread_t *threads;        // Worker threads
    size_t num_threads;        // Number of threads
    
    pthread_mutex_t queue_mutex;   // Mutex for queue access
    pthread_cond_t queue_not_empty; // Condition for queue not empty
    pthread_cond_t queue_not_full;  // Condition for queue not full
    
    bool stop;                 // Flag to stop workers
    size_t completed_tasks;    // Number of completed tasks
    size_t failed_tasks;       // Number of failed tasks
    pthread_mutex_t stats_mutex; // Mutex for stats access
} threadpool_t;

/**
 * @brief Initialize a thread pool with specified number of threads
 * 
 * @param num_threads Number of worker threads to create (0 for auto)
 * @return Pointer to thread pool or NULL on error
 */
threadpool_t *threadpool_init(size_t num_threads);

/**
 * @brief Add a task to the thread pool
 * 
 * @param pool Thread pool
 * @param file_path Path to the audio file to process
 * @param args Command-line arguments (shared)
 * @return true if successful, false otherwise
 */
bool threadpool_add_task(threadpool_t *pool, const char *file_path, const waver_args_t *args);

/**
 * @brief Wait for all tasks to complete and destroy the thread pool
 * 
 * @param pool Thread pool
 * @return true if all tasks completed successfully, false otherwise
 */
bool threadpool_destroy(threadpool_t *pool);

/**
 * @brief Process audio files or directories in parallel
 * 
 * @param args Command-line arguments
 * @param num_threads Number of worker threads to use (0 for auto)
 * @return true if all files were processed successfully, false otherwise
 */
bool waver_process_files_parallel(const waver_args_t *args, size_t num_threads);

#endif /* THREADPOOL_H */