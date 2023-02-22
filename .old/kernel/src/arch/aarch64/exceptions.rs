use super::*;
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

use crate::task::Task;

#[no_mangle]
#[naked]
pub unsafe extern fn handle_early_exception(x0: usize) {
	crate::eprintln!("An unexpected exception occurred: ESR_EL1 = {}, ELR_EL1={:#018x}, FAR_EL1={:#018x}",
			  ESR_EL1.read(), ELR_EL1.read(), FAR_EL1.read());
	park();
}

#[no_mangle]
#[naked]
pub unsafe extern fn handle_sync_exception() {
	// TODO backup some registers
	
	let es = ESR_EL1.read() & ESR_EL1::EC_MASK;
	let task = Task::current();
	
	if let ESR_EC_SVC_A64 | ESR_EC_SVC_A32 = es {
		llvm_asm!("handle_exception_syscall:"::::"volatile");
		task.syscall((ESR_EL1.read() & ESR_EL1::ISS) as usize)
	} else if let ESR_EC_INST_ABORT_SAME_EL | ESR_EC_INST_ABORT_LOWER_EL
		| ESR_EC_DATA_ABORT_SAME_EL | ESR_EC_DATA_ABORT_LOWER_EL = es {
		llvm_asm!("handle_exception_page_fault:"::::"volatile");
		crate::mem::handle_page_fault(task, stval.read() as _)
	}
	
	context_save();
	
	// everything from here is the task's fault
	
	llvm_asm!("mrs x1, ELR_EL1"); // pass return address as argument
	
	let int = match ESR_EL1.read() {
		ESR_EC_PC_ALIGNMENT_FAULT | ESR_EC_SP_ALIGNMENT_FAULT => Interrupt::InvalidMemRef,
		ESR_EC_FP_A32 | ESR_EC_FP_A64 => Interrupt::FloatingPointException,
		ESR_EC_BREAKPOINT_LOWER_EL | ESR_EC_BREAKPOINT_SAME_EL
		| ESR_EC_WATCHPOINT_LOWER_EL | ESR_EC_WATCHPOINT_SAME_EL
		| ESR_EC_SOFTWARE_STEP_LOWER_EL | ESR_EC_SOFTWARE_STEP_SAME_EL
		| ESR_EC_VECTOR_CATCH_A32 => Interrupt::Debug,
		ESR_EC_INST_ABORT_SAME_EL | ESR_EC_INST_ABORT_LOWER_EL
		| ESR_EC_DATA_ABORT_SAME_EL | ESR_EC_DATA_ABORT_LOWER_EL => {
			llvm_asm!("mrs x2, FAR_EL1");
			Interrupt::InvalidMemRef;
		}
		_ => ()
	};
	
	task.interrupt(int);
}

#[no_mangle]
#[naked]
pub unsafe extern fn handle_irq() {
	context_save();
	
	// TODO
	
	context_restore();
}

#[no_mangle]
#[naked]
pub unsafe extern fn handle_fiq() {
	context_save();
	
	// TODO
	
	context_restore();
}

#[no_mangle]
#[naked]
pub unsafe extern fn handle_serr() {
	park();
}