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

use super::*;
use crate::arch::park;

#[no_mangle]
pub unsafe extern fn handle_early_exception() -> ! {
	crate::eprintln!("An unexpected exception occurred: scause={}, sepc={:#018x}, stval={:#018x}",
			  scause.read(), sepc.read(), stval.read());
	park()
}

global_asm!(r"
.global handle_exception
handle_exception:
	csrrw		x1, sscratch, x1
	sd			t0, 0(x1)
	sd			t1, 8(x1)
	sd			t2, 16(x1)
	sd			t3, 24(x1)
	mv			t0, x1
	csrrw		x1, sscratch, x1
	csrr		t1, scause
	li			t2, 8
	bne			t1, t2, handle_exception__context_save
	
handle_exception__syscall:
	csrr		t1, sepc
	addi		t1, t1, 4
	csrw		sepc, t1
	
	ld      	t1, 696(t0)
	bgeu		a0, t1, handle_exception__syscall_not_implemented
	ld      	t1, 688(t0)
	slli    	a0, a0, 0x3
	add			t1, t1, a0
	ld			t1, (t1)
	mv			a0, t0
	jalr		t1
	sret
	
handle_exception__syscall_not_implemented:
	li			a0, 2
	sret
	
handle_exception__context_save:
	csrr		t2, sstatus
	sd			a0, 0(t0)
	sd			a1, 0(t0)
	sd			a2, 0(t0)
	sd			a3, 0(t0)
	sd			a4, 0(t0)
	sd			a5, 0(t0)
	sd			a6, 0(t0)
	sd			a7, 0(t0)
	sd			t4, 0(t0)
	sd			t5, 0(t0)
	sd			t6, 0(t0)
	sd			ra, 0(t0)

handle_exception__match:
	li 			a0, -5
	beq			t1, a0, handle_exception__timer_int
	li			a0, 11
	beq			t1, a0, handle_exception__page_fault__inst
	li			a0, 13
	beq			t1, a0, handle_exception__page_fault__load
	li			a0, 15
	beq			t1, a0, handle_exception__page_fault__store
	li			a0, 0
	beq			t1, a0, handle_exception__inv_mem_ref
	li			a0, 1
	beq			t1, a0, handle_exception__inv_mem_ref
	li			a0, 4
	beq			t1, a0, handle_exception__inv_mem_ref
	li			a0, 5
	beq			t1, a0, handle_exception__inv_mem_ref
	li			a0, 6
	beq			t1, a0, handle_exception__inv_mem_ref
	li			a0, 7
	beq			t1, a0, handle_exception__inv_mem_ref
	li			a0, 2
	beq			t1, a0, handle_exception__ill_inst
	li			a0, 3
	beq			t1, a0, handle_exception__debug
	
handle_exception__ignore:
	csrr		a0, sepc
	addi		a0, a0, 4
	csrw		sepc, a0
	sret
	
handle_exception__timer_int:
	mv			a0, t0
	jal			handle_timer_interrupt
	
handle_exception__page_fault__inst:
	li			a2, 12
	j			handle_exception__page_fault
	
handle_exception__page_fault__load:
	li			a2, 9
	j 			handle_exception__page_fault

handle_exception__page_fault__store:
	li			a2, 10

handle_exception__page_fault:
	mv			a0, t0
	csrr		a1, stval
	jal			handle_page_fault
	
handle_exception__inv_mem_ref:
	li			a0, 0
	j			handle_exception__task_int
	
handle_exception__ill_inst:
	li			a0, 1
	j			handle_exception__task_int
	
handle_exception__debug:
	li			a0, 3
	j			handle_exception__task_int
	
handle_exception__task_int:
	csrr		a1, sepc
	csrr		a2, stval
	
	ld      	a0, 704(t0)
	bnez		a0, handle_exception__task_int__abort
	csrw		sepc, a0
	sret
	
handle_exception__task_int__abort:
	mv			a0, t0
	li			a1, 0
	j			sys_task_exit
");

extern "C" {
	pub fn handle_exception();
}

/*#[naked]
#[no_mangle]
pub unsafe extern fn handle_exception(a0: usize) -> ! {
	let task:   &mut Task;
	let cause:  isize;
	let status: usize;
	
	llvm_asm!(r"
		csrrw		s0, sscratch, s0
		sd			t0, 0(s0)
		sd			t1, 8(s0)
		sd			t2, 16(s0)
		sd			t3, 24(s0)
		mv			t0, s0
		csrrw		s0, sscratch, s0
		csrr		t1, scause
		li			t2, 8
		bne			t1, t2, handle_exception__context_save
		
handle_exception__syscall:
		csrr		t1, sepc
		addi		t1, t1, 4
		csrw		sepc, t1
		
		ld      	t1, 696(t0)
		bgeu		a0, t1, handle_exception__syscall_not_implemented
		ld      	t1, 688(t0)
		slli    	a0, a0, 0x3
		add			t1, t1, a0
		ld			t1, (t1)
		mv			a0, t0
		jalr		t1
		sret
		
handle_exception__syscall_not_implemented:
		li			a0, 2
		sret
		
handle_exception__context_save:
		csrr		t2, sstatus
		sd			a0, 0(t0)
		sd			a1, 0(t0)
		sd			a2, 0(t0)
		sd			a3, 0(t0)
		sd			a4, 0(t0)
		sd			a5, 0(t0)
		sd			a6, 0(t0)
		sd			a7, 0(t0)
		mv			a0, t0
		mv			a1, t1
		mv			a2, t2
	" : "={a0}"(task), "={a1}"(cause), "={a2}"(status) :: "memory" : "volatile");
	
	let int = match cause {
		//EXCEPTION_ECALL_U_MODE => {
		//	tag!(handle_exception__syscall);
		//	sepc.write(sepc.read() + 4);
		//	task.syscall(a0);
		//},
		EXCEPTION_SUPERVISOR_TIMER_INTERRUPT => {
			tag!(handle_exception__timer_int);
			crate::sched::handle_timer_interrupt(task);
		}
		EXCEPTION_INSTRUCTION_PAGE_FAULT => {
			tag!(handle_exception__page_fault__exec);
			crate::mem::handle_page_fault(task, stval.read() as _, AREA_FLAG_EXEC | AREA_FLAG_USER);
			Interrupt::InvalidMemRef
		},
		EXCEPTION_LOAD_PAGE_FAULT => {
			tag!(handle_exception__page_fault__read);
			crate::mem::handle_page_fault(task, stval.read() as _, AREA_FLAG_READ | AREA_FLAG_USER);
			Interrupt::InvalidMemRef
		},
		EXCEPTION_STORE_PAGE_FAULT => {
			tag!(handle_exception__page_fault__write);
			crate::mem::handle_page_fault(task, stval.read() as _, AREA_FLAG_WRITE | AREA_FLAG_USER);
			Interrupt::InvalidMemRef
		},
		EXCEPTION_INSTRUCTION_ADDRESS_MISALIGNED
		| EXCEPTION_INSTRUCTION_ACCESS_FAULT
		| EXCEPTION_LOAD_ADDRESS_MISALIGNED
		| EXCEPTION_LOAD_ACCESS_FAULT
		| EXCEPTION_STORE_ADDRESS_MISALIGNED
		| EXCEPTION_STORE_ACCESS_FAULT => {
			tag!(handle_exception__invalid_mem_ref);
			Interrupt::InvalidMemRef
		},
		EXCEPTION_ILLEGAL_INSTRUCTION  => {
			tag!(handle_exception__ill_inst);
			Interrupt::IllegalInstruction
		},
		EXCEPTION_BREAKPOINT           => {
			tag!(handle_exception__debug);
			Interrupt::Debug
		},
		_ => {
			tag!(handle_exception__ignore);
			sepc.write(sepc.read() + 4);
			context_restore();
		}
	};
	
	tag!(handle_exception__task_int);
	llvm_asm!("csrr a1, sepc; csrr a2, stval");
	task.interrupt(int)
}*/