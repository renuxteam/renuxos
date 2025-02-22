## Building Renux OS for the x86_64 Architecture

To compile Renux OS with the specific configuration for the x86_64 architecture and generate a bootable ISO, follow the steps below:

### Requirements

Make sure you have the necessary tools installed:

- **Rust**: To compile the kernel code.
- **GCC**: To compile the C/C++ drivers.
- **grub-mkrescue**: To create the bootable ISO image.
- **nproc**: To get the number of CPU cores (used to optimize the build process).
## Steps
### Steps

1. **Clone the Repository:**

   If you haven’t done so already, clone the Renux OS repository:

   ```bash
   git clone https://github.com/username/renuxos.git
   cd renuxos
   ```
2. **Build with `cargo build`**
  ```bash
  cargo build --target=config/arch/x86_64-renux.json -j $(nproc)
  ```
  The `-j $(nproc)` option uses all available CPU cores to speed up the build.
3. **Create the Bootable ISO**
  ```bash
  grub-mkrescue -o build_iso/renux.iso build_iso/
  ```
4. **Run with Qemu**
   ```bash
   qemu-system-x86_64 -cdrom renux.iso
   ```
