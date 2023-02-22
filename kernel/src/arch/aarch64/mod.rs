
pub mod int;
pub mod boot;


// TODO align these via linker script

pub struct Timer;

impl Timer {
    pub fn set(&self, ns: u32) {
		hw::arch::CNTP_TVAL_EL0.set(hw::arch::CNTFRQ_EL0.get() / (1000000000000 / ns));
        hw::arch::CNTP_CTL_EL0.set(1);
    }
}

pub struct Context {
    pub spsr: u64,
    pub elr: u64,
    pub esr: u64,
    pub sp:  u64,
    pub tpidr: u64,
    pub gpr: [u64; 31],
    pub fpr: [u128; 32],
    pub fsr: [u64; 2]
}