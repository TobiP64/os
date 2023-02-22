// MIT License
//
// Copyright (c) 2019-2023 Tobias Pfeiffer
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

pub mod gic;
/// VSMAv8-64, see ARMv8 ARM, chapter D5
pub mod vmsa;

#[inline]
pub unsafe fn data_mem_barrier() {
	llvm_asm!("dmb sy" ::: "memory" : "volatile")
}

macro_rules! def_reg {
	($reg:ident) => {
		#[allow(non_camel_case_types)]
		pub struct $reg;
		
		impl $reg {
			#[inline]
			pub fn write(&self, val: u64) {
				unsafe { llvm_asm!(concat!("msr ", stringify!($reg), ", $0") :: "r"(val) :: "volatile"); }
			}
			
			#[inline]
			pub fn write_ptr<T>(&self, val: *const T) {
				self.write(val as u64)
			}
			
			#[inline]
			pub fn read(&self) -> u64 {
				let val;
				unsafe { llvm_asm!(concat!("mrs $0, ", stringify!($reg)) : "=r"(val) ::: "volatile"); }
				val
			}
		}
	};
	( $( $reg:ident ),* ) => {
		$( def_reg!($reg); )*
	};
}

def_reg!(
	// general
	ACTLR_EL1,
	ACTLR_EL2,
	ACTLR_EL3,
	AFSR0_EL1,
	AFSR0_EL2,
	AFSR0_EL3,
	AFSR1_EL1,
	AFSR1_EL2,
	AFSR1_EL3,
	AIDR_EL1,
	AMAIR_EL1,
	AMAIR_EL2,
	AMAIR_EL3,
	APDAKeyHi_EL1,
	APDAKeyLo_EL1,
	APDBKeyHi_EL1,
	APDBKeyLo_EL1,
	APGAKeyHi_EL1,
	APGAKeyLo_EL1,
	APIAKeyHi_EL1,
	APIAKeyLo_EL1,
	APIBKeyHi_EL1,
	APIBKeyLo_EL1,
	CCSIDR2_EL1,
	CCSIDR_EL1,
	CLIDR_EL1,
	CONTEXTIDR_EL1,
	CONTEXTIDR_EL2,
	CPACR_EL1,
	CPTR_EL2,
	CPTR_EL3,
	CSSELR_EL1,
	CTR_EL0,
	DACR32_EL2,
	DCZID_EL0,
	ESR_EL1,
	ESR_EL2,
	ESR_EL3,
	FAR_EL1,
	FAR_EL2,
	FAR_EL3,
	FPEXC32_EL2,
	GCR_EL1,
	GMID_EL1,
	HACR_EL2,
	HCR_EL2,
	HPFAR_EL2,
	HSTR_EL2,
	ID_AA64AFR0_EL1,
	ID_AA64AFR1_EL1,
	ID_AA64DFR0_EL1,
	ID_AA64DFR1_EL1,
	ID_AA64ISAR0_EL1,
	ID_AA64ISAR1_EL1,
	ID_AA64MMFR0_EL1,
	ID_AA64MMFR1_EL1,
	ID_AA64MMFR2_EL1,
	ID_AA64PFR0_EL1,
	ID_AA64PFR1_EL1,
	ID_AFR0_EL1,
	ID_DFR0_EL1,
	ID_ISAR0_EL1,
	ID_ISAR1_EL1,
	ID_ISAR2_EL1,
	ID_ISAR3_EL1,
	ID_ISAR4_EL1,
	ID_ISAR5_EL1,
	ID_ISAR6_EL1,
	ID_MMFR0_EL1,
	ID_MMFR1_EL1,
	ID_MMFR2_EL1,
	ID_MMFR3_EL1,
	ID_MMFR4_EL1,
	ID_PFR0_EL1,
	ID_PFR1_EL1,
	ID_PFR2_EL1,
	IFSR32_EL2,
	ISR_EL1,
	LORC_EL1,
	LOREA_EL1,
	LORID_EL1,
	LORN_EL1,
	LORSA_EL1,
	MAIR_EL1,
	MAIR_EL2,
	MAIR_EL3,
	MIDR_EL1,
	MPIDR_EL1,
	MVFR0_EL1,
	MVFR1_EL1,
	MVFR2_EL1,
	PAR_EL1,
	REVIDR_EL1,
	RGSR_EL1,
	RMR_EL1,
	RMR_EL2,
	RMR_EL3,
	RNDR,
	RNDRRS,
	RVBAR_EL1,
	RVBAR_EL2,
	RVBAR_EL3,
	SCR_EL3,
	SCTLR_EL1,
	SCTLR_EL2,
	SCTLR_EL3,
	SCXTNUM_EL0,
	SCXTNUM_EL1,
	SCXTNUM_EL2,
	SCXTNUM_EL3,
	TCR_EL1,
	TCR_EL2,
	TCR_EL3,
	TFSRE0_EL1,
	TFSR_EL1,
	TFSR_EL2,
	TFSR_EL3,
	TPIDR_EL0,
	TPIDR_EL1,
	TPIDR_EL2,
	TPIDR_EL3,
	TPIDRRO_EL0,
	TTBR0_EL1,
	TTBR0_EL2,
	TTBR0_EL3,
	TTBR1_EL1,
	TTBR1_EL2,
	VBAR_EL1,
	VBAR_EL2,
	VBAR_EL3,
	VMPIDR_EL2,
	VNCR_EL2,
	VPIDR_EL2,
	VSTCR_EL2,
	VSTTBR_EL2,
	VTCR_EL2,
	VTTBR_EL2,
	// special purpose
	CurrentEL,
	DAIF,
	DIT,
	ELR_EL1,
	ELR_EL2,
	ELR_EL3,
	FPCR,
	FPSR,
	NZCV,
	PAN,
	SP_EL0,
	SP_EL1,
	SP_EL2,
	SP_EL3,
	SPSel,
	SPSR_abt,
	SPSR_EL1,
	SPSR_EL2,
	SPSR_EL3,
	SPSR_fiq,
	SPSR_irq,
	SPSR_und,
	SSBS,
	TCO,
	UAO,
	// timer
	CNTFRQ_EL0,
	CNTHCTL_EL2,
	CNTHP_CTL_EL2,
	CNTHP_CVAL_EL2,
	CNTHP_TVAL_EL2,
	CNTHPS_CTL_EL2,
	CNTHPS_CVAL_EL2,
	CNTHPS_TVAL_EL2,
	CNTHV_CTL_EL2,
	CNTHV_CVAL_EL2,
	CNTHV_TVAL_EL2,
	CNTHVS_CTL_EL2,
	CNTHVS_CVAL_EL2,
	CNTHVS_TVAL_EL2,
	CNTKCTL_EL1,
	CNTP_CTL_EL0,
	CNTP_CVAL_EL0,
	CNTP_TVAL_EL0,
	CNTPCT_EL0,
	CNTPS_CTL_EL1,
	CNTPS_CVAL_EL1,
	CNTPS_TVAL_EL1,
	CNTV_CTL_EL0,
	CNTV_CVAL_EL0,
	CNTV_TVAL_EL0,
	CNTVCT_EL0,
	CNTVOFF_EL2
);

