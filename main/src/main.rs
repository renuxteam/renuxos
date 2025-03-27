#![no_std] // Disable the standard library
#![no_main] // Disable the main entry point
#![allow(dead_code)] // Allow dead code

extern crate kernel; // Import the kernel crate

fn main()
{
  kernel::_start(); // Call the _start function from the kernel crate
}
