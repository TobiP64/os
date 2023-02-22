#!/bin/bash

targets=(amd64-unknown-driver-uefi aarch64-unknown-none aarch64-unknown-albl aarch64-unknown-driver-uefi ppc64-unknown-none riscv32gc-unknown-none riscv32gc-unknown-sbi riscv32gc-unknown-driver-uefi riscv64gc-unknown-none riscv64gc-unknown-sbi riscv64gc-unknown-driver-uefi riscv128gc-unknown-none riscv128gc-unknown-sbi riscv128gc-unknown-driver-uefi)
targets_binutils=(["amd64"]="x86_64" ["aarch64"]="aarch64" ["ppc64"]="powerpc64le" ["riscv32gc"]="riscv64" ["riscv64gc"]="riscv64" ["riscv128gc"]="riscv128" )
targets_arch=(["amd64"]="i386:x86-64" ["aarch64"]="aarch64" ["ppc64"]="powerpc:common64" ["riscv32gc"]="rv32" ["riscv64gc"]="rv64" ["riscv128gc"]="rv128" )
qemu_opts="-smp 4 -m 4G -no-reboot -serial mon:stdio
	-drive if=none,format=raw,file=res/d0.img,id=d0
    -device virtio-rng-device
    -device virtio-gpu-device
    -device virtio-net-device
    -device virtio-tablet-device
    -device virtio-keyboard-device"
qemu_dbg="-d guest_errors,unimp,in_asm,int"

