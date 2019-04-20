EXAMPLE = example
OUTPUT = target/x86_64-unknown-uefi/debug/$(EXAMPLE).efi
OVMF_CODE = /usr/share/ovmf/x64/OVMF_CODE.fd
OVMF_VARS = OVMF_VARS.fd
ESP = esp

.PHONY: $(OUTPUT) qemu

qemu: $(OVMF_CODE) $(OVMF_VARS) $(ESP)
	qemu-system-x86_64 -machine q35 -m 256M \
		-drive if=pflash,format=raw,readonly,file=$(OVMF_CODE) \
		-drive if=pflash,format=raw,readonly,file=$(OVMF_VARS) \
		-drive format=raw,file=fat:rw:$(ESP)

$(ESP): $(OUTPUT) startup.nsh
	mkdir -p $(ESP)
	cp $^ $(ESP)

$(OUTPUT):
	cargo xbuild --target x86_64-unknown-uefi --bin $(EXAMPLE)

startup.nsh:
	echo "$(EXAMPLE)" > $@
