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

use super::{Volatile, CPUID_BCM2711};

#[derive(Debug)]
#[repr(C, align(0x1000))]
pub struct GpioRegisters {
	fsel: [u32; 6],
	set:  [u32; 3],
	clr:  [u32; 3],
	lev:  [u32; 3],
	eds:  [u32; 3],
	ren:  [u32; 3],
	fen:  [u32; 3],
	hen:  [u32; 3],
	len:  [u32; 3],
	aren: [u32; 3],
	afen: [u32; 3],
	pud:  u32,
	clk:  [u32; 3],
	_pad: [u32; 12],
	pull: [u32; 3]
}

#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub struct Gpio(pub usize);

impl Gpio {
	#[inline]
	pub fn act_led() -> Self {
		Self(match unsafe { crate::arch::cpuid() } {
			CPUID_BCM2711 => 42,
			_ => 47
		})
	}
	
	#[inline]
	pub unsafe fn select_input(self) {
		let ptr = &mut super::peripherals().gpio.fsel[self.0 / 0xA] as *mut u32;
		ptr.write_volatile(ptr.read_volatile() & !(0b111 << (self.0 % 0xA * 3)) as u32);
	}
	
	#[inline]
	pub unsafe fn select_output(self) {
		let ptr = &mut super::peripherals().gpio.fsel[self.0 / 0xA] as *mut u32;
		ptr.write_volatile(ptr.read_volatile() | (0b1 << (self.0 % 0xA * 3)) as u32);
	}
	
	#[inline]
	pub unsafe fn set(self) {
		super::peripherals().gpio.set[self.0 >> 5].ref_write_volatile(1 << (self.0 as u32 & 0x1F));
	}
	
	#[inline]
	pub unsafe fn clear(self) {
		super::peripherals().gpio.clr[self.0 >> 5].ref_write_volatile(1 << (self.0 as u32 & 0x1F));
	}
	
	#[inline]
	pub unsafe fn set_value(self, value: bool) {
		match value {
			true => self.set(),
			false => self.clear()
		}
	}
	
	#[inline]
	pub unsafe fn set_pull(self, pull: Pull) {
		if cfg!(feature = "bcm2711") {
			let ptr = &mut super::peripherals().gpio.pull[self.0 >> 4] as *mut u32;
			let shift = (self.0 as u32 & 0xF) << 1;
			ptr.write_volatile(ptr.read_volatile() & !(3 << shift) | ((pull as u32) << shift))
		} else {
			let ptr = &mut super::peripherals().gpio.clk[self.0 >> 5] as *mut u32;
			super::peripherals().gpio.pud.ref_write_volatile(pull as _);
			ptr.write_volatile(1 << (self.0 as u32 & 0x1F));
			super::peripherals().gpio.pud.ref_write_volatile(0);
			ptr.write_volatile(0);
		}
	}
	
	#[inline]
	pub unsafe fn get_pull(self) -> Pull {
		match super::peripherals().gpio.pull[self.0 >> 4]
			.ref_read_volatile() >> ((self.0 as u32 & 0xF) << 1) & 0x3 {
			1 => Pull::Up,
			2 => Pull::Down,
			_ => Pull::None,
		}
	}
}

pub enum Fn {
	In,
	Out,
	Alt0,
	Alt1,
	Alt2
}

#[repr(u32)]
pub enum Pull {
	None = 0,
	Up   = 1,
	Down = 2
}