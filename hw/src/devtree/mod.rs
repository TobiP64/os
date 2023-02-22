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

use core::marker::PhantomData;
use core::mem::size_of;

pub const MAGIC: u32 = 0xd00dfeedu32.to_be();

#[repr(C)]
#[derive(Copy, Clone)]
pub struct FdtHeader {
	magic:                 u32,
	total_size:            u32,
	off_dt_struct:         u32,
	off_dt_strings:        u32,
	off_mem_rsvmap:        u32,
	pub version:           u32,
	pub last_comp_version: u32,
	pub boot_cpuid_phys:   u32,
	size_dt_strings:       u32,
	size_dt_structs:       u32
}

impl FdtHeader {
	pub fn is_valid(&self) -> bool {
		self.magic == MAGIC
	}
	
	pub fn memory_reservation_block(&self) -> FdtReserveEntryIter {
		FdtReserveEntryIter(unsafe { (self as *const FdtHeader as *const u8)
			.add(self.off_mem_rsvmap.to_be() as _) } as _, PhantomData)
	}
	
	pub fn memory_reservation_block_slice(&self) -> &[FdtReserveEntry] {
		let iter = self.memory_reservation_block();
		unsafe { core::slice::from_raw_parts(iter.0, iter.count()) }
	}
	
	pub fn structure_block(&self) -> FdtStructureIter {
		FdtStructureIter {
			ptr: unsafe { (self as *const FdtHeader as *const u8).add(self.off_dt_struct.to_be() as _) } as _,
			str: unsafe { (self as *const FdtHeader as *const u8).add(self.off_dt_strings.to_be() as _) } as _,
			depth:  0,
			marker: PhantomData
		}
	}
	
	pub fn structure_block_tree(&self) -> FdtStructureTreeIter {
		FdtStructureTreeIter(self.structure_block())
	}
	
	pub fn get<'a>(&'a self, path: &'a [&'a str]) -> FdtPathIter<'a> {
		FdtPathIter {
			iter:  self.structure_block(),
			path,
			i:     0,
			depth: 0
		}
	}
}

impl core::fmt::Debug for FdtHeader {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("FdtHeader")
			.field("version", &self.version.to_be())
			.field("last_comp_version", &self.last_comp_version.to_be())
			.field("boot_cpuid_phys", &self.boot_cpuid_phys.to_be())
			.field("memory_reservation_block", &self.memory_reservation_block())
			.field("structure_block", &self.structure_block_tree())
			.finish()
	}
}

pub struct FdtPathIter<'a> {
	iter:  FdtStructureIter<'a>,
	path:  &'a [&'a str],
	i:     usize,
	depth: usize
}

impl<'a> Iterator for FdtPathIter<'a> {
	type Item = FdtStructureToken<'a>;
	
	fn next(&mut self) -> Option<Self::Item> {
		for v in &mut self.iter {
			if self.depth > 0 {
				match v {
					FdtStructureToken::BeginNone { .. } => self.depth += 1,
					FdtStructureToken::EndNode          => self.depth -= 1,
					_ => ()
				}
			} else {
				match v {
					token @ FdtStructureToken::Prop { name, .. } if name == self.path[self.i] => return (self.i == self.path.len() - 1).then_some(token),
					token @ FdtStructureToken::BeginNone { name, .. } if &name[..name.find('@').unwrap_or_else(|| name.len())] == self.path[self.i] => if self.i < self.path.len() - 1 {
						self.i += 1;
						self.depth = 0;
					} else {
						self.depth += 1;
						return Some(token);
					},
					FdtStructureToken::BeginNone { .. } => self.depth += 1,
					FdtStructureToken::EndNode          => self.i -= 1,
					FdtStructureToken::Prop { .. }      => ()
				}
			}
		}
		
		None
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct FdtReserveEntry {
	pub address: u64,
	pub size:    u64
}

impl FdtReserveEntry {
	pub fn term(&self) -> bool {
		self.address == 0 && self.size == 0
	}
}

#[derive(Copy, Clone)]
pub struct FdtReserveEntryIter<'a>(*const FdtReserveEntry, PhantomData<&'a ()>);

impl<'a> Iterator for FdtReserveEntryIter<'a> {
	type Item = &'a FdtReserveEntry;
	
	fn next(&mut self) -> Option<Self::Item> {
		let e = unsafe { self.0.as_ref() }.unwrap();
		(!e.term()).then(|| {
			unsafe { self.0 = self.0.add(1); }
			e
		})
	}
}

impl core::fmt::Debug for FdtReserveEntryIter<'_> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_list().entries(*self).finish()
	}
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum FdtStructureToken<'a> {
	BeginNone { name: &'a str, tokens: FdtStructureIter<'a> },
	EndNode,
	Prop { name: &'a str, value: FdtValue<'a> }
}

#[derive(Copy, Clone, PartialEq)]
pub struct FdtStructureIter<'a> {
	ptr:    *mut u8,
	str:    *mut u8,
	depth:  usize,
	marker: PhantomData<&'a ()>
}

impl<'a> Iterator for FdtStructureIter<'a> {
	type Item = FdtStructureToken<'a>;
	
