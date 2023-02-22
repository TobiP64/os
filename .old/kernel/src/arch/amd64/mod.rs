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

use crate::{dri::arch::*, task::{Task, Interrupt}};
use core::hint::unreachable_unchecked;

pub mod exceptions;

static REG_NAMES: [&str; 16] = [
	"rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rsp", "rbp", "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15"
];

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Default)]
pub struct CoreImage {
	gpr:    [usize; 16],
	sse:    [[u64; 8]; 16],
	rflags: usize,
	cr3:    usize,
	gs:     usize,
	fs:     usize
}

impl core::fmt::Display for CoreImage {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		for i in 0..self.gpr.len() {
			writeln!(f, "r{} ({}) {:#016X}", i, REG_NAMES[i], self.gpr[i])?;
		}
		
		for i in 0..self.sse.len() {
			writeln!(f, "ymm{} {:#016X}{:#016X}{:#016X}{:#016X}{:#016X}{:#016X}{:#016X}{:#016X}", i,
					 self.sse[i][0], self.sse[i][1], self.sse[i][2], self.sse[i][3],
					 self.sse[i][4], self.sse[i][5], self.sse[i][6], self.sse[i][7])?;
		}
		
		writeln!(f, "rflags {:#016X}", self.rflags)?;
		writeln!(f, "cr3    {:#016X}", self.cr3)?;
		writeln!(f, "gs     {:#016X}", self.gs)?;
		writeln!(f, "fs     {:#016X}", self.fs)?;
		Ok(())
	}
}

#[naked]
#[inline]
pub unsafe fn wfi() {
	hlt();
}

#[naked]
#[inline]
pub unsafe fn set_sp(val: usize) {
	llvm_asm!("mv %rsp, $0"::"r"(val)::"volatile");
}

#[naked]
#[inline]
pub unsafe fn set_epc(val: usize) {
	unimplemented!()
}

#[naked]
#[inline]
pub unsafe fn eret() -> ! {
	swapgs();
	sysret();
}

pub fn enable_user_mode_access() {
	CR4.set(CR4.get() & !CR4::SMAP);
}

pub fn disable_user_mode_access() {
	CR4.set(CR4.get() | CR4::SMAP);
}

/// Only works if `swapgs` was called beforehand
#[inline]
pub fn context_set_current(ptr: *mut Task) {
	unsafe { llvm_asm!("rwgsbase $0":: "r"(ptr) ::"volatile"); }
}

/// Only works if `swapgs` was called beforehand
#[inline]
pub fn context_get_current() -> *mut Task {
	let ptr;
	unsafe { llvm_asm!("rdgsbase $0": "=r"(ptr) :::"volatile"); }
	ptr
}

#[naked]
#[inline]
pub unsafe fn context_save() {
	llvm_asm!(r"
		swapgs
		mov		%rax, %gs:(0)
		rdgsbase %rax
		mov		%rbx,   8(%rax)
		mov		%rcx,  16(%rax)
		mov		%rdx,  24(%rax)
		mov		%rsi,  32(%rax)
		mov		%rdi,  40(%rax)
		mov		%rsp,  48(%rax)
		mov		%rbp,  56(%rax)
		mov		%r8,   64(%rax)
		mov		%r9,   72(%rax)
		mov		%r10,  80(%rax)
		mov		%r11,  88(%rax)
		mov		%r12,  96(%rax)
		mov		%r13, 104(%rax)
		mov		%r14, 112(%rax)
		mov		%r15, 120(%rax)
		#mov		%rflags, 128(%rax)
		movq	%cr3, %rbx
		movq	%rbx, 136(%rax)
		swapgs
		rdgsbase %rbx
		swapgs
		rdfsbase %rcx
		mov		%rbx, 144(%rax)
		mov		%rcx, 152(%rax)
	"::::"volatile");
}

#[naked]
#[inline]
pub unsafe fn context_restore(task: &Task) -> ! {
	llvm_asm!(r"
		rwgsbase	%rax
		swapgs
		mov			%rcx, 152(%rax)
		mov			%rbx, 144(%rax)
		rwfsbase 	%rcx
		rwgsbase 	%rbx
		mov			136(%rax), %cr3
		#mov		128(%rax), %rflags
		movdqa		128(%rax), xmm0
		mov			120(%rax), %r15
		mov			112(%rax), %r14
		mov			104(%rax), %r13
		mov			96(%rax), %r12
		mov			88(%rax), %r11
		mov			80(%rax), %r10
		mov			72(%rax), %r9
		mov			64(%rax), %r8
		mov			56(%rax), %rbp
		mov			48(%rax), %rsp
		mov			40(%rax), %rdi
		mov			32(%rax), %rsi
		mov			24(%rax), %rdx
		mov			16(%rax), %rcx
		mov			8(%rax), %rbx
		mov			0(%rax), %rax
		sysretq
	":: "{rax}"(task) ::"volatile");
	unreachable_unchecked();
}

#[naked]
#[inline]
pub unsafe fn interrupts_enable() {
	sti();
}

#[naked]
#[inline]
pub unsafe fn interrupts_disable() {
	cli();
}
