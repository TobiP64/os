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
use core::ops::RangeBounds;
use crate::arch::*;

pub const VALID:     u64 = 0x0001;
pub const READ:      u64 = 0x0002;
pub const WRITE:     u64 = 0x0004;
pub const EXEC:      u64 = 0x0008;
pub const USER:      u64 = 0x0010;
pub const GLOBAL:    u64 = 0x0020;
pub const ACCESSED:  u64 = 0x0040;
pub const DIRTY:     u64 = 0x0080;
pub const RESERVED:  u64 = 0x0300;
pub const PPN_MASK:  u64 = 0x003F_FFFF_FFFF_FC00;
pub const PPN_SHIFT: u64 = 10;

pub type Sv32Table = [u32; 1024];
pub type Sv39Table = [u64; 512];
pub type Sv48Table = [u64; 512];

pub const MAX_LEVEL: usize = 3;

pub struct Entry(u64);

impl crate::arch::PageTableEntry for Entry {
	fn set_read(&mut self, v: bool) {
		if v {
			self.0 |= READ;
		} else {
			self.0 &= !READ;
		}
	}
	
	fn get_read(&self) -> bool {
		self.0 & READ == READ
	}
	
	fn set_write(&mut self, v: bool) {
		if v {
			self.0 |= WRITE;
		} else {
			self.0 &= !WRITE;
		}
	}
	
	fn get_write(&self) -> bool {
		self.0 & WRITE == WRITE
	}
	
	fn set_exec(&mut self, v: bool) {
		if v {
			self.0 |= EXEC;
		} else {
			self.0 &= !EXEC;
		}
	}
	
	fn get_exec(&self) -> bool {
		self.0 & EXEC == EXEC
	}
	
	fn set_user(&mut self, v: bool) {
		if v {
			self.0 |= USER;
		} else {
			self.0 &= !USER;
		}
	}
	
	fn get_user(&self) -> bool {
		self.0 & USER == USER
	}
	
	fn set_global(&mut self, v: bool) {
		if v {
			self.0 |= GLOBAL;
		} else {
			self.0 &= !GLOBAL;
		}
	}
	
	fn get_global(&self) -> bool {
		self.0 & GLOBAL == GLOBAL
	}
	
	fn set_valid(&mut self, v: bool) {
		if v {
			self.0 |= VALID;
		} else {
			self.0 &= !VALID
		}
	}
	
	fn get_valid(&self) -> bool {
		self.0 & VALID == VALID
	}
	
	fn set_ppn(&mut self, v: usize) {
		(self.0 = self.0 & !PPN_MASK | ((v as u64) << PPN_SHIFT)) as _
	}
	
	fn get_ppn(&self) -> usize {
		(self.0 & PPN_MASK >> PPN_SHIFT) as _
	}
}

pub struct RangeIter<'a> {
	t:   &'a PageTables,
	i:   usize,
	end: usize,
	e:   Entry,
}

impl<'a> Iterator for RangeIter<'a> {
	type Item = (Entry, usize);
	
	fn next(&mut self) -> Option<Self::Item> {
		if self.i >= self.end {
			return None;
		}

		unimplemented!()
	}
}

fn free_sub_tables(entry: u64, alloc: &mut impl PageAlloc) {
	let table = unsafe { ((entry & PPN_MASK << 2) as *mut Sv48Table).as_ref().unwrap() };
	
	for j in 0..512 {
		if table[j] & (READ | WRITE | EXEC) == 0 {
			for j in 0..512 {
				if table[j] & (READ | WRITE | EXEC) == 0 {
					alloc.free((table[j] & PPN_MASK << 2) as _);
				}
			}
			
			alloc.free((table[j] & PPN_MASK << 2) as _);
		}
	}
	
	alloc.free((entry & PPN_MASK << 2) as _);
}

pub struct PageTables(u64);

impl crate::arch::PageTableTrait for PageTables {
	type Entry     = Entry;
	type RangeIter<'a> = RangeIter<'a>;
	
	fn new() -> Self {
		Self(0 | satp::MODE_SV39 << satp::MODE_SHIFT)
	}
	
	fn current() -> Self {
		Self(satp.read())
	}
	