	fn next(&mut self) -> Option<Self::Item> {
		unsafe {
			loop {
				let id = (*(self.ptr as *const u32)).to_be();
				self.ptr = self.ptr.add(4);
				
				return match id {
					1 => {
						let mut name_end = self.ptr;
						while *name_end != 0 { name_end = name_end.add(1); }
						let name = core::str::from_utf8_unchecked(core::slice::from_raw_parts(self.ptr, name_end.offset_from(self.ptr) as _));
						add_align(&mut self.ptr, name.len() + 1, 4);
						self.depth += 1;
						Some(FdtStructureToken::BeginNone { name, tokens: Self { depth: 0, ..*self } })
					}
					3 => {
						let len      = (*(self.ptr as *const u32).add(0)).to_be();
						let name_off = (*(self.ptr as *const u32).add(1)).to_be();
						
						let name_ptr = self.str.add(name_off as _);
						let mut name_end = name_ptr;
						while *name_end != 0 { name_end = name_end.add(1); }
						
						let name = core::str::from_utf8_unchecked(core::slice::from_raw_parts(name_ptr, name_end.offset_from(name_ptr) as _));
						let value = FdtValue(core::slice::from_raw_parts(self.ptr.add(8), len as _));
						add_align(&mut self.ptr, 8 + value.len(), 4);
						Some(FdtStructureToken::Prop { name, value })
					}
					2 if self.depth == 0 => None,
					2 => {
						self.depth -= 1;
						Some(FdtStructureToken::EndNode)
					}
					4 => continue,
					_ => None
				}
			}
		}
	}
}

impl core::fmt::Debug for FdtStructureIter<'_> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_list().entries(*self).finish()
	}
}

#[derive(Debug)]
pub enum FdtStructureTreeToken<'a> {
	Node { name: &'a str, tokens: &'a mut FdtStructureTreeIter<'a> },
	Prop { name: &'a str, value: FdtValue<'a> }
}

#[derive(Clone)]
pub struct FdtStructureTreeIter<'a>(FdtStructureIter<'a>);

impl<'a> Iterator for FdtStructureTreeIter<'a> {
	type Item = FdtStructureTreeToken<'a>;
	
	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().and_then(|v| match v {
			FdtStructureToken::BeginNone { name, .. } => Some(FdtStructureTreeToken::Node {
				name,
				tokens: unsafe { (self as *mut Self).as_mut() }.unwrap()
			}),
			FdtStructureToken::Prop { name, value } => Some(FdtStructureTreeToken::Prop { name, value }),
			_ => None
		})
	}
}

impl core::fmt::Debug for FdtStructureTreeIter<'_> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let mut dbg = f.debug_struct("Node");
		
		unsafe { (self as *const Self as *mut Self).as_mut() }.unwrap().for_each(|v| match v {
			FdtStructureTreeToken::Node { name, tokens } => { dbg.field(name, tokens); },
			FdtStructureTreeToken::Prop { name, value } => { dbg.field(name, &value); }
		});
		
		dbg.finish()
	}
}

impl Drop for FdtStructureTreeIter<'_> {
	fn drop(&mut self) {
		self.count();
	}
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FdtValue<'a>(&'a [u8]);

impl<'a> FdtValue<'a> {
	pub fn as_slice<T: Copy>(&self) -> Option<&[T]> {
		(*self).into()
	}
}

impl<'a> core::ops::Deref for FdtValue<'a> {
	type Target = [u8];
	
	fn deref(&self) -> &Self::Target {
		self.0
	}
}

impl Into<Option<u8>> for FdtValue<'_> {
	fn into(self) -> Option<u8> {
		(self.0.len() == size_of::<u8>()).then(|| self.0[0])
	}
}

impl Into<Option<u16>> for FdtValue<'_> {
	fn into(self) -> Option<u16> {
		(self.0.len() == size_of::<u16>()).then(|| u16::from_be_bytes(
			[self.0[0], self.0[1]]))
	}
}

impl Into<Option<u32>> for FdtValue<'_> {
	fn into(self) -> Option<u32> {
		(self.0.len() == size_of::<u32>()).then(|| u32::from_be_bytes(
			[self.0[0], self.0[1], self.0[2], self.0[3]]))
	}
}

impl Into<Option<u64>> for FdtValue<'_> {
	fn into(self) -> Option<u64> {
		(self.0.len() == size_of::<u64>()).then(|| u64::from_be_bytes(
			[self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], self.0[6], self.0[7]]))
	}
}

impl<'a, T: Copy> Into<Option<&'a [T]>> for FdtValue<'a> {
	fn into(self) -> Option<&'a [T]> {
		(!self.0.is_empty() && self.0.len() % size_of::<T>() == 0).then(||
			unsafe { core::slice::from_raw_parts(self.0.as_ptr() as *const T, self.len() / size_of::<T>()) })
	}
}

impl<'a> Into<Option<&'a str>> for FdtValue<'a> {
	fn into(self) -> Option<&'a str> {
		if self.0.is_empty() { return None; }
		core::str::from_utf8(&self.0[..self.0.len() - 1]).ok()
	}
}

impl core::fmt::Debug for FdtValue<'_> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		if let Some(s) = Into::<Option<&str>>::into(*self) {
			if !s.contains('\0') {
				return write!(f, "{:?}", s);
			}/* else {
				return f.debug_list().entries(s.split('\0')).finish()
			}*/
		}
		
		match self.0.len() {
			0 => write!(f, "{:?}", ()),
			1 => write!(f, "{}", self.0[0]),
			2 => write!(f, "{}", u16::from_be_bytes([self.0[0], self.0[1]])),
			3 => write!(f, "{}", u32::from_be_bytes([self.0[0], self.0[1], self.0[2], 0])),
			4 => write!(f, "{}", u32::from_be_bytes([self.0[0], self.0[1], self.0[2], self.0[3]])),
			8 => write!(f, "{}", u64::from_be_bytes([self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], self.0[6], self.0[7]])),
			_ => {
				write!(f, "0x")?;
				for v in self.iter() {
					write!(f, "{:x}", v)?;
				}
				Ok(())
			}
		}
	}
}

unsafe fn add_align<T>(ptr: &mut *mut T, v: usize, align: usize) {
	*ptr = ptr.add(v);
	*ptr = ptr.add(ptr.align_offset(align));
}