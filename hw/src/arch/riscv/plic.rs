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

#[repr(C)]
pub struct PLIC {
	/// If PLIC supports Interrupt Priorities, then each PLIC interrupt source can
	/// be assigned a priority by writing to its 32-bit memory-mapped priority register.
	/// A priority value of 0 is reserved to mean ''never interrupt'' and effectively
	/// disables the interrupt. Priority 1 is the lowest active priority while the
	/// maximun level of priority depends on PLIC implementation. Ties between global
	/// interrupts of the same priority are broken by the Interrupt ID; interrupts
	/// with the lowest ID have the highest effective priority.
	priority: [u32; 1024],
	/// The current status of the interrupt source pending bits in the PLIC core can
	/// be read from the pending array, organized as 32-bit register. The pending bit
	/// for interrupt ID N is stored in bit (N mod 32) of word (N/32). Bit 0 of word 0,
	/// which represents the non-existent interrupt source 0, is hardwired to zero.
	///
	/// A pending bit in the PLIC core can be cleared by setting the associated enable
	/// bit then performing a claim.
	pending:  [u32; 0x20],
	_res0:    [u32; 0x3E0],
	/// Each global interrupt can be enabled by setting the corresponding bit in the
	/// enables register. The enables registers are accessed as a contiguous array of
	/// 32-bit registers, packed the same way as the pending bits. Bit 0 of enable
	/// register 0 represents the non-existent interrupt ID 0 and is hardwired to 0.
	/// PLIC has 15872 Interrupt Enable blocks for the contexts. The context is referred
	/// to the specific privilege mode in the specific Hart of specific RISC-V processor
	/// instance. How PLIC organizes interrupts for the contexts (Hart and privilege mode)
	/// is out of RISC-V PLIC specification scope, however it must be spec-out in vendor’s
	/// PLIC specification.
	enable:   [[u32; 0x20]; 0x3E00],
	_res1:    [u32; 3800],
	ctxt:     [Context; 0x3E00]
}

#[repr(C)]
struct Context {
	/// LIC provides context based threshold register for the settings of a interrupt
	/// priority threshold of each context. The threshold register is a WARL field.
	/// The PLIC will mask all PLIC interrupts of a priority less than or equal to
	/// threshold. For example, a `threshold` value of zero permits all interrupts
	/// with non-zero priority.
	priority:       u32,
	/// Interrupt Claim Process
	///
	/// The PLIC can perform an interrupt claim by reading the claim/complete register,
	/// which returns the ID of the highest priority pending interrupt or zero if there
	/// is no pending interrupt. A successful claim will also atomically clear the
	/// corresponding pending bit on the interrupt source.
	///
	/// The PLIC can perform a claim at any time and the claim operation is not affected
	/// by the setting of the priority threshold register. The Interrupt Claim Process
	/// register is context based and is located at (4K alignement + 4) starts from
	/// offset 0x200000.
	///
	/// Interrupt Completion
	///
	/// The PLIC signals it has completed executing an interrupt handler by writing the
	/// interrupt ID it received from the claim to the claim/complete register. The PLIC
	/// does not check whether the completion ID is the same as the last claim ID for that
	/// target. If the completion ID does not match an interrupt source that is currently
	/// enabled for the target, the completion is silently ignored. The Interrupt Completion
	/// registers are context based and located at the same address with Interrupt Claim
	/// Process register, which is at (4K alignement + 4) starts from offset 0x200000.
	claim_complete: u32,
	_res0:          [u32; 0x3FE]
}

impl PLIC {
	pub fn set_priority(&mut self, id: usize, pri: u32) {
		self.priority[id] = pri;
	}
	
	pub fn get_priority(&self, id: usize) -> u32 {
		self.priority[id]
	}
	
	pub fn is_pending(&self, id: usize) -> bool {
		self.pending[id / 32] & !(1 << (id as u32 % 32)) != 0
	}
	
	pub fn enable(&mut self, context: usize, id: usize) {
		self.enable[context][id / 32] |= 1 << (id as u32 % 32);
	}
	
	pub fn disable(&mut self, context: usize, id: usize) {
		self.enable[context][id / 32] &= !(1 << (id as u32 % 32));
	}
	
	pub fn is_enabled(&self, context: usize, id: usize) -> bool {
		self.enable[context][id / 32] & !(1 << (id as u32 % 32)) != 0
	}
	
	pub fn set_priority_threshold(&mut self, context: usize, threshold: u32) {
		self.ctxt[context].priority = threshold;
	}
	
	pub fn get_priority_threshold(&mut self, context: usize) -> u32 {
		self.ctxt[context].priority
	}
	
	pub fn claim(&self, context: usize) -> u32 {
		self.ctxt[context].claim_complete
	}
	
	pub fn complete(&mut self, context: usize, id: u32) {
		self.ctxt[context].claim_complete = id;
	}
}