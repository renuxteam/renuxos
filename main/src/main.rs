#![no_std] // Disable the standard library
#![no_main] // Disable the main entry point

extern crate kernel; // Import the renux_kernel crate

fn main()
{
  kernel::_start(); // Call the _start function from the renux_kernel crate
}