	fn map(
		&mut self,
		mut vpn:   usize,
		mut ppn:   usize,
		mut pages: usize,
		attrs:     usize,
		alloc:     &mut impl PageAlloc
	) {
		if self.0 & satp::PPN_MASK == 0 {
			self.0 |= alloc.alloc() as u64 >> 12;
		}
		
		match self.0 & satp::MODE_MASK >> satp::MODE_SHIFT {
			satp::MODE_SV32 => {
				// TODO
			}
			mode @ satp::MODE_SV39 | mode @ satp::MODE_SV48 => {
				let first_level = if mode == satp::MODE_SV48 { 0 } else { 1 };
				
				'outer: while pages > 0 {
					let mut table = unsafe { ((self.0 & satp::PPN_MASK << 12) as *mut Sv48Table)
						.as_mut() }.unwrap();
					
					for i in first_level..=MAX_LEVEL {
						let block_mask = !(!0 << (MAX_LEVEL - i) * 9);
						let block_pages = 512usize.pow((MAX_LEVEL - i) as _);
						let entry       = &mut table[vpn >> ((MAX_LEVEL - i) * 9) & 0x1FF];
						
						if vpn & block_mask == 0 && ppn & block_mask == 0 && pages >= block_pages {
							if *entry & VALID != 0 && *entry & (READ | WRITE | EXEC) == 0 { // if entry is a table, free all sub tables
								free_sub_tables(*entry, alloc);
							}
							
							vpn    += block_pages;
							ppn    += block_pages;
							pages  -= block_pages;
							*entry = (attrs as u64 & !PPN_MASK) | ((ppn as u64) << PPN_SHIFT);
							continue 'outer;
						} else if *entry & VALID == 0 {
							*entry = VALID | alloc.alloc() as u64 >> 2;
						} else if *entry & (READ | WRITE | EXEC) != 0 {
							let e = *entry;
							*entry = VALID | alloc.alloc() as u64 >> 2;
							
							table = unsafe { ((*entry & PPN_MASK << 2) as *mut Sv48Table).as_mut() }
								.unwrap();
							
							let mut vpn = e & PPN_MASK >> PPN_SHIFT;
							for entry in table {
								*entry = (e & !PPN_MASK) | (vpn << PPN_SHIFT);
								vpn += 1;
							}
						}
						
						table = unsafe { ((*entry & PPN_MASK << 2) as *mut Sv48Table).as_mut() }
							.unwrap();
					}
					
					unreachable!("mapping the given range failed");
				}
			},
			_ => unreachable!("invalid mode")
		}
	}
	
	fn remap(
		&mut self,
		_vpn:     usize,
		_new_vpn: usize,
		_pages:   usize,
		_attrs:   usize,
		_alloc:   &mut impl PageAlloc
	) {
		unimplemented!()
	}
	
	fn unmap(
		&mut self,
		mut vpn:   usize,
		mut pages: usize,
		alloc:     &mut impl PageAlloc
	) {
		if self.0 & satp::PPN_MASK == 0 {
			return;
		}
		
		match self.0 & satp::MODE_MASK >> satp::MODE_SHIFT {
			satp::MODE_SV32 => {
				// TODO
			}
			mode @ satp::MODE_SV39 | mode @ satp::MODE_SV48 => {
				let first_level = if mode == satp::MODE_SV48 { 0 } else { 1 };
				
				'outer: while pages > 0 {
					let mut table = unsafe { ((self.0 & satp::PPN_MASK << 12) as *mut Sv48Table)
						.as_mut() }.unwrap();
					
					for i in first_level..4 {
						let block_mask  = !(!0 << (MAX_LEVEL - i) * 9);
						let block_pages = 512usize.pow((MAX_LEVEL - i) as _);
						let entry       = &mut table[vpn >> ((MAX_LEVEL - i) * 9) & 0x1FF];
						
						if vpn & block_mask == 0 && pages >= block_pages {
							if *entry & VALID != 0 && *entry & (READ | WRITE | EXEC) == 0 { // if entry is a table, free all sub tables
								free_sub_tables(*entry, alloc);
							}
							
							vpn    += block_pages;
							pages  -= block_pages;
							*entry &= !VALID;
							continue 'outer;
						} else if *entry & VALID == 0 {
							vpn    += block_pages;
							pages  -= block_pages;
							continue 'outer;
						} else if *entry & (READ | WRITE | EXEC) != 0 {
							let e = *entry;
							*entry = VALID | alloc.alloc() as u64 >> 2;
							
							table = unsafe { ((*entry & PPN_MASK << 2) as *mut Sv48Table)
								.as_mut() }.unwrap();
							
							let mut vpn = e & PPN_MASK >> PPN_SHIFT;
							for entry in table {
								*entry = (e & !PPN_MASK) | (vpn << PPN_SHIFT);
								vpn += 1;
							}
						}
						
						table = unsafe { ((*entry & PPN_MASK << 2) as *mut Sv48Table)
							.as_mut() }.unwrap();
					}
					
					unreachable!("unmapping the given range failed");
				}
			},
			_ => unreachable!("invalid mode")
		}
	}
	
	#[inline]
	fn load(&self, flush: bool) {
		satp.write(self.0 as _);
		if flush { unsafe { sfence_vma2(); } }
	}
	
	fn translate(&self, mut vpn: usize) -> Self::Entry {
		let mut table = (self.0 & satp::PPN_MASK) as *mut Sv48Table;
		
		loop {
			let entry = unsafe { (*table)[vpn & 0x1FF] };
			vpn >>= 9;
			
			if entry & VALID == 0 || entry & (READ | WRITE | EXEC) != 0 {
				return Entry(entry);
			}
			
			table = ((entry & atp::PPN_MASK) >> PPN_SHIFT << 12) as *mut Sv48Table;
		}
	}
	
	fn iter<'a>(&'a self, _vpn_range: impl RangeBounds<usize>) -> Self::RangeIter<'a> {
		unimplemented!()
	}
	
	#[inline]
	fn set_asid(&mut self, asid: usize) {
		self.0 |= (asid as u64) << satp::ASID_SHIFT;
	}
	
	#[inline]
	fn get_asid(&self) -> usize {
		(self.0 & satp::ASID_MASK >> satp::ASID_SHIFT) as _
	}
}
