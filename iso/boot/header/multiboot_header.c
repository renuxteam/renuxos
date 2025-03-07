// Multiboot header
#include <sys/types.h>
typedef unsigned char u_int8_t;
typedef unsigned short u_int16_t;
typedef unsigned int u_int32_t;
// boot
__attribute__((section(".multiboot2"), used))
volatile u_int32_t multiboot_header[] = {
    0xe85250d6, // Magic number
    0,          // Architecture (0 for i386)
    0,          // Header length
    0           // Checksum
};
// Pass control to the kernel
void _start() {
    while (1) {};
}