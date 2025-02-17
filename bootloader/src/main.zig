const std = @import("std");

pub fn main() void {
    const screen = std.io.getStdOut().writer();
    screen.print("Renux OS Bootlaoder");

    // Init hardware
    // init_hardware();
    // Load Renux kernel
    // load_kernel();
}
