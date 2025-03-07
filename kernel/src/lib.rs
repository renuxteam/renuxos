#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

// Load Multiboot
mod init_system;
mod multiboot;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
pub fn panic(_info: &PanicInfo) -> !
{
  wrappers::vga::clear_screen();
  wrappers::vga::print_text("Kernel Panic");
  loop {}
}

// Import wrappers
mod wrappers;
#[unsafe(no_mangle)] // don't mangle the name of this function
pub extern "C" fn _start() -> !
{
  init_system::init::start();
  loop {}
}
