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

pub mod apic;
pub mod atp;

pub use atp::*;

use core::arch::asm;

/// If this bit is cleared to 0, the page fault was caused by a not-present page. If this bit is set
/// to 1, the page fault was caused by a page-protection violation.
pub const PAGE_FAULT_ERROR_NOT_PRESENT: usize = 0x1;
/// If this bit is cleared to 0, the access that caused the page fault is a memory read. If this
/// bit is set to 1, the memory access that caused the page fault was a write. This bit does not
/// necessarily indicate the cause of the page fault was a read or write violation.
pub const PAGE_FAULT_ERROR_READ_WRITE:  usize = 0x2;
/// If this bit is cleared to 0, an access in supervisor mode (CPL=0, 1, or 2) caused the
/// page fault. If this bit is set to 1, an access in user mode (CPL=3) caused the page fault. This bit does
/// not necessarily indicate the cause of the page fault was a privilege violation.
pub const PAGE_FAULT_ERROR_USER_MODE:   usize = 0x4;
/// If this bit is set to 1, the page fault is a result of the processor reading a 1 from a
/// reserved field within a page-translation-table entry. This type of page fault occurs only when
/// CR4.PSE=1 or CR4.PAE=1. If this bit is cleared to 0, the page fault was not caused by the
/// processor reading a 1 from a reserved field.
pub const PAGE_FAULT_ERROR_RSV:         usize = 0x8;
/// If this bit is set to 1, it indicates that the access that caused the page fault was an
/// instruction fetch. Otherwise, this bit is cleared to 0. This bit is only defined if no-execute feature is
/// enabled (EFER.NXE=1 && CR4.PAE=1).
pub const PAGE_FAULT_ERROR_INST_FETCH:  usize = 0x10;

