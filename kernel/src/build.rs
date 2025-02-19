// Disable std warning
#[allow(warnings)]
use std::process::Command;
// Main function
fn main() {
    // Compile VGA driver
    cc::Build::new()
        .file("drivers/c/gpu/vga/vga.c")
        .compile("vga");
}
