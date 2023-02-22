
/// An unknown error occurred while handling the syscall.
/// This might be an architecture or hardware dependent error.
pub const ERR_UNKNOWN:                usize = 0x1;
/// The syscall is not implemented.
pub const ERR_NOT_IMPLEMENTED:        usize = 0x2;
/// The kernel has run out of memory.
pub const ERR_OUT_OF_KERNEL_MEMORY:   usize = 0x3;
/// The kernel was interrupted while handling the syscall
pub const ERR_INTERRUPTED:            usize = 0x4;
/// A low-level IO error occurred.
pub const ERR_IO:                     usize = 0x5;
/// One or more invalid arguments were passed.
pub const ERR_INVALID_ARG:            usize = 0x6;
/// A passed memory reference is invalid, or not accessible by the task.
pub const ERR_INVALID_MEM_REF:        usize = 0x7;
/// The task does not have permission to call the syscall with the passed arguments.
pub const ERR_PROTECTION:             usize = 0x8;
/// An asynchronous operation has not completed yet.
pub const ERR_NOT_READY:              usize = 0x9;

/// Open a resource with read access
pub const RD_OPEN_FLAG_READ:          usize = 0x1;
/// Open a resource with write access
pub const RD_OPEN_FLAG_WRITE:         usize = 0x2;
/// Open a resource with execution access
pub const RD_OPEN_FLAG_EXEC:          usize = 0x4;
/// Open a resource relative to a directory
pub const RD_OPEN_FLAG_RELATIVE:      usize = 0x8;
/// Create the resource if it does not exist
pub const RD_OPEN_CREATE:             usize = 0x10;
/// Creates a new resource, fails if the resource already exists
pub const RD_OPEN_CREATE_NEW:         usize = 0x20;
/// Delete the resource after the RD is closed
pub const RD_OPEN_DELETE:             usize = 0x10;

/// The resource could not be found
pub const RD_OPEN_RESOURCE_NO_EXISTS: usize = 0x1000;
/// The resource already exists
pub const RD_OPEN_RESOURCE_EXISTS:    usize = 0x1001;

/// The callee will be blocked until the operation as completed
pub const RD_IO_MODE_BLOCK:     usize = 0b00;
/// If the operation cannot be completed without blocking the callee, `RD_IO_ERR_WOULD_BLOCK` is returned
pub const RD_IO_MODE_NON_BLOCK: usize = 0b01;
/// The callee can poll operation completion by doing the same syscall at a later point
pub const RD_IO_MODE_POLLABLE:  usize = 0b10;
/// The callee will be interrupted once the operation has completed
pub const RD_IO_MODE_INTERRUPT: usize = 0b11;
/// The operation data is provided as a linear buffer
pub const RD_IO_FORMAT_BUF:     usize = 0b00;
/// The operation data is provided as an array of linear buffers
pub const RD_IO_FORMAT_VEC:     usize = 0b01;
/// The operation data is provided as argument 4, 5, 6 and 7 of the syscall
pub const RD_IO_FORMAT_REG:     usize = 0b10;

/// The data will directly be written/read from disk, skipping caches
pub const RD_IO_FLAG_SYNC:      usize = 0x10;
/// A file name is passed instead of an RD
pub const RD_IO_FLAG_FILE_NAME:       usize = 0x20;

pub const RD_IO_REG_RD_ARG0:      usize = 0x100;
pub const RD_IO_REG_RD_ARG1:      usize = 0x101;
pub const RD_IO_REG_RD_ARG2:      usize = 0x102;
pub const RD_IO_REG_RD_ARG3:      usize = 0x103;

/// All possible flags
pub const RD_IO_FLAGS:                usize = 0x3F;
pub const RD_IO_LEN_WHOLE_LEN:        usize = !0;
/// The IO operation would block the task to complete
pub const RD_IO_ERR_WOULD_BLOCK:      usize = 0x1000;

/// Lock the resource exclusively
pub const RD_LOCK_FLAG_EXCLUSIVE:     usize = 0x1;

