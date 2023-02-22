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

pub use atp::*;

pub mod atp;
pub mod plic;
pub mod clint;

macro_rules! def_reg {
	( $( #[$outer:meta] )* $reg:ident) => {
		$( #[$outer] )*
		#[allow(non_camel_case_types)]
		pub struct $reg;
		
		impl $reg {
			#[inline(always)]
			pub fn write(&self, val: u64) {
				unsafe { llvm_asm!(concat!("mv ", stringify!($reg), ", $0") :: "r"(val) ::); }
			}
			
			#[inline(always)]
			pub fn write_ptr<T>(&self, val: *const T) {
				self.write(val as u64)
			}
			
			#[inline(always)]
			pub fn read(&self) -> u64 {
				let val;
				unsafe { llvm_asm!(concat!("mv $0, ", stringify!($reg)) : "=r"(val) :::); }
				val
			}
		}
	};
	( $( $( #[$outer:meta] )* $reg:ident ),* ) => {
		$( def_reg!( $( #[$outer] )* $reg ); )*
	};
}

def_reg!(
	sp,
	fp,
	gp,
	tp
);

macro_rules! def_csr {
	( $( #[$outer:meta] )* $reg:ident) => {
		$( #[$outer] )*
		#[allow(non_camel_case_types)]
		pub struct $reg;
		
		impl $reg {
			#[inline]
			pub fn write(&self, val: u64) {
				unsafe { llvm_asm!(concat!("csrw ", stringify!($reg), ", $0") :: "r"(val) :: "volatile"); }
			}
			
			#[inline]
			pub fn write_ptr<T>(&self, val: *const T) {
				self.write(val as u64)
			}
			
			#[inline]
			pub fn write_fn(&self, val: fn()) {
				self.write(val as *const () as u64)
			}
			
			#[inline]
			pub fn read(&self) -> u64 {
				let val;
				unsafe { llvm_asm!(concat!("csrr $0, ", stringify!($reg)) : "=r"(val) ::: "volatile"); }
				val
			}
		}
	};
	( $( $( #[$outer:meta] )* $reg:ident ),* ) => {
		$( def_csr!( $( #[$outer] )* $reg ); )*
	};
}

def_csr!(
	/// Machine ISA Register
	misa,
	/// Machine Vendor ID Register
	mvendorid,
	/// Machine Architecture ID
	marchid,
	/// Machine Implementation ID Register
	mimpid,
	/// Hart ID Register
	mhartid,
	/// Machine Status Register
	mstatus,
	/// Machine Trap-Vector Base-Address Register
	mtvec,
	/// Machine Exception Delegation Register
	medeleg,
	/// Machine Interrupt Delegation Register
	mideleg,
	/// Machine Interrupt-Pending Register
	mip,
	/// Machine Interrupt-Enable Register
	mie,
	/// Machine Scratch Register
	mscratch,
	/// Machine Exception Program Counter
	mepc,
	/// Machine Cause Register
	mcause,
	/// Machine Trap Value
	mtval,
	/// Machine Second Trap Value (Hypervisor Extension)
	mtval2,
	/// Machine Trap Instruction Register (Hypervisor Extension)
	mtinst,
	/// Hypervisor Status Register (Hypervisor Extension)
	hstatus,
	/// Hypervisor exception delegation register (Hypervisor Extension)
	hedeleg,
	/// Hypervisor interrupt delegation register (Hypervisor Extension)
	hideleg,
	/// Hypervisor virtual-interrupt-pending register (Hypervisor Extension)
	hvip,
	/// Hypervisor interrupt-pending register (Hypervisor Extension)
	hip,
	/// Hypervisor interrupt-pending register (Hypervisor Extension)
	hie,
	/// Hypervisor guest external interrupt-pending register (Hypervisor Extension)
	hgeip,
	/// Hypervisor guest external interrupt-enable register (Hypervisor Extension)
	hgeie,
	/// Hypervisor trap value register (Hypervisor Extension)
	htval,
	/// Hypervisor trap instruction register (Hypervisor Extension)
	htinst,
	/// Hypervisor guest address translation and protection register (Hypervisor Extension)
	hgatp,
	/// Supervisor Status Register
	sstatus,
	/// Supervisor Trap Vector Base Address Register
	stvec,
	/// Supervisor Exception Delegation Register (`N` User-level interrupts extension)
	sedeleg,
	/// Supervisor Interrupt Delegation Register (`N` User-level interrupts extension)
	sideleg,
	/// Supervisor Interrupt-Pending Register
	sip,
	/// Supervisor Interrupt-Enable Register
	sie,
	/// Supervisor Scratch Register
	sscratch,
	/// Supervisor Exception Program Counter
	sepc,
	/// Supervisor Cause Register
	scause,
	/// Supervisor Trap Value
	stval,
	/// Supervisor Address Translation and Protection Register
	satp,
	/// User Status Register (`N` User-level interrupts extension)
	ustatus,
	/// User Trap Vector Base Address Register (`N` User-level interrupts extension)
	utvec,
	/// User Interrupt-Pending Register (`N` User-level interrupts extension)
	uip,
	/// User Interrupt-Enable Register (`N` User-level interrupts extension)
	uie,
	/// User Scratch Register (`N` User-level interrupts extension)
	uscratch,
	/// User Exception Program Counter (`N` User-level interrupts extension)
	uepc,
	/// User Cause Register (`N` User-level interrupts extension)
	ucause,
	/// User Trap Value (`N` User-level interrupts extension)
	utval,
	/// Physical memory protection configuration
	pmpcfg0,
	/// Physical memory protection configuration
	pmpcfg1,
	/// Physical memory protection configuration
	pmpcfg2,
	/// Physical memory protection configuration
	pmpcfg3,
	/// Physical memory protection configuration
	pmpcfg4,
	/// Physical memory protection configuration
	pmpcfg5,
	/// Physical memory protection configuration
	pmpcfg6,
	/// Physical memory protection configuration
	pmpcfg7,
	/// Physical memory protection configuration
	pmpcfg8,
	/// Physical memory protection configuration
	pmpcfg9,
	/// Physical memory protection configuration
	pmpcfg10,
	/// Physical memory protection configuration
	pmpcfg11,
	/// Physical memory protection configuration
	pmpcfg12,
	/// Physical memory protection configuration
	pmpcfg13,
	/// Physical memory protection configuration
	pmpcfg14,
	/// Physical memory protection configuration
	pmpcfg15,
	/// Physical memory protection address register
	pmpaddr0
);

impl mstatus {
	/// User mode interrupt enable
	pub const UIE: u64 = 1 << 0;
	
	/// Supervisor mode interrupt enable
	pub const SIE: u64 = 1 << 1;
	
	/// Machine mode interrupt enable
	pub const MIE: u64 = 1 << 3;
	
	/// User mode previous interrupt enable
	pub const UPIE: u64 = 1 << 4;
	
	/// Supervisor mode previous interrupt enable
	pub const SPIE: u64 = 1 << 5;
	
	/// Machine mode previous interrupt enable
	pub const MPIE: u64 = 1 << 7;
	
	/// Supervisor mode previous privilege mode
	pub const SPP: u64 = 1 << 8;
	
	/// Supervisor mode previous privilege mode
	pub const MPP_MASK: u64 = 1 << 11 | 1 << 12;
	
	/// Floating point status
	pub const FS_MASK: u64 = 1 << 13 | 1 << 14;
	
	/// Extensions status
	pub const XS_MASK: u64 = 1 << 15 | 1 << 16;
	
	/// Modify privilege
	pub const MPRIV: u64 = 1 << 17;
	
	/// Supervisor User Memory access
	pub const SUM: u64 = 1 << 18;
	
	/// Make executable Readable
	pub const MXR: u64 = 1 << 19;
	
	/// Trap Virtual Memory
	pub const TVM: u64 = 1 << 20;
	
	/// Timeout Wait
	pub const TW: u64 = 1 << 21;
	
	/// Trap SRET
	pub const TSR: u64 = 1 << 22;
	
	/// User mode XLEN
	pub const UXL_MASK: u64 = 1 << 32 | 1 << 33;
	
	/// Supervisor mode XLEN
	pub const SXL_MASK: u64 = 1 << 34 | 1 << 35;
	
	/// Supervisor mode big endian
	pub const SBE: u64 = 1 << 36;
	
	/// Machine mode big endian
	pub const MBE: u64 = 1 << 36;
	
	/// Status dirty
	pub const SD: u64 = 1 << 63;
}

impl mcause {
	pub const USER_SOFTWARE_INTERRUPT:             isize = -0;
	pub const SUPERVISOR_SOFTWARE_INTERRUPT:       isize = -1;
	pub const VIRT_SUPERVISOR_SOFTWARE_INTERRUPT:  isize = -2;
	pub const MACHINE_SOFTWARE_INTERRUPT:          isize = -3;
	pub const USER_TIMER_INTERRUPT:                isize = -4;
	pub const SUPERVISOR_TIMER_INTERRUPT:          isize = -5;
	pub const VIRT_SUPERVISOR_TIMER_INTERRUPT:     isize = -6;
	pub const MACHINE_TIMER_INTERRUPT:             isize = -7;
	pub const USER_EXTERNAL_INTERRUPT:             isize = -8;
	pub const SUPERVISOR_EXTERNAL_INTERRUPT:       isize = -9;
	pub const VIRT_SUPERVISOR_EXTERNAL_INTERRUPT:  isize = -9;
	pub const MACHINE_EXTERNAL_INTERRUPT:          isize = -11;
	pub const SUPERVISOR_GUEST_EXTERNAL_INTERRUPT: isize = -12;
	pub const INSTRUCTION_ADDRESS_MISALIGNED:      isize = 0;
	pub const INSTRUCTION_ACCESS_FAULT:            isize = 1;
	pub const ILLEGAL_INSTRUCTION:                 isize = 2;
	pub const BREAKPOINT:                          isize = 3;
	pub const LOAD_ADDRESS_MISALIGNED:             isize = 4;
	pub const LOAD_ACCESS_FAULT:                   isize = 5;
	pub const STORE_ADDRESS_MISALIGNED:            isize = 6;
	pub const STORE_ACCESS_FAULT:                  isize = 7;
	pub const ECALL_U_MODE:                        isize = 8;
	pub const ECALL_S_MODE:                        isize = 9;
	pub const ECALL_M_MODE:                        isize = 11;
	pub const INSTRUCTION_PAGE_FAULT:              isize = 12;
	pub const LOAD_PAGE_FAULT:                     isize = 13;
	pub const STORE_PAGE_FAULT:                    isize = 15;
}

#[inline]
pub unsafe fn ecall() {
	llvm_asm!("ecall"::::"volatile")
}

#[inline]
pub unsafe fn ebreak() {
	llvm_asm!("ebreak"::::"volatile")
}

#[inline]
pub unsafe fn mret() -> ! {
	llvm_asm!("mret"::::"volatile");
	core::hint::unreachable_unchecked()
}

#[inline]
pub unsafe fn sret() -> ! {
	llvm_asm!("sret"::::"volatile");
	core::hint::unreachable_unchecked()
}

#[inline]
pub unsafe fn uret() -> ! {
	llvm_asm!("uret"::::"volatile");
	core::hint::unreachable_unchecked()
}

#[inline]
pub unsafe fn wfi() {
	llvm_asm!("wfi"::::"volatile")
}

#[inline]
pub unsafe fn sfence_vma(vaddr: usize, asid: usize) {
	llvm_asm!("sfence.vma $0, $1" :: "r"(vaddr), "r"(asid));
}

#[inline]
pub unsafe fn sfence_vma2() {
	llvm_asm!("sfence.vma" ::::);
}

#[repr(C)]
#[derive(Default)]
pub struct InterruptVectorTable {
	pub user_software:            Option<fn()>,
	pub supervisor_software:      Option<fn()>,
	pub virt_supervisor_software: Option<fn()>,
	pub machine_software:         Option<fn()>,
	pub user_timer:               Option<fn()>,
	pub supervisor_timer:         Option<fn()>,
	pub virt_supervisor_timer:    Option<fn()>,
	pub machine_timer:            Option<fn()>,
	pub user_external:            Option<fn()>,
	pub supervisor_external:      Option<fn()>,
	pub virt_supervisor_external: Option<fn()>,
	pub machine_external:         Option<fn()>
}

impl satp {
	pub const PPN_MASK:    u64 = 0x0000_0FFF_FFFF_FFFF;
	pub const ASID_MASK:   u64 = 0x0FFF_F000_0000_0000;
	pub const ASID_SHIFT:  u64 = 44;
	pub const MODE_MASK:   u64 = 0xF000_0000_0000_0000;
	pub const MODE_SHIFT:  u64 = 60;
	pub const MODE_BARE:   u64 = 0 << Self::MODE_SHIFT;
	pub const MODE_SV32:   u64 = 1 << Self::MODE_SHIFT;
	pub const MODE_SV39:   u64 = 8 << Self::MODE_SHIFT;
	pub const MODE_SV48:   u64 = 9 << Self::MODE_SHIFT;
	pub const MODE_SV57:   u64 = 10 << Self::MODE_SHIFT;
	pub const MODE_SV64:   u64 = 11 << Self::MODE_SHIFT;
}