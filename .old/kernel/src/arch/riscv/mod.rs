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

use crate::{task::Task, GLOBAL_DATA};
use hw::arch::*;

pub mod exceptions;

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Default)]
pub struct CoreImage {
	gpr:      [usize; 31],
	sstatus:  usize,
	satp:     usize,
	sepc: 	  usize,
	// F extension
	fpr:      [usize; 32],
	fcsr:     usize,
	// N extension
	sedeleg:  usize,
	sideleg:  usize,
	ustatus:  usize,
	utvec:    usize,
	uip:      usize,
	uie:      usize,
	uscratch: usize,
	uepc:     usize,
	ucause:   usize,
	utval:    usize
}

impl core::fmt::Display for CoreImage {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		for i in 0..self.gpr.len() {
			writeln!(f, "x{} {:#016X}", i + 1, self.gpr[i])?;
		}
		
		for i in 0..self.fpr.len() {
			writeln!(f, "f{} {:#016X}", i, self.fpr[i])?;
		}
		
		writeln!(f, "fcsr     {:#016X}", self.fcsr)?;
		writeln!(f, "sstatus  {:#016X}", self.sstatus)?;
		writeln!(f, "satp     {:#016X}", self.satp)?;
		writeln!(f, "sepc     {:#016X}", self.sepc)?;
		writeln!(f, "sedeleg  {:#016X}", self.sedeleg)?;
		writeln!(f, "sideleg  {:#016X}", self.sideleg)?;
		writeln!(f, "ustatus  {:#016X}", self.ustatus)?;
		writeln!(f, "utvec    {:#016X}", self.utvec)?;
		writeln!(f, "uip      {:#016X}", self.uip)?;
		writeln!(f, "uie      {:#016X}", self.uie)?;
		writeln!(f, "uscratch {:#016X}", self.uscratch)?;
		writeln!(f, "uepc     {:#016X}", self.uepc)?;
		writeln!(f, "ucause   {:#016X}", self.ucause)?;
		writeln!(f, "utval    {:#016X}", self.utval)?;
		Ok(())
	}
}

#[inline]
pub unsafe fn init_early_exception_handler() {
	stvec.write(exceptions::handle_early_exception as *const () as _);
}

