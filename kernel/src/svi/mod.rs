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

#![no_std]
#![feature(llvm_asm, untagged_unions, naked_functions)]
#![allow(improper_ctypes_definitions, unused_variables)]

use {core::{marker::PhantomData}};

pub mod sys;
pub mod wrapper;

pub type Flags   = usize;
pub type Rd      = usize;
pub type IoOpId  = usize;
pub type TaskId  = usize;
pub type GroupId = usize;

/// An invalid resource descriptor
pub const INVALID_RD: usize = !0;

pub const SPI_VERSION_0_1_0:  u32 = version(0, 1, 0);
pub const SPI_VERSION_1_0_0:  u32 = version(1, 0, 0);





/// Copy open resource descriptors to the new task
pub const CLONE_SHARE_RDS:             usize = 0x1;
/// Copy memory regions to the new task
pub const CLONE_SHARE_MEM:             usize = 0x2;
/// The cloned task will have the same id as the parent task, this is called a thread group
pub const CLONE_SHARE_ID:              usize = 0x4;
/// Copy interrupt handling data to the new task
pub const CLONE_SHARE_INT:             usize = 0x8;
/// The newly created task will begin executing at the passed address
pub const CLONE_JUMP:                  usize = 0x10;

pub const GROUP_CREATE_INHERIT_MOUNTS: usize = 0x1;
pub const GROUP_CREATE_INHERIT_PROCS:  usize = 0x2;

pub const fn version(minor: u32, major: u32, patch: u32) -> u32 {
	((minor & 0xFF) << 24) | ((major & 0xFF) << 16) | (patch & 0xFFFF)
}

#[repr(transparent)]
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Result<T>(isize, PhantomData<T>);

impl<T> Result<T> {
	pub fn into_result(self) -> core::result::Result<usize, usize> {
		if self.0 < 0 {
			Err(-self.0 as _)
		} else {
			Ok(self.0 as _)
		}
	}
}

#[inline]
pub fn ok<T>(val: usize) -> Result<T> {
	Result(val as _, PhantomData)
}

#[inline]
pub fn err<T>(err: usize) -> Result<T> {
	Result(-(err as isize), PhantomData)
}

#[repr(C)]
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Default)]
pub struct SysInfo {
	pub len:         usize,
	pub page_size:   usize,
	pub harts:       usize,
	pub spi_version: usize
}

#[repr(C)]
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Default)]
pub struct TaskInfo {
	pub len: usize,
	pub id:  TaskId,
	pub pid: TaskId
}

#[repr(C)]
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum TaskAttr {
	Uid,
	Gid, // administrative group
	CGid, // control group
	Priority,
	Affinity
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum SyncWaitOp {
	One,
	All,
	Any
}

#[repr(C)]
pub union RdOrPath {
	pub rd:   Rd,
	pub path: *const u8
}

#[repr(C)]
pub union IoReadBuf<'a> {
	pub buf: Option<&'a mut [u8]>,
	pub vec: Option<&'a mut [&'a mut [u8]]>
}

#[repr(C)]
pub union IoWriteBuf<'a> {
	pub buf: Option<&'a [u8]>,
	pub vec: Option<&'a [&'a [u8]]>
}

#[repr(C)]
pub union IoBufOrLen<'a> {
	pub buf: Option<&'a [u8]>,
	pub vec: Option<&'a [&'a [u8]]>,
	pub len: usize
}

#[repr(C)]
pub struct IoOp<'a> {
	pub rd:    RdOrPath,
	pub id:    IoOpId,
	pub flags: usize,
	pub buf:   IoBufOrLen<'a>,
	pub off:   usize
}

#[repr(C)]
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Interrupt {
	/// The task tried to access memory that was not mapped, or the accessed address was misaligned.
	InvalidMemRef,
	IllegalInstruction,
	FloatingPointException,
	Debug,
	/// Timer triggered interrupt
	Timer,
	/// Request the task to terminate
	Term,
	/// Kill the task violently
	Abort,
	/// An IO operation finished
	IoReady,
	/// A synchronization primitive is available
	Sync,
	/// Triggered when a task's status transitions to halted or stopped
	TaskStatusChange,
	/// A task sent an Rd to this task
	SendRd,
	/// Some file system event occurred that this task has subscribed to
	FsEvent
}

#[repr(C)]
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ResourceType {
	File,
	Dir,
	Pipe,
	Link,
	Special
}

