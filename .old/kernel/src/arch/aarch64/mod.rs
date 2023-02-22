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

use crate::{dri::arch::*, task::{Task, Interrupt}, mem};
use core::hint::unreachable_unchecked;

pub mod exceptions;

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug, Default)]
pub struct CoreImage {
	gpr:   [usize; 32],
	fpr:   [u128;  32],
	sp:    usize,
	elr:   usize,
	spsr:  usize,
	ttbr0: usize,
	tpidr: usize
}

#[inline]
pub unsafe fn init_state() {

}

#[inline]
pub unsafe fn init_early_exception_handler() {

}

#[inline]
pub unsafe fn init_paging() {

}

#[inline]
pub unsafe fn set_sp(val: usize) {
	llvm_asm!("mov sp, $0":: "r"(val) ::"volatile");
}

#[inline]
pub unsafe fn set_epc(val: usize) {
	ELR_EL1.write(val as _);
}

#[naked]
#[inline]
pub unsafe fn eret() {
	super::eret();
}


pub fn enable_user_mode_access() {
}

pub fn disable_user_mode_access() {
}

#[inline]
pub fn context_set_current(ptr: *mut Task) {
	TPIDR_EL0.write(ptr as _);
}

#[inline]
pub fn context_get_current() -> *mut Task {
	TPIDR_EL0.read() as *mut Task
}

#[naked]
#[inline]
pub unsafe fn context_save() {
	llvm_asm!(r"
		str		x30,      [sp, #8]		// use redzone of kernel stack for temp fake sp
		mrs		x30, TPIDR_EL1
		stp		x0,  x1,  [x30], #16
		stp		x2,  x3,  [x30], #16
		stp		x4,  x5,  [x30], #16
		stp		x6,  x7,  [x30], #16
		stp		x8,  x9,  [x30], #16
		stp		x10, x11, [x30], #16
		stp		x12, x13, [x30], #16
		stp		x14, x15, [x30], #16
		stp		x16, x17, [x30], #16
		stp		x18, x19, [x30], #16
		stp		x20, x21, [x30], #16
		stp		x22, x23, [x30], #16
		stp		x24, x25, [x30], #16
		stp		x26, x27, [x30], #16
		stp		x28, x29, [x30], #16
		ldr		x0,       [sp, #8]
		str		x0,       [x30], #16
		stp		q0,  q1,  [x30], #32
		stp		q2,  q3,  [x30], #32
		stp		q4,  q5,  [x30], #32
		stp		q6,  q7,  [x30], #32
		stp		q8,  q9,  [x30], #32
		stp		q10, q11, [x30], #32
		stp		q12, q13, [x30], #32
		stp		q14, q15, [x30], #32
		stp		q16, q17, [x30], #32
		stp		q18, q19, [x30], #32
		stp		q20, q21, [x30], #32
		stp		q22, q23, [x30], #32
		stp		q24, q25, [x30], #32
		stp		q26, q27, [x30], #32
		stp		q28, q29, [x30], #32
		stp		q30, q31, [x30], #32
		mrs		x0,  SP_EL0
		mrs		x1,  ELR_EL1
		mrs		x2,  SPSR_EL1
		mrs		x3,  TTBR0_EL1
		mrs		x4,  TPIDR_EL0
		stp		x0,  x1,  [x30], #16
		stp		x2,  x3,  [x30], #16
		stp		x4,  x5,  [x30], #16
	"::::"volatile");
}

#[naked]
#[inline]
pub unsafe fn context_restore() -> ! {
	llvm_asm!(r"
		mrs		x0, TPIDR_EL1
		add		x0, x0, #816
		ldp		x5,  x6,  [x0], #-16
		ldp		x3,  x4,  [x0], #-16
		ldp		x1,  x2,  [x0], #-16
		msr		TPIDR_EL0, x5
		msr		TTBR0_EL1, x4
		msr		SPSR_EL1,  x3
		msr		ELR_EL1,   x2
		msr		SP_EL0,    x1
		ldp		q30, q31, [x0], #-32
		ldp		q28, q29, [x0], #-32
		ldp		q26, q27, [x0], #-32
		ldp		q24, q25, [x0], #-32
		ldp		q22, q23, [x0], #-32
		ldp		q20, q21, [x0], #-32
		ldp		q18, q19, [x0], #-32
		ldp		q16, q17, [x0], #-32
		ldp		q14, q15, [x0], #-32
		ldp		q12, q13, [x0], #-32
		ldp		q10, q11, [x0], #-32
		ldp		q8,  q9,  [x0], #-32
		ldp		q6,  q7,  [x0], #-32
		ldp		q4,  q5,  [x0], #-32
		ldp		q2,  q3,  [x0], #-32
		ldp		q0,  q1,  [x0], #-32
		ldr		x30,      [x0], #-16
		ldp		x28, x29, [x0], #-16
		ldp		x26, x27, [x0], #-16
		ldp		x24, x25, [x0], #-16
		ldp		x22, x23, [x0], #-16
		ldp		x20, x21, [x0], #-16
		ldp		x18, x19, [x0], #-16
		ldp		x16, x17, [x0], #-16
		ldp		x14, x15, [x0], #-16
		ldp		x12, x13, [x0], #-16
		ldp		x10, x11, [x0], #-16
		ldp		x8,  x9,  [x0], #-16
		ldp		x6,  x7,  [x0], #-16
		ldp		x4,  x5,  [x0], #-16
		ldp		x2,  x3,  [x0], #-16
		ldp		x0,  x1,  [x0]
		eret
	"::::"volatile");
	unreachable_unchecked();
}
