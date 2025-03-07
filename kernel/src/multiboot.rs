#[repr(C, packed)]
pub struct MultibootHeader {
    magic: u32,
    flags: u32,
    checksum: u32,
}

impl MultibootHeader {
    pub const fn new() -> Self {
        let magic = 0x1BADB002; // Valor mágico do Multiboot
        let flags = 0x0; // Pode ajustar os flags conforme necessário
        let checksum = 0u32.wrapping_sub(magic).wrapping_sub(flags);

        MultibootHeader {
            magic,
            flags,
            checksum,
        }
    }
}

#[unsafe(link_section = ".multiboot")]
#[unsafe(no_mangle)]
pub static MULTIBOOT_HEADER: MultibootHeader = MultibootHeader::new();