check_args () {
	if [[ $# < 2 ]]; then
		echo "Expecting 1 argument: $(basename $0) $1 <target>"
		echo "Try '$(basename $0) help' for more information."
		exit 1
	elif [[ ! " ${targets[@]} " =~ " $2 " ]]; then
		echo "Unsupported target: '$2'"
		echo "Try '$(basename $0) help' for a list of supported targets."
		exit 1
	fi
}

on_error () {
	echo -e "\033[0;31mfailed\033[0m"
	exit 1
}

on_success () {
	echo -e "\033[0;32mdone\033[0m"
}

check_continue () {
	read cont
	if [[ $cont != "y" && $cont != "Y" && $cont != "yes" ]]; then
		exit
	fi
}

if [[ $@ =~ "-h" || $@ =~ "--help" || $# == 0 || $1 == "help" ]]; then

	echo "
Usage: $(basename $0) <action>

Actions:
	setup   [--fix]                      Install the required dependencies for building the OS
	build|run|test|check|clippy <target> Execute 'cargo <build|run|test|check|clippy>'
	image   <image types>                Create a kernel or disk image
	install <target> <device>            Install the specified target to the specified device
	dump    <target>                     Display disassembly
	runner  <executable>                 Run qemu with UEFI, for internal use only
	help, -h, --help                     Show this help

Supported targets:"
	printf '	%s\n' "${targets[@]}"
	echo "
Supported image types:
	raw                       Creates a raw kernel image
	uefi                      Creates a bootable UEFI disk image
	rootfs                    Creates a root fs disk image
"

elif [[ $1 == "setup" ]]; then

	if [[ $2 == "--fix" ]]; then
		echo "Entered fix mode. Select your Problem:"
		echo "1: rust-std won't compile"

		read idx

		if [[ $idx == "1" ]]; then
			echo "This will reinstall rust-src. Continue? [y/N]: "
			check_continue
			# sometimes rust-src is buggy, re-add it
			rustup component remove rust-src
			rustup component add rust-src
		else
			echo "Invalid index"
			exit 1
		fi

		exit
	fi

	cargo install cargo-binutils
	sudo dnf install binutils-x86_64-linux-gnu binutils-aarch64-linux-gnu binutils-riscv64-linux-gnu binutils-powerpc64le-linux-gnu qemu

elif [[ $1 == "build" || $1 == "run" || $1 == "test" || $1 == "check" || $1 == "clippy" ]]; then

	check_args $@

	if [[ $2 == *"riscv"* && $2 == *"uefi"* ]]; then
		echo "LTO MUST BE DISABLED IN cargo.toml TO BUILD RISC-V UEFI TARGETS"
	fi

	if [[ $2 == "amd64-unknown-uefi" ]]; then
		target=x86_64-unknown-driver-uefi
	else
		target=targets/$2.json
	fi

	cargo $1 --release --target=$target

elif [[ $1 == "runner" ]]; then

	if [[ $2 == *"uefi"* ]]; then

		echo -e "    \033[0;1;32mCreating\033[0m UEFI bootable disk image \`target/efi.img\`\n"
		$0 image driver-uefi || exit

		uefi_tmp=$(mktemp -d -t driver-uefi-XXXXXXXX)
		cp -r /usr/share/edk2 $uefi_tmp

		if [[ $2 == *"x86_64-unknown-uefi"* ]]; then
			echo -e "\n     \033[0;1;32mRunning\033[0m qemu-system-x86_64\n"
			qemu-system-x86_64 -M q35 -cpu qemu64 -smp 4 -m 4G -no-reboot -serial mon:stdio \
				-pflash $uefi_tmp/edk2/ovmf/OVMF_CODE.fd \
				-pflash $uefi_tmp/edk2/ovmf/OVMF_VARS.fd \
				-hda target/efi.img \
				-device virtio-gpu-pci
		elif [[ $2 == *"aarch64-unknown-uefi"* ]]; then
			echo -e "\n     \033[0;1;32mRunning\033[0m qemu-system-aarch64\n"
			qemu-system-aarch64 -M virt -cpu cortex-a72 $qemu_opts \
				-pflash $uefi_tmp/edk2/aarch64/QEMU_EFI-pflash.raw \
				-pflash $uefi_tmp/edk2/aarch64/vars-template-pflash.raw \
				-hda target/efi.img
		else
			echo "Unsupported target: $2" && exit 1
		fi

		echo -e "\033[0m"

	else
		echo "Unsupported target: $2" && exit 1
	fi

elif [[ $1 == "image" ]]; then

	if [[ $# < 2 ]]; then
		echo "Expecting 1 argument: $(basename $0) $1 <image type>"
		echo "Try '$(basename $0) help' for more information."
		exit 1
	fi

	if [[ $2 == "uefi" ]]; then

		echo -n "Creating disk image      ... "
		dd if=/dev/zero of=./target/efi.img bs=1K count=102400 > /dev/null 2> /dev/null && on_success || on_error
		echo -n "Creating partitions      ... "
		echo -e "g\nn\n\n\n\nt\n1\nw"  | fdisk ./target/efi.img > /dev/null && on_success || on_error
		LOOP_DEV=$(sudo losetup --find)
		sudo losetup -P $LOOP_DEV target/efi.img || exit
		echo -n "Formatting EFI partition ... "
		sudo mkfs.fat -F32 "${LOOP_DEV}p1" > /dev/null && on_success || on_error
		mkdir mnt
		echo -n "Mounting EFI partition   ... "
		sudo mount "${LOOP_DEV}p1" mnt && on_success || on_error
		sudo mkdir mnt/EFI
		sudo mkdir mnt/EFI/boot
		sudo cp target/x86_64-unknown-driver-uefi/release/kernel.efi mnt/EFI/boot/bootx64.efi 2> /dev/null && echo "Added amd64 EFI executable"
		sudo cp target/aarch64-unknown-driver-uefi/release/kernel.efi mnt/EFI/boot/bootaa64.efi 2> /dev/null && echo "Added aarch64 EFI executable"
		sudo cp target/riscv32-unknown-driver-uefi/release/kernel.efi mnt/EFI/boot/bootrv32.efi 2> /dev/null && echo "Added riscv32 EFI executable"
		sudo cp target/riscv64-unknown-driver-uefi/release/kernel.efi mnt/EFI/boot/bootrv64.efi 2> /dev/null && echo "Added riscv64 EFI executable"
		sudo cp target/riscv128-unknown-driver-uefi/release/kernel.efi mnt/EFI/boot/bootrv128.efi 2> /dev/null && echo "Added riscv128 EFI executable"
        sudo unmount mnt
		echo -n "Formatting system partition ... "
		sudo mkfs.btrfs "${LOOP_DEV}p2" > /dev/null && on_success || on_error
        sudo mount "${LOOP_DEV}p2" mnt && on_success || on_error
		echo -n "Mounting system partition   ... "
        mkdir mnt/SYSCFG
        mkdir mnt/sys
        mkdir mnt/root
        mkdir mnt/template
        mkdir mnt/unauth
		echo -n "Cleaning up              ... "
		sudo umount mnt && rmdir mnt && sudo losetup -d $LOOP_DEV && on_success || on_error

		#fdisk -l target/efi.img

	elif [[ $2 == "raw" ]]; then

	    ${targets_binutils[$2]}-linux-gnu-objcopy -O binary ./target/$2-unknown-none/release/kernel ./target/kernel.$2.img --strip-all

	elif [[ $2 == "rootfs" ]]; then

		echo -n "Creating disk image ... "
		dd if=/dev/zero of=res/d0.img bs=1K count=102400 > /dev/null 2> /dev/null && on_success || on_error

	else
		echo "Unsupported image type: '$2'"
		echo "Try '$(basename $0) help' for a list of supported image types."
		exit 1
	fi

elif [[ $1 == "install" ]]; then

	if [[ $# < 3 ]]; then
		echo "Expecting 3 arguments:"
		echo "$(basename $0) install <target> <device>"
		echo "Try '$(basename $0) help' for more information."
		exit
	fi

	if [[ $@ != "-y" && $@ != "-Y" ]]; then
		echo "This will completely erase '$3' and install the '$2' target. Continue? [y/N]"
		check_continue
	fi

	echo "Not supported" && exit 1

elif [[ $1 == "dump" ]]; then

	check_args $@

	if [[ $2 == *"none"* ]]; then
		${targets_binutils[$2]}-linux-gnu-objdump -D ./target/$2/release/kernel --target=binary --architecture=${targets_arch[$2]}
	else
		echo "objdump not supported for boot mode '$3'" && exit 1
	fi

else

	echo "Unsupported action: '$1'"
	echo "Try '$(basename $0) help' for a list of available actions."
	exit 1

fi