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

pub use alloc::{boxed::*, str::*, string::*, vec::*, *};

mod handlers {

    #[global_allocator]
    static GLOBAL: alloc::System = alloc::System;

    /// This is required for unwinding when the program panics, but since we abort on panic this
    /// function is never actually called. This just exists to make the compiler happy.
    #[lang = "eh_personality"]
    extern "C" fn eh_personality() {}

    #[panic_handler]
    fn panic(info: &core::panic::PanicInfo) -> ! {
        crate::eprintln!("{}", info);
        crate::arch::park();
    }

    #[alloc_error_handler]
    fn alloc_error(layout: core::alloc::Layout) -> ! {
        panic!("alloc error: layout = {:?}", layout)
    }

    /// This is not implemented by compiler-builtins for aarch64, but required by aarch64-unknown-driver-uefi.
    /// Its called when local variables exceed 4KB/8KB of size, to avoid accessing stack memory after
    /// the guard page. Since we don't have an OS, we neither have a guard page and the stack is (almost)
    /// unlimited.
    #[cfg(all(target_arch = "aarch64", target_os = "driver-uefi"))]
    #[naked]
    #[no_mangle]
    pub unsafe fn __chkstk() {
        llvm_asm!("ret" ::: "memory" : "volatile");
    }
}

mod alloc {
    pub use ::alloc::*;

    use crate::GLOBAL_DATA;
    pub use core::alloc::*;
    use core::ptr::{null_mut, NonNull};

    pub struct System;

    unsafe impl GlobalAlloc for System {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            GLOBAL_DATA
                .cache
                .allocate(layout)
                .map_or(null_mut(), |p| p.as_mut_ptr())
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            GLOBAL_DATA
                .cache
                .deallocate(NonNull::new_unchecked(ptr), layout);
        }

        unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
            GLOBAL_DATA
                .cache
                .allocate_zeroed(layout)
                .map_or(null_mut(), |p| p.as_mut_ptr())
        }
    }
}

pub mod sync {
    pub use alloc::sync::*;
    use core::cell::UnsafeCell;
    use core::sync::atomic::AtomicUsize;

    #[derive(Default, Debug)]
    pub struct RwLock<T: ?Sized> {
        lock: AtomicUsize,
        data: UnsafeCell<T>,
    }

    impl<T> RwLock<T> {
        pub fn new(v: T) -> Self {
            Self {
                lock: AtomicUsize::new(0),
                data: UnsafeCell::new(v),
            }
        }
    }
}

pub mod io {
    #[link_section = ".data"]
    static mut OUT: fn(&str) = dummy_out;

    fn dummy_out(c: char) {}

    pub fn set_out(out: fn(&str)) {
        unsafe { OUT = out; }
    }

    pub struct OutWriter;

    impl core::fmt::Write for OutWriter {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            (unsafe { OUT })(s);
            Ok(())
        }
    }

    #[macro_export]
    macro_rules! print {
        ($($arg:tt)*) => ({ core::write!($crate::OutWriter, $($arg)* ).unwrap_or(()) });
    }

    #[macro_export]
    macro_rules! println {
        ($($arg:tt)*) => ({ core::writeln!($crate::OutWriter, $($arg)* ).unwrap_or(()) });
    }

    #[macro_export]
    macro_rules! eprint {
        ($($arg:tt)*) => ({ core::write!($crate::OutWriter, $($arg)* ).unwrap_or(()) });
    }

    #[macro_export]
    macro_rules! eprintln {
        ($($arg:tt)*) => ({ core::writeln!($crate::OutWriter, $($arg)* ).unwrap_or(()) });
    }
}
