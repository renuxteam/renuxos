import os
import subprocess
import platform

def get_cpu_cores():
    system = platform.system()
    
    if system == "Windows":
        # Use WMIC to get the number of CPU cores on Windows
        result = subprocess.run(["wmic", "cpu", "get", "NumberOfCores"], capture_output=True, text=True)
        cores = int(result.stdout.split("\n")[1].strip())
    elif system == "Linux":
        # Use nproc to get the number of CPU cores on Linux
        result = subprocess.run(["nproc"], capture_output=True, text=True)
        cores = int(result.stdout.strip())
    else:
        raise NotImplementedError(f"Unsupported OS: {system}")
    
    return cores

def build_project(cores):
    # Compile the project using all available CPU cores and specify the target architecture
    subprocess.run(["cargo", "bootimage", "--target", "x86_64-unknown-none", f"-j{cores}"])

def main():
    try:
        cores = get_cpu_cores()
        print(f"==> Using {cores} cores for compilation.")
        build_project(cores)
    except Exception as e:
        print(f"Error: {e}")

if __name__ == "__main__":
    main()

