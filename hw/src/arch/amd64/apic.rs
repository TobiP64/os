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

pub struct Apic {
	pub _res0:            [u32; 0x8],
	pub id:               u32,
	pub _res1:            [u32; 0x7],
	pub version:          u32,
	pub _res2:            [u32; 0x7],
	pub tp:               u32,
	pub _res3:            [u32; 0x7],
	pub ap:               u32,
	pub _res4:            [u32; 0x7],
	pub pp:               u32,
	pub _res5:            [u32; 0x7],
	pub eoi:              u32,
	pub _res6:            [u32; 0x7],
	pub rr:               u32,
	pub _res7:            [u32; 0x7],
	pub ld:               u32,
	pub _res8:            [u32; 0x7],
	pub df:               u32,
	pub _res9:            [u32; 0x7],
	pub spiv:             u32,
	pub _res10:           [u32; 0x7],
}