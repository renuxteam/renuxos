const std = @import("std");

pub fn build(b: *std.build) void {
    const mode = b.standardReleaseOptions();

    const exe = b.addExecutable("bootloader", "src/main.zig");
    exe.setBuildMode(mode);
    exe.install();

    const run_cmd = exe.run();
    run_cmd.step.dependOn(b.getInstallStep());
    run_cmd.step.dependOn(b.getBuildStep());

    b.installArtifact(exe);
}