#[repr(C)]
#[non_exhaustive]
#[derive(Copy, Clone, Debug)]
pub enum GroupAttr {
	Uid,
	Gid, // administrative group
	CGid, // control group
	Priority,
	Affinity,
	CpuTime,
	Memory,
	MaxMemory,
	ParentGroup
}

/// test
///
/// # Description
///
/// # Arguments
///
/// | Argument | Description
/// |----------|------------
///
/// # Flags
///
/// | Bit | Flag                   | Description
/// |-----|------------------------|------------
///
/// # Returns
///
/// ## On Success
///
/// Zero (0).
///
/// ## On Failure
///
/// | Value | Error                      | Description
/// |-------|----------------------------|------------


macro_rules! arch_svc {
    ( $arg0:expr ) => {
		unsafe {
			let r;
			#[cfg(any(target_arch = "riscv32", target_arch = "riscv64", target_arch = "riscv128"))]
			llvm_asm!("ecall" : "={x1}"(r) : "{x1}"($arg0));
			#[cfg(target_arch = "x86_64")]
			llvm_asm!("syscall" : "={rax}"(r) : "{rax}"($arg0));
			#[cfg(target_arch = "aarch64")]
			llvm_asm!("svc 0" : "={x0}"(r) : "{x0}"($arg0));
			r
		}
	};
    ( $arg0:expr, $arg1:expr ) => {
		unsafe {
			let r;
			#[cfg(any(target_arch = "riscv32", target_arch = "riscv64", target_arch = "riscv128"))]
			llvm_asm!("ecall" : "={x1}"(r) : "{x1}"($arg0), "{x2}"($arg1));
			#[cfg(target_arch = "x86_64")]
			llvm_asm!("syscall" : "={rax}"(r) : "{rax}"($arg0), "{rbx}"($arg1));
			#[cfg(target_arch = "aarch64")]
			llvm_asm!("svc 0" : "={x0}"(r) : "{x0}"($arg0), "{x1}"($arg1));
			r
		}

	};
    ( $arg0:expr, $arg1:expr, $arg2:expr ) => {
		unsafe {
			let r;
			#[cfg(any(target_arch = "riscv32", target_arch = "riscv64", target_arch = "riscv128"))]
			llvm_asm!("ecall" : "={x1}"(r) : "{x1}"($arg0), "{x2}"($arg1), "{x3}"($arg2));
			#[cfg(target_arch = "x86_64")]
			llvm_asm!("syscall" : "={rax}"(r) : "{rax}"($arg0), "{rbx}"($arg1), "{rcx}"($arg2));
			#[cfg(target_arch = "aarch64")]
			llvm_asm!("svc 0" : "={x0}"(r) : "{x0}"($arg0), "{x1}"($arg1), "{x2}"($arg2));
			r
		}
	};
    ( $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr ) => {
		unsafe {
			let r;
			#[cfg(any(target_arch = "riscv32", target_arch = "riscv64", target_arch = "riscv128"))]
			llvm_asm!("ecall" : "={x1}"(r) : "{x1}"($arg0), "{x2}"($arg1), "{x3}"($arg2), "{x4}"($arg3));
			#[cfg(target_arch = "x86_64")]
			llvm_asm!("syscall" : "={rax}"(r) : "{rax}"($arg0), "{rbx}"($arg1), "{rcx}"($arg2), "{rdx}"($arg3));
			#[cfg(target_arch = "aarch64")]
			llvm_asm!("svc 0" : "={x0}"(r) : "{x0}"($arg0), "{x1}"($arg1), "{x2}"($arg2), "{x3}"($arg3));
			r
		}
	};
    ( $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr ) => {
		unsafe {
			let r;
			#[cfg(any(target_arch = "riscv32", target_arch = "riscv64", target_arch = "riscv128"))]
			llvm_asm!("ecall" : "={x1}"(r) : "{x1}"($arg0), "{x2}"($arg1), "{x3}"($arg2), "{x4}"($arg3), "{x5}"($arg4));
			#[cfg(target_arch = "x86_64")]
			llvm_asm!("syscall" : "={rax}"(r) : "{rax}"($arg0), "{rbx}"($arg1), "{rcx}"($arg2), "{rdx}"($arg3), "{r8}"($arg4));
			#[cfg(target_arch = "aarch64")]
			llvm_asm!("svc 0" : "={x0}"(r) : "{x0}"($arg0), "{x1}"($arg1), "{x2}"($arg2), "{x3}"($arg3), "{x4}"($arg4));
			r
		}
	};
    ( $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr ) => {
		unsafe {
			let r;
			#[cfg(any(target_arch = "riscv32", target_arch = "riscv64", target_arch = "riscv128"))]
			llvm_asm!("ecall" : "={x1}"(r) : "{x1}"($arg0), "{x2}"($arg1), "{x3}"($arg2), "{x4}"($arg3), "{x5}"($arg4), "{x6}"($arg5));
			#[cfg(target_arch = "x86_64")]
			llvm_asm!("syscall" : "={rax}"(r) : "{rax}"($arg0), "{rbx}"($arg1), "{rcx}"($arg2), "{rdx}"($arg3), "{r8}"($arg4), "{r9}"($arg5));
			#[cfg(target_arch = "aarch64")]
			llvm_asm!("svc 0" : "={x0}"(r) : "{x0}"($arg0), "{x1}"($arg1), "{x2}"($arg2), "{x3}"($arg3), "{x4}"($arg4), "{x5}"($arg5));
			r
		}
	};
    ( $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr ) => {
		unsafe {
			let r;
			#[cfg(any(target_arch = "riscv32", target_arch = "riscv64", target_arch = "riscv128"))]
			llvm_asm!("ecall" : "={x1}"(r) : "{x1}"($arg0), "{x2}"($arg1), "{x3}"($arg2), "{x4}"($arg3), "{x5}"($arg4), "{x6}"($arg5), "{x7}"($arg6));
			#[cfg(target_arch = "x86_64")]
			llvm_asm!("syscall" : "={rax}"(r) : "{rax}"($arg0), "{rbx}"($arg1), "{rcx}"($arg2), "{rdx}"($arg3), "{r8}"($arg4), "{r9}"($arg5), "{r10}"($arg6));
			#[cfg(target_arch = "aarch64")]
			llvm_asm!("svc 0" : "={x0}"(r) : "{x0}"($arg0), "{x1}"($arg1), "{x2}"($arg2), "{x3}"($arg3), "{x4}"($arg4), "{x5}"($arg5), "{x6}"($arg6));
			r
		}
	};
    ( $arg0:expr, $arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr, $arg7:expr ) => {
		unsafe {
			let r;
			#[cfg(any(target_arch = "riscv32", target_arch = "riscv64", target_arch = "riscv128"))]
			llvm_asm!("ecall" : "={x1}"(r) : "{x1}"($arg0), "{x2}"($arg1), "{x3}"($arg2), "{x4}"($arg3), "{x5}"($arg4), "{x6}"($arg5), "{x7}"($arg6), "{x8}"($arg7));
			#[cfg(target_arch = "x86_64")]
			llvm_asm!("syscall" : "={rax}"(r) : "{rax}"($arg0), "{rbx}"($arg1), "{rcx}"($arg2), "{rdx}"($arg3), "{r8}"($arg4), "{r9}"($arg5), "{r10}"($arg6), "{r11}"($arg7));
			#[cfg(target_arch = "aarch64")]
			llvm_asm!("svc 0" : "={x0}"(r) : "{x0}"($arg0), "{x1}"($arg1), "{x2}"($arg2), "{x3}"($arg3), "{x4}"($arg4), "{x5}"($arg5), "{x6}"($arg6), "{x7}"($arg7));
			r
		}
	};
}

