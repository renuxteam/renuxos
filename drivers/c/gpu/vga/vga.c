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