// Disable std warning
use std::fs;
use std::path::Path;
#[allow(warnings)]
// import
use std::process::Command;
// Main function
fn main() -> std::io::Result<()>
{
  println!("Running build.rs (Renux_kernel)");
  println!("Building drivers");
  println!("Compiling drivers");
  build_drivers()?;
  println!("cargo:rustc-link-arg=-T./linker.ld");
  Ok(())
}
// Collect C/C++ files
fn collect_c_files(dir: &Path, build: &mut cc::Build) -> std::io::Result<()>
{
  for entry in fs::read_dir(dir)? {
    let entry = entry?;
    let path = entry.path();

    if path.is_dir() {
      collect_c_files(&path, build)?;
    } else if path.extension().and_then(|s| s.to_str()) == Some("c") {
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
  let drivers_path = Path::new("../drivers");
  let entries = fs::read_dir(drivers_path).expect("Failed to read drivers directory");

  // Ensure the driver_path exists
  if !drivers_path.exists() {
    // Print Error
    eprintln!("Error: Drivers directory does not exist");
    return Err(std::io::Error::new(
      std::io::ErrorKind::NotFound,
      "Error: Drivers directory not found",
    ));
  }

  for entry in entries {
    let entry = entry?;
    let path = entry.path();

    if path.extension().and_then(|s| s.to_str()) == Some("c") {
      build.file(path);
    }
  }

  // Using collect C files
  collect_c_files(drivers_path, &mut build)?;

  // Compile all C/C++ files
  build.include(drivers_path).warnings(true).debug(true).compile("libvga.a");

  Ok(())
}
