
#![no_std]
#![warn(clippy::all)]
#![feature(
    naked_functions,
    panic_info_message,
    alloc_error_handler,
    lang_items,
    format_args_nl,
    const_option,
    exclusive_range_pattern,
    slice_ptr_get,
    allocator_api
)]
#![allow(incomplete_features, dead_code)]

extern crate alloc;
extern crate rlibc;

pub use misc::std;

pub mod ctx;
pub mod int;
pub mod mnt;
pub mod mem;
pub mod hart;
pub mod svc;
pub mod arch;
pub mod misc;

pub mod hv;
pub mod sv;
pub mod us;

#[no_mangle]
pub static KERNEL_DATA: SvData = unsafe { core::mem::zeroed() };

#[no_mangle]
fn tests() {
    println!("\n\n================================ TESTS ================================\n");
    mem::test::buddy_alloc();
    println!("\n\n=======================================================================\n");
}

extern "C" {
    pub static _text_start:     usize;
    pub static _rodata_start:   usize;
    pub static _data_start:     usize;
    pub static _bss_start:      usize;
    pub static _kernel_end:     usize;
    pub static _global_pointer: usize;
}