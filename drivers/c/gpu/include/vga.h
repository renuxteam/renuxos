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