x86_64_asm_source_files := $(shell find kernel/impl/x86_64 -name *.asm)
x86_64_asm_object_files := $(patsubst kernel/impl/x86_64/%.asm, build/x86_64/%.o, $(x86_64_asm_source_files))

$(x86_64_asm_object_files): build/x86_64/%.o : kernel/impl/x86_64/%.asm
	mkdir -p $(dir $@) && \
	nasm -f elf64 $(patsubst build/x86_64/%.o, kernel/impl/x86_64/%.asm, $@) -o $@

.PHONY: build-x86_64
build-x86_64: $(x86_64_asm_object_files)
	mkdir -p dist/x86_64 && \
	x86_64-elf-ld -n -o dist/x86_64/kernel.bin -T kernel/targets/x86_64/linker.ld $(x86_64_asm_object_files)
	cp dist/x86_64/kernel.bin targets/x86_64/iso/boot/kernel.bin && \
	grub-mkrecue /usr/lib/grub/i386-pc -o dist/x86_64/kernel.iso targets/x86_64/iso
