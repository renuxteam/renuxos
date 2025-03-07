SECTIONS
{
  . = 0x00100000; /* Endereço inicial do kernel */
  .multiboot :
  {
    KEEP(*(.multiboot))
  }
  .text :
  {
    *(.text)
  }
  .rodata :
  {
    *(.rodata)
  }
  .data :
  {
    *(.data)
  }
  .bss :
  {
    *(COMMON)
    *(.bss)
  }
}
