#![no_std] // Disable the standard library
#![no_main] // Disable the main entry point

extern crate renux_kernel; // Import the renux_kernel crate

fn main()
{
    renux_kernel::_start(); // Call the _start function from the renux_kernel crate
}
