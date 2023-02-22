FROM fedora
RUN dnf update && dnf install
    qemu-system-aarch64
    qemu-system-riscv
    qemu-system-x86
    