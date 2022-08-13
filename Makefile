# SPDX-License-Identifier: GPL-2.0

# make KDIR=../rust-for-linux M=$$PWD ARCH=riscv CROSS_COMPILE=riscv64-linux-gnu- O=build
KDIR ?= ../rust-for-linux
ARCH ?= riscv
CROSS_COMPILE ?= riscv64-linux-gnu
O ?= build

default: build-ko rootfs

build-ko:
	cd add && cargo build -v
	cp add/target/target/debug/deps/add-*.o add.o_shipped
	$(MAKE) -C $(KDIR) M=$$PWD ARCH=$(ARCH) CROSS_COMPILE=$(CROSS_COMPILE) O=$(O)

rootfs:
	mkdir -p rootfs
	sudo mount -o loop rootfs.img rootfs
	sudo cp rust_out_of_tree.ko rootfs
	sudo umount rootfs

clean:
	@rm -f \
		.*.cmd *.o *_shipped rust_out_of_tree.mod rust_out_of_tree.mod.c rust_out_of_tree.ko Module.symvers modules.order

.PHONY: build-ko rootfs clean
