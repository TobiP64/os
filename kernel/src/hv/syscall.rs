
#[no_mangle]
pub static SYSCALL_TABLE: [extern fn() -> u32; 42] = unsafe { [

] };

#[no_mangle]
pub extern fn sys_not_implemented() -> Result<usize> {
    err(ERR_NOT_IMPLEMENTED)
}

pub const SV_STATE_RUNNING:    u32 = 0;
pub const SV_STATE_SUSPENDEND: u32 = 1;
pub const SV_STATE_STOPPED:    u32 = 2;
pub const SV_STATE_ABORTED:    u32 = 3;

#[inline(always)]
pub fn sys_sv_create(flags: Flags) -> Result<u32> {

}

#[inline(always)]
pub fn sys_sv_delete(ctx: u32) -> Result<()> {

}

#[inline(always)]
pub fn sys_sv_change_state(ctx: u32, state: u32) -> Result<()> {

}

#[inline(always)]
pub fn sys_sv_set_attr(ctx: u32) -> Result<()> {

}

#[inline(always)]
pub fn sys_sv_get_attr(ctx: u32) -> Result<()> {

}

#[inline(always)]
pub fn sys_sv_map(ctx: u32, src_addr: usize, dst_addr: usize, length: usize) -> Result<()> {

}

#[inline(always)]
pub fn sys_sv_unmap(ctx: u32, addr: usize, length: usize) -> Result<()> {

}

#[inline(always)]
pub fn sys_sv_int(ctx: u32, intid: u32) -> Result<()> {

}