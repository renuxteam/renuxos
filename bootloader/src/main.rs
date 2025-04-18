fn main() {
    let uefi_path = env!("UEFI_PATH");
    let bios_path = env!("BIOS_PATH");
    let uefi = true;

    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    if uefi {
        cmd.arg("-bios")
            .arg("/usr/share/OVMF/OVMF_CODE.fd")
            .arg("-drive")
            .arg(format!("file={},if=pflash,format=raw", bios_path))
            .arg("-serial")
            .arg("stdio")
            .arg("-smp")
            .arg("4");
    } else {
        cmd.arg("-drive")
            .arg(format!("file={},format=raw", bios_path))
            .arg("-serial")
            .arg("stdio")
            .arg("-smp")
            .arg("4");
    }

    let mut child = cmd.spawn().expect("Failed to start QEMU");
    child.wait().expect("QEMU process was terminated");
}
