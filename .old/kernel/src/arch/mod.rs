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
#[cfg(target_arch = "powerpc64")]
pub mod ppc64;

#[cfg(target_arch = "x86_64")]
pub use amd64::*;
#[cfg(target_arch = "aarch64")]
pub use aarch64::*;
#[cfg(any(target_arch = "riscv32", target_arch = "riscv64", target_arch = "riscv128"))]
pub use riscv::*;
#[cfg(target_arch = "powerpc64")]
pub use ppc64::*;

pub use hw::arch::*;

#[naked]
#[no_mangle]
#[inline(never)]
pub extern fn park() -> ! {
	loop { unsafe { wfi(); } }
}