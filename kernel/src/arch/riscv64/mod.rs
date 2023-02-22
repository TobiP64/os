
pub mod int;
pub mod boot;

pub struct Timer {
    frq:      u64,
    mtime:    *mut u64,
	mtimecmp: *mut u64
}

impl Timer {
    pub fn set(&self, ns: u32) {
		unsafe { *self.mtimecmp = (*self.mtime) + self.frq / (1000000000000 / ns); }
    }
}

pub struct Context {
    pub status: u64,
    pub epc:    u64,
    pub tval:   u64,
    pub gpr: [u64; 31],
    pub fpr: [u128; 32],
    pub fsr: u32,
    pub vcr: [u128; 32],
    pub vsr: [u64; 7]
}