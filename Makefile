# Define the target for the makefile
TARGET=config/arch/x86_64-renux.json
# Define the number of jobs to run in parallel
JOBS ?=$(shell nproc)
# Define the workspace to build
BUILD_WORKSPACE=main
# Default target to build the project
all: init_submodules build_renux 
# Build the project using cargo rustc with the specified target and number of jobs
build_renux:
	@echo "==> Starting build process"
	@if cargo rustc -p $(BUILD_WORKSPACE) --target=$(TARGET) -j $(JOBS) -vv; then \
		echo "==> Build process completed"; \
	else \
		echo "==> Build process failed"; \
		exit 1; \
	fi
# Initialize and update git submodules
init_submodules:
	@git submodule update --init --recursive
# Run the menuconfig utility to configure the kernel
menuconfig:
	@cargo +stable run -p menuconfig -j $(JOBS) -vv 
# Clean the project by removing the target directory
clean:
	@cargo clean -vv

# Phony targets to avoid conflicts with files of the same name
.PHONY: all init_submodules build_renux clean menuconfig