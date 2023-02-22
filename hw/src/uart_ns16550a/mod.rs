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

use hw::arch::*;

const DIVISOR: u16 = 592;

#[repr(C)]
pub struct Ns16550a {
	pub data: RW<u8>,
	pub ier:  RW<u8>,
	pub fcr:  RW<u8>,
	pub lcr:  RW<u8>,
	pub mcr:  RW<u8>,
	pub lsr:  RW<u8>,
	pub msr:  RW<u8>,
	pub scr:  RW<u8>
}

impl Ns16550a {
	pub unsafe fn init(&mut self) {
		//self.lcr.write(0b11);
		//self.fcr.write(0b1);
		//self.ier.write(0b1);
		
		let ptr = self as *mut Self as *mut u8;
		let lcr: u8 = (1 << 0) | (1 << 1);
		ptr.add(3).write_volatile(lcr);
		ptr.add(2).write_volatile(1 << 0);
		ptr.add(1).write_volatile(1 << 0);
		ptr.add(3).write_volatile(lcr | 1 << 7);
		ptr.add(0).write_volatile((DIVISOR & 0xff) as _);
		ptr.add(1).write_volatile((DIVISOR >> 8) as _);
		ptr.add(3).write_volatile(lcr);
	}
	
	pub unsafe fn read(&self) -> u8 {
		self.data.read()
	}
	
	pub unsafe fn write(&mut self, val: u8) {
		self.data.write(val)
	}
}

impl core::fmt::Write for Ns16550a {
	fn write_str(&mut self, s: &str) -> core::fmt::Result {
		s.bytes().for_each(|b| unsafe { self.write(b) });
		Ok(())
	}
}