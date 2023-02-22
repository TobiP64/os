use std::ptr;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct TableHeader {
	pub signature:   u64,
	pub revision:    u32,
	pub header_size: u32,
	pub crc32:       u32,
	_res0:           u32
}

type Status = isize;

const BASE_ADDR: usize = 0x1000_0000;
const TRAMPOLINE_STRIDE: usize = 0x10;

fn dispatch_table<const N: usize>(ptr: &mut usize) -> [extern "efiapi" fn() -> Status; N] {
	let mut ptrs = [ptr::null() as _; N];
	let mut i = 0;
	let end = *ptr + N * TRAMPOLINE_STRIDE;
	
	while *ptr < end {
		ptrs[i] = *ptr as extern "efiapi" fn() -> Status;
		i += 1;
		*ptr += TRAMPOLINE_STRIDE;
	}
	
	ptrs
}

fn init_data(data: &mut Data) {
	let base = data as *mut _ as usize;
	let trampoline_ptr = base + 0x1000;
	data.system_table_header.signature     = 0x5453595320494249;
	data.runtime_services_header.signature = 0x56524553544e5552;
	data.boot_services_header.signature    = 0x56524553544f4f42;
	data.system_table_firmware_vendor      = base + 0x10;
	data.system_table_firmware_revision    = env!("CRATE_VERSION");
	data.system_table_console_in           = base + 0x20;
	data.system_table_console_out          = base + 0x30;
	data.system_table_std_err              = base + 0x40;
	data.system_table_runtime_services     = base + 0x50;
	data.system_table_boot_services        = base + 0x60;
	data.console_in_ptrs                   = dispatch_table(&mut trampoline_ptr);
	data.console_out_ptrs                  = dispatch_table(&mut trampoline_ptr);
	data.console_err_ptrs                  = dispatch_table(&mut trampoline_ptr);
	data.runtime_services_ptrs             = dispatch_table(&mut trampoline_ptr);
	data.boot_services_ptrs                = dispatch_table(&mut trampoline_ptr);
	data.gop_ptrs                          = dispatch_table(&mut trampoline_ptr);
}

struct Data {
	// bytes    0 -   15 system table
	// bytes   16 -   31 firmware vendor
	// bytes   16 -   31 console in
	// bytes   32 -   47 console out
	// bytes   48 -   63 console err
	// bytes   64 -   79 runtime services
	// bytes   80 -  143 boot services
	// bytes  144 -  159 GOP
	// bytes  160 - 4095 cfg tables
	// bytes  160 - 4095 ACPI tables
	// bytes 4096 - 8191 trampoline
	
	system_table_header:                  TableHeader,
	system_table_firmware_vendor:         *const u16,
	system_table_firmware_revision:       u32,
	system_table_console_in_handle:       *const (),
	system_table_console_in:              *const (),
	system_table_console_out_handle:      *const (),
	system_table_console_out:             *const (),
	system_table_std_err_handle:          *const (),
	system_table_std_err:                 *const (),
	system_table_runtime_services:        *const (),
	system_table_boot_services:           *const (),
	system_table_number_of_table_entries: usize,
	system_table_configuration_table:     *const (),
	_pad0:                                [u64; 1],
	
	firmware_vendor_val:             [u8; 16],
	
	console_in_ptrs:                 [extern "efiapi" fn() -> Status; 2],
	console_in_wait_for_key:         usize,
	_pad1:                           [u64; 13],
	
	console_out_ptrs:                [extern "efiapi" fn() -> Status; 9],
	console_out_mode_ptr:            *const (),
	console_out_max_mode:            u32,
	console_out_mode:                u32,
	console_out_attribute:           u32,
	console_out_cursor_column:       u32,
	console_out_cursor_row:          u32,
	console_out_cursor_visible:      bool,
	_pad2:                           [u64; 3],
	
	console_err_ptrs:                [extern "efiapi" fn() -> Status; 9],
	console_err_mode_ptr:            *const (),
	console_err_max_mode:            u32,
	console_err_mode:                u32,
	console_err_attribute:           u32,
	console_err_cursor_column:       u32,
	console_err_cursor_row:          u32,
	console_err_cursor_visible:      bool,
	
	_pad3:                           [u64; 3],
	
	runtime_services_header:         TableHeader,
	runtime_services_ptrs:           [extern "efiapi" fn() -> Status; 13],
	
	boot_services_header:            TableHeader,
	boot_services_ptrs:              [extern "efiapi" fn() -> Status; 61],
	
	gop_ptrs:                        [extern "efiapi" fn() -> Status; 3],
	gop_mode_ptr:                    *const (),
	gop_max_mode:                    u32,
	gop_mode:                        u32,
	gop_info:                        *const (),
	gop_size_of_info:                u32,
	gop_frame_buffer_base:           *mut (),
	gop_frame_buffer_size:           usize,
	gop_version:                     u32,
	gop_horizontal_resolution:       u32,
	gop_vertical_resolution:         u32,
	gop_pixel_format:                u32,
	gop_pixel_information:           u32,
	gop_pixels_per_scan_line:        u32,
	_pad4:                           [u64; 4],
	
	trampoline:                      [u64; 256]
}