pub const MEM_MAP_FLAG_PROT_READ:     usize = 0x1;
pub const MEM_MAP_FLAG_PROT_WRITE:    usize = 0x2;
pub const MEM_MAP_FLAG_PROT_EXEC:     usize = 0x4;
pub const MEM_MAP_FLAG_PROT_SHARE:    usize = 0x8;
pub const MEM_MAP_FLAG_INIT_UNINIT:   usize = 0x10;
pub const MEM_MAP_FLAG_INIT_POPULATE: usize = 0x20;
pub const MEM_MAP_FLAG_USAGE_STACK:   usize = 0x40;
pub const MEM_MAP_FLAG_PHYSICAL_CONT: usize = 0x80;
pub const MEM_MAP_FLAG_ADDRESS_HINT:  usize = 0x100;
pub const MEM_MAP_FLAG_DISCARD_OLD:   usize = 0x200;
/// Map the whole file. If the rd is an anonymous region this will fail
pub const MEM_MAP_LEN_WHOLE_LEN:      usize = !0;

pub const MEM_UNMAP_FLAG_DETACH:      usize = 0x1;

pub const MEM_LOCK_FLAG_EXCLUSIVE:    usize = 0x1;

pub const RD_DELETE_RANGE_EOF:        u64 = !0;

pub const RD_ATTR_DESCRIPTION:            u32 = 0x0001;
pub const RD_ATTR_PERMISSIONS:            u32 = 0x0002;
pub const RD_ATTR_CREATOR:                u32 = 0x0003;
pub const RD_ATTR_CREATED:                u32 = 0x0004;
pub const RD_ATTR_WRITTEN:                u32 = 0x0005;
pub const RD_ATTR_READ:                   u32 = 0x0006;
pub const RD_ATTR_CTX_SCHED_AFFINITY:     u32 = 0x1000;
pub const RD_ATTR_CTX_SCHED_PRIORITY:     u32 = 0x1001;
pub const RD_ATTR_CTX_SCHED_RUNTIME:      u32 = 0x1002;
pub const RD_ATTR_CTX_SCHED_STATE:        u32 = 0x1003;
pub const RD_ATTR_CTX_LIMITS_MEM_PRESENT: u32 = 0x1004;
pub const RD_ATTR_CTX_LIMITS_MEM_SWAPPED: u32 = 0x1005;
pub const RD_ATTR_CTX_LIMITS_CPU_TIME:    u32 = 0x1006;
pub const RD_ATTR_CTX_LIMITS_IO_TIME:     u32 = 0x1007;
pub const RD_ATTR_CTX_LIMITS_IO_OPS:      u32 = 0x1008;
pub const RD_ATTR_CTX_LIMITS_IO_RW:       u32 = 0x1009;
pub const RD_ATTR_CTX_USAGE_MEM_PRESENT:  u32 = 0x100A;
pub const RD_ATTR_CTX_USAGE_MEM_SWAPPED:  u32 = 0x100B;
pub const RD_ATTR_CTX_USAGE_CPU_TIME:     u32 = 0x100C;
pub const RD_ATTR_CTX_USAGE_IO_TIME:      u32 = 0x100D;
pub const RD_ATTR_CTX_USAGE_IO_OPS:       u32 = 0x100E;
pub const RD_ATTR_CTX_USAGE_IO_RW:        u32 = 0x100F;
pub const RD_ATTR_INT_PHY_ID:             u32 = 0x2000;

pub const CTX_STATE_RUNNING:              u32 = 0;
pub const CTX_STATE_BLOCKED:              u32 = 1;
pub const CTX_STATE_SUSPENDED:            u32 = 2;
pub const CTX_STATE_STOPPED:              u32 = 3;
pub const CTX_STATE_ABORTED:              u32 = 4;

