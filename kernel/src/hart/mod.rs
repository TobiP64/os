use super::*;

pub struct Hart {
	pub id:              u32,
	pub status:          u32,
	pub flags:           u32,
	pub timer:           arch::Timer,
    pub int:             arch::Hart,
    pub preferred_node:  *mut mem::NodeDescriptor,
	pub current:         *mut ctx::Context,
    pub queue:           *mut ctx::Context,
    pub migration_queue: AtomicPtr<ctx::Context>,
    /// The accumulated priority of all task on this hart, used for load balancing
	pub load:          u32,
    pub latency:       u32,
    pub runtime:       u64,
    pub min_load:      u32,
	pub min_latency:   u32,
	pub min_granulity: u32,
    pub min_runtime:   u64,
}

impl Hart {
    pub const STATUS_UNKNOWN:        usize = 0;
    pub const STATUS_PRE_BOOT:       usize = 1;
    pub const STATUS_BOOTING:        usize = 2;
    pub const STATUS_BOOT_COMPLETED: usize = 3;
    pub const STATUS_UP:             usize = 4;
    pub const STATUS_DOWN:           usize = 5;

    pub const FLAG_BSC:              usize = 1;

    pub fn up() {

    }

    pub fn down() {

    }

    pub fn enqueue() {

    }

    pub fn dequeue() {

    }



    pub fn up() {

    }

    pub fn down() {

    }

    pub fn enqueue(&mut self, ctx: &mut ctx::Context) {
        self.load      += ctx.sch_priority;
        ctx.sch_hart    = self;
        ctx.sch_state   = ctx::Context::STATE_READY;
        ctx.sch_runtime = self.min_runtime;
        ctx.sch_parent  = ;
        ctx.sch_left    = ;
        ctx.sch_right   = ;
        self.preempt(ctx::Context::STATE_READY);
    }

    pub fn dequeue(&mut self, ctx: &mut ctx::Context, state: u8) {
        self.load    -= ctx.sch_priority;
        ctx.sch_state = state;
        self.preempt(ctx::Context::STATE_READY);
    }

    pub fn preempt(&mut self, state: u8) {
        if self.current.is_null() {

        }



        // TODO select ctx

        self.schedule()
    }

    pub fn preempt(&mut self) {
        arch::context_save();
        let ctx = unsafe { &*self.current };
        ctx.sch_runtime += self.timer.get() * PRIO_TABLE[ctx.sch_priority] / self.load;
        ctx.sch_state = state;
    }

    pub fn schedule(&mut self) {
        if self.current.is_null() {
            hw::arch::park();
        }

        let ctx          = unsafe { &*self.current };
        let time_slice   = self.min_granulity.max(self.latency * PRIO_TABLE[ctx.sch_priority] / self.load);
        ctx.sch_state    = ctx::Context::STATE_RUNNING;
        self.min_runtime = ctx.sch_runtime;
        self.timer.set(time_slice);
        arch::context_restore_return();
    }
}

static PRIO_TABLE: [u32; 256] = [

];