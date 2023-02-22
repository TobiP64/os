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

static mut EXCEPTION_VECTOR_TABLE: InterruptDescriptorTable = InterruptDescriptorTable::zeroed();

pub unsafe fn init_exception_handling() {
	EXCEPTION_VECTOR_TABLE = InterruptDescriptorTable {
		divide_by_0:              IdtEntry::new(int_divide_by_0 as _),
		debug:                    IdtEntry::new(int_debug as _),
		non_maskable_interrupt:   IdtEntry::new(int_nmi as _),
		breakpoint:               IdtEntry::new(int_breakpoint as _),
		overflow:                 IdtEntry::new(int_overflow as _),
		bound_range:              IdtEntry::new(int_bound_range as _),
		invalid_opcode:           IdtEntry::new(int_invalid_opcode as _),
		device_not_available:     IdtEntry::new(int_device_not_available as _),
		double_fault:             IdtEntry::new(int_double_fault as _),
		_reserved0:               IdtEntry::empty(),
		invalid_tss:              IdtEntry::new(int_invalid_tss as _),
		segment_not_present:      IdtEntry::new(int_segment_not_present as _),
		stack:                    IdtEntry::new(int_stack as _),
		general_protection:       IdtEntry::new(int_general_protection as _),
		page_fault:               IdtEntry::new(int_page_fault as _),
		_reserved1:               IdtEntry::empty(),
		x87_fp_exception_pending: IdtEntry::new(int_x87_fp as _),
		alignment_check:          IdtEntry::new(int_alignment_check as _),
		machine_check:            IdtEntry::new(int_machine_check as _),
		simd_fp:                  IdtEntry::new(int_simd_fp as _),
		_reserved2:               [IdtEntry::empty(); 9],
		vmm_communication_event:  IdtEntry::new(int_vmm_com as _),
		security_exception:       IdtEntry::new(int_security as _),
		_reserved3:               IdtEntry::empty(),
		software_external:        [IdtEntry::empty(); 256 - 32]
	};
	EXCEPTION_VECTOR_TABLE.load();
	LStar.set(int_syscall as *const () as _);
}

#[naked]
#[no_mangle]
pub unsafe extern fn int_divide_by_0() {
	Task::current().interrupt(Interrupt::IllegalInstruction)
}

#[naked]
#[no_mangle]
pub unsafe extern fn int_debug() {
	Task::current().interrupt(Interrupt::Debug)
}

#[naked]
#[no_mangle]
pub unsafe extern fn int_nmi() {

}

#[naked]
#[no_mangle]
pub unsafe extern fn int_breakpoint() {
	Task::current().interrupt(Interrupt::Debug)
}

#[naked]
#[no_mangle]
pub unsafe extern fn int_overflow() {
	Task::current().interrupt(Interrupt::FloatingPointException)
}

#[naked]
#[no_mangle]
pub unsafe extern fn int_bound_range() {
	Task::current().interrupt(Interrupt::FloatingPointException)
}

#[naked]
#[no_mangle]
pub unsafe extern fn int_invalid_opcode() {
	Task::current().interrupt(Interrupt::IllegalInstruction)
}
#[naked]
#[no_mangle]
pub unsafe extern fn int_device_not_available() {
	Task::current().interrupt(Interrupt::FloatingPointException);
}

#[naked]
#[no_mangle]
pub unsafe extern fn int_double_fault() {

}

#[naked]
#[no_mangle]
pub unsafe extern fn int_invalid_tss() {

}

#[naked]
#[no_mangle]
pub unsafe extern fn int_segment_not_present() {

}

#[naked]
#[no_mangle]
pub unsafe extern fn int_stack() {

}

#[naked]
#[no_mangle]
pub unsafe extern fn int_general_protection() {
	Task::current().interrupt(Interrupt::InvalidMemRef);
}

#[naked]
#[no_mangle]
pub unsafe extern fn int_page_fault() {
	let addr = CR2.get();
}

#[naked]
#[no_mangle]
pub unsafe extern fn int_x87_fp() {
	Task::current().interrupt(Interrupt::FloatingPointException);
}


#[naked]
#[no_mangle]
pub unsafe extern fn int_alignment_check() {
	Task::current().interrupt(Interrupt::InvalidMemRef);
}


#[naked]
pub unsafe extern fn int_machine_check() {

}

#[naked]
#[no_mangle]
pub unsafe extern fn int_simd_fp() {
	Task::current().interrupt(Interrupt::FloatingPointException);
}


#[naked]
#[no_mangle]
pub unsafe extern fn int_vmm_com() {

}


#[naked]
#[no_mangle]
pub unsafe extern fn int_security() {

}

#[no_mangle]
pub unsafe extern fn int_syscall(rax: usize) {
	((*(*context_get_current()).syscall_table)[rax])();
	sysret();
}