macro_rules! def_reg {
	( $( #[$outer:meta] )* $reg:ident) => {
		$( #[$outer] )*
		#[allow(non_camel_case_types)]
		pub struct $reg;

		impl $reg {
			#[inline(always)]
			pub fn set(&self, val: u64) {
                unsafe { asm!(concat!("mov ", stringify!($reg), ", {0}"), in(reg) val); }
			}

			#[inline(always)]
			pub fn get(&self) -> u64 {
				let val;
                unsafe { asm!(concat!("mov {0}, ", stringify!($reg)), out(reg) val); }
				val
			}
		}
	};
	( $( $( #[$outer:meta] )* $reg:ident ),* ) => {
		$( def_reg!( $( #[$outer] )* $reg ); )*
	};
}

def_reg!(
	CR0,
	CR2,
	CR3,
	CR4,
	CR8,
	DR0,
	DR1,
	DR2,
	DR3,
	DR6,
	DR7
);


impl CR0 {
    /// Protection Enabled
	pub const PE: u64 = 1 << 0;

    /// Monitor Coprocessor
	pub const MP: u64 = 1 << 1;

    /// Emulation
	pub const EM: u64 = 1 << 2;

    /// Task Switched
	pub const TS: u64 = 1 << 3;

    /// Extension Type
	pub const ET: u64 = 1 << 4;

    /// Numeric Error
	pub const NE: u64 = 1 << 5;

    /// Write Protect
	pub const WP: u64 = 1 << 16;

    /// Alignment Mask
	pub const AM: u64 = 1 << 18;

    /// Not Writethrough
	pub const NW: u64 = 1 << 29;

    /// Cache Disable
	pub const CD: u64 = 1 << 30;

    /// Paging
	pub const PG: u64 = 1 << 31;
}

impl CR4 {
    /// Virtual-8086 Mode Extensions
	pub const VME: u64 = 1 << 0;

    /// Protected-Mode Virtual Interrupts
	pub const PVI: u64 = 1 << 1;

    /// Time Stamp Disable
	pub const TSD: u64 = 1 << 2;

    /// Debugging Extensions
	pub const DE: u64 = 1 << 3;

    /// Page Size Extensions
	pub const PSE: u64 = 1 << 4;

    /// Physical-Address Extension
	pub const PAE: u64 = 1 << 5;

    /// Machine Check Enable
	pub const MCE: u64 = 1 << 6;

    /// Page-Global Enable
	pub const PGE: u64 = 1 << 7;

    /// Performance-Monitoring Counter Enable
	pub const PCE: u64 = 1 << 8;

    /// Operating System FXSAVE/FXRSTOR Support
	pub const OSFXSR: u64 = 1 << 9;

    /// Operating System Unmasked Exception Support
	pub const OSXMMEXCPT: u64 = 1 << 10;

    /// Enable RDFSBASE, RDGSBASE, WRFSBASE, and WRGSBASE instructions
	pub const FSGSBASE: u64 = 1 << 16;

    /// XSAVE and Processor Extended States Enable Bit
	pub const OSXSAVE: u64 = 1 << 18;

    /// Supervisor Mode Execution Prevention
	pub const SMEP: u64 = 1 << 20;

    /// Supervisor Mode Access Protection
	pub const SMAP: u64 = 1 << 21;
}

macro_rules! def_msr {
	( $( #[$outer:meta] )* $reg:ident: $msr:expr) => {
		$( #[$outer] )*
		#[allow(non_camel_case_types)]
		pub struct $reg;

		impl $reg {
			#[inline(always)]
			pub fn set(&self, val: u64) {
				let (val0, val1) = ((val & 0xFFFF_FFFF) as u32, (val >> 32) as u32);
                unsafe { asm!("wrmsr", in("ecx") $msr, in("eax") val0, in("edx") val1); }
			}

			#[inline(always)]
			pub fn get(&self) -> u64 {
				let (val0, val1): (u32, u32);
                unsafe { asm!("rdmsr", in("ecx") $msr, out("eax") val0, out("edx") val1); }
				val0 as u64 | (val1 as u64) << 32
			}
		}
	};
	( $( $( #[$outer:meta] )* $reg:ident: $msr:expr ),* ) => {
		$( def_msr!( $( #[$outer] )* $reg: $msr ); )*
	};
}

def_msr!(
	Efer:			0xC000_0080u32,
	Star:			0xC000_0081u32,
	LStar:			0xC000_0082u32,
	CStar:			0xC000_0083u32,
	FMask:			0xC000_0084u32,
	FsBase:			0xC000_0100u32,
	GsBase:			0xC000_0101u32,
	KernelGsBase:	0xC000_0102u32,
    APIC_BASE:      0x0000_001Bu32,
	x2APIC_ID:		0x0000_0802u32,
	x2APIC_VERSION:	0x0000_0803u32,
	x2APIC_TPR:		0x0000_0808u32,
	x2APIC_APR:		0x0000_0809u32,
	x2APIC_PPR:		0x0000_080Au32,
	x2APIC_EOI:		0x0000_080Bu32,
	x2APIC_LDR:		0x0000_080Du32,
	x2APIC_SIV:		0x0000_080Fu32,
	x2APIC_ISR0:	0x0000_0810u32,
	x2APIC_ISR1:	0x0000_0811u32,
	x2APIC_ISR2:	0x0000_0812u32,
	x2APIC_ISR3:	0x0000_0813u32,
	x2APIC_ISR4:	0x0000_0814u32,
	x2APIC_ISR5:	0x0000_0815u32,
	x2APIC_ISR6:	0x0000_0816u32,
	x2APIC_ISR7:	0x0000_0817u32,
	x2APIC_TMR0:	0x0000_0818u32,
	x2APIC_TMR1:	0x0000_0819u32,
	x2APIC_TMR2:	0x0000_081Au32,
	x2APIC_TMR3:	0x0000_081Bu32,
	x2APIC_TMR4:	0x0000_081Cu32,
	x2APIC_TMR5:	0x0000_081Du32,
	x2APIC_TMR6:	0x0000_081Eu32,
	x2APIC_TMR7:	0x0000_081Fu32,
	x2APIC_IRR0:	0x0000_0820u32,
	x2APIC_IRR1:	0x0000_0821u32,
	x2APIC_IRR2:	0x0000_0822u32,
	x2APIC_IRR3:	0x0000_0823u32,
	x2APIC_IRR4:	0x0000_0824u32,
	x2APIC_IRR5:	0x0000_0825u32,
	x2APIC_IRR6:	0x0000_0826u32,
	x2APIC_IRR7:	0x0000_0827u32,
	x2APIC_ESR:		0x0000_0828u32,
	x2APIC_ICR:		0x0000_0830u32,
	x2APIC_TILV:	0x0000_0832u32,
	x2APIC_TMLV:	0x0000_0833u32,
	x2APIC_PCLV:	0x0000_0834u32,
	x2APIC_LI0V:	0x0000_0835u32,
	x2APIC_LI1V:	0x0000_0836u32,
	x2APIC_ERRV:	0x0000_0837u32,
	x2APIC_TICR:	0x0000_0838u32,
	x2APIC_TCCR:	0x0000_0839u32,
	x2APIC_TDCR:	0x0000_083Eu32,
	x2APIC_SIPI:	0x0000_083Fu32,
	x2APIC_EAFR:	0x0000_0840u32,
	x2APIC_EACR:	0x0000_0841u32,
	x2APIC_SEOI:	0x0000_0842u32,
	x2APIC_IER0:	0x0000_0848u32,
	x2APIC_IER1:	0x0000_0849u32,
	x2APIC_IER2:	0x0000_084Au32,
	x2APIC_IER3:	0x0000_084Bu32,
	x2APIC_IER4:	0x0000_084Cu32,
	x2APIC_IER5:	0x0000_084Du32,
	x2APIC_IER6:	0x0000_083Eu32,
	x2APIC_IER7:	0x0000_084Fu32,
	x2APIC_EILV0:	0x0000_0850u32,
	x2APIC_EILV1:	0x0000_0851u32,
	x2APIC_EILV2:	0x0000_0852u32,
	x2APIC_EILV3:	0x0000_0853u32
);

pub struct RFLAGS;

impl RFLAGS {
	/// Alignment Check
	pub const AC: u64 = 1 << 18;

	pub fn set(&self, val: u64) {
		unsafe { asm!("pushq {0}; popfq", in(reg) val) }
	}

	pub fn get(&self) -> u64 {
		let val;
        unsafe { asm!("pushfq; popq {0}", out(reg) val); }
		val
	}
}

#[inline]
pub fn park() {
    loop {
        unsafe { hlt(); }
    }
}

#[inline]
pub unsafe fn hlt() {
    asm!("hlt");
}

#[inline]
pub unsafe fn sti() {
    asm!("sti");
}

#[inline]
pub unsafe fn cli() {
    asm!("cli");
}

#[inline]
pub unsafe fn cld() {
    asm!("cld");
}

#[inline]
pub unsafe fn int(v: usize) {
    asm!("int {0}", in(reg) v);
}

#[inline]
pub unsafe fn int3() {
    asm!("int3");
}


#[inline]
pub fn sysret() -> ! {
    asm!("sysret");
	core::hint::unreachable_unchecked();
}

#[inline]
pub unsafe fn iret() -> ! {
    asm!("iret");
	core::hint::unreachable_unchecked();
}

#[inline]
pub unsafe fn invlpg(addr: usize) {
    asm!("invlpg [{}]", in(reg) addr)

}

/// Flushes the TLB
#[inline]
pub unsafe fn reload_cr3() {
	CR3.set(CR3.get())
}

#[inline]
pub unsafe fn swapgs() {
    asm!("swapgs");
}

#[inline]
pub unsafe fn lgdt(gdt: &DescriptorTablePointer) {
    asm!("lgdt [{0}]", in(reg) gdt);
}

#[inline]
pub unsafe fn lidt(idt: &DescriptorTablePointer) {
    asm!("lidt [{0}]", in(reg) idt);
}

#[inline]
pub fn rdtsc32() -> (u32, u32) {
    let eax: u32;
    let edx: u32;
	asm!("rdtsc", out("eax") eax, out("edx") edx);
    (eax, edx)
}

#[inline]
pub fn rdtsc64() -> u64 {
    let (eax, edx) = rdtsc32();
    eax as u64 | ((edx as u64) << 32)
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
#[repr(align(16))]
pub struct InterruptDescriptorTable {
	pub divide_by_0:              IdtEntry,
	pub debug:                    IdtEntry,
	pub non_maskable_interrupt:   IdtEntry,
	pub breakpoint:               IdtEntry,
	pub overflow:                 IdtEntry,
	pub bound_range:              IdtEntry,
	pub invalid_opcode:           IdtEntry,
	pub device_not_available:     IdtEntry,
	pub double_fault:             IdtEntry,
	pub _reserved0:               IdtEntry,
	pub invalid_tss:              IdtEntry,
	pub segment_not_present:      IdtEntry,
	pub stack:                    IdtEntry,
	pub general_protection:       IdtEntry,
	pub page_fault:               IdtEntry,
	pub _reserved1:               IdtEntry,
	pub x87_fp_exception_pending: IdtEntry,
	pub alignment_check:          IdtEntry,
	pub machine_check:            IdtEntry,
	pub simd_fp:                  IdtEntry,
	pub _reserved2:               [IdtEntry; 9],
	pub vmm_communication_event:  IdtEntry,
	pub security_exception:       IdtEntry,
	pub _reserved3:               IdtEntry,
	pub software_external:        [IdtEntry; 256 - 32]
}

impl InterruptDescriptorTable {
	pub const fn zeroed() -> Self {
		Self {
			divide_by_0:              IdtEntry::zeroed(),
			debug:                    IdtEntry::zeroed(),
			non_maskable_interrupt:   IdtEntry::zeroed(),
			breakpoint:               IdtEntry::zeroed(),
			overflow:                 IdtEntry::zeroed(),
			bound_range:              IdtEntry::zeroed(),
			invalid_opcode:           IdtEntry::zeroed(),
			device_not_available:     IdtEntry::zeroed(),
			double_fault:             IdtEntry::zeroed(),
			_reserved0:               IdtEntry::zeroed(),
			invalid_tss:              IdtEntry::zeroed(),
			segment_not_present:      IdtEntry::zeroed(),
			stack:                    IdtEntry::zeroed(),
			general_protection:       IdtEntry::zeroed(),
			page_fault:               IdtEntry::zeroed(),
			_reserved1:               IdtEntry::zeroed(),
			x87_fp_exception_pending: IdtEntry::zeroed(),
			alignment_check:          IdtEntry::zeroed(),
			machine_check:            IdtEntry::zeroed(),
			simd_fp:                  IdtEntry::zeroed(),
			_reserved2:               [IdtEntry::zeroed(); 9],
			vmm_communication_event:  IdtEntry::zeroed(),
			security_exception:       IdtEntry::zeroed(),
			_reserved3:               IdtEntry::zeroed(),
			software_external:        [IdtEntry::zeroed(); 256 - 32]
		}
	}

	pub fn load(&'static self) {
		unsafe {
			lidt(&DescriptorTablePointer {
				base: self as *const _ as _,
				limit: (core::mem::size_of::<Self>() - 1) as _
			});
		}
	}
}

#[derive(Copy, Clone, Debug, Default)]
#[repr(C)]
pub struct DescriptorTablePointer {
	pub base:  u64,
	pub limit: u16
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct IdtEntry {
	ptr_low:          u16,
	pub gdt_selector: u16,
	pub options:      u16,
	ptr_middle:       u16,
	ptr_high:         u32,
	reserved:         u32
}

impl IdtEntry {
	pub const fn zeroed() -> Self {
		Self {
			ptr_low:      0,
			gdt_selector: 0,
			options:      0,
			ptr_middle:   0,
			ptr_high:     0,
			reserved:     0
		}
	}

	pub fn empty() -> Self {
		Self::default()
	}

	pub fn new(ptr: *const usize) -> Self {
		let mut s = Self::default();
		s.set_ptr(ptr);
		s
	}

	pub fn set_ptr(&mut self, ptr: *const usize) {
		let ptr = ptr as usize;
		self.ptr_low    = (ptr & 0xFFFF) as u16;
		self.ptr_middle = (ptr >> 16 & 0xFFFF) as u16;
		self.ptr_high   = (ptr >> 32) as u32;
	}
}

impl Default for IdtEntry {
	fn default() -> Self {
		Self {
			ptr_low:      0,
			gdt_selector: 0,
			options:      0b1110_0000_0000,
			ptr_middle:   0,
			ptr_high:     0,
			reserved:     0
		}
	}
}

impl core::fmt::Debug for IdtEntry {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("IdtEntry")
			.field("ptr", &((self.ptr_low as usize | (self.ptr_middle as usize) << 16 | (self.ptr_high as usize) << 32) as *const usize))
			.field("gdt_selector", &self.gdt_selector)
			.field("options", &self.options)
			.finish()
	}
}