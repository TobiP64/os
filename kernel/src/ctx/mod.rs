use super::*;

pub const PRIO_REALTIME:   i8 = 127;
pub const PRIO_DEFAULT:    i8 = 0;
pub const PRIO_BACKGROUND: i8 = -127;

pub struct HvContext {
	pub id:           u32,
	pub mem_table:    *mut [u64; 512],
	pub time_offset:  i64,
	pub page_table:   *mut (),
	pub res_harts:    *mut HvHart,
	pub res_mems:     *mut HvMemory,
	pub res_drives:   *mut HvDrive,
    pub ctx_end:      *mut ()
}

pub struct HvHart {
    pub hart:         *mut hart::Hart,
	pub time:         u64,
	pub state:        u8,
	pub priority:     u8,
	pub runtime:      u32,
    pub next:         *mut HvHart,
	pub kernel_stack: *mut (),
    pub core_img:     crate::arch::Context,
}

pub struct HvMemory {
    pub mem:     *mut mem::NodeDescriptor,
    pub size:    u32,
	pub present: u32
}

pub struct HvDrive {
    pub drive:    *mut (),
	pub partid:   u128,
	pub lim_ops:  u32,
	pub lim_rw:   u32,
	pub lim_time: u32
}

pub struct Context {
	pub id:              u32,
	pub flags:           u32,
	pub parent:          *mut Self,
	pub sibling_prev:    *mut Self,
	pub sibling_next:    *mut Self,
	pub children:        *mut Self,
	pub kernel_stack:    *mut (),
	pub time_offset:     i64,
	pub sch_state:       u8,
	pub sch_priority:    u8,
	pub sch_runtime:     u32,
	pub sch_affinity:    [u8; 128],
    pub sch_hart:        *mut hart::Hart,
    pub sch_parent:      *mut Self,
	pub sch_left:        *mut Self,
	pub sch_right:       *mut Self,
	pub cid_counter:     AtomicU32,
	pub cid_table:       Tree<Self>,
	pub int_mask:        u128,
	pub int_vector:      InterruptVector,
	pub mem_table:       *mut [u64; 512],
	pub mem_areas:       Tree<crate::mem::VirtMemoryArea>,
	pub mem_descs:       Tree<ResourceDescriptor>,
	pub mnt_nodes:       Trie<mnt::Node>,
	pub lim_cpu_time:    u32,
	pub lim_mem_mapped:  u32,
	pub lim_mem_present: u32,
	pub lim_mem_swapped: u32,
	pub lim_io_ops_ram:  u32,
	pub lim_io_ops_msm:  u32,
	pub lim_io_rw_ram:   u32,
	pub lim_io_rw_msm:   u32,
	pub lim_io_time_ram: u32,
	pub lim_io_time_msm: u32,
	pub lim_open_rds:    u32,
	pub lim_threats:     u32,
	pub usg_cpu_time:    u32,
	pub usg_mem_mapped:  u32,
	pub usg_mem_present: u32,
	pub usg_mem_swapped: u32,
	pub usg_io_ops_ram:  u32,
	pub usg_io_ops_msm:  u32,
	pub usg_io_rw_ram:   u32,
	pub usg_io_rw_msm:   u32,
	pub usg_io_time_ram: u32,
	pub usg_io_time_msm: u32,
	pub usg_open_rds:    u32,
	pub usg_threads:     u32,
	pub svc:             *mut [fn (svc::SvcId) -> svc::Status]
}

impl Context {
	pub const STATE_READY:     u8 = 0;
	pub const STATE_RUNNING:   u8 = 1;
	pub const STATE_BLOCKED:   u8 = 2;
	pub const STATE_SUSPENDED: u8 = 3;
	pub const STATE_STOPPED:   u8 = 4;

