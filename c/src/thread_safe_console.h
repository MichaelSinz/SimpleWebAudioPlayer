#include <pthread.h>

// Mutex for protecting console output
static pthread_mutex_t console_mutex = PTHREAD_MUTEX_INITIALIZER;