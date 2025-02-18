
![Renux_logo](https://github.com/user-attachments/assets/99a8117c-bd7e-4633-b6bb-3f6ce2c29bcb)
#

Renux OS is a complete Unix-like operating system written in Rust. This project aims to provide a hybrid kernel that can be extended with additional features as needed. The main focus is to learn and explore developing an operating system in Rust.

## Features [In planning]

- [X] **Written in Rust**: Leveraging the safety and concurrency features of Rust.
- [ ] **Hybrid Kernel**: Combines elements of both monolithic and microkernel designs.
- [X] **Python script**: Build Renux easily
- [ ] **C and C++ languages**: Add tools and drivers in C and C++ to communicate directly on hardware
- [X] **Bootable Image**: Create a bootable image using `cargo bootimage`.
- [X] **QEMU Support**: Test your OS in a virtual environment using QEMU.

## Development Status
> [!WARNING]
> Renux OS is currently in the development phase. Many features are still being implemented and tested. Contributions and feedback are welcome to help improve and expand the project.


## Getting Started

### Prerequisites

To build and run Renux OS, you need to have the following tools installed:

- [Rust](https://www.rust-lang.org/): Install Rust using `rustup`.
- `cargo bootimage`: Install it using `cargo install bootimage --version "^0.10.0"`.
- [QEMU](https://www.qemu.org/): Optional, for emulating the OS.

### Build this OS

1. **Clone the Repository**:

    ```sh
    git https://github.com/Renan2010/renuxos.git
    cd renuxos
    ```

2. **Install Rust Nightly and Components**:

    ```sh
    rustup install nightly
    rustup component add rust-src --toolchain nightly
    ```

3. **Install `bootimage`**:

    ```sh
    cargo install bootimage --version "^0.10.0"
    ```

4. **Build the Bootable Image**

    Easy mode build
    ```sh
    python build.py
    ```
    Hard mode build
    ```sh
    cargo bootimage --target x86_64-unknown-none -j cores # number of  CPU cores
    ```
6. **Run with QEMU** (optional):

    ```sh
    qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-renuxos.bin
    ```
