#[no_mangle]
unsafe extern fn _start() -> ! {
    // load tables
    hw::arch::lgdt(AMD64_GDT_BASE_LEN);
    hw::arch::lidt(AMD64_IDT_BASE_LEN);

    // setup syscalls
    hw::arch::Star.set(0);
    hw::arch::LStar.set(super::int::amd64_int_syscall as _);
    hw::arch::CStar.set(super::int::amd64_int_syscall as _);
    hw::arch::FMask.set(0);

    // init x2APIC
    hw::arch::APIC_BASE.set(0xFEE0_0000);
    hw::arch::APIC_BASE.set(0xFEE0_0800);
    hw::arch::APIC_BASE.set(0xFEE0_0C00);
    hw::arch::x2APIC_TILV.set(0x0002_0020);
    hw::arch::x2APIC_TMLV.set(0x0002_0021);
    hw::arch::x2APIC_PCLV.set(0x0002_0022);
    hw::arch::x2APIC_LI0V.set(0x0002_0023);
    hw::arch::x2APIC_LI1V.set(0x0002_0024);
    hw::arch::x2APIC_ERRV.set(0x0002_0025);
    hw::arch::x2APIC_SIV.set(0x0002_0026);

    (*AP).status = crate::hart::Hart::STATUS_BOOT_COMPLETED;

    hw::arch::sti();
}

#[link_section = "AMD64_AP_trampoline"]
#[no_mangle]
unsafe fn AMD64_AP_trampoline_start() {
    // disable interrupts
    hw::arch::cli();
    hw::arch::cld();

    (*AP).status = kernel_sv::hart::Hart::STATUS_BOOTING;

    asm!("
		mov	al 0xFF
		out 0xA1, al
		out 0x21, al
		nop
		nop

		mov sp, cs
		add sp, 0x1000

		; check if cpuid available
		pushfd
	    pop eax
	    mov ecx, eax
	    xor eax, 0x200000
	    push eax
	    popfd
	    pushfd
	    pop eax
	    xor eax, ecx
	    shr eax, 21
	    and eax, 1
	    push ecx
	    popfd
		test eax, eax
	    jz .AMD64_AP_trampoline_park
	");

    let check = core::arch::x86_64::__cpuid(0x00000001).ecx & (1 << 21) != 0			// x2APIC available
                && core::arch::x86_64::__cpuid(0x80000000).eax == 0x80000001			// cpuid 0x80000001 avalable
                && core::arch::x86_64::__cpuid(0x80000001).eax & 0x20102820 != 0;		// long mode available

    if !check {
        AMD64_AP_trampoline_park();
    }

    // init long mode
    asm!("
		xor ax, ax
		mov ds, ax
		mov es, ax
		mov fs, ax
		mov gs, ax
		mov ss, ax
	");

    hw::arch::lgdt(gdt);
    hw::arch::lidt(idt);
    hw::arch::CR0.set(0xE0000001);
    hw::arch::CR3.set(CR3);
    hw::arch::CR4.set(0x003300A0);
    hw::arch::Efer.set(0x00008901);

    // check long mode active
    if hw::arch::Efer.get() & (1 << 10) == 0 {
        AMD64_AP_trampoline_park();
    }

    _start();
}

#[link_section = "AMD64_AP_trampoline"]
#[no_mangle]
fn AMD64_AP_trampoline_park() {
    hw::arch::park();
}

#[link_section = "AMD64_AP_trampoline"]
#[no_mangle]
static mut CR3: u32 = 0;

#[link_section = "AMD64_AP_trampoline"]
#[no_mangle]
static mut AP: *mut kernel_sv::hart::Hart = core::ptr::null_mut();

global_asm!("
ALIGN 8
BITS 16
AMD64_AP_trampoline_start:
	; disable interrupts
	cli
	cld
	mov	al 0xFF
	out 0xA1, al
	out 0x21, al
	nop
	nop

	mov sp, cs

	; check if cpuid available
	pushfd
    pop eax
    mov ecx, eax
    xor eax, 0x200000
    push eax
    popfd
    pushfd
    pop eax
    xor eax, ecx
    shr eax, 21
    and eax, 1
    push ecx
    popfd
	test eax, eax
    jz .AMD64_AP_trampoline_park

	; check if x2APIC available
	mov eax, 0x00000001
	cpuid
	text ecx, 1 << 21
	jz .AMD64_AP_trampoline_park

	; check if cpuid 0x80000001 available
	mov eax, 0x80000000
	cpuid
	cmp eax, 0x80000001
	jb .AMD64_AP_trampoline_park

	; check if long mode available
	mov eax, 0x80000001
	cpuid
	test edx, 0x20102820
	jz .AMD64_AP_trampoline_park

	; init long mode
	xor ax, ax
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax
	mov ss, ax
	lidt [AMD64_AP_trampoline_data.IDT_length]
	lgdt [AMD64_AP_trampoline_data.GDT_length]
	mov eax, 0xE0000001
	mov cr0, eax
    mov eax, [AMD64_AP_trampoline_data.CR3]
    mov cr3, eax
	mov eax, 0x003300A0
    mov cr4, eax
    mov eax, 0x00008901
	mov ecx, 0xC0000080
    wrmsr

	; check long mode active
	mov ecx, 0xC0000080
    rdmsr
	test eax, 1 << 10
	jz .AMD64_AP_trampoline_park



AMD64_AP_trampoline_park:
	hlt
	jmp .AMD64_AP_trampoline_park

ALIGN 8
AMD64_AP_trampoline_data:
	.IDT_length		dw 0
	.IDT_base		dd 0
	.GDT_length		dw 24
	.GDT_base		dd GDT_null
	.GDT_null		dq 0x0000000000000000
	.GDT_code		dq 0x00209A0000000000
	.GDT_data		dq 0x0000920000000000
	.CR3     		dq 0
	.AP_active		dq 0
");