impl ESR_EL1 {
	pub const ISS:                       u64 = 0x01FF_FFFF;
	pub const IL:                        u64 = 0x0200_0000;
	pub const EC_MASK:                   u64 = 0xFC00_0000;
	pub const EC_UNKNOWN:                u64 = 0b00_0000 << 26; // 0x00
	pub const EC_WFI_WFE:                u64 = 0b00_0001 << 26; // 0x01
	pub const EC_MCR_MRC:                u64 = 0b00_0011 << 26; // 0x02
	pub const EC_MCRR_MRRC:              u64 = 0b00_0100 << 26; // 0x04
	pub const EC_MCR_MRC_2:              u64 = 0b00_0101 << 26; // 0x05
	pub const EC_LDC_STC:                u64 = 0b00_0110 << 26; // 0x06
	pub const EC_SVE_SIMD_FP:            u64 = 0b00_0111 << 26; // 0x07
	pub const EC_VMRS:                   u64 = 0b00_1000 << 26; // 0x08
	pub const EC_PAUTH:                  u64 = 0b00_1001 << 26; // 0x09
	pub const EC_MRRC2:                  u64 = 0b00_1100 << 26; // 0x0C
	pub const EC_ILLEGAL_EXEC_STATE:     u64 = 0b00_1110 << 26; // 0x0E
	pub const EC_SVC_A32:                u64 = 0b01_0001 << 26; // 0x11
	pub const EC_HVC_A32:                u64 = 0b01_0010 << 26; // 0x12
	pub const EC_SMC_A32:                u64 = 0b01_0011 << 26; // 0x13
	pub const EC_SVC_A64:                u64 = 0b01_0101 << 26; // 0x15
	pub const EC_HVC_A64:                u64 = 0b01_0110 << 26; // 0x16
	pub const EC_SMC_A64:                u64 = 0b01_0111 << 26; // 0x17
	pub const EC_MSR_MRS_IDST:           u64 = 0b01_1000 << 26; // 0x18
	pub const EC_SVE:                    u64 = 0b01_1001 << 26; // 0x19
	pub const EC_ERET:                   u64 = 0b01_1010 << 26; // 0x1A
	pub const EC_IMPL_DEF_EL3:           u64 = 0b01_1111 << 26; // 0x1F
	pub const EC_INST_ABORT_LOWER_EL:    u64 = 0b10_0000 << 26; // 0x20
	pub const EC_INST_ABORT_SAME_EL:     u64 = 0b10_0001 << 26; // 0x21
	pub const EC_PC_ALIGNMENT_FAULT:     u64 = 0b10_0010 << 26; // 0x22
	pub const EC_DATA_ABORT_LOWER_EL:    u64 = 0b10_0100 << 26; // 0x24
	pub const EC_DATA_ABORT_SAME_EL:     u64 = 0b10_0101 << 26; // 0x25
	pub const EC_SP_ALIGNMENT_FAULT:     u64 = 0b10_0110 << 26; // 0x26
	pub const EC_FP_A32:                 u64 = 0b10_1000 << 26; // 0x28
	pub const EC_FP_A64:                 u64 = 0b10_1100 << 26; // 0x2C
	pub const EC_SERROR_INT:             u64 = 0b10_1111 << 26; // 0x2F
	pub const EC_BREAKPOINT_LOWER_EL:    u64 = 0b11_0000 << 26; // 0x30
	pub const EC_BREAKPOINT_SAME_EL:     u64 = 0b11_0001 << 26; // 0x31
	pub const EC_SOFTWARE_STEP_LOWER_EL: u64 = 0b11_0010 << 26; // 0x32
	pub const EC_SOFTWARE_STEP_SAME_EL:  u64 = 0b11_0011 << 26; // 0x33
	pub const EC_WATCHPOINT_LOWER_EL:    u64 = 0b11_0100 << 26; // 0x34
	pub const EC_WATCHPOINT_SAME_EL:     u64 = 0b11_0101 << 26; // 0x35
	pub const EC_BKPT_A32:               u64 = 0b11_1000 << 26; // 0x38
	pub const EC_VECTOR_CATCH_A32:       u64 = 0b11_1010 << 26; // 0x3A
	pub const EC_BRK_A32:                u64 = 0b11_1100 << 26; // 0x3C
}

