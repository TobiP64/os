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

use super::*;
use crate::dri::arch::PageAlloc;

pub const VALID:                                 u64 = 0x0000_0000_0000_0001;
pub const TYPE:                                  u64 = 0x0000_0000_0000_0002;

// types, level 0, 1, 2
pub const TYPE_BLOCK:                            u64 = 0x0000_0000_0000_0000;
pub const TYPE_TABLE:                            u64 = 0x0000_0000_0000_0002;

// types, level 3
pub const TYPE_RESERVED:                         u64 = 0x0000_0000_0000_0000;
pub const TYPE_PAGE:                             u64 = 0x0000_0000_0000_0002;

// tables
pub const TABLE_IGNORED_MASK:                    u64 = 0x07F8_0000_0000_FFC0;
pub const TABLE_4K_NEXT_LEVEL_TABLE_ADDR_MASK:   u64 = 0x0000_FFFF_FFFF_F000;
pub const TABLE_16K_NEXT_LEVEL_TABLE_ADDR_MASK:  u64 = 0x0000_FFFF_FFFF_C000;
pub const TABLE_64K_NEXT_LEVEL_TABLE_ADDR_MASK:  u64 = 0x0000_FFFF_FFFF_0000;
pub const TABLE_PRIVILEGED_EXEC_NEVER:           u64 = 0x0800_0000_0000_0000;
pub const TABLE_UNPRIVILEGED_EXEC_NEVER:         u64 = 0x1000_0000_0000_0000;
pub const TABLE_ACCESS_DISABLE_UNPRIVILEGED:     u64 = 0x2000_0000_0000_0000;
pub const TABLE_ACCESS_DISABLE_WRITE:            u64 = 0x4000_0000_0000_0000;
pub const TABLE_NON_SECURE:                      u64 = 0x8000_0000_0000_0000;

// block and page
// lower attributes
pub const ATTR_ATTR_IDX_MASK:                    u64 = 0x0000_0000_0000_001C;
pub const ATTR_NON_SECURE:                       u64 = 0x0000_0000_0000_0020;
pub const ATTR_ACCESS_ENABLE_UNPRIVILEGED:       u64 = 0x0000_0000_0000_0040;
pub const ATTR_ACCESS_DISABLE_WRITE:             u64 = 0x0000_0000_0000_0080;
pub const ATTR_SHAREABILITY_MASK:                u64 = 0x0000_0000_0000_0300;
pub const ATTR_ACCESS_FLAG:                      u64 = 0x0000_0000_0000_0400;
pub const ATTR_NOT_GLOBAL:                       u64 = 0x0000_0000_0000_0800;
pub const ATTR_OA_MASK:                          u64 = 0x0000_0000_0000_F000;
pub const ATTR_BLOCK_TRANSITION_ENTRY:           u64 = 0x0000_0000_0001_0000;
// upper attributes
pub const ATTR_DIRTY_BIT_MODIFIER:               u64 = 0x0004_0000_0000_0000;
pub const ATTR_CONTIGUOUS:                       u64 = 0x0008_0000_0000_0000;
pub const ATTR_PRIVILEGED_EXEC_NEVER:            u64 = 0x0010_0000_0000_0000;
pub const ATTR_UNPRIVILEGED_EXEC_NEVER:          u64 = 0x0020_0000_0000_0000;
pub const ATTR_IGNORED:                          u64 = 0x8780_0000_0000_0000;
pub const ATTR_PAGE_BASED_HW_ATTRS_MASK:         u64 = 0x7800_0000_0000_0000;

// blocks
pub const BLOCK_NT:                              u64 = 0x0000_0000_0001_0000;
pub const BLOCK_GP:                              u64 = 0x0004_0000_0000_0000;
pub const BLOCK_LEVEL1_4K_OUTPUT_ADDR_MASK:      u64 = 0x0000_FFFF_C000_0000;
pub const BLOCK_LEVEL2_4K_OUTPUT_ADDR_MASK:      u64 = 0x0000_FFFF_FFE0_0000;
pub const BLOCK_LEVEL2_16K_OUTPUT_ADDR_MASK:     u64 = 0x0000_FFFF_FE00_0000;
pub const BLOCK_LEVEL2_64K_OUTPUT_ADDR_MASK:     u64 = 0x0000_FFFF_E000_0000;

// pages
pub const PAGE_4K_OUTPUT_ADDR_MASK:              u64 = 0x0000_FFFF_FFFF_F000;
pub const PAGE_16K_OUTPUT_ADDR_MASK:             u64 = 0x0000_FFFF_FFFF_C000;
pub const PAGE_64K_OUTPUT_ADDR_MASK:             u64 = 0x0000_FFFF_FFFF_0000;
pub const PAGE_64K_TA_MASK:                      u64 = 0x0000_0000_0000_F000;

// block/page sizes
pub const LEVEL1_4K_BLOCK_SIZE:  usize = 0x4000_0000;
pub const LEVEL1_64K_BLOCK_SIZE: usize = 0x400_0000_0000;
pub const LEVEL2_4K_BLOCK_SIZE:  usize = 0x20_0000;
pub const LEVEL2_16K_BLOCK_SIZE: usize = 0x200_0000;
pub const LEVEL2_64K_BLOCK_SIZE: usize = 0x2000_0000;
pub const LEVEL3_4K_PAGE_SIZE:   usize = 0x1000;
pub const LEVEL3_16K_PAGE_SIZE:  usize = 0x4000;
pub const LEVEL3_64K_PAGE_SIZE:  usize = 0x1_0000;

// table types
pub type Level0Table4Kb  = [u64; 256];
pub type Level1Table4Kb  = [u64; 256];
pub type Level2Table4Kb  = [u64; 256];
pub type Level3Table4Kb  = [u64; 256];
pub type Level0Table16Kb = [u64; 2];
pub type Level1Table16Kb = [u64; 1024];
pub type Level2Table16Kb = [u64; 1024];
pub type Level3Table16Kb = [u64; 1024];
pub type Level1Table64Kb = [u64; 32];
pub type Level2Table64Kb = [u64; 4096];
pub type Level3Table64Kb = [u64; 4096];

pub struct PageTables {
	ttbr0: u64,
	ttbr1: u64,
	tcr:   u64
}

impl crate::dri::arch::PageTableTrait for PageTables {
	fn new() -> Self {
		unimplemented!()
	}
	
	fn current() -> Self {
		unimplemented!()
	}
	
	fn map_free(&mut self, _vpn: usize, _ppn: usize, _pages: usize, _flags: usize, _alloc: &mut impl PageAlloc) -> usize {
		unimplemented!()
	}
	
	fn map(&mut self, _vpn: usize, _ppn: usize, _pages: usize, _flags: usize, _alloc: &mut impl PageAlloc) {
		unimplemented!()
	}
	
	fn unmap(&mut self, _vpn: usize, _pages: usize, _alloc: &mut impl PageAlloc) {
		unimplemented!()
	}
	
	fn load(&self, _flush: bool) {
		TTBR0_EL1.store(self.ttbr0);
		TTBR1_EL1.store(self.ttbr1);
		TCR_EL1.store(self.tcr);
	}
	
	fn translate(&self, _vpn: usize) -> u64 {
		unimplemented!()
	}
	
	fn set_asid(&mut self, _asid: usize) {
		unimplemented!()
	}
	
	fn get_asid(&self) -> usize {
		unimplemented!()
	}
}