#[inline(always)]
pub fn sys_task_clone(flags: Flags, addr: usize, rds: &[Rd]) -> Result<TaskId> {
	arch_svc!(SYS_TASK_CLONE, flags, addr, rds)
}

#[inline(always)]
pub fn sys_task_yield() -> Result<()> {
	arch_svc!(SYS_TASK_YIELD)
}

#[inline(always)]
pub fn sys_task_halt() -> Result<()> {
	arch_svc!(SYS_TASK_HALT)
}

#[inline(always)]
pub fn sys_task_exit(status: usize) -> ! {
	arch_svc!(SYS_TASK_EXIT, status)
}

/// If rd is invalid, listen for directed interrupts
#[inline(always)]
pub fn sys_int_set_vector(rd: Rd, int: Interrupt, handler: Option<fn()>, pri: u8) -> Result<()> {
	arch_svc!(SYS_INT_SET_VECTOR, rd, int, handler, pri)
}

/// If rd is invalid broadcast the interrupt
#[inline(always)]
pub fn sys_int(rd: Rd, id: usize, args: *mut u8) -> Result<()> {
	arch_svc!(SYS_INT, rd, id, args)
}

/// Return from an interrupt handler
#[inline(always)]
pub fn sys_int_ret() -> ! {
	arch_svc!(SYS_INT_RET)
}