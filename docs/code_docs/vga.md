# VGA Driver Documentation

## File: `drivers/c/gpu/vga/vga.c`

> [!NOTE]
> This GPU directory is directed to onboard video cards such as Intel Iris Xe graphics, Radeon APUs and also off boards such as Intel AMD and Nvdia

### Header File: `drivers/c/gpu/include/vga.h`
```c
#ifndef VGA_H
#define VGA_H

typedef struct
{
    unsigned char character;
    unsigned char color;
} ScreenChar;

void vga_clear_screen();
void vga_print(const char* str);

#endif
```
### Source File `drivers/c/gpu/vga/vga.c`
```c
#include "../include/vga.h"

#define VGA_BUFFER ((volatile ScreenChar*)0xB8000)
#define VGA_WIDTH 80
#define VGA_HEIGHT 25

// Function to clear the screen
void vga_clear_screen()
{
    for (int i = 0; i < VGA_WIDTH * VGA_HEIGHT; i++)
    {
        VGA_BUFFER[i].character = ' '; // Clear the character
        VGA_BUFFER[i * 2 + 1].color = 0x07; // Default color (white on black background)
    }
}

// Function to print a string on the screen
void vga_print(const char *str)
{
    int i = 0;
    while (str[i] != '\0')
    {
        VGA_BUFFER[i].character = str[i]; // Write the character
        VGA_BUFFER[i].color = 0x07; // Default color (white on black background)
        i++;
    }
}
```
### Description of Files
- vga.h: This header file defines the ScreenChar structure used to represent a character on the VGA screen, including the character and its color. It also declares two functions: vga_clear_screen to clear the screen and vga_print to print a string on the screen.
- vga.c: Implements the functions declared in vga.h, manipulating the VGA video buffer to clear the screen and print text. The vga_clear_screen function iterates over the entire screen buffer, setting each character to a space and the color to the default (white on black). The vga_print function writes a string to the screen buffer, character by character, also using the default color (white on black).
