// Disable std warning
#[allow(warnings)]
// import 
use std::process::Command;
use std::fs;
use std::path::Path;
// Main function
fn main() -> std::io::Result<()>
{
    build_multiboot()?;
    build_drivers()?;
    Ok(())
}
// Build Multiboot Header (or Multiboot 2)
fn build_multiboot() -> std::io::Result<()>
{
    // Build command
    let mut build = Command::new("llc");
    // LLVM IR (Or ASM) path
    let multiboot2 = "build_iso/boot/header/multiboot_header.ll";

    let status = build
        .arg(multiboot2)
        .status()?;
    
    // Error mensage
    let error = "Failed to generate multiboot header.";

    if !status.success()
    {
        eprintln!("{}", error);
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "llc command failed"));

    }

    Ok(())
}
// Collect C/C++ files
fn collect_c_files(dir: &Path, build: &mut cc::Build) -> std::io::Result<()>
{
    for entry in fs::read_dir(dir)?
    {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir()
        {
            collect_c_files(&path, build)?;
        } else if path.extension().and_then(|s| s.to_str()) == Some("c")
        {
            println!("Compiling: {:?}", path);
            build.file(path);
        }
    }
    Ok(())
}
// Compile C/C++ Scripts
fn build_drivers() -> std::io::Result<()>
{
    let mut build = cc::Build::new();
    let drivers_path = Path::new("drivers");
    let entries = fs::read_dir(drivers_path).expect("Failed to read drivers directory");


    for entry in entries
    {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("c")
        {
            build.file(path);
        }

    }

    // Using collect C files
    collect_c_files(drivers_path, &mut build)?;
    
    // Compile all C/C++ files
    build
        .include(drivers_path)
        .warnings(true)
        .debug(true)
        .compile("vga");


    Ok(())
    
}