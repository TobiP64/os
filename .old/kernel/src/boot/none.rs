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

use crate::{*, arch::*};

//pub const DEVICE_TREE: &[u8] = include_bytes!("../device_tree.dtb");

#[cfg(target_arch = "aarch64")]
#[naked]
#[no_mangle]
pub unsafe extern fn _start() -> ! {
	if MPIDR_EL1.read() & 0xFF != 0 {						// check core id
		
		park();												// park non-boot cores
		
	} else if CurrentEL.read() != 0b1000 {					// executing in EL1, no EL transition required
		
		set_sp(0x100000000);								// setup stack
		main();												// jump to main
		
	} else {												// executing in EL2, transition to EL1 required
		//               A IF   EL1h
		SPSR_EL2.write(0b1_1100_0101);						// disable interrupts, set target EL
		SP_EL1.write(0x100000000);							// setup stack
		ELR_EL2.write_ptr(main as *const ());				// write return address
		eret();												// jump to main and switch to EL1
	}
}

#[cfg(any(target_arch = "riscv32", target_arch = "riscv64", target_arch = "riscv128"))]
#[naked]
#[no_mangle]
pub unsafe extern fn _start() {
	if mhartid.read() != 0 {
		park();												// park non-boot cores
	}
	
	// TODO setup trap vector and sstatus, enable interrupts
	
	mtvec.write_ptr(crate::arch::park as *const ());		// setup interrupt handler
	mie.write(0);											// disable interrupts
	medeleg.write(0xFFFF);									// delegate all exceptions to supervisor mode
	mideleg.write(0xFFFF);									// delegate all interrupts to supervisor mode
	satp.write(0);											// disable paging
	sp.write_ptr(&_stack_end);								// set stack pointer
	fp.write_ptr(&_stack_end);								// set frame pointer
	gp.write_ptr(&_global_pointer);							// set global pointer
	llvm_asm!("csrr a0, mhartid");
	llvm_asm!("mv   a1, $0" :: "r"(DEVICE_TREE.as_ptr()) ::);
	mepc.write_ptr(main as *const ());						// set return address
	mret();													// jump to main and switch to supervisor mode
}