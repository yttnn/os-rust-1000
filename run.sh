#!/bin/bash

set -xue

QEMU=$HOME/tools/qemu-riscv32-8.1.2/bin/qemu-system-riscv32
KERNEL_ELF=./target/riscv32i-unknown-none-elf/debug/os-rust-1000

$QEMU -machine virt -bios opensbi-riscv32-generic-fw_dynamic.bin -nographic -serial mon:stdio --no-reboot \
    -kernel $KERNEL_ELF