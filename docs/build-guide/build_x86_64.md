## Building Renux OS for the x86_64 Architecture

To compile Renux OS with the specific configuration for the x86_64 architecture and generate a bootable ISO, follow the steps below:

### Requirements

Make sure you have the necessary tools installed:

- **Rust**: To compile the kernel code.
- **Clang**: To compile the C/C++ drivers.
- **grub-mkrescue**: To create the bootable ISO image.
- **nproc**: To get the number of CPU cores (used to optimize the build process).
- **LLVM development libraries**: To build multiboot 2(or multibootheader)
## Steps

1. **Clone the Repository:**

   If you haven’t done so already, clone the Renux OS repository:

   ```bash
   git clone https://github.com/Renan2010/renuxos.git
   cd renuxos
   ```
2. **Build with `build.py` or `cargo build`**
   
   Python build.py scripty (more friendly)
   ```bash
   python build.py
   ```
   Or cargo build (especially for Nerds)
   ```bash
   cargo build -p main --target=config/arch/x86_64-renux.json -j $(nproc)
   ```
   The `-j $(nproc)` option uses all available CPU cores to speed up the build.

4. **Create the Bootable ISO**
   ```bash
   grub-mkrescue -o build_iso/renux.iso build_iso/
   ```
5. **Test with Qemu**
   ```bash
   qemu-system-x86_64 -cdrom renux.iso
   ```
