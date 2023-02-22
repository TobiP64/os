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

#![warn(clippy::all)]
#![allow(improper_ctypes_definitions, unused_variables)]

use {
	crate::{task::{*, TaskId, Interrupt}, fs::*, arch::context_save},
	core::{mem::*, sync::atomic::Ordering, hint::unreachable_unchecked},
};

pub use spi::*;

pub type Syscall = extern fn(&Task) -> usize;

#[no_mangle]
pub static SYSCALL_TABLE: [Syscall; 42] = unsafe { [
	transmute(sys_rd_open         as *const ()),
	transmute(sys_rd_close        as *const ()),
	transmute(sys_rd_ops          as *const ()),
	transmute(sys_rd_read         as *const ()),
	transmute(sys_rd_write        as *const ()),
	transmute(sys_rd_poll         as *const ()),
	transmute(sys_rd_sync         as *const ()),
	transmute(sys_rd_lock         as *const ()),
	transmute(sys_rd_unlock       as *const ()),
	transmute(sys_rd_send         as *const ()),
	transmute(sys_mem_map         as *const ()),
	transmute(sys_mem_remap       as *const ()),
	transmute(sys_mem_unmap       as *const ()),
	transmute(sys_mem_sync        as *const ()),
	transmute(sys_mem_lock        as *const ()),
	transmute(sys_mem_unlock      as *const ()),
	transmute(sys_sync_wake       as *const ()),
	transmute(sys_sync_wait       as *const ()),
	transmute(sys_task_clone      as *const ()),
	transmute(sys_task_yield       as *const ()),
	transmute(sys_task_halt       as *const ()),
	transmute(sys_task_exit       as *const ()),
	transmute(sys_task_get_attr   as *const ()),
	transmute(sys_task_set_attr   as *const ()),
	transmute(sys_int_set_vector  as *const ()),
	transmute(sys_int             as *const ()),
	transmute(sys_int_ret         as *const ()),
	transmute(sys_info            as *const ()),
	transmute(sys_fs_mount        as *const ()),
	transmute(sys_fs_unmount      as *const ()),
	transmute(sys_fs_create       as *const ()),
	transmute(sys_fs_delete       as *const ()),
	transmute(sys_fs_move         as *const ()),
	transmute(sys_fs_truncate     as *const ()),
	transmute(sys_fs_get_attr     as *const ()),
	transmute(sys_fs_set_attr     as *const ()),
	transmute(sys_group_create    as *const ()),
	transmute(sys_group_delete    as *const ()),
	transmute(sys_group_clone     as *const ()),
	transmute(sys_group_ctrl      as *const ()),
	transmute(sys_group_get_attr  as *const ()),
	transmute(sys_group_set_attr  as *const ())
] };

