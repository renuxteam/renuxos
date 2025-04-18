use std::path::PathBuf;
use std::{env, fs};

fn main() {
    // Re-run this build script if build.rs changes
    println!("cargo:rerun-if-changed=build.rs");

    // Get the current build profile (e.g., "debug" or "release")
    let profile = env::var("PROFILE").expect("PROFILE environment variable not set");

    // Determine the workspace root (parent of the 'bootloader' crate)
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let workspace_root = manifest_dir
        .parent()
        .expect("Failed to determine workspace root");

    // Construct the path to the kernel binary in the workspace-level target directory
    // e.g., workspace_root/target/{profile}/kernel
    let mut kernel_binary = workspace_root.join("target").join(&profile).join("kernel");

    // On some platforms or configs, the binary may have an .elf extension
    if !kernel_binary.exists() {
        let mut alt = kernel_binary.clone();
        alt.set_extension("elf");
        if alt.exists() {
            kernel_binary = alt;
        }
    }

    // Verify the kernel binary exists
    if !kernel_binary.exists() {
        panic!("Kernel binary not found at {:?}", kernel_binary);
    }

    // Prepare the output directory for disk images (under bootloader crate)
    let out_dir = manifest_dir.join("out");
    fs::create_dir_all(&out_dir).expect("Failed to create output directory");

    // Define the paths for UEFI and BIOS disk images
    let uefi_image = out_dir.join("uefi.img");
    let bios_image = out_dir.join("bios.img");

    // Generate the UEFI image using the bootloader crate
    bootloader::UefiBoot::new(&kernel_binary)
        .create_disk_image(&uefi_image)
        .expect("Failed to create UEFI disk image");

    // Generate the BIOS image using the bootloader crate
    bootloader::BiosBoot::new(&kernel_binary)
        .create_disk_image(&bios_image)
        .expect("Failed to create BIOS disk image");

    // Print warnings with the generated image paths
    println!("cargo:warning=UEFI image generated at {:?}", uefi_image);
    println!("cargo:warning=BIOS image generated at {:?}", bios_image);
}