	/// Context is a context id namespace
	pub const FLAG_CTX_CID:    u32 = 1 << 0;
	/// Context can be executed
	pub const FLAG_CTX_EXE:    u32 = 1 << 1;
	/// Context has its own interrupt handler
	pub const FLAG_CTX_INT:    u32 = 1 << 2;
	/// Context has its own translation tables
	pub const FLAG_CTX_MEM:    u32 = 1 << 3;
	/// Context is a mount namespace
	pub const FLAG_CTX_MNT:    u32 = 1 << 4;
	/// Context has resource limits
	pub const FLAG_CTX_LIM:    u32 = 1 << 5;
	/// Context records resource usage
	pub const FLAG_CTX_USG:    u32 = 1 << 6;
	/// Context can be scheduled
	pub const FLAG_CTX_SCH:    u32 = 1 << 7;
    /// Context captures its children's syscalls
	pub const FLAG_CTX_SVC:    u32 = 1 << 8;

    /// Kernel context, runs in a privileged environment, used for memory/hardware management, scheduler rebalancing
    pub const FLAG_PRIVILEGED:        u32 = 1 << 9;
    pub const FLAG_INT_VECTORED:      u32 = 1 << 10;
    pub const FLAG_INT_MASKED:        u32 = 1 << 11;
	/// If FLAG_CTX_MNT is set, writes are visible to the parent context
	pub const FLAG_MNT_WRITE_THROUGH: u32 = 1 << 12;
	/// If FLAG_CTX_MNT is set, writes of the parent context are visible
	pub const FLAG_MNT_READ_THROUGH:  u32 = 1 << 13;
}

pub union InterruptVector {
    pub handler: extern "C" fn(usize),
	pub table:   [extern "C" fn(); 128],
}

pub struct ResourceDescriptor {
    pub flags:  usize,
	pub mapped: Tree<crate::mem::VirtMemoryArea>,
	pub node:   *mut mnt::Node,
	pub ctx:    *mut Context,
	pub next:   *mut Self,
	pub prev:   *mut Self
}

pub const INTID_INVALID_MEM_REF:     usize = 1;
pub const INTID_ILLEGAL_INSTRUCTION: usize = 2;
pub const INTID_FP_EXCEPTION:        usize = 3;
pub const INTID_DEBUG:               usize = 4;
pub const INTID_TIMER:               usize = 5;
pub const INTID_TERMINATE:           usize = 6;
pub const INTID_ABORT:               usize = 7;

#[repr(C)]
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum InterruptWithArgs {
	/// The task tried to access memory that was not mapped, or the accessed address was misaligned.
	InvalidMemRef { address: *mut usize, reason: usize, faulting_address: *mut u8 },
	IllegalInstruction { address: *mut usize, reason: usize },
	FloatingPointException { address: *mut usize, reason: usize },
	Debug { address: *mut usize, r#type: usize },
	/// Timer triggered interrupt
	Timer,
	/// Request the task to terminate
	Term,
	/// Kill the task violently
	Abort,
	/// An IO operation finished
	IoReady { rd: Rd, op_id: usize, result: crate::spi::Result<usize> },
	/// A synchronization primitive is available
	Sync { address: *mut u8 },
	/// Triggered when a task's status transitions to halted or stopped
	TaskStatusChange { task: TaskId, old_state: TaskState, new_state: TaskState },
	/// A task send an Rd to this task
	SendRd { task: TaskId, rd: Rd },
	/// Some file system event occurred that this task has subscribed to
	FsEvent { rd: Rd, event: FsEvent }
}

pub enum InterruptAction {
	/// Ignore the interrupt
	Ignore,
	/// Abort the task
	Abort,
	/// Interrupt the task and call the interrupt handler
	Handle,
	/// Does not interrupt the task, instead the interrupt is handled as its own task
	HandleDetached,
	/// If the task was halted, resume execution
	Resume
}

struct IoOp<'a> {
	id:    IoOpId,
	rd:    *mut ,
	ctx:   *mut Context,
	intid: u8,
	flags: usize,
	off:   usize,
	buf:   IoReadBuf<'a>
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FsEvent {
	Create,
	Modify { flags: usize },
	Remove
}

pub const FS_EVENT_MODIFY_FLAG_NAME:        usize = 0x1;
pub const FS_EVENT_MODIFY_FLAG_DATA:        usize = 0x2;
pub const FS_EVENT_MODIFY_FLAG_PERMISSIONS: usize = 0x4;
pub const FS_EVENT_MODIFY_FLAG_ATTR:        usize = 0x8;