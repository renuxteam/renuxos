# Building a Custom Kernel in Renux OS

### This guide outlines the steps required to create and configure a custom kernel in Renux OS using menuconfig.

> [!IMPORTANT]
> **Menuconfig** should only be used with the **stable toolchain**. Using the nightly toolchain for `menuconfig` compilation will result in an error(`duplicate core`).
> Other components of the system can be compiled using the **nightly toolchain**.

## Steps to Build the Kernel
### Step 1: Configure the Kernel with `menuconfig`
1. Run the following command to use the stable Rust toolchain for `menuconfig`:
   ```bash
   cargo +stable run -p menuconfig
   ```
2. Navigate through the available configuration options and make necessary adjustments.
3. Exit the configuration tool once all required changes have been made.
### Step 2: Compile the Kernel (Nightly toolchain)
To compile the kernel for the specified architecture (in this case, `x86_64`-> or `AMD64`), execute:
  ```bash
  cargo build -p main --target=config/arch/x86_64-renux.json
  ```
### Toolchains in Use
- Stable toolchain is mandatory for running menuconfig.
- Nightly toolchain can be used for compiling other system components.
**By following these steps, you'll have a custom-built kernel ready to run on Renux OS tailored to your specific requirements.**
