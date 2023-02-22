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

#[cfg(target_os = "none")]
pub mod none;
#[cfg(target_os = "sbi")]
pub mod sbi;
#[cfg(target_os = "albl")]
pub mod albl;
#[cfg(target_os = "driver-uefi")]
pub mod uefi;

/// This constant exists to check if an entry point is present, causing a compilation error if not.
#[cfg(any(target_os = "none", target_os = "sbi", target_os = "albl", target_os = "driver-uefi"))]
const ENTRY: usize = 0;
const ENTRY_DEFINED: usize = ENTRY;

#[cfg(target_os = "driver-uefi")]
pub struct BootParams<'a> {
	pub hartid:                usize,
	pub mem_map:               &'a [crate::mem::PhysMemoryArea],
	pub services:              &'a crate::dri::uefi::RuntimeServices,
	pub cfg_table:             &'a [crate::dri::uefi::ConfigurationTable],
	pub gop_info:              &'a crate::dri::uefi::GraphicsOutputModeInformation,
	pub gop_frame_buffer_base: *mut u8,
	pub gop_frame_buffer_size: usize
}

#[cfg(any(target_os = "none", target_os = "sbi"))]
pub struct BootParams<'a> {
	pub hartid:      usize,
	pub device_tree: &'a hw::device_tree::FdtHeader
}