/// Opens a resource, identified by `filename`.
///
/// # Description
///
/// # Arguments
///
/// | Argument   | Description
/// |------------|------------
/// | `filename` | The file to open. If `RD_OPEN_FLAG_RELATIVE` is set, the path is relative to `dir`.
/// |            | If the the path is `NULL`, an anonymous resource will be created.
/// | `flags`    | A bitfield, see *Flags*.
/// | `dir`      | If `RD_OPEN_FLAG_RELATIVE` is set, the directory to which `filename` is relative,
/// |            | ignored otherwise. If `dir` is `INVALID_RD` the task's working directory will be
/// |            | used instead.
///
/// # Flags
///
/// | Bit | Flag                    | Description
/// |-----|-------------------------|------------
/// |   1 | `RD_OPEN_FLAG_READ`     | Enable read access for the resource.
/// |   2 | `RD_OPEN_FLAG_WRITE`    | Enable write access for the resource.
/// |   3 | `RD_OPEN_FLAG_EXEC`     | Enable execution access for the resource.
/// |   4 | `RD_OPEN_FLAG_RELATIVE` | `filename` is relative to `dir`.
///
/// # Returns
///
/// ## On Success
///
/// A resource descriptor referencing the file.
///
/// ## On Failure
///
/// | Value | Error                      | Description
/// |-------|----------------------------|------------
/// |    -2 | `ERR_NOT_IMPLEMENTED`      | This syscall is not implemented.
/// |    -3 | `ERR_OUT_OF_KERNEL_MEMORY` | The kernel ran out of memory.
/// |    -4 | `ERR_INTERRUPTED`          | The operation was interrupted, but can be retried.
/// |    -5 | `ERR_IO`                   | A low-level IO error occurred.
/// |    -6 | `ERR_INVALID_ARG`          | An invalid argument was passed, one of:
/// |       |                            | - `filename` doesn't exist
/// |       |                            | - `dir`      is not open
/// |       |                            | - `dir`      is not a directory
/// |       |                            | - `flags`    had an unknown flag set
/// |       |                            | - `flags`    had neither `RD_OPEN_FLAG_READ`, `RD_OPEN_FLAG_WRITE` nor `RD_OPEN_FLAG_EXEC` set
/// |    -7 | `ERR_INVALID_MEM_REF`      | `filename` is outside of the task's accessible address space.
/// |    -8 | `ERR_PROTECTION`           | The task does not have permission to open the file with the
/// |       |                            | specified protection flags.
#[inline(always)]
pub fn sys_rd_open(filename: Option<&str>, flags: Flags, dir: Rd) -> Result<Rd> {
    arch_svc!(0, filename.map_or(core::ptr::null(), str::as_ptr), flags, dir)
}

/// Closes a resource.
///
/// # Description
///
/// # Arguments
///
/// | Argument | Description
/// |----------|------------
/// | `rd`     | The resource descriptor to close.
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
/// |    -2 | `ERR_NOT_IMPLEMENTED`      | This syscall is not implemented.
/// |    -6 | `ERR_INVALID_ARG`          | `rd` is not open
#[inline(always)]
pub fn sys_rd_close(rd: Rd) -> Result<()> {
    arch_svc!(1, rd)
}

/// Reads bytes from the given resource descriptor or file.
///
/// # Description
///
/// # Arguments
///
/// | Argument | Description
/// |----------|------------
/// | `rd`     | The resource descriptor or a file name, if `RD_IO_FLAG_FILE_NAME` is set, to write to.
/// | `flags`  | A bitfield, see *Flags*.
/// | `buf`    | A buffer or a vector of buffers, if `RD_IO_FLAG_VECTORED` is set. The buffer may be
/// |          | null or empty, this can be used to check if an io operation would succeed.
/// | `offset` | The offset into the resource, or ignored, if `rd` is a char device.
/// | `op_id`  | If an async flag is set, the id of this operation, ignored otherwise.
///
/// # Flags
///
/// | Bit | Flag                   | Description
/// |-----|------------------------|------------
/// |     | `RD_IO_FLAG_VECTORED`  | The passed buf is a vector of one or more io buffers.
/// |     | `RD_IO_FLAG_ASYNC`     | The operation will be executed asynchronously, an IO OP id must be
/// |     |                        | specified which is then used to poll the operation's result.
/// |     | `RD_IO_FLAG_ASYNC_INT` | The operation will be executed asynchronously, an IO OP id must be
/// |     |                        | specified. Once the operation is finished an `IoReady` interrupt will
/// |     |                        | be triggered.
/// |     | `RD_IO_FLAG_NON_BLOCK` | If no bytes are available, instead of blocking the task,
/// |     |                        | `RD_IO_ERR_WOULD_BLOCK` is returned.
/// |     | `RD_IO_FLAG_SYNC`      | The data will be directly read from disk, skipping the kernel's cache.
/// |     | `RD_IO_FLAG_FILE_NAME` | `rd` is a null-terminated file name instead of a resource descriptor.
/// |     |                        | This allows to skip calling `sys_rd_open` and `sys_rd_close`, thus
/// |     |                        | reducing overhead for simple io operations.
///
/// # Returns
///
/// ## On Success
///
/// The number of read bytes or 0, if `RD_IO_FLAG_ASYNC` or `RD_IO_FLAG_ASYNC_INT` is set.
///
/// ## On Failure
///
/// | Value | Error                      | Description
/// |-------|----------------------------|------------
/// |    -2 | `ERR_NOT_IMPLEMENTED`      | This syscall is not implemented.
/// |    -3 | `ERR_OUT_OF_KERNEL_MEMORY` | The kernel ran out of memory.
/// |    -4 | `ERR_INTERRUPTED`          | The operation was interrupted, but can be retried.
/// |    -5 | `ERR_IO`                   | A low-level IO error occurred.
/// |    -6 | `ERR_INVALID_ARG`          | An invalid argument was passed, one of:
/// |       |                            | - `rd`     is not open, or if a file name was passed, the file doesn't exist
/// |       |                            | - `rd`     is not readable (e.g. directory or pipe)
/// |       |                            | - `flags`  had an unknown flag set
/// |       |                            | - `buf`    was null or empty, and `RD_IO_FLAG_ASYNC` or `RD_IO_FLAG_ASYNC_INT` was set
/// |       |                            | - `offset` was out of bounds
/// |    -7 | `ERR_INVALID_MEM_REF`      | `buf` is outside of the task's accessible address space.
/// |    -8 | `ERR_PROTECTION`           | The rd was not opened with the `RD_OPEN_FLAG_READ` set.
/// | -4096 | `RD_IO_ERR_WOULD_BLOCK`    | The `RD_IO_FLAG_NON_BLOCK` flag was set, but no bytes were available.
#[inline(always)]
pub fn sys_rd_read(rd: RdOrPath, flags: usize, buf: IoReadBuf, offset: u64, op_id: IoOpId) -> Result<usize> {
    arch_svc!(2, rd, flags, buf, offset, op_id)
}