impl SPSel {
	pub const USE_SP_ELX:     usize = 1 << 0;
}

impl TCR_EL1 {
	pub const T0SZ:     u64 = 0x3F;
	pub const EPD0:     u64 = 1 << 7;
	pub const IRGN0:    u64 = 0b11 << 8;
	pub const ORGN0:    u64 = 0b11 << 10;
	pub const SH0:      u64 = 0b11 << 12;
	pub const TG0:      u64 = 0b11 << 14;
	pub const TG0_4KB:  u64 = 0b00 << 14;
	pub const TG0_64KB: u64 = 0b01 << 14;
	pub const TG0_16KB: u64 = 0b10 << 14;
	pub const T1SZ:     u64 = 0x3F_0000;
	pub const A1:       u64 = 1 << 22;
	pub const EPD1:     u64 = 1 << 23;
	pub const IRGN1:    u64 = 0b11 << 24;
	pub const ORGN1:    u64 = 0b11 << 26;
	pub const SH1:      u64 = 0b11 << 28;
	pub const TG1:      u64 = 0b11 << 30;
	pub const TG1_4KB:  u64 = 0b00 << 30;
	pub const TG1_64KB: u64 = 0b01 << 30;
	pub const TG1_16KB: u64 = 0b10 << 30;
	pub const IPS:      u64 = 0b111 << 32;
	pub const AS:       u64 = 1 << 36;
	pub const TBI0:     u64 = 1 << 37;
	pub const TBI1:     u64 = 1 << 38;
}

