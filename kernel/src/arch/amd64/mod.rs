
pub mod int;
pub mod boot;

pub struct Hart {
    timer_frq: u64,
    arch_id:   u64,
}

impl Hart {
    pub fn timer_set(&self, ns: u64) {
        hw::arch::x2APIC_TDCR.set(0b111);
        hw::arch::x2APIC_TICR.set(self.timer_frq / (1000000000000 / ns));
    }

    pub fn timer_get(&self) -> u64 {
        hw::arch::x2APIC_TCCR.get()
    }

    pub fn send_ipi(vec: usize) {

    }
}

pub struct Timer {
    frq: u64
}

impl Timer {
    pub fn set(&self, ns: u64) {
        hw::arch::x2APIC_TDCR.set(0b111);
        hw::arch::x2APIC_TICR.set(self.frq / (1000000000000 / ns));
    }

    pub fn get(&self) -> u64 {
        hw::arch::x2APIC_TCCR.get()
    }
}

#[inline]
#[naked]
pub fn svc_enter() -> (usize, usize, usize, usize) {
    let (rbx, rdx, rdi, rsi);
    asm!("", out("rbx") = rbx, out("rdx") = rdx, out("rdi") = rdi, out("rsi") = rsi);
    (rbx, rdx, rdi, rsi)
}

#[inline]
#[naked]
pub fn svc_exit(rbx: usize, rdx: usize, rdi: usize, rsi: usize) -> ! {
    asm!("
		pop r15
		pop r14
		pop r13
		pop r12
		pop r11
		pop r10
		pop r9
		pop r8
		pop rcx
		mov rsp, r10
		sysret
	", in("rbx") = rbx, in("rdx") = rdx, in("rdi") = rdi, in("rsi") = rsi);
}

#[naked]
#[no_mangle]
pub fn context_save() {
    asm!("

	");
}

#[naked]
#[no_mangle]
pub fn context_restore_return() -> ! {
    asm!("

	");
}

#[naked]
#[no_mangle]
pub fn context_restore_return_partial() -> ! {
    asm!("

	");
}

pub struct Context {
    pub gpr: [u64; 16],
    pub fpr: [u64; 8],
    pub vcr: [[u64; 8]; 32],
    pub omr: [u64; 8]
}