/// Writes bytes to the given resource descriptor or file.
///
/// # Description
///
/// # Arguments
///
/// | Argument | Description
/// |----------|------------
/// | `rd`     | The resource descriptor or a file name, if `RD_IO_FLAG_FILE_NAME` is set, to write to.
/// | `flags`  | A bitfield, see *Flags*.
/// | `buf`    | A buffer or a vector of buffers, if `RD_IO_FLAG_VECTORED` is set. The buffer may be
/// |          | null or empty, this can be used to check if an io operation would succeed.
/// | `offset` | The offset into the resource, or ignored, if `rd` is a char device.
/// | `op_id`  | If an async flag is set, the id of this operation, ignored otherwise.
///
/// # Flags
///
/// | Bit | Flag                   | Description
/// |-----|------------------------|------------
/// |     | `RD_IO_FLAG_VECTORED`  | The passed buf is a vector of one or more io buffers.
/// |     | `RD_IO_FLAG_ASYNC`     | The operation will be executed asynchronously, an IO OP id must be
/// |     |                        | specified which is then used to poll the operation's result.
/// |     | `RD_IO_FLAG_ASYNC_INT` | The operation will be executed asynchronously, an IO OP id must be
/// |     |                        | specified. Once the operation is finished an `IoReady` interrupt will
/// |     |                        | be triggered.
/// |     | `RD_IO_FLAG_NON_BLOCK` | If the operation needs to block the task to complete, `RD_IO_ERR_WOULD_BLOCK` is returned.
/// |     | `RD_IO_FLAG_SYNC`      | The data will be directly written to disk, skipping the kernel's cache.
/// |     | `RD_IO_FLAG_FILE_NAME` | `rd` is a null-terminated file name instead of a resource descriptor.
/// |     |                        | This allows to skip calling `sys_rd_open` and `sys_rd_close`, thus
/// |     |                        | reducing overhead for simple io operations.
///
/// # Returns
///
/// ## On Success
///
/// The number of read bytes or 0, if `RD_IO_FLAG_ASYNC` or `RD_IO_FLAG_ASYNC_INT` is set.
///
/// ## On Failure
///
/// | Value | Error                      | Description
/// |-------|----------------------------|------------
/// |    -2 | `ERR_NOT_IMPLEMENTED`      | This syscall is not implemented.
/// |    -3 | `ERR_OUT_OF_KERNEL_MEMORY` | The kernel ran out of memory.
/// |    -4 | `ERR_INTERRUPTED`          | The operation was interrupted, but can be retried.
/// |    -5 | `ERR_IO`                   | A low-level IO error occurred.
/// |    -6 | `ERR_INVALID_ARG`          | An invalid argument was passed, one of:
/// |       |                            | - `rd`     is not open, or if a file name was passed, the file doesn't exist
/// |       |                            | - `rd`     is not writeable (e.g. directory or pipe)
/// |       |                            | - `flags`  had an unknown flag set
/// |       |                            | - `buf`    was null or empty, and `RD_IO_FLAG_ASYNC` or `RD_IO_FLAG_ASYNC_INT` was set
/// |       |                            | - `offset` was out of bounds
/// |    -7 | `ERR_INVALID_MEM_REF`      | `buf` is outside of the task's accessible address space.
/// |    -8 | `ERR_PROTECTION`           | The rd was not opened with the `RD_OPEN_FLAG_WRITE` set.
/// | -4096 | `RD_IO_ERR_WOULD_BLOCK`    | The `RD_IO_FLAG_NON_BLOCK` flag was set, but the operation would block the task.
#[inline(always)]
pub fn sys_rd_write(rd: RdOrPath, flags: usize, buf: IoWriteBuf, offset: u64, op_id: IoOpId) -> Result<usize> {
    arch_svc!(3, rd, buf, offset, op_id)
}

