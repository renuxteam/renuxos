# Renux OS

Renux OS is a complete Unix-like operating system written in Rust. This project aims to provide a hybrid kernel that can be extended with additional features as needed. The main focus is to learn and explore developing an operating system in Rust.

## Features [In planning]

- **Written in Rust**: Leveraging the safety and concurrency features of Rust.
- **Hybrid Kernel**: Combines elements of both monolithic and microkernel designs.
- **Bootable Image**: Create a bootable image using `cargo bootimage`.
- **QEMU Support**: Test your OS in a virtual environment using QEMU.

## Development Status
> [!WARNING]
> Renux OS is currently in the development phase. Many features are still being implemented and tested. Contributions and feedback are welcome to help improve and expand the project.


## Getting Started

### Prerequisites

To build and run Renux OS, you need to have the following tools installed:

- [Rust](https://www.rust-lang.org/): Install Rust using `rustup`.
- `cargo bootimage`: Install it using `cargo install bootimage --version "^0.10.0"`.
- [QEMU](https://www.qemu.org/): Optional, for emulating the OS.

### Installation

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

4. **Build the Bootable Image**:

    ```sh
    cargo bootimage --target x86_64-unknown-none
    ```

5. **Run with QEMU** (optional):

    ```sh
    qemu-system-x86_64 -drive format=raw,file=target/x86_64/debug/bootimage-renux_os.bin
    `
