RUST_BUILD := cargo build
RUST_OUT_DIR := target/out

KERNEL_DIR := kernel
KERNEL_ELF := $(RUST_OUT_DIR)/kernel
KERNEL_BIN := $(RUST_OUT_DIR)/kernel.bin

BOOT_DIR := boot

OBJCOPY := rust-objcopy -O binary
OBJDUMP := rust-objdump -h
QEMU := qemu-system-aarch64 -machine raspi3b -nographic -serial null -serial mon:stdio

.PHONY: clear qemu kernel all

all: qemu

clean:
	rm -r target

qemu: $(KERNEL_BIN)
	$(QEMU) -kernel $(KERNEL_BIN)

kernel: $(KERNEL_ELF)

objdump: $(KERNEL_ELF)
	$(OBJDUMP) $(KERNEL_ELF)

$(KERNEL_BIN): $(KERNEL_ELF)
	$(OBJCOPY) $(KERNEL_ELF) $(KERNEL_BIN)

$(KERNEL_ELF): $(shell find $(BOOT_DIR) $(KERNEL_DIR) -type f -name '*.rs' -or -name '*.s' -or -name '*.ld')
	cd $(KERNEL_DIR) && $(RUST_BUILD)