/// Commits the resource's cached data to disk.
///
/// # Description
///
/// # Arguments
///
/// | Argument | Description
/// |----------|------------
/// | `rd`     | The resource to sync
/// | `flags`  | A bitfield, see *flags*
/// | `offset` | The offset within the resource of the range to be synced
/// | `len`    | The length of the range to be synced
///
/// # Flags
///
/// | Bit | Flag                   | Description
/// |-----|------------------------|------------
/// |   3 | `RD_IO_FLAG_NON_BLOCK` | If the operation needs to block in order to complete, `RD_IO_ERR_WOULD_BLOCK` is returned.
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
/// |    -2 | `ERR_NOT_IMPLEMENTED`      | This syscall is not implemented.
/// |    -3 | `ERR_OUT_OF_KERNEL_MEMORY` | The kernel ran out of memory.
/// |    -4 | `ERR_INTERRUPTED`          | The operation was interrupted, but can usually be retried.
/// |    -5 | `ERR_IO`                   | A low-level IO error occurred.
/// |    -6 | `ERR_INVALID_ARG`          | An invalid argument was passed, one of:
/// |       |                            | - `rd`     is not open
/// |       |                            | - `rd`     cannot be synced (because it is e.g. a directory)
/// |       |                            | - `flags`  had an unknown flag set
/// |       |                            | - `offset` is out of bounds for the resource
/// |       |                            | - `len`    is out of bounds for the resource
/// | -4096 | `RD_IO_ERR_WOULD_BLOCK`    | The `RD_IO_FLAG_NON_BLOCK` flag was set, but the operation needs to block in order to complete.
#[inline(always)]
pub fn sys_rd_sync(rd: Rd, flags: Flags, offset: usize, len: usize) -> Result<usize> {
    arch_svc!(4, rd, flags, offset, len)
}

/// Locks a range of the resource. If the resource is a directory or a char device, `offset` and
/// `length` must both be zero.
///
/// # Description
///
/// # Arguments
///
/// | Argument | Description
/// |----------|------------
/// | `rd`     | The resource to lock
/// | `flags`  | A bitfield, see *flags*
/// | `offset` | The offset within the resource of the range to be locked
/// | `len`    | The length of the range to be locked
///
/// # Flags
///
/// | Bit | Flag                     | Description
/// |-----|--------------------------|------------
/// |   1 | `RD_LOCK_FLAG_EXCLUSIVE` | Lock the range exclusively
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
/// |    -2 | `ERR_NOT_IMPLEMENTED`      | This syscall is not implemented.
/// |    -3 | `ERR_OUT_OF_KERNEL_MEMORY` | The kernel ran out of memory.
/// |    -6 | `ERR_INVALID_ARG`          | An invalid argument was passed, one of:
/// |       |                            | - `rd`     is not open
/// |       |                            | - `flags`  had an unknown flag set
/// |       |                            | - `offset` is out of bounds for the resource
/// |       |                            | - `len`    is out of bounds for the resource
#[inline(always)]
pub fn sys_rd_lock(rd: Rd, flags: Flags, offset: usize, len: usize) -> Result<usize> {
    arch_svc!(5, rd, flags, offset, len)
}

