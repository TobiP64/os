
use core::sync::atomic::*;

pub const EVENT_CTX_CREATE:   u16 = 0x1;
pub const EVENT_CTX_DELETE:   u16 = 0x2;
pub const EVENT_CTX_PREEMT:   u16 = 0x3;
pub const EVENT_CTX_SCHEDULE: u16 = 0x4;
pub const EVENT_CTX_ENQUEUE:  u16 = 0x5;
pub const EVENT_CTX_DEQUEUE:  u16 = 0x6;
pub const EVENT_CTX_INTERRUPT: u16 = 0x7;

pub const EVENT_ALLOC_PAGE:  u16 = 0x10;

pub struct LogBuf {
    pub buf:   *mut LogEntry,
    pub end:   *mut LogEntry,
    pub ptr:   core::sync::atomic::AtomicPtr<LogEntry>,
}

impl LogBuf {
    pub fn log(&self, event: LogEntry) {
        loop {
            let ptr = self.ptr.fetch_ptr_add(1, Ordering::Acquire);

            if ptr >= self.end {
                if self.ptr.compare_exchange(ptr, self.buf, Ordering::Release, Ordering::Release).is_err() {
                    continue;
                }

                ptr = self.buf;
            }

            unsafe { *ptr = event }
        }
    }
}

pub struct LogEntry {
    pub event:  u16,
    pub hart:   u16,
    pub ctx:    u16,
    pub time:   u64,
    pub param0: u32,
    pub param1: u32,
    pub param2: u32,
    pub param3: u32,
}