impl CurrentEL {
	pub const EL0:           u64 = 0b0000;
	pub const EL1:           u64 = 0b0100;
	pub const EL2:           u64 = 0b1000;
	pub const EL3:           u64 = 0b1100;
}

impl DAIF {
	pub const F:                u64 = 1 << 6;
	pub const I:                u64 = 1 << 7;
	pub const A:                u64 = 1 << 8;
	pub const D:                u64 = 1 << 9;
}

impl SSBS {
	pub const SBSS: u64 = 1 << 12;
}

impl PAN {
	pub const PAN: u64 = 1 << 21;
}

impl UAO {
	pub const UAO: u64 = 1 << 23;
}

impl DIT {
	pub const DIT: u64 = 1 << 24;
}

impl TCO {
	pub const TCO: u64 = 1 << 25;
}

impl NZCV {
	pub const V: u64 = 1 << 28;
	pub const C: u64 = 1 << 29;
	pub const Z: u64 = 1 << 30;
	pub const N: u64 = 1 << 31;
}

impl SPSR_EL1 {
	pub const A32_MODE_MASK:           u64 = 0x7;
	pub const A32_EXEC_STATE:          u64 = 1 << 4;
	pub const T32_EXEC_STATE:          u64 = 1 << 5;
	pub const F:                       u64 = 1 << 6;
	pub const I:                       u64 = 1 << 7;
	pub const A:                       u64 = 1 << 8;
	pub const ENDIANNESS:              u64 = 1 << 9;
	pub const IT_MASK:                 u64 = 0x0600_FC00;
	pub const GE_MASK:                 u64 = 0x000F_0000;
	pub const IL:                      u64 = 1 << 20;
	pub const SS:                      u64 = 1 << 21;
	pub const PAN:                     u64 = 1 << 22;
	pub const SSBS:                    u64 = 1 << 23;
	pub const DIT:                     u64 = 1 << 24;
	pub const Q:                       u64 = 1 << 27;
	pub const V:                       u64 = 1 << 28;
	pub const C:                       u64 = 1 << 29;
	pub const Z:                       u64 = 1 << 30;
	pub const N:                       u64 = 1 << 31;
}

impl ISR_EL1 {
	pub const F:                        u64 = 1 << 6;
	pub const I:                        u64 = 1 << 7;
	pub const A:                        u64 = 1 << 8;
}

#[inline]
pub unsafe fn svc<const N: usize>() {
	asm!("svc {0}", const N);
}

#[inline]
pub unsafe fn wfe() {
	llvm_asm!("wfe"::::"volatile")
}

#[inline]
pub unsafe fn wfi() {
	llvm_asm!("wfi"::::"volatile")
}

#[inline]
pub unsafe fn eret() -> ! {
	llvm_asm!("eret" :::: "volatile");
	core::hint::unreachable_unchecked()
}

#[inline]
pub unsafe fn r#yield() {
	llvm_asm!("yield"::::"volatile")
}

/// see ARMv8 ARM, chapter D1
pub mod interrupts {
	pub type ExceptionVector = [u64; 0x80];
	
	#[repr(C, align(2048))]
	pub struct ExceptionVectorTable {
		pub current_sp0_sync: ExceptionVector,
		pub current_sp0_irq:  ExceptionVector,
		pub current_sp0_fiq:  ExceptionVector,
		pub current_sp0_serr: ExceptionVector,
		pub current_spx_sync: ExceptionVector,
		pub current_spx_irq:  ExceptionVector,
		pub current_spx_fiq:  ExceptionVector,
		pub current_spx_serr: ExceptionVector,
		pub lower_a64_sync:   ExceptionVector,
		pub lower_a64_irq:    ExceptionVector,
		pub lower_a64_fiq:    ExceptionVector,
		pub lower_a64_serr:   ExceptionVector,
		pub lower_a32_sync:   ExceptionVector,
		pub lower_a32_irq:    ExceptionVector,
		pub lower_a32_fiq:    ExceptionVector,
		pub lower_a32_serr:   ExceptionVector
	}
}