/// Unlocks a range of the resource. If the resource is a directory or a char device, `offset` and
/// `length` must both be zero.
///
/// # Description
///
/// # Arguments
///
/// | Argument | Description
/// |----------|------------
/// | `rd`     | The resource to unlock
/// | `flags`  | A bitfield, see *flags*
/// | `offset` | The offset within the resource of the range to be unlocked
/// | `len`    | The length of the range to be unlocked
///
/// # Flags
///
/// | Bit | Flag                     | Description
/// |-----|--------------------------|------------
/// |   1 | `RD_LOCK_FLAG_EXCLUSIVE` | Unlock the range exclusively
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
/// |    -2 | `ERR_NOT_IMPLEMENTED`      | This syscall is not implemented.
/// |    -3 | `ERR_OUT_OF_KERNEL_MEMORY` | The kernel ran out of memory.
/// |    -6 | `ERR_INVALID_ARG`          | An invalid argument was passed, one of:
/// |       |                            | - `rd`     is not open
/// |       |                            | - `flags`  had an unknown flag set
/// |       |                            | - `offset` is out of bounds for the resource
/// |       |                            | - `len`    is out of bounds for the resource
/// |       |                            | - the specified range is not locked
#[inline(always)]
pub fn sys_rd_unlock(rd: Rd, flags: Flags, offset: usize, len: usize) -> Result<usize> {
    arch_svc!(6, rd, flags, offset, len)
}

/// Maps the given resource into the task's address space.
///
/// # Description
///
///
///
/// # Arguments
///
/// | Argument | Description
/// |----------|------------
/// | `addr`   | The address where the area will be mapped to
/// | `len`    | The length of the area being mapped, in pages (4KB)
/// | `rd`     | The resource to map
/// | `flags`  | A bitfield, see *flags*
///
/// # Flags
///
/// | Bit | Flag                         | Description
/// |-----|------------------------------|------------
/// |   1 | `MEM_MAP_FLAG_PROT_READ`     | Enable read access for this memory region
/// |   2 | `MEM_MAP_FLAG_PROT_WRITE`    | Enable write access for this memory region
/// |   3 | `MEM_MAP_FLAG_PROT_EXEC`     | Enable execution access for this memory region
/// |   4 | `MEM_MAP_FLAG_PROT_SHARE`    | Updates to this mapping are visible to other tasks. If this flag is *not* set, a copy-on-write mapping will be created.
/// |   5 | `MEM_MAP_FLAG_INIT_UNINIT`   | Do not clear (zero) anonymous mappings (if this option is disabled in the kernel, the memory will still be zeroed)
/// |   6 | `MEM_MAP_FLAG_INIT_POPULATE` | Preload the rd's data, instead of loading lazily
/// |   7 | `MEM_MAP_FLAG_USAGE_STACK`   | This memory region may be used as a stack
/// |   8 | `MEM_MAP_FLAG_PHYSICAL_CONT` | The area's physical address range should be continuous. Only for anonymous areas.
/// |   9 | `MEM_MAP_FLAG_ADDRESS_HINT`  | The passed address is just a hint, the actual address of the mapping will be returned.
/// |  10 | `MEM_MAP_FLAG_DISCARD_OLD`   | Discards mappings that lie in the specified range
/// |  11 | `MEM_MAP_FLAG_ATTACH`        | The specified address rane is already mapped and contains data that should be attached to this resource
///
/// # Returns
///
/// ## On Success
///
/// If the `MEM_MAP_FLAG_ADDRESS_HINT` flag was set, the address of the mapped area, 0 otherwise.
///
/// ## On Failure
///
/// | Value | Error                      | Description
/// |-------|----------------------------|------------
/// |    -2 | `ERR_NOT_IMPLEMENTED`      | This syscall is not implemented.
/// |    -3 | `ERR_OUT_OF_KERNEL_MEMORY` | The kernel ran out of memory.
/// |    -4 | `ERR_INTERRUPTED`          | The operation was interrupted, but can usually be retried.
/// |    -5 | `ERR_IO`                   | A low-level IO error occurred.
/// |    -6 | `ERR_INVALID_ARG`          | An invalid argument was passed, one of:
/// |       |                            | - `rd`    is not open
/// |       |                            | - `rd`    cannot be mapped (because it is e.g. a char device)
/// |       |                            | - `flags` had an unknown flag set
/// |       |                            | - the specified area is already partially mapped and `MEM_MAP_FLAG_DISCARD_OLD` was not set
/// |    -7 | `ERR_INVALID_MEM_REF`      | The specified memory area is not mappable
/// |    -8 | `ERR_PROTECTION`           | The rd was not opened with the required protection flags.
#[inline(always)]
pub fn sys_rd_mem_map(addr: *mut u8, len: usize, rd: Rd, flags: Flags) -> Result<*mut u8> {
    arch_svc!(7, addr, len, rd, flags)
}

