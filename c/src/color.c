/**
 * @file color.c
 * @brief Color handling functionality for waveform visualization
 */

#include "waver.h"
#include <string.h>
#include <stdio.h>
#include <ctype.h>

/**
 * @brief Parse a color from a string in RGB, RRGGBB, or RRGGBBAA format
 * 
 * @param color_str Color string (RGB, RRGGBB, or RRGGBBAA format)
 * @param color Output color
 * @return true if successful, false otherwise
 */
bool waver_color_parse(const char *color_str, waver_color_t *color) {
    if (!color_str || !color) {
        return false;
    }

    // Skip whitespace and leading #
    while (isspace(*color_str)) {
        color_str++;
    }
    if (*color_str == '#') {
        color_str++;
    }

    // Trim trailing whitespace by finding the end of the string
    size_t len = strlen(color_str);
    while (len > 0 && isspace(color_str[len-1])) {
        len--;
    }

    // Make a copy to avoid modifying the original
    char hex[9] = {0};
    if (len >= sizeof(hex)) {
        return false;
    }
    
    // Copy the needed characters directly (safer than strncpy)
    for (size_t i = 0; i < len; i++) {
        hex[i] = color_str[i];
    }
    hex[len] = '\0';

    // Parse the hex value
    unsigned int value = 0;
    if (sscanf(hex, "%x", &value) != 1) {
        return false;
    }

    // Parse different color formats
    switch (len) {
        case 3: // RGB
            color->red = ((value & 0xF00) >> 8) * 17;
            color->green = ((value & 0x0F0) >> 4) * 17;
            color->blue = (value & 0x00F) * 17;
            color->alpha = 255;
            break;
        
        case 6: // RRGGBB
            color->red = (value & 0xFF0000) >> 16;
            color->green = (value & 0x00FF00) >> 8;
            color->blue = value & 0x0000FF;
            color->alpha = 255;
            break;
        
        case 8: // RRGGBBAA
            color->red = (value & 0xFF000000) >> 24;
            color->green = (value & 0x00FF0000) >> 16;
            color->blue = (value & 0x0000FF00) >> 8;
            color->alpha = value & 0x000000FF;
            break;
        
        default:
            return false;
    }

    return true;
}