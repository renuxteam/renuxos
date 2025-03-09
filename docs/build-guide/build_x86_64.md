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
   git clone --recurse-submodules https://github.com/Renan2010/renuxos.git
   cd renuxos
   ```
2. **Build Renux OS:**
   To compile do
   ```bash
   make build_renux 
   ```
> [!TIP]
> To further speed up compilation I strongly recommend you add two more threads to compile Renux OS
> 
> `Example -> CPU 4C/4T`
> ```bash
> make build_renux JOBS=6
> ```
3. **Run with QEMU:**
   Test Renux OS in an VM
   ```bash
   make run
   ```
   
  
   

