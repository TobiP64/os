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

pub type Result<T> = core::result::Result<T, usize>;

#[derive(Debug)]
pub struct System;

impl System {
	pub fn info(&self) -> Result<SysInfo> {
		let mut info = SysInfo::default();
		sys_info(&mut info)
			.into_result()
			.map(|_| info)
	}
}

#[derive(Debug)]
pub struct Resource(Rd);

impl Resource {
	pub unsafe fn new(rd: Rd) -> Self {
		Self(rd)
	}
	
	pub fn open(filename: &str, flags: Flags) -> Result<Self> {
		sys_rd_open(Some(filename), flags, INVALID_RD)
			.into_result()
			.map(Self)
	}
	
	pub fn open_in(&self, filename: &str, flags: Flags) -> Result<Self> {
		sys_rd_open(Some(filename), flags, self.0)
			.into_result()
			.map(Self)
	}
	
	pub fn close(self) -> Result<()> {
		sys_rd_close(self.0)
			.into_result()
			.map(|_| ())
	}
	
	pub fn sync(&self, flags: Flags, offset: usize, len: usize) -> Result<()> {
		sys_rd_sync(self.0, flags, offset, len)
			.into_result()
			.map(|_| ())
	}
	
	pub fn lock(&self, flags: Flags, offset: usize, len: usize) -> Result<()> {
		sys_rd_lock(self.0, flags, offset, len)
			.into_result()
			.map(|_| ())
	}
	
	pub fn unlock(&self, flags: Flags, offset: usize, len: usize) -> Result<()> {
		sys_rd_unlock(self.0, flags, offset, len)
			.into_result()
			.map(|_| ())
	}
	
	pub fn map(&self, addr: *mut u8, len: usize, flags: Flags) -> Result<MemoryMapping> {
		sys_mem_map(addr, len, self.0, flags)
			.into_result()
			.map(|_| MemoryMapping { addr, len })
	}
}

#[derive(Debug)]
pub struct MemoryMapping {
	addr: *mut u8,
	len:  usize
}

impl MemoryMapping {
	pub unsafe fn new(addr: *mut u8, len: usize) -> Self {
		Self { addr, len }
	}
	
	pub fn sub_mapping(&self, offset: usize, len: usize) -> Self {
		if offset + len > self.len {
			panic!();
		}
		
		Self {
			addr: unsafe { self.addr.add(offset) },
			len
		}
	}
	
	pub fn unmap(self) -> Result<()> {
		sys_mem_unmap(self.addr, self.len)
			.into_result()
			.map(|_| ())
	}
}

#[derive(Debug)]
pub struct Task(TaskId);

impl Task {
	pub unsafe fn new(id: TaskId) -> Self {
		Self(id)
	}
	
	pub const fn current() -> Self {
		Self(0)
	}
	
	pub fn group(&self) -> Result<Group> {
		sys_task_get_attr(self.0, TaskAttr::CGid)
			.into_result()
			.map(Group)
	}
	
	pub fn set_group(&self, group: &Group) -> Result<()> {
		sys_task_set_attr(self.0, TaskAttr::CGid, group.0)
			.into_result()
			.map(|_| ())
	}
	
	pub fn halt(&self) -> Result<()> {
		sys_task_halt()
			.into_result()
			.map(|_| ())
	}
	
	pub fn interrupt(&self, id: usize, args: *mut u8) -> Result<()> {
		unimplemented!()
	}
}

#[derive(Debug)]
pub struct Group(GroupId);

impl Group {
	pub unsafe fn new(id: TaskId) -> Self {
		Self(id)
	}
	
	pub fn create(flags: Flags) -> Result<Self> {
		sys_group_create(flags)
			.into_result()
			.map(Self)
	}
	
	pub fn delete(self) -> Result<()> {
		sys_group_delete(self.0)
			.into_result()
			.map(|_| ())
	}
	
	pub fn clone(&self, flags: Flags) -> Result<Self> {
		sys_group_clone(self.0, flags)
			.into_result()
			.map(Self)
	}
}