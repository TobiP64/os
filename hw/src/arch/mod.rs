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

#[cfg(target_arch = "x86_64")]
pub mod amd64;
#[cfg(target_arch = "aarch64")]
pub mod aarch64;
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64", target_arch = "riscv128"))]
pub mod riscv;

#[cfg(target_arch = "x86_64")]
pub use amd64::*;
#[cfg(target_arch = "aarch64")]
pub use aarch64::*;
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64", target_arch = "riscv128"))]
pub use riscv::*;

#[repr(transparent)]
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct RW<T>(T);

impl<T> RW<T> {
	pub fn read(&self) -> T {
		unsafe { (&self.0 as *const T).read_volatile() }
	}

	pub fn write(&mut self, val: T) {
		unsafe { (&mut self.0 as *mut T).write_volatile(val); }
	}

	pub fn swap(&mut self, val: T) -> T {
		let v = self.read();
		self.write(val);
		v
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct RO<T>(T);

impl<T> RO<T> {
	pub fn read(&self) -> T {
		unsafe { (&self.0 as *const T).read_volatile() }
	}
}

#[repr(transparent)]
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct WO<T>(T);

impl<T> WO<T> {
	pub fn write(&mut self, val: T) {
		unsafe { (&mut self.0 as *mut T).write_volatile(val); }
	}
}