#[inline]
pub unsafe fn jump_to_kernel_space() {
	llvm_asm!(r"
			auipc	t0, 0			# get pc value
			sub		t0, t0, $0		# subtract physical address
			add		t0, t0, $1		# add virtual address
			addi	t0, t0, 14		# add offset to address after jump
			jr		t0				# jump to virtual kernel
			nop
	" :: "r"(&crate::_text_start), "r"(crate::KERNEL_VIRT_BASE) : "t0" : "volatile");
}

#[inline]
pub unsafe fn set_sp(val: usize) {
	llvm_asm!("mv sp, $0"::"r"(val)::"volatile");
}

#[inline]
pub unsafe fn set_epc(val: usize) {
	sepc.write(val as _);
}

#[inline]
pub unsafe fn eret() -> ! {
	sret()
}

pub fn enable_user_mode_access() {
	unsafe { llvm_asm!("csrsi sstatus, $0" :: "i"(mstatus::SUM)); }
}

pub fn disable_user_mode_access() {
	unsafe { llvm_asm!("csrci sstatus, $0" :: "i"(mstatus::SUM)); }
}

#[inline]
pub fn context_set_current(ptr: *mut Task) {
	sscratch.write(ptr as _);
}

#[inline]
pub fn context_get_current() -> *mut Task {
	sscratch.read() as _
}

#[inline(always)]
pub extern fn context_save() {
	llvm_asm!(r"
		csrrw	x31, sscratch, x31
		sd		x1,  0(x31)
		sd		x2,  8(x31)
		sd		x3,  16(x31)
		sd		x4,  24(x31)
		sd		x5,  32(x31)
		sd		x6,  40(x31)
		sd		x7,  48(x31)
		sd		x8,  56(x31)
		sd		x9,  64(x31)
		sd		x10, 72(x31)
		sd		x11, 80(x31)
		sd		x12, 88(x31)
		sd		x13, 96(x31)
		sd		x14, 104(x31)
		sd		x15, 112(x31)
		sd		x16, 120(x31)
		sd		x17, 128(x31)
		sd		x18, 136(x31)
		sd		x19, 144(x31)
		sd		x20, 152(x31)
		sd		x21, 160(x31)
		sd		x22, 168(x31)
		sd		x23, 176(x31)
		sd		x24, 184(x31)
		sd		x25, 192(x31)
		sd		x26, 200(x31)
		sd		x27, 208(x31)
		sd		x28, 216(x31)
		sd		x29, 224(x31)
		sd		x30, 232(x31)
		csrrw	x1,  sscratch, x31
		csrr	x2,  sstatus
		csrr	x3,  satp
		csrr	x4,  sepc
		sd		x1,  240(x31)
		sd		x2,  248(x31)
		sd		x3,  256(x31)
		sd		x4,  264(x31)

		# check if extension save is required
		mv		x30, x1
		li		x2, 1
		slli	x2, x2, 63
		and		x1, x30, x2
		bne		x1, x2, 2f

0:		# save fprs
		li		x2, 0x6000
		and		x1, x30, x2
		bne		x1, x2, 1f
		
		fsd		f0,  272(x31)
		fsd		f1,  280(x31)
		fsd		f2,  288(x31)
		fsd		f3,  296(x31)
		fsd		f4,  304(x31)
		fsd		f5,  312(x31)
		fsd		f6,  320(x31)
		fsd		f7,  328(x31)
		fsd		f8,  336(x31)
		fsd		f9,  344(x31)
		fsd		f10, 352(x31)
		fsd		f11, 360(x31)
		fsd		f12, 368(x31)
		fsd		f13, 376(x31)
		fsd		f14, 384(x31)
		fsd		f15, 392(x31)
		fsd		f16, 400(x31)
		fsd		f17, 408(x31)
		fsd		f18, 416(x31)
		fsd		f19, 424(x31)
		fsd		f20, 432(x31)
		fsd		f21, 440(x31)
		fsd		f22, 448(x31)
		fsd		f23, 456(x31)
		fsd		f24, 464(x31)
		fsd		f25, 472(x31)
		fsd		f26, 480(x31)
		fsd		f27, 488(x31)
		fsd		f28, 496(x31)
		fsd		f29, 504(x31)
		fsd		f30, 512(x31)
		fsd		f31, 520(x31)
		csrr	x1, fcsr
		sd		x1,  528(x31)
		
1:		# save user mode interrupt registers
		li		x2,  0x18000
		and		x1,  x30, x2
		bne		x1,  x2, 2f
		
		csrr	x1,  sedeleg
		csrr	x2,  sideleg
		csrr	x3,  ustatus
		csrr	x4,  utvec
		csrr	x5,  uip
		csrr	x6,  uie
		csrr	x7,  uscratch
		csrr	x8,  uepc
		csrr	x9,  ucause
		csrr	x10, utval
		sd		x1,  536(x31)
		sd		x2,  544(x31)
		sd		x3,  552(x31)
		sd		x4,  560(x31)
		sd		x5,  568(x31)
		sd		x6,  576(x31)
		sd		x7,  584(x31)
		sd		x8,  592(x31)
		sd		x9,  600(x31)
		sd		x10, 608(x31)

2:
	"::::"volatile");
}

#[inline]
pub unsafe fn context_restore(task: &Task) -> ! {
	llvm_asm!(r"
		csrw	sscratch, x31
		ld		x30, 504(x31)		# load task's sstatus

		# check if any extension resotre is requried
		li		x1, 1
		slli	x1, x1, 63
		and		x30, x30, x1
		beqz	x30, 2f

0:		# restore user mode interrupts registers
		li		x1, 0x18000
		and		x30, x30, x1
		beqz	x30, 1f
		
		#csrci	sstatus, 0x18000
		ld		x1,  536(x31)
		ld		x2,  544(x31)
		ld		x3,  552(x31)
		ld		x4,  560(x31)
		ld		x5,  568(x31)
		ld		x6,  576(x31)
		ld		x7,  584(x31)
		ld		x8,  592(x31)
		ld		x9,  600(x31)
		ld		x10, 608(x31)
		csrw	sedeleg,  x1
		csrw	sideleg,  x2
		csrw	ustatus,  x3
		csrw	utvec,    x4
		csrw	uip,      x5
		csrw	uie,      x6
		csrw	uscratch, x7
		csrw	uepc,     x8
		csrw	ucause,   x9
		csrw	utval,    x10

1:		# restore fprs
		li		x1, 0x6000
		and		x30, x30, x1
		beqz	x30, 2f
		
		#csrci	sstatus, 0x6000
		fld		f0,  272(x31)
		fld		f1,  280(x31)
		fld		f2,  288(x31)
		fld		f3,  296(x31)
		fld		f4,  304(x31)
		fld		f5,  312(x31)
		fld		f6,  320(x31)
		fld		f7,  328(x31)
		fld		f8,  336(x31)
		fld		f9,  344(x31)
		fld		f10, 352(x31)
		fld		f11, 360(x31)
		fld		f12, 368(x31)
		fld		f13, 376(x31)
		fld		f14, 384(x31)
		fld		f15, 392(x31)
		fld		f16, 400(x31)
		fld		f17, 408(x31)
		fld		f18, 416(x31)
		fld		f19, 424(x31)
		fld		f20, 432(x31)
		fld		f21, 440(x31)
		fld		f22, 448(x31)
		fld		f23, 456(x31)
		fld		f24, 464(x31)
		fld		f25, 472(x31)
		fld		f26, 480(x31)
		fld		f27, 488(x31)
		fld		f28, 496(x31)
		fld		f29, 504(x31)
		fld		f30, 512(x31)
		fld		f31, 520(x31)
		ld		x1,  528(x31)
		csrw	fcsr, x1
		
2:		# restore gprs
		ld		x1,  248(x31)
		ld		x2,  256(x31)
		ld		x2,  264(x31)
		csrw	sstatus, x1
		csrw    satp,    x2
		csrw	sepc,    x3
		ld      x1,  0(x31)
		ld      x2,  8(x31)
		ld      x3,  16(x31)
		ld      x4,  24(x31)
		ld      x5,  32(x31)
		ld      x6,  40(x31)
		ld      x7,  48(x31)
		ld      x8,  56(x31)
		ld      x9,  64(x31)
		ld      x10, 72(x31)
		ld      x11, 80(x31)
		ld      x12, 88(x31)
		ld      x13, 96(x31)
		ld      x14, 104(x31)
		ld      x15, 112(x31)
		ld      x16, 120(x31)
		ld      x17, 128(x31)
		ld      x18, 136(x31)
		ld      x19, 144(x31)
		ld      x20, 152(x31)
		ld      x21, 160(x31)
		ld      x22, 168(x31)
		ld      x23, 176(x31)
		ld      x24, 184(x31)
		ld      x25, 192(x31)
		ld      x26, 200(x31)
		ld      x27, 208(x31)
		ld      x28, 216(x31)
		ld      x29, 224(x31)
		ld      x30, 232(x31)
		ld      x31, 240(x31)
		sret
	":: "{x31}"(task) ::"volatile");
	core::hint::unreachable_unchecked()
}

#[inline]
pub unsafe fn interrupts_enable() {
	unimplemented!()
}

#[inline]
pub unsafe fn interrupts_disable() {
	unimplemented!()
}

#[allow(unused_assignments)]
pub unsafe fn get_page_attrs(mut addr: usize) -> u64 {
	// switch to kernel address space, to access page tables
	let mut satp_ = GLOBAL_DATA.ttb_id as u64;
	llvm_asm!("csrrw $0, satp, $0" : "=r"(satp_) );
	
	let mut table = (satp_ & satp::PPN_MASK) as *mut Sv48Table;
	
	loop {
		let entry = (*table)[addr & 0x1FF];
		addr >>= 9;
		
		if entry & VALID == 0 || entry & (READ | WRITE | EXEC) != 0 {
			satp.write(satp_);
			return entry;
		}
		
		table = ((entry & atp::PPN_MASK) >> PPN_SHIFT << 12) as *mut Sv48Table;
	}
}