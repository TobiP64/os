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

//use {crate::stage2_arch::{PageAlloc, PageTableTrait}, core::ops::RangeBounds};

// all levels
pub const PRESENT:                    u64 = 0x0000_0000_0000_0001;
pub const ENABLE_WRITE_ACCESS:        u64 = 0x0000_0000_0000_0002;
pub const ENABLE_UNPRIVILEGED:        u64 = 0x0000_0000_0000_0004;
pub const PAGE_LEVEL_WRITE_THROUGH:   u64 = 0x0000_0000_0000_0008;
pub const PAGE_LEVEL_CACHE_DISABLE:   u64 = 0x0000_0000_0000_0010;
pub const ACCESSED:                   u64 = 0x0000_0000_0000_0020;
pub const AVAILABLE_MASK:             u64 = 0x7FFF_0000_0000_0E00;
pub const NO_EXEC:                    u64 = 0x8000_0000_0000_0000;

// last level only (4KB: level 1, 2MB: level 2, 1GB: level 3), IGNORED otherwise
pub const DIRTY:                      u64 = 0x0000_0000_0000_0040;
pub const GLOBAL_PAGE:                u64 = 0x0000_0000_0000_0100;

pub const PAGE_SIZE:                  u64 = 0x0000_0000_0000_0080;
// 4KB/2MB: level 2, 3, 1GB: level 3
pub const LVL1_PAGE_ATTR_TABLE:       u64 = 0x0000_0000_0000_0080;
// level 1 only
pub const LVL2_2MB_PAGE_ATTR_TABLE:   u64 = 0x0000_0000_0000_1000;
// level 2 @ 2MB only
pub const LVL3_1GB_PAGE_ATTR_TABLE:   u64 = 0x0000_0000_0000_1000;
// level 3 @ 1GB only

pub const BASE_ADDR_MASK:             u64 = 0x0000_FFFF_FFFF_F000;

pub const L2P_PAGE_ATTR_TABLE:        u64 = 0x0000_0000_0000_0800;
pub const L2P_BASE_ADDR_MASK:         u64 = 0x0000_FFFF_FFE0_0000;
pub const L3P_PAGE_ATTR_TABLE:        u64 = 0x0000_0000_0000_0800;
pub const L3P_BASE_ADDR_MASK:         u64 = 0x0000_FFFF_C000_0000;

pub type Table = [u64; 512];

pub struct Entry(u64);

impl crate::arch::PageTableEntry for Entry {
	fn set_read(&mut self, _: bool) {
	}
	
	fn get_read(&self) -> bool {
		true
	}
	
	fn set_write(&mut self, v: bool) {
		if v {
			self.0 |= ENABLE_WRITE_ACCESS;
		} else {
			self.0 &= !ENABLE_WRITE_ACCESS;
		}
	}
	
	fn get_write(&self) -> bool {
		self.0 & ENABLE_WRITE_ACCESS == ENABLE_WRITE_ACCESS
	}
	
	fn set_exec(&mut self, v: bool) {
		if !v {
			self.0 |= NO_EXEC;
		} else {
			self.0 &= !NO_EXEC;
		}
	}
	
	fn get_exec(&self) -> bool {
		self.0 & NO_EXEC != NO_EXEC
	}
	
	fn set_user(&mut self, v: bool) {
		if v {
			self.0 |= ENABLE_UNPRIVILEGED;
		} else {
			self.0 &= !ENABLE_UNPRIVILEGED;
		}
	}
	
	fn get_user(&self) -> bool {
		self.0 & ENABLE_UNPRIVILEGED == ENABLE_UNPRIVILEGED
	}
	
	fn set_global(&mut self, v: bool) {
		if v {
			self.0 |= GLOBAL_PAGE;
		} else {
			self.0 &= !GLOBAL_PAGE;
		}
	}
	
	fn get_global(&self) -> bool {
		self.0 & GLOBAL_PAGE == GLOBAL_PAGE
	}
	
	fn set_valid(&mut self, v: bool) {
		if v {
			self.0 |= PRESENT;
		} else {
			self.0 &= !PRESENT
		}
	}
	
	fn get_valid(&self) -> bool {
		self.0 & PRESENT == PRESENT
	}
	
	fn set_ppn(&mut self, _v: usize) {
		//self = self & !PPN_MASK | (v << PPN_SHIFT)
		unimplemented!()
	}
	
	fn get_ppn(&self) -> usize {
		//self & PPN_MASK >> PPN_SHIFT
		unimplemented!()
	}
}

pub struct PageTables(usize);

/*impl crate::stage2_arch::PageTableTrait for PageTables {
	type Entry     = Entry;
	type RangeIter = ();
	
	fn new() -> Self {
		unimplemented!()
	}
	
	fn current() -> Self {
		unimplemented!()
	}
	
	fn map(&mut self, vpn: usize, ppn: usize, pages: usize, attrs: usize, alloc: &mut impl PageAlloc) {
		unimplemented!()
	}
	
	fn remap(&mut self, vpn: usize, new_vpn: usize, pages: usize, attrs: usize, alloc: &mut impl PageAlloc) {
		unimplemented!()
	}
	
	fn unmap(&mut self, vpn: usize, pages: usize, alloc: &mut impl PageAlloc) {
		unimplemented!()
	}
	
	fn load(&self, flush: bool) {
		unimplemented!()
	}
	
	fn translate(&self, vpn: usize) -> Self::Entry {
		unimplemented!()
	}
	
	fn iter(&self, vpn_range: impl RangeBounds<usize>) -> Self::RangeIter {
		unimplemented!()
	}
	
	fn set_asid(&mut self, asid: usize) {
		unimplemented!()
	}
	
	fn get_asid(&self) -> usize {
		unimplemented!()
	}
}*/
