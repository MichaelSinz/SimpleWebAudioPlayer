CC = gcc
CFLAGS = -Wall -Wextra -std=c11 -O3 -I./include -DMINIMP3_IMPLEMENTATION -DMINIMP3_NO_SIMD
LDFLAGS = -lm -lz -pthread

# Define sources - use only our optimized PNG implementation
SOURCES = src/main.c src/audio.c src/image.c src/cli.c src/color.c src/optimized_png.c src/threadpool.c
OBJECTS = $(SOURCES:.c=.o)
TARGET = waver

# Default target
all: $(TARGET)

# Linking
$(TARGET): $(OBJECTS)
	$(CC) $(OBJECTS) -o $@ $(LDFLAGS)

# Generic rule for building object files
%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

# Clean
clean:
	rm -f $(OBJECTS) $(TARGET)

.PHONY: all clean
