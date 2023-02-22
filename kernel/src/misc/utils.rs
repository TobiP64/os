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

pub fn get_mem_unit(memory: usize) -> (&'static str, usize) {
	match memory {
		0..0x100000               => ("KB", memory + 0x200 >> 10),
		0x100000..0x40000000      => ("MB", memory + 0x80000 >> 20),
		0x40000000..0x10000000000 => ("GB", memory + 0x20000000 >> 30),
		_                         => ("TB", memory + 0x8000000000 >> 40)
	}
}

pub struct NoDbg<T>(pub T);

impl<T> core::fmt::Debug for NoDbg<T> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct(core::any::type_name::<T>())
			.finish()
	}
}

impl<T> core::ops::Deref for NoDbg<T> {
	type Target = T;
	
	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

impl<T> core::ops::DerefMut for NoDbg<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.0
	}
}

#[macro_export]
macro_rules! tag {
    ($ident:ident) => { #[allow(unused_unsafe)] unsafe { llvm_asm!(stringify!($ident: ) ::::"volatile"); } };
}