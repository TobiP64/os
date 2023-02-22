
#[no_mangle]
pub static AMD64_GDT_LEN_BASE: (u16, *mut u64) = (AMD64_GDT.len() - 1, AMD64_GDT.as_ptr());

#[no_mangle]
pub static AMD64_IDT_LEN_BASE: (u16, *mut u128) = (AMD64_IDT.len() - 1, AMD64_IDT.as_ptr());

#[no_mangle]
pub static AMD64_GDT: [u64; 3] = [0; 3];

#[no_mangle]
pub static AMD64_IDT: [u128; 256] = [0; 256];

pub const fn entry(f: fn(), options: u32) -> u128 {
    let offset = f as usize as u128;
    options << 16
    | offset & 0xFFFF
    | offset & 0xFFFF_FFFF_FFFF_0000 << 32
}

pub const INTERRUPT_VECTOR_APIC_TIMER:    u8 = 32;
pub const INTERRUPT_VECTOR_APIC_THERMAL:  u8 = 33;
pub const INTERRUPT_VECTOR_APIC_COUNTER:  u8 = 34;
pub const INTERRUPT_VECTOR_APIC_LINT0:    u8 = 35;
pub const INTERRUPT_VECTOR_APIC_LINT1:    u8 = 36;
pub const INTERRUPT_VECTOR_APIC_ERROR:    u8 = 37;
pub const INTERRUPT_VECTOR_APIC_SPURI:    u8 = 38;
pub const INTERRUPT_VECTOR_IPI_HART_UP:   u8 = 39;
pub const INTERRUPT_VECTOR_IPI_HART_DOWN: u8 = 40;
pub const INTERRUPT_VECTOR_IPI_PING:      u8 = 41;
pub const INTERRUPT_VECTOR_IPI_PONG:      u8 = 42;
pub const INTERRUPT_VECTOR_SYSCALL:       u8 = 128;

#[no_mangle]
pub extern "C" fn amd64_int_divide_by_zero() {

}

#[no_mangle]
extern "C" fn amd64_int_debug() {

}

#[no_mangle]
extern "C" fn amd64_int_nmi() {

}

#[no_mangle]
extern "C" fn amd64_int_breakpoint() {

}

#[no_mangle]
extern "C" fn amd64_int_overflow() {

}

#[no_mangle]
extern "C" fn amd64_int_bound_range() {

}

#[no_mangle]
extern "C" fn amd64_int_invalid_opcode() {

}

#[no_mangle]
extern "C" fn amd64_int_device_not_available() {

}

#[no_mangle]
extern "C" fn amd64_int_double_fault() {

}

#[no_mangle]
extern "C" fn amd64_int_invalid_tss() {

}

#[no_mangle]
extern "C" fn amd64_int_segment_not_present() {

}

#[no_mangle]
extern "C" fn amd64_int_stack() {

}

#[no_mangle]
extern "C" fn amd64_int_general_protection() {

}

#[no_mangle]
extern "C" fn amd64_int_page_fault() {

}

#[no_mangle]
extern "C" fn amd64_int_x87() {

}

#[no_mangle]
extern "C" fn amd64_int_alignment_check() {

}

#[no_mangle]
extern "C" fn amd64_int_machine_check() {

}

#[no_mangle]
extern "C" fn amd64_int_simd_floating_point() {

}

#[no_mangle]
extern "C" fn amd64_int_control_protection_exception() {

}

#[no_mangle]
extern "C" fn amd64_int_hypervisor_injection_exception() {

}

#[no_mangle]
extern "C" fn amd64_int_vmm_communication_exception() {

}

#[no_mangle]
extern "C" fn amd64_int_apic_timer() {

}

#[no_mangle]
extern "C" fn amd64_int_apic_thermal() {

}

#[no_mangle]
extern "C" fn amd64_int_apic_counter() {

}

#[no_mangle]
extern "C" fn amd64_int_apic_lint0() {

}

#[no_mangle]
extern "C" fn amd64_int_apic_lint1() {

}

#[no_mangle]
extern "C" fn amd64_int_apic_error() {

}

#[no_mangle]
extern "C" fn amd64_int_apic_spurious() {

}

#[no_mangle]
#[naked]
extern "C" fn amd64_int_syscall() {
    asm!("
		swapgs
		mov r10, rsp
		rdgsbase rsp
		swapgs
		push rcx
		push r8
		push r9
		push r10
		push r11
		push r12
		push r13
		push r14
		push r15
		cmp rax, 60 ; number of syscalls
		jle .1
		mov rax, 0 ; SYS_NOT_IMPLEMENTED
		sysret
	1:
		ljmp [SVC_TABLE + rax * 8]
	");
}


#[no_mangle]
#[naked]
extern "C" fn amd64_int_generic() {
    asm!("
        push rax
        push rbx
		push rcx
        push rdx
        push rdi
        push rsi
		push r8
		push r9
		push r10
		push r11
		push r12
		push r13
		push r14
		push r15
	");
}

#[no_mangle]
#[naked]
extern "C" fn amd64_int_sysret() {

}

#[no_mangle]
extern "C" fn amd64_int_ipi_hart_up() {
    crate::hart::handle_hart_up();
}

#[no_mangle]
extern "C" fn amd64_int_ipi_hart_down() {
    crate::hart::handle_hart_down();
}
