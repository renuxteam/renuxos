# ==========================
# Renux OS Makefile
# ==========================

# Default target architecture for the project
ARCH ?= x86_64-renux

# Target configuration file based on the architecture
TARGET = config/arch/$(ARCH).json

# Number of jobs to run in parallel (default: number of CPU cores)
JOBS ?= $(shell nproc)

# Workspace to build
KERNEL_WORKSPACE = kernel

# ==========================
# Default Target
# ==========================
all: build_renux

# ==========================
# Build Targets
# ==========================

# Build the project using cargo rustc with the specified target and number of jobs
build_kernel:
	@echo "==> Starting build process"
	@if RUSTC_WRAPPER=sccache \
		cargo +nightly build -p $(KERNEL_WORKSPACE) --target=$(TARGET) -j $(JOBS) -vv; then \
		echo "==> Build hybrid kernel success"; \
	else \
		echo "==> Build process failed"; \
		exit 1; \
	fi

# Initialize and update git submodules
init_submodules:
	@echo "==> Initializing git submodules"
	@git submodule update --init --recursive

# Run the menuconfig utility to configure the kernel
menuconfig:
	@echo "==> Running menuconfig"
	@RUSTC_WRAPPER=sccache \
		cargo +stable run -p menuconfig -j $(JOBS) -vv

# ==========================
# Run and Clean Targets
# ==========================

# Run the Renux OS in QEMU
run:
	@echo "==> Running Renux OS in QEMU"
	@qemu-system-x86_64 -drive format=raw,file=target/$(ARCH)/debug/bootimage-main.bin

# Clean the project by removing the target directory
clean:
	@echo "==> Cleaning project"
	@cargo clean -vv
	@echo "==> Cleaning target directory"
	@rm -rf target/$(ARCH)
	@echo "==> Cleaning The ISO"
	@rm -rf renux.iso

codiname:
	@echo "==> Renux OS Aurora 0.1.0 (DEV)"
iso:
	@cp target/$(ARCH)/debug/bootimage-main.bin ./iso/boot
	@echo "==> Renux OS ISO copied to iso/boot"
	@echo "==> Building ISO image"
	@grub-mkrescue -o renux.iso iso
run_iso:
	@echo "==> Running Renux OS ISO in QEMU"
	@qemu-system-x86_64 -cdrom renux.iso
help:
	@echo "Renux OS Makefile"
	@echo "Usage: make [target]"
	@echo ""
	@echo "Targets:"
	@echo "  all             Build the Kernel (default target)"
	@echo "  init_submodules Initialize git submodules"
	@echo "  build_kernel    Build the Kernel"
	@echo "  menuconfig      Run the menuconfig utility"
	@echo "  run             Run the Renux OS in QEMU"
	@echo "  clean           Clean the project"
	@echo "  help            how this help message"
	@echo "  codiname        Show the codename for current release"
	@echo "  iso             Build the ISO image"
	@echo "  run_iso         Run the Renux OS ISO in QEMU"

# ==========================
# Phony Targets
# ==========================
.PHONY: all init_submodules build_renux clean menuconfig run iso