/// Maps the given resource into the task's address space.
///
/// # Description
///
///
///
/// # Arguments
///
/// | Argument   | Description
/// |------------|------------
/// | `addr`     | The address of a currently mapped area
/// | `len`      | The length of the area, in pages (4KB)
/// | `new_addr` | The new address where the area will be remapped to
/// | `flags`    | A bitfield, see *flags*
///
/// # Flags
///
/// | Bit | Flag                         | Description
/// |-----|------------------------------|------------
/// |   1 | `MEM_MAP_FLAG_PROT_READ`     | Enable read access for this memory region
/// |   2 | `MEM_MAP_FLAG_PROT_WRITE`    | Enable write access for this memory region
/// |   3 | `MEM_MAP_FLAG_PROT_EXEC`     | Enable execution access for this memory region
/// |   4 | `MEM_MAP_FLAG_PROT_SHARE`    | Updates to this mapping are visible to other tasks. If this flag is *not* set, a copy-on-write mapping will be created.
/// |   5 | `MEM_MAP_FLAG_INIT_UNINIT`   | Do not clear (zero) anonymous mappings (if this option is disabled in the kernel, the memory will still be zeroed)
/// |   6 | `MEM_MAP_FLAG_INIT_POPULATE` | Preload the rd's data, instead of loading lazily
/// |   7 | `MEM_MAP_FLAG_USAGE_STACK`   | This memory region may be used as a stack
/// |   9 | `MEM_MAP_FLAG_ADDRESS_HINT`  | The passed address is just a hint, the actual address of the mapping will be returned.
/// |  10 | `MEM_MAP_FLAG_DISCARD_OLD`   | Discards mappings that lie in the specified range
///
/// # Returns
///
/// ## On Success
///
/// If the `MEM_MAP_FLAG_ADDRESS_HINT` flag was set, the address of the mapped area, 0 otherwise.
///
/// ## On Failure
///
/// | Value | Error                      | Description
/// |-------|----------------------------|------------
/// |    -2 | `ERR_NOT_IMPLEMENTED`      | This syscall is not implemented.
/// |    -3 | `ERR_OUT_OF_KERNEL_MEMORY` | The kernel ran out of memory.
/// |    -4 | `ERR_INTERRUPTED`          | The operation was interrupted, but can usually be retried.
/// |    -5 | `ERR_IO`                   | A low-level IO error occurred.
/// |    -6 | `ERR_INVALID_ARG`          | An invalid argument was passed, one of:
/// |       |                            | - `flags` had an unknown flag set
/// |       |                            | - the specified area is already partially mapped and `MEM_MAP_FLAG_DISCARD_OLD` was not set
/// |    -7 | `ERR_INVALID_MEM_REF`      | The new memory area is not mappable, or the old memory area was not mapped.
/// |    -8 | `ERR_PROTECTION`           | The rd was not opened with the required protection flags.
#[inline(always)]
pub fn sys_rd_remap(addr: *mut u8, len: usize, new_addr: *mut u8, flags: Flags) -> Result<*mut u8> {
    arch_svc!(8, addr, len, new_addr, flags)
}

#[inline(always)]
pub fn sys_rd_unmap(addr: *mut u8, len: usize) -> Result<()> {
    arch_svc!(9, addr, len)
}

#[inline(always)]
pub fn sys_rd_mem_sync(addr: *mut u8, len: usize, flags: Flags) -> Result<()> {
    arch_svc!(10, addr, len, flags)
}

#[inline(always)]
pub fn sys_rd_mem_lock(addr: *mut u8, len: usize, flags: Flags) -> Result<()> {
    arch_svc!(11, addr, len, flags)
}

#[inline(always)]
pub fn sys_rd_mem_unlock(addr: *mut u8, len: usize, flags: Flags) -> Result<()> {
    arch_svc!(12, addr, len, flags)
}

#[inline(always)]
pub fn sys_rd_ops(ops: &[IoOp]) -> Result<()> {
    arch_svc!(13, ops)
}

