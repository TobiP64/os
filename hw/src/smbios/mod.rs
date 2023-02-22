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
#[derive(Debug)]
pub struct SmBios2EntryPoint {
	pub anchor:                  [u8; 4],
	pub checksum:                u8,
	pub length:                  u8,
	pub major_version:           u8,
	pub minor_version:           u8,
	pub max_struct_size:         u32,
	pub revision:                u8,
	pub formatted_area:          [u8; 5],
	pub intermediate_anchor:     [u8; 5],
	pub intermediate_checksum:   u8,
	pub structure_table_length:  u32,
	pub structure_table_address: u64,
	pub number_of_structures:    u32,
	pub bcd_revision:            u8
}

#[repr(C)]
#[derive(Debug)]
pub struct SmBios3EntryPoint {
	pub anchor:                   [u8; 4],
	pub checksum:                 u8,
	pub length:                   u8,
	pub major_version:            u8,
	pub minor_version:            u8,
	pub docrev:                   u8,
	pub revision:                 u8,
	pub _reserved0:               u8,
	pub structure_table_max_size: u64,
	pub structure_table_address:  u128
}