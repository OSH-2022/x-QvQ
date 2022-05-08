RUST_BUILD := cargo build
RUST_OUT_DIR := target/out

KERNEL_DIR := kernel
KERNEL_ELF := $(RUST_OUT_DIR)/kernel
KERNEL_BIN := $(RUST_OUT_DIR)/kernel.bin

OBJCOPY := rust-objcopy -O binary
QEMU := qemu-system-aarch64 -machine raspi3b -nographic -serial null -serial mon:stdio

.PHONY: clear qemu kernel all

all: qemu

clean:
	rm -r target

qemu: $(KERNEL_BIN)
	$(QEMU) -kernel $(KERNEL_BIN)

kernel: $(KERNEL_ELF)

$(KERNEL_BIN): $(KERNEL_ELF)
	$(OBJCOPY) $(KERNEL_ELF) $(KERNEL_BIN)

$(KERNEL_ELF): $(shell find $(KERNEL_DIR) -type f -name *.rs)
	cd $(KERNEL_DIR) && $(RUST_BUILD)