/// Completes (or polls) an IO operation.
///
/// # Description
///
/// # Arguments
///
/// | Argument | Description
/// |----------|------------
/// | `op_id`  | An IO OP ID
/// | `flags`  | A bitfield, see *Flags*.
///
/// # Flags
///
/// | Bit | Flag                   | Description
/// |-----|------------------------|------------
/// |     | `RD_IO_FLAG_NON_BLOCK` | If the operation needs to block the task to complete, `RD_IO_ERR_WOULD_BLOCK` is returned.
///
/// # Returns
///
/// ## On Success
///
/// The number of transferred bytes.
///
/// ## On Failure
///
/// | Value | Error                      | Description
/// |-------|----------------------------|------------
/// |    -2 | `ERR_NOT_IMPLEMENTED`      | This syscall is not implemented.
/// |    -3 | `ERR_OUT_OF_KERNEL_MEMORY` | The kernel ran out of memory.
/// |    -4 | `ERR_INTERRUPTED`          | The operation was interrupted, but can be retried.
/// |    -5 | `ERR_IO`                   | A low-level IO error occurred.
/// |    -6 | `ERR_INVALID_ARG`          | An invalid argument was passed, one of:
/// |       |                            | - `op_id` was invalid
/// |    -7 | `ERR_INVALID_MEM_REF`      | `buf` is outside of the task's accessible address space.
/// | -4096 | `RD_IO_ERR_WOULD_BLOCK`    | The `RD_IO_FLAG_NON_BLOCK` flag was set, but the operation would block the task.
#[inline(always)]
pub fn sys_rd_poll(id: IoOpId, flags: usize) -> Result<usize> {
    arch_svc!(14, id, flags)
}

#[inline(always)]
pub fn sys_sync_wake(addr: *mut usize, count: usize, len: usize) -> Result<()> {
    arch_svc!(15, addr, count, len)
}

#[inline(always)]
pub fn sys_sync_wait(addr: *mut usize, val: usize, len: usize, op: SyncWaitOp) -> Result<()> {
    arch_svc!(16, addr, val, len, op)
}

#[inline(always)]
pub fn sys_rd_enumerate(rd: RdOrPath, buf: *mut u8, flags: u32) -> Result<()> {
    arch_svc!(17, rd, buf)
}

#[inline(always)]
pub fn sys_rd_create(path: &str, dir: Rd) -> Result<()> {
    arch_svc!(18, path, dir)
}

#[inline(always)]
pub fn sys_rd_create_range(rd: RdOrPath, dir: Rd, from: u64, to: u64) -> Result<()> {
    arch_svc!(19, rd, dir)
}

#[inline(always)]
pub fn sys_rd_delete(rd: RdOrPath, dir: Rd, flags: u32) -> Result<()> {
    arch_svc!(19, rd, dir)
}

#[inline(always)]
pub fn sys_rd_delete_range(rd: RdOrPath, dir: Rd, from: u64, to: u64) -> Result<()> {
    arch_svc!(19, rd, dir)
}

#[inline(always)]
pub fn sys_rd_move(rd: RdOrPath, path: &str, dir: Rd) -> Result<()> {
    arch_svc!(20, rd, path, dir)
}

#[inline(always)]
pub fn sys_set_attr(rd: RdOrPath, key: usize, val: usize, dir: Rd, flags: u32) -> Result<()> {
    arch_svc!(21, rd, val, dir)
}

#[inline(always)]
pub fn sys_get_attr(rd: RdOrPath, key: usize, dir: Rd, flags: u32) -> Result<usize> {
    arch_svc!(22, rd, key, dir)
}

#[inline(always)]
pub fn sys_ctx_alloc() -> Result<Rd> {

}

#[inline(always)]
pub fn sys_ctx_free(rd: Rd) -> Result<()> {

}

#[inline(always)]
pub fn sys_ctx_change_state(rd: Rd, state: u32) -> Result<()> {

}

#[inline(always)]
pub fn sys_ctx_mask(rd: Rd) -> Result<()> {

}

#[inline(always)]
pub fn sys_ctx_unmask(rd: Rd) -> Result<()> {

}

#[inline(always)]
pub fn sys_ctx_mount(rd: Rd, mnt: Rd, flags: usize) -> Result<()> {

}

#[inline(always)]
pub fn sys_ctx_unmount(rd: Rd, mnt: Rd) -> Result<()> {

}

#[inline(always)]
pub fn sys_int_alloc() -> Result<Rd> {

}

#[inline(always)]
pub fn sys_int_free(rd: Rd) -> Result<()> {

}

#[inline(always)]
pub fn sys_int_mask(rd: Rd) -> Result<()> {

}

#[inline(always)]
pub fn sys_int_unmask(rd: Rd) -> Result<()> {

}