#![no_std]
#![warn(clippy::all)]
#![feature(
        naked_functions,
    panic_info_message,
    alloc_error_handler,
    abi_efiapi,
    lang_items,
    format_args_nl,
    const_option,
    exclusive_range_pattern,
    slice_ptr_get,
    allocator_api
)]
#![allow(incomplete_features)]

pub mod arch;
pub mod ctx;
pub mod hart;
pub mod int;
pub mod mem;
pub mod misc;
pub mod mnt;
pub mod log;
pub mod svc;
pub mod svi;
pub mod hvc;
pub mod hvi;

const VIRT_NULL_OFFSET:     usize = 0;
const VIRT_BOOT_IDENTITY:   usize = 0x1000;
const VIRT_KERN_XO_OFFSET:  usize = 0x1_0000;
const VIRT_KERN_RO_OFFSET:  usize = 0;
const VIRT_KERN_RW_OFFSET:  usize = 0;
const VIRT_KERN_DYN_OFFSET: usize = 0;
const VIRT_PERI_OFFSET:     usize = 0x40_0000_0000;
const VIRT_USER_OFFSET:     usize = 0x80_0000_0000;

pub struct SvData {
	pub ctx:       *mut ctx::Context, // sorted by cid
	pub ctx_lru:   *mut ctx::Context,
	pub mnt:       TrieNode<mnt::Node>,
    pub harts:     Tree<hart::Hart, sort_hart_by_load>, // sorted by load
	pub mem_nodes: Tree<mem::NodeDescriptor, sort_node_by_address>, // sorted by start_page
	pub mem_areas: Tree<mem::PhysMemoryArea>, // sorted by address
	pub mem_table: *mut [u64; 512],
	pub log_buf:   log::LogBuf
}

fn sort_hart_by_load() {

}

fn sort_node_by_address() {

}

impl SvData {
    // Maps the given page into the kernel's virtual memory space for dynamic data
    pub fn map_page(ppn: u32, len: u32) -> u32 {
		unimplemented!()
    }

	pub fn unmap_page(vpn: u32, len: u32) -> u32 {
		unimplemented!()
    }
}