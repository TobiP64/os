#### WARNING: This project currently requires the nightly-2020-08-13 toolchain, otherwise compilation will result in a segfault.

## Setup

- Install binutils `$ sudo dnf install binutils-x86_64-linux-gnu binutils-aarch64-linux-gnu binutils-riscv64-linux-gnu binutils-powerpc64le-linux-gnu`
- Install QEMU `$ sudo dnf install qemu`

Or run `$ ./os.sh setup`

## Build and Run

`$ cargo <check|build|run|test|...> --release --target=<ARCH>-unknown-<none|sbi|uefi|albl>`

For most use cases, `./os.sh` will be sufficient. Try `./os.sh help` for a list of commands.

## Useful Resources

- [Rust Embedded Book](https://doc.rust-lang.org/embedded-book/)
- [AMD64 OS in Rust](https://os.phil-opp.com)
- [RISC-V OS in Rust](https://osblog.stephenmarz.com/index.html)
- [Raspberry Pi 3 Rust OS Tutorials](https://github.com/rust-embedded/rust-raspi3-OS-tutorials)
- [OSDev Wiki](https://wiki.osdev.org/Expanded_Main_Page)

## Documentation

[Find the documentation here](./DOCS.md)

## Quirks and UB

### Targets

#### aarch64-unknown-uefi

Code model must be set to `small` in the target spec, otherwise compilation fails with
`LLVM ERROR: unsupported relocation type: fixup_aarch64_movw`.

The `__chkst` function is required, but compiler-builtins does not define the function for aarch64, so there is a dummy
(empty) implementation in `crate::std`, to shut the compiler up. This is ok, since this function is not necessary for
bare metal environments.

#### riscv<32|64|128>gc-unknown-none

Setting `eliminate-frame-pointer` to false will cause havoc, including, but not limited to, broken trait objects, jumps to
random locations and loads/stores from invalid addresses.

#### riscv<32|64|128>gc-unknown-uefi

LTO (link time optimization) must be disabled in `cargo.toml` for UEFI targets, otherwise compilation will fail with
`error: unknown flag`. Disabling LTO means that LLVM bitcode (sections `llvmbc` and `llvmcmd`) will not be included
in the binary.

### Assembly/Binary

#### Zero-sized types as statics

Storing zero-sized types as statics will result in a linker error, because the symbol was discarded and can thus not be
found.

#### SP, FP, GP

If something doesn't work, always check if these pointers are set correctly.

### Other

##### .cargo/config.toml

Renaming .cargo/config to .cargo/config.toml will result in the linker args being ignored by the linker.

##### QEMU RISC-V

Qemu won't load the kernel when linked with the default linker script (`linker/default.ld`). The kernel
must be linked with the layout defined in `linker/qemu-riscv-<ENV>.ld`.