#[no_mangle]
pub extern fn sys_not_implemented(_task: &mut Task) -> Result<usize> {
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_rd_open(task: &mut Task, filename: Option<&str>, flags: Flags, dir: Rd) -> Result<Rd> {
	if flags & RD_OPEN_FLAG_RELATIVE != 0 && !rd_check(task, dir, 0) {
		return err(ERR_INVALID_ARG);
	} else if let Some(filename) = filename {
		if !mem_ref_check(filename.as_bytes().as_ptr(), filename.len(), MEM_MAP_FLAG_PROT_READ) {
			return err(ERR_INVALID_MEM_REF);
		}
	}
	
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_rd_close(task: &mut Task, rd: Rd) -> Result<()> {
	if !rd_check(task, rd, 0) {
		return err(ERR_INVALID_ARG);
	}
	
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_rd_ops(task: &mut Task, ops: &[IoOp]) -> Result<()> {
	if ops.is_empty() {
		return ok(0);
	} else if !mem_ref_check(ops.as_ptr(), ops.len(), MEM_MAP_FLAG_PROT_READ) {
		return err(ERR_INVALID_MEM_REF);
	}
	
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_rd_read(task: &mut Task, rd: RdOrPath, flags: usize, buf: IoReadBuf, offset: usize, op_id: IoOpId) -> Result<usize> {
	
	if flags & !RD_IO_FLAGS != 0
		|| (flags & RD_IO_FLAG_FILE_NAME == 0 && (rd.rd >= task.res_descs.len() || task.res_descs[rd.rd].rd.is_null()))
		|| (flags & (RD_IO_FLAG_ASYNC | RD_IO_FLAG_ASYNC_INT) != 0 && buf.buf.as_ref().filter(|b| !b.is_empty()).is_none()) {
		return err(ERR_INVALID_ARG);
	} else if buf.buf.as_ref().filter(|b| !b.is_empty()).is_none() {
		return ok(0);
	}
	
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_rd_write(task: &mut Task, rd: RdOrPath, flags: usize, buf: IoWriteBuf, offset: usize, op_id: IoOpId) -> Result<usize> {
	
	if flags & !RD_IO_FLAGS != 0
		|| (flags & RD_IO_FLAG_FILE_NAME == 0 && (rd.rd >= task.res_descs.len() || task.res_descs[rd.rd].rd.is_null()))
		|| (flags & (RD_IO_FLAG_ASYNC | RD_IO_FLAG_ASYNC_INT) != 0 && buf.buf.as_ref().filter(|b| !b.is_empty()).is_none()) {
		return err(ERR_INVALID_ARG);
	} else if buf.buf.as_ref().filter(|b| !b.is_empty()).is_none() {
		return ok(0);
	}
	
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_rd_poll(task: &mut Task, id: IoOpId, flags: Flags) -> Result<usize> {
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_rd_sync(task: &mut Task, rd: Rd, offset: usize, len: usize, flags: Flags) -> Result<usize> {
	if !rd_range_check(task, rd, offset, len, 0) {
		return err(ERR_INVALID_ARG);
	}
	
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_rd_lock(task: &mut Task, rd: Rd, offset: usize, len: usize, flags: Flags) -> Result<usize> {
	if !rd_range_check(task, rd, offset, len, 0) {
		return err(ERR_INVALID_ARG);
	}
	
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_rd_unlock(task: &mut Task, rd: Rd, offset: usize, len: usize, flags: Flags) -> Result<usize> {
	if !rd_range_check(task, rd, offset, len, 0) {
		return err(ERR_INVALID_ARG);
	}
	
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_rd_send(task: &mut Task, task_: TaskId, rd: Rd, flags: Flags) -> Result<()> {
	if rd != INVALID_RD && !rd_check(task, rd, 0) {
		return err(ERR_INVALID_ARG);
	}
	
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_mem_map(task: &mut Task, addr: *mut u8, len: usize, rd: Rd, flags: Flags) -> Result<*mut u8> {
	if !mem_map_check(addr, len) {
		return err(ERR_INVALID_ARG);
	} else if !rd_range_check(task, rd, 0, len, flags) {
		return err(ERR_INVALID_ARG);
	}
	
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_mem_remap(task: &mut Task, addr: *mut u8, len: usize, new_addr: *mut u8, flags: Flags) -> Result<*mut u8> {
	if !mem_map_check(addr, len) {
		return err(ERR_INVALID_ARG);
	}
	
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_mem_unmap(task: &mut Task, addr: *mut u8, len: usize) -> Result<()> {
	if !mem_map_check(addr, len) {
		return err(ERR_INVALID_ARG);
	}
	
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_mem_sync(task: &mut Task, addr: *mut u8, len: usize, flags: Flags) -> Result<()> {
	if !mem_map_check(addr, len) {
		return err(ERR_INVALID_ARG);
	}
	
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_mem_lock(task: &mut Task, addr: *mut u8, len: usize, flags: Flags) -> Result<()> {
	if !mem_map_check(addr, len) {
		return err(ERR_INVALID_ARG);
	}
	
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_mem_unlock(task: &mut Task, addr: *mut u8, len: usize, flags: Flags) -> Result<()> {
	if !mem_map_check(addr, len) {
		return err(ERR_INVALID_ARG);
	}
	
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_sync_wake(task: &mut Task, addr: *mut usize, count: usize, len: usize) -> Result<()> {
	if !mem_ref_check(addr, len.max(1), RD_OPEN_FLAG_READ) {
		return err(ERR_INVALID_MEM_REF)
	}
	
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_sync_wait(task: &mut Task, addr: *mut usize, val: usize, len: usize, op: SyncWaitOp) -> Result<()> {
	if !mem_ref_check(addr, len.max(1), RD_OPEN_FLAG_READ) {
		return err(ERR_INVALID_MEM_REF)
	}
	
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_task_clone(task: &mut Task, flags: Flags, addr: usize, rds: &[Rd]) -> Result<TaskId> {
	if !mem_ref_check(addr as *const u8, 1, RD_OPEN_FLAG_EXEC) {
		return err(ERR_INVALID_MEM_REF)
	}
	
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_task_yield(task: &mut Task) -> Result<()> {
	unsafe {
		llvm_asm!("li a0, 0"); // set return value
		context_save();
		llvm_asm!("j handle_exception__timer_int");
		unreachable_unchecked();
	}
}

#[no_mangle]
pub extern fn sys_task_halt(task: &mut Task) -> Result<()> {
	task.state.store(TaskState::Blocked as _, Ordering::SeqCst);
	unsafe {
		llvm_asm!("li a0, 0"); // set return value
		context_save();
		llvm_asm!("j handle_exception__timer_int");
		unreachable_unchecked();
	}
}

#[no_mangle]
pub extern fn sys_task_exit(task: &mut Task, status: usize) -> ! {
	task.state.store(TaskState::Stopped as _, Ordering::SeqCst);
	unsafe {
		llvm_asm!("j handle_exception__timer_int");
		unreachable_unchecked();
	}
}

#[no_mangle]
pub extern fn sys_task_get_attr(task: &mut Task, id: TaskId, attr: TaskAttr) -> Result<usize> {
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_task_set_attr(task: &mut Task, id: TaskId, attr: TaskAttr, val:  usize) -> Result<()> {
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_int_set_vector(task: &mut Task, rd: Rd, int: Interrupt, handler: Option<fn()>, pri: u8) -> Result<()> {
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_int(task: &mut Task, rd: Rd) -> Result<()> {
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_int_ret(task: &mut Task) -> ! {
	unreachable!()
}

#[no_mangle]
pub extern fn sys_info(task: &mut Task, info: &mut SysInfo) -> Result<()> {
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_fs_mount(task: &mut Task, src: &str, target: &str, fs_type: FsType, flags: Flags, data: *mut u8) -> Result<()> {
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_fs_unmount(task: &mut Task, target: &str, flags: Flags) -> Result<()> {
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_fs_create(task: &mut Task, path: &str, flags: Flags, attr: &[(usize, usize)]) -> Result<()> {
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_fs_delete(task: &mut Task, path: &str) -> Result<()> {
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub fn sys_fs_move(task: &mut Task, rd_src: RdOrPath, rd_dst: RdOrPath, flags: Flags, dir_src: Rd, dir_dst: Rd) -> Result<()> {
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub fn sys_fs_truncate(task: &mut Task, rd: RdOrPath, flags: Flags, new_len: u64) -> Result<()> {
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_fs_get_attr(task: &mut Task, rd: Rd, attr: usize) -> Result<usize> {
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_fs_set_attr(task: &mut Task, rd: Rd, attr: usize, val: usize) -> Result<()> {
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_group_create(task: &mut Task, flags: Flags) -> Result<GroupId> {
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_group_delete(task: &mut Task, group: GroupId) -> Result<()> {
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_group_clone(task: &mut Task, group: GroupId, flags: Flags) -> Result<()> {
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_group_ctrl(task: &mut Task, group: GroupId, op: usize) -> Result<()> {
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_group_set_attr(task: &mut Task, group: GroupId, attr: usize, val: usize) -> Result<()> {
	err(ERR_NOT_IMPLEMENTED)
}

#[no_mangle]
pub extern fn sys_group_get_attr(task: &mut Task, group: GroupId, attr: usize) -> Result<usize> {
	err(ERR_NOT_IMPLEMENTED)
}

// UTILITY FUNCTIONS

fn mem_map_check(addr: *const u8, len: usize) -> bool {
	len > 0 && (addr as usize) < crate::USER_SPACE_END || addr as usize + len < crate::USER_SPACE_END
}

fn mem_ref_check<T>(addr: *const T, len: usize, flags: usize) -> bool {
	unimplemented!()
}

fn rd_check(task: &mut Task, rd: Rd, flags: usize) -> bool {
	unimplemented!()
}

/// Checks if the RD is open and the range is valid with the given protection flags.
fn rd_range_check(task: &mut Task, rd: Rd, offset: usize, len: usize, flags: usize) -> bool {
	if !rd_check(task, rd, flags) {
		return false;
	}
	
	unimplemented!()
}