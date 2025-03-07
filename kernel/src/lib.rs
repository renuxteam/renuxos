// kernel/src/lib.rs

#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> !
{
  loop {}
}

// Import wrappers
mod wrappers;

#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn _start() -> !
{
  // this function is the entry point, since the linker looks for a function
  // named `_start` by default

  wrappers::vga::clear_screen();
  wrappers::vga::print_text("Hello Renux Kernel (Aurora)!");

  loop {}
}