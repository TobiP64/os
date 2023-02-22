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

use core::{ptr::{null, null_mut}, mem::size_of};
use crate::utils::DbgBin;

pub type Status = isize;
pub type Handle = *mut u8;
pub type Event  = usize;
pub type Guid   = u128;

/// The operation completed successfully.
pub const STATUS_SUCCESS: isize = 0;

/// The image failed to load.
pub const STATUS_LOAD_ERROR: isize = -1;
/// A parameter was incorrect.
pub const STATUS_INVALID_PARAMETER: isize = -2;
/// The operation is not supported.
pub const STATUS_UNSUPPORTED: isize = -3;
/// The buffer was not the proper size for the request.
pub const STATUS_BAD_BUFFER_SIZE: isize = -4;
/// The buffer is not large enough to hold the requested data. The required buffer size is returned in the appropriate parameter when this error occurs.
pub const STATUS_BUFFER_TOO_SMALL: isize = -5;
/// There is no data pending upon return.
pub const STATUS_NOT_READY: isize = -6;
/// The physical device reported an error while attempting the operation.
pub const STATUS_DEVICE_ERROR: isize = -7;
/// The device cannot be written to.
pub const STATUS_WRITE_PROTECTED: isize = -8;
/// A resource has run out.
pub const STATUS_OUT_OF_RESOURCES: isize = -9;
/// An inconstancy was detected on the file system causing the operating to fail.
pub const STATUS_VOLUME_CORRUPTED: isize = -10;
/// There is no more space on the file system.
pub const STATUS_VOLUME_FULL: isize = -11;
/// The device does not contain any medium to perform the operation.
pub const STATUS_NO_MEDIA: isize = -12;
/// The medium in the device has changed since the last access.
pub const STATUS_MEDIA_CHANGED: isize = -13;
/// The item was not found.
pub const STATUS_NOT_FOUND: isize = -14;
/// Access was denied.
pub const STATUS_ACCESS_DENIED: isize = -15;
/// The server was not found or did not respond to the request.
pub const STATUS_NO_RESPONSE: isize = -16;
/// A mapping to a device does not exist.
pub const STATUS_NO_MAPPING: isize = -17;
/// The timeout time expired.
pub const STATUS_TIMEOUT: isize = -18;
/// The protocol has not been started.
pub const STATUS_NOT_STARTED: isize = -19;
/// The protocol has already been started.
pub const STATUS_ALREADY_STARTED: isize = -20;
/// The operation was aborted.
pub const STATUS_ABORTED: isize = -21;
/// An ICMP error occurred during the network operation.
pub const STATUS_ICMP_ERROR: isize = -22;
/// A TFTP error occurred during the network operation.
pub const STATUS_TFTP_ERROR: isize = -23;
/// A protocol error occurred during the network operation.
pub const STATUS_PROTOCOL_ERROR: isize = -24;
/// The function encountered an internal version that was incompatible with a version requested by the caller.
pub const STATUS_INCOMPATIBLE_VERSION: isize = -25;
/// The function was not performed due to a security violation.
pub const STATUS_SECURITY_VIOLATION: isize = -26;
/// A CRC error was detected.
pub const STATUS_CRC_ERROR: isize = -27;
/// Beginning or end of media was reached
pub const STATUS_END_OF_MEDIA: isize = -28;
/// The end of the file was reached.
pub const STATUS_END_OF_FILE: isize = -31;
/// The language specified was invalid.
pub const STATUS_INVALID_LANGUAGE: isize = -32;
/// The security status of the data is unknown or compromised and the data must be updated or replaced to restore a valid security status.
pub const STATUS_COMPROMISED_DATA: isize = -33;
/// There is an address conflict address allocation
pub const STATUS_IP_ADDRESS_CONFLICT: isize = -34;
/// A HTTP error occurred during the network operation.
pub const STATUS_HTTP_ERROR: isize = -35;
/// The string contained one or more characters that the device could not render and were skipped.
pub const STATUS_WARN_UNKNOWN_GLYPH: isize = -1;
/// The handle was closed, but the file was not deleted.
pub const STATUS_WARN_DELETE_FAILURE: isize = -2;
/// The handle was closed, but the data to the file was not flushed properly.
pub const STATUS_WARN_WRITE_FAILURE: isize = -3;
/// The resulting buffer was too small, and the data was truncated to the buffer size.
pub const STATUS_WARN_BUFFER_TOO_SMALL: isize = -4;
/// The data has not been updated within the timeframe set by local policy for this type of data.
pub const STATUS_WARN_STALE_DATA: isize = -5;
/// The resulting buffer contains UEFI-compliant file system.
pub const STATUS_WARN_FILE_SYSTEM: isize = -6;
/// The operation will be processed across a system reset.
pub const STATUS_WARN_RESET_REQUIRED: isize = -7;

pub const SYSTEM_TABLE_SIGNATURE:         u64  = 0x5453595320494249;
pub const BOOT_SERVICES_SIGNATURE:        u64  = 0x56524553544f4f42;
pub const RUNTIME_SERVICES_SIGNATURE:     u64  = 0x56524553544e5552;
pub const ACPI_10_TABLE_GUID:             Guid = 0x4DC13F279000169A11D32D88EB9D2D30;
pub const ACPI_20_TABLE_GUID:             Guid = 0x81883CC7800022BC11D3E4F18868E871;
pub const SAL_SYSTEM_TABLE_GUID:          Guid = 0x4dc13f279000169a11d32d88eb9d2d32;
pub const SMBIOS_TABLE_GUID:              Guid = 0x4dc13f279000169a11d32d88eb9d2d31;
pub const SMBIOS3_TABLE_GUID:             Guid = 0x94e320cfbbe52e994a2c9794f2fd1544;
pub const MPS_TABLE_GUID:                 Guid = 0x4dc13f279000169a11d32d88eb9d2d2f;
pub const JSON_CONFIG_DATA_TABLE_GUID:    Guid = 0x8a551f11e08becaa41ce111987367f87;
pub const JSON_CAPSULE_DATA_TABLE_GUID:   Guid = 0x569010a8cd3311804cac8dd235e7a725;
pub const JSON_CAPSULE_RESULT_TABLE_GUID: Guid = 0xe5a149fd8698b4b9422ab3dedbc461c3;
pub const GRAPHICS_OUTPUT_PROTOCOL_GUID:  Guid = 0x6a5180d0de7afb964a3823dc9042a9de;

pub const MEMORY_UNCACHABLE:          u64 = 0x1;
pub const MEMORY_WRITE_COMBINING:     u64 = 0x2;
pub const MEMORY_WRITE_THROUGH:       u64 = 0x4;
pub const MEMORY_WRITE_BACK:          u64 = 0x8;
pub const MEMORY_UNCACHABLE_EXPORTED: u64 = 0x10;
pub const MEMORY_WRITE_PROTECTED:     u64 = 0x1000;
pub const MEMORY_READ_PROTECTED:      u64 = 0x2000;
pub const MEMORY_EXEC_PROTECTED:      u64 = 0x4000;
pub const MEMORY_PERSISTENT:          u64 = 0x8000;
pub const MEMORY_MORE_RELIABLE:       u64 = 0x10000;
pub const MEMORY_READ_ONLY:           u64 = 0x20000;
pub const MEMORY_SPECIAL_PURPOSE:     u64 = 0x40000;
pub const MEMORY_CPU_CRYPTO:          u64 = 0x80000;
pub const MEMORY_RUNTIME:             u64 = 0x8000000000000000;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct TableHeader {
	pub signature:   u64,
	pub revision:    u32,
	pub header_size: u32,
	pub crc32:       u32,
	_res0:           u32
}

#[repr(C)]
pub struct SystemTable<'a> {
	pub header:              TableHeader,
	firmware_vendor:         *const u16,
	pub firmware_revision:   u32,
	pub console_in_handle:   Handle,
	pub console_in:          &'a SimpleTextInputProtocol,
	pub console_out_handle:  Handle,
	pub console_out:         &'a SimpleTextOutputProtocol<'a>,
	pub std_err_handle:      Handle,
	pub std_err:             &'a SimpleTextOutputProtocol<'a>,
	pub runtime_services:    &'a RuntimeServices,
	pub boot_services:       &'a BootServices,
	number_of_table_entries: usize,
	configuration_table:     *const ConfigurationTable
}

impl SystemTable<'_> {
	pub fn firmware_vendor<'a>(&self, buf: &'a mut [u8]) -> &'a str {
		from_efi_str(self.firmware_vendor, buf)
	}

	pub fn configuration_table(&self) -> &[ConfigurationTable] {
		unsafe { core::slice::from_raw_parts(self.configuration_table, self.number_of_table_entries) }
	}
}

impl core::fmt::Debug for SystemTable<'_> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("SystemTable")
			.field("header", &self.header)
			.field("firmware_vendor", &self.firmware_vendor(&mut [0u8; 0x100]))
			.field("firmware_revision", &self.firmware_revision)
			.field("configuration_table", &self.configuration_table())
			.finish()
	}
}

#[repr(C)]
pub struct SimpleTextInputProtocol {
	reset:            extern "efiapi" fn(&Self, bool) -> Status,
	read_key_stroke:  extern "efiapi" fn(&Self, &mut InputKey) -> Status,
	pub wait_for_key: Event
}

impl SimpleTextInputProtocol {
	pub fn reset(&self, extended_verification: bool) -> Status {
		(self.reset)(self, extended_verification)
	}

	pub fn read_key_stroke(&self) -> (Status, InputKey) {
		let mut key = InputKey::default();
		((self.read_key_stroke)(self, &mut key), key)
	}
}

impl core::fmt::Debug for SimpleTextInputProtocol {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("SimpleTextInputProtocol")
			.field("wait_for_key", &self.wait_for_key)
			.finish()
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct InputKey {
	scan_code:    u16,
	unicode_char: u16
}

impl Into<Option<char>> for InputKey {
	fn into(self) -> Option<char> {
		core::char::from_u32(self.unicode_char as _)
	}
}

#[repr(C)]
pub struct SimpleTextOutputProtocol<'a> {
	reset:               extern "efiapi" fn(&Self, bool) -> Status,
	output_string:       extern "efiapi" fn(&Self, *const u16) -> Status,
	test_string:         extern "efiapi" fn(&Self, *const u16) -> Status,
	query_mode:          extern "efiapi" fn(&Self, usize, &mut usize, &mut usize) -> Status,
	set_mode:            extern "efiapi" fn(&Self, usize) -> Status,
	set_attribute:       extern "efiapi" fn(&Self, usize) -> Status,
	clear_screen:        extern "efiapi" fn(&Self) -> Status,
	set_cursor_position: extern "efiapi" fn(&Self, usize, usize) -> Status,
	enable_cursor:       extern "efiapi" fn(&Self, bool) -> Status,
	pub mode:            &'a SimpleTextOutputMode
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct SimpleTextOutputMode {
	max_mode:       u32,
	mode:           u32,
	attribute:      u32,
	cursor_column:  u32,
	cursor_row:     u32,
	cursor_visible: bool
}

impl SimpleTextOutputProtocol<'_> {
	pub fn reset(&self, extended_verification: bool) -> Status {
		(self.reset)(self, extended_verification)
	}

	pub fn output_string(&self, s: &str, buf: &mut [u16]) -> Status {
		(self.output_string)(self, to_efi_str(s, buf))
	}

	pub fn test_string(&self, s: &str, buf: &mut [u16]) -> Status {
		(self.test_string)(self, to_efi_str(s, buf))
	}

	pub fn query_mode(&self, mode_number: usize) -> (Status, usize, usize) {
		let mut columns = 0;
		let mut rows = 0;
		((self.query_mode)(self, mode_number, &mut columns, &mut rows), columns, rows)
	}

	pub fn set_mode(&self, mode_number: usize) -> Status {
		(self.set_mode)(self, mode_number)
	}

	pub fn set_attribute(&self, attr: usize) -> Status {
		(self.set_attribute)(self, attr)
	}

	pub fn clear_screen(&self) -> Status {
		(self.clear_screen)(self)
	}

	pub fn set_cursor_position(&self, column: usize, row: usize) -> Status {
		(self.set_cursor_position)(self, column, row)
	}

	pub fn enable_cursor(&self, visible: bool) -> Status {
		(self.enable_cursor)(self, visible)
	}
}

impl core::fmt::Write for SimpleTextOutputProtocol<'_> {
	fn write_str(&mut self, s: &str) -> core::fmt::Result {
		s.split("").for_each(|s|  { self.output_string(s, &mut [0u16; 16]); });
		//self.output_string(s, &mut [0u16, 0x800]);
		Ok(())
	}
}

impl core::fmt::Debug for SimpleTextOutputProtocol<'_> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("SimpleTextOutputProtocol")
			.field("mode", &self.mode)
			.finish()
	}
}

#[repr(C)]
pub struct BootServices {
	pub header:                    TableHeader,
	raise_tpl:                     extern "efiapi" fn() -> Status,
	restore_tpl:                   extern "efiapi" fn() -> Status,

	allocate_pages:                extern "efiapi" fn() -> Status,
	free_pages:                    extern "efiapi" fn() -> Status,
	get_memory_map:                extern "efiapi" fn(*mut usize, *mut u8, *mut usize, *mut usize, *mut u32) -> Status,
	allocate_pool:                 extern "efiapi" fn() -> Status,
	free_pool:                     extern "efiapi" fn() -> Status,

	create_event:                  extern "efiapi" fn() -> Status,
	set_timer:                     extern "efiapi" fn() -> Status,
	wait_for_event:                extern "efiapi" fn() -> Status,
	signal_event:                  extern "efiapi" fn() -> Status,
	close_event:                   extern "efiapi" fn() -> Status,
	check_event:                   extern "efiapi" fn() -> Status,
	install_protocol_interface:    extern "efiapi" fn() -> Status,
	reinstall_protocol_interface:  extern "efiapi" fn() -> Status,
	uninstall_protocol_interface:  extern "efiapi" fn() -> Status,
	handle_protocol:               extern "efiapi" fn(Handle, *const Guid, *mut *mut ()) -> Status,
	_res0:                         extern "efiapi" fn(),
	register_protocol_notify:      extern "efiapi" fn() -> Status,
	locate_handle:                 extern "efiapi" fn(LocateSearchType, *const Guid, *mut (), *mut usize, *mut Handle) -> Status,
	locate_device_path:            extern "efiapi" fn() -> Status,
	install_configuration_table:   extern "efiapi" fn() -> Status,

	image_load:                    extern "efiapi" fn() -> Status,
	image_start:                   extern "efiapi" fn() -> Status,
	exit:                          extern "efiapi" fn() -> Status,
	image_unload:                  extern "efiapi" fn() -> Status,
	exit_boot_services:            extern "efiapi" fn(Handle, usize) -> Status,

	get_next_monotonic_count:      extern "efiapi" fn() -> Status,
	stall:                         extern "efiapi" fn() -> Status,
	set_watchdog_timer:            extern "efiapi" fn() -> Status,

	connect_controller:            extern "efiapi" fn() -> Status,
	disconnect_controller:         extern "efiapi" fn() -> Status,

	open_protocol:                 extern "efiapi" fn() -> Status,
	close_protocol:                extern "efiapi" fn() -> Status,
	open_protocol_information:     extern "efiapi" fn() -> Status,

	protocols_per_handle:          extern "efiapi" fn() -> Status,
	locate_handle_buffer:          extern "efiapi" fn() -> Status,
	locate_protocol:               extern "efiapi" fn() -> Status,
	install_multiple_interfaces:   extern "efiapi" fn() -> Status,
	uninstall_multiple_interfaces: extern "efiapi" fn() -> Status,

	calculate_crc32:               extern "efiapi" fn() -> Status,

	copy_mem:                      extern "efiapi" fn() -> Status,
	set_mem:                       extern "efiapi" fn() -> Status,
	create_event_ex:               extern "efiapi" fn() -> Status
}

impl BootServices {
	pub fn get_memory_map(&self, size: &mut usize, memory_map: *mut u8, map_key: &mut usize, descriptor_size: &mut usize, descriptor_version: &mut usize) -> Status {
        (self.get_memory_map)(
            &mut size,
			memory_map,
			&mut map_key,
			&mut descriptor_size,
			&mut descriptor_version
		)
    }

	pub fn handle_protocol(&self, handle: Handle, protocol: Guid, interface: &mut *mut ()) -> Status {
		(self.handle_protocol)(handle, &protocol, interface)
	}

	pub fn locate_handle(
		&self,
		search_type: LocateSearchType,
		protocol:    Guid,
		search_key:  *mut (),
		buffer_size: &mut usize,
		buffer:      &mut Handle
	) -> Status {
		(self.locate_handle)(search_type, &protocol, search_key, buffer_size, buffer)
	}

	pub fn exit_boot_services(&self, image: Handle, map_key: usize) -> Status {
		(self.exit_boot_services)(image, map_key)
	}
}

impl core::fmt::Debug for BootServices {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("BootServices")
			.field("header", &self.header)
			.finish()
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LocateSearchType {
	AllHandles,
	ByRegisterNotify,
	ByProtocol
}

#[repr(C)]
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MemoryType {
	ReservedMemoryType,
	LoaderCode,
	LoaderData,
	BootServicesCode,
	BootServicesData,
	RuntimeServicesCode,
	RuntimeServicesData,
	ConventionalMemory,
	UnusableMemory,
	AcpiReclaimMemory,
	AcpiMemoryNvs,
	MemoryMappedIo,
	MemoryMappedIoPortSpace,
	PalCode,
	PersistentMemory
}

impl Default for MemoryType {
	fn default() -> Self {
		Self::ReservedMemoryType
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct MemoryDescriptor {
	pub r#type:          MemoryType,
	pub physical_start:  u64,
	pub virtual_start:   u64,
	pub number_of_pages: u64,
	pub attribute:       u64
}

impl core::fmt::Display for MemoryDescriptor {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "{:016X} - {:016X} @ {:016X} - {:016X} {:6} pages {:?}",
			   self.virtual_start, self.virtual_start + (self.number_of_pages << 12),
			   self.physical_start, self.physical_start + (self.number_of_pages << 12),
			   self.number_of_pages, self.r#type)
	}
}

pub struct MemoryMap<const N: usize> {
    buf: [u8; N],
    size: usize,
    desc: usize
}

impl<const N: usize> MemoryMap<N> {
    pub fn new(buf: [u8; N], size: usize, desc: usize) -> Self {
        Self { buf, size, desc}
    }
}

impl<'a, const N: usize> IntoIterator for &'a MemoryMap<N> {
    type Item = &'a MemoryDescriptor;
    type IntoIter = MemoryMapIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MemoryMapIter {
            buf: &self.buf[self.size],
            len: self.desc,
        }
    }
}

pub struct MemoryMapIter<'a> {
	buf: &'a [u8],
	len: usize
}

impl<'a> Iterator for MemoryMapIter<'a> {
	type Item = &'a MemoryDescriptor;

	fn next(&mut self) -> Option<Self::Item> {
		if self.buf.len() < self.len {
			None
		} else {
			let desc = unsafe { (self.buf.as_ptr() as *const MemoryDescriptor).as_ref() }.unwrap();
			self.buf = &self.buf[self.len..];
			Some(desc)
		}
	}
}

pub struct GraphicsOutputProtocol<'a> {
	pub query_mode: extern "efiapi" fn(*const Self, u32, *mut usize, *mut *const GraphicsOutputModeInformation) -> Status,
	pub set_mode:   extern "efiapi" fn(*const Self, u32) -> Status,
	pub blt:        extern "efiapi" fn(*const Self, *mut GraphicsOutputBltPixel, GraphicsOutputBltOperation, usize, usize, usize, usize, usize, usize, usize) -> Status,
	pub mode:       &'a GraphicsOutputProtocolMode<'a>
}

impl GraphicsOutputProtocol<'_> {
	pub fn query_mode(
		&self,
		mode_number:  u32,
		size_of_info: &mut usize,
		info:         &mut *const GraphicsOutputModeInformation
	) -> Status {
		(self.query_mode)(self, mode_number, size_of_info, info)
	}

	pub fn set_mode(&self, mode_number: u32) -> Status {
		(self.set_mode)(self, mode_number)
	}

	pub fn blyat(
		&self,
		blt_buffer:    Option<&mut [GraphicsOutputBltPixel]>,
		blt_operation: GraphicsOutputBltOperation,
		src:           (usize, usize),
		dst:           (usize, usize),
		dim:           (usize, usize),
		delta:         Option<usize>
	) -> Status {
		(self.blt)(
			self,
			blt_buffer.map_or(null_mut(), <[_]>::as_mut_ptr),
			blt_operation,
			src.0,
			src.1,
			dst.0,
			dst.1,
			dim.0,
			dim.1,
			delta.unwrap_or(0)
		)
	}
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PixelBitmask {
	pub red_mask:      u32,
	pub green_mask:    u32,
	pub blue_mask:     u32,
	pub reserved_mask: u32
}

impl core::fmt::Debug for PixelBitmask {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("PixelBitmask")
			.field("red_mask", &DbgBin(self.red_mask))
			.field("green_mask", &DbgBin(self.green_mask))
			.field("blue_mask", &DbgBin(self.blue_mask))
			.field("reserved_mask", &DbgBin(self.reserved_mask))
			.finish()
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GraphicsPixelFormat {
	R8G8B8X8,
	B8G8R8X8,
	BitMask,
	BltOnly,
	Max
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GraphicsOutputModeInformation {
	pub version:               u32,
	pub horizontal_resolution: u32,
	pub vertical_resolution:   u32,
	pub pixel_format:          GraphicsPixelFormat,
	pub pixel_information:     PixelBitmask,
	pub pixels_per_scan_line:  u32
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct GraphicsOutputProtocolMode<'a> {
	pub max_mode:          u32,
	pub mode:              u32,
	pub info:              &'a GraphicsOutputModeInformation,
	pub size_of_info:      u32,
	pub frame_buffer_base: *mut u8,
	pub frame_buffer_size: usize
}

impl GraphicsOutputProtocolMode<'_> {
	pub fn frame_buffer(&mut self) -> &mut [u8] {
		unsafe { core::slice::from_raw_parts_mut(self.frame_buffer_base, self.frame_buffer_size) }
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct GraphicsOutputBltPixel {
	pub blue:     u8,
	pub green:    u8,
	pub red:      u8,
	pub reserved: u8
}

impl GraphicsOutputBltPixel {
	pub const fn from_rgba(color: u32) -> Self {
		Self {
			blue:     (color >> 16 & 0xFF) as _,
			green:    (color >> 8 & 0xFF) as _,
			red:      (color & 0xFF) as _,
			reserved: (color >> 24 & 0xFF) as _
		}
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GraphicsOutputBltOperation {
	VideoFill,
	VideoToBltBuffer,
	BufferToVideo,
	VideoToVideo,
	Max
}

pub const RT_SUPPORTED_GET_TIME:                   usize = 0x0001;
pub const RT_SUPPORTED_SET_TIME:                   usize = 0x0002;
pub const RT_SUPPORTED_GET_WAKEUP_TIME:            usize = 0x0004;
pub const RT_SUPPORTED_SET_WAKEUP_TIME:            usize = 0x0008;
pub const RT_SUPPORTED_GET_VARIABLE:               usize = 0x0010;
pub const RT_SUPPORTED_GET_NEXT_VARIABLE_NAME:     usize = 0x0020;
pub const RT_SUPPORTED_SET_VARIABLE:               usize = 0x0040;
pub const RT_SUPPORTED_SET_VIRTUAL_ADDRESS_MAP:    usize = 0x0080;
pub const RT_SUPPORTED_CONVERT_POINTER:            usize = 0x0100;
pub const RT_SUPPORTED_GET_NEXT_HIGH_MONO_COUNT:   usize = 0x0200;
pub const RT_SUPPORTED_RESET_SYSTEM:               usize = 0x0400;
pub const RT_SUPPORTED_UPDATE_CAPSULE:             usize = 0x0800;
pub const RT_SUPPORTED_QUERY_CAPSULE_CAPABILITIES: usize = 0x1000;
pub const RT_SUPPORTED_QUERY_VARIABLE_INFO:        usize = 0x2000;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Time {
	pub year:       u16,
	pub month:      u8,
	pub day:        u8,
	pub hour:       u8,
	pub minute:     u8,
	pub second:     u8,
	pad1:           u8,
	pub nanosecond: u32,
	pub time_zone:  i16,
	pub day_light:  u8,
	pad2:           u8
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub struct Capabilities {
	resolution:   u32,
	accuracy:     u32,
	sets_to_zero: bool
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ResetType {
	Cold,
	Warm,
	Shutdown,
	PlatformSpecific
}

#[repr(C)]
pub struct RuntimeServices {
	pub header:                 TableHeader,
	get_time:                   extern "efiapi" fn(*mut Time, *const Capabilities) -> Status,
	set_time:                   extern "efiapi" fn(*const Time) -> Status,
	get_wakeup_time:            extern "efiapi" fn(*mut bool, *mut bool, *mut Time) -> Status,
	set_wakeup_time:            extern "efiapi" fn(bool, *const Time) -> Status,
	set_virtual_address_map:    extern "efiapi" fn(usize, usize, u32, *const MemoryDescriptor) -> Status,
	convert_pointer:            extern "efiapi" fn(usize, *mut *const ()) -> Status,
	get_variable:               extern "efiapi" fn(*const u16, *const Guid, *mut u32, *mut usize, *mut u8) -> Status,
	get_next_variable_name:     extern "efiapi" fn(*mut usize, *mut u16, *mut Guid) -> Status,
	set_variable:               extern "efiapi" fn(*const u16, *const Guid, u32, usize, *const u8) -> Status,
	get_next_high_mono_count:   extern "efiapi" fn(*mut u32) -> Status,
	reset_system:               extern "efiapi" fn(ResetType, Status, usize, *const u8) -> !,
	update_capsule:             extern "efiapi" fn(*const *const (), usize, u64) -> Status,
	query_capsule_capabilities: extern "efiapi" fn() -> Status,
	query_variable_info:        extern "efiapi" fn() -> Status
}

impl RuntimeServices {
	pub fn get_time(&self, caps: &Capabilities) -> (Status, Time) {
		let mut time = Time::default();
		((self.get_time)(&mut time, caps), time)
	}

	pub fn set_time(&self, time: &Time) -> Status {
		(self.set_time)(time)
	}

	pub fn get_wakeup_time(&self) -> (Status, bool, bool, Time) {
		let mut enabled = false;
		let mut pending = false;
		let mut time = Time::default();
		((self.get_wakeup_time)(&mut enabled, &mut pending, &mut time), enabled, pending, time)
	}

	pub fn set_wakeup_time(&self, enable: bool, time: Option<&Time>) -> Status {
		(self.set_wakeup_time)(enable, time.map_or(null(), |t| t as _))
	}

	pub fn set_virtual_address_map(&self, descriptor_version: u32, virtual_map: &[MemoryDescriptor]) -> Status {
		(self.set_virtual_address_map)(
			size_of::<MemoryDescriptor>() * virtual_map.len(),
			size_of::<MemoryDescriptor>(),
			descriptor_version,
			virtual_map.as_ptr()
		)
	}

	pub fn convert_pointer(&self, debug_disposition: usize, mut address: *const ()) -> (Status, *const ()) {
		((self.convert_pointer)(debug_disposition, &mut address), address)
	}

	pub fn get_variable(
		&self,
		name:       &str,
		vendor:     &Guid,
		attributes: Option<&mut u32>,
		data_size:  &mut usize,
		data:       *mut u8,
		buf:        &mut [u16]
	) -> Status {
		(self.get_variable)(
			to_efi_str(name, buf),
			vendor,
			attributes.map_or(null_mut(), |v| v as _),
			data_size,
			data
		)
	}

	pub fn get_next_variable(
		&self,
		name_size: &mut usize,
		name:      *mut u16,
		vendor:    &mut Guid
	) -> Status {
		(self.get_next_variable_name)(name_size, name, vendor)
	}

	pub fn set_variable(
		&self,
		name:       &str,
		vendor:     &Guid,
		attributes: u32,
		data:       &[u8],
		buf:        &mut [u16]
	) -> Status {
		(self.set_variable)(
			to_efi_str(name, buf),
			vendor,
			attributes,
			data.len(),
			data.as_ptr()
		)
	}

	pub fn get_next_high_mono_count(&self) -> (Status, u32) {
		let mut high_count = 0;
		((self.get_next_high_mono_count)(&mut high_count), high_count)
	}

	pub fn reset_system(&self, r#type: ResetType, status: Status, data: Option<&[u8]>) -> ! {
		(self.reset_system)(
			r#type,
			status,
			data.map_or(0, <[_]>::len),
			data.map_or(null(), <[_]>::as_ptr)
		)
	}
}

impl core::fmt::Debug for RuntimeServices {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("RuntimeServices")
			.field("header", &self.header)
			.finish()
	}
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ConfigurationTable {
	guid:  Guid,
	table: *mut ()
}

impl ConfigurationTable {
	pub fn cast<T: Table>(&self) -> Option<&T> {
		(self.guid == T::GUID).then(|| unsafe { (self.table as *mut T).as_ref() }.unwrap())
	}
}

impl core::fmt::Debug for ConfigurationTable {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		core::fmt::Debug::fmt(&CfgTable::from(*self), f)
	}
}

impl core::fmt::Display for ConfigurationTable {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "{:032X} @ {:?}", self.guid, self.table)
	}
}

#[derive(Copy, Clone, Debug)]
pub enum CfgTable<'a> {
	Acpi10(&'a acpi::RSDP),
	Acpi20(&'a acpi::RSDP),
	SalSystem(*mut ()),
	SmBios2(&'a smbios::SmBios2EntryPoint),
	SmBios3(&'a smbios::SmBios3EntryPoint),
	JsonConfigData(*mut ()),
	JsonCapsuleData(*mut ()),
	JsonCapsuleResult(*mut ()),
	Other(*mut ())
}

impl From<ConfigurationTable> for CfgTable<'_> {
	fn from(table: ConfigurationTable) -> Self {
		match table.guid {
			ACPI_10_TABLE_GUID             => Self::Acpi10(unsafe { (table.table as *const acpi::RSDP).as_ref().unwrap() }),
			ACPI_20_TABLE_GUID             => Self::Acpi20(unsafe { (table.table as *const acpi::RSDP).as_ref().unwrap() }),
			SAL_SYSTEM_TABLE_GUID          => Self::SalSystem(table.table as _),
			SMBIOS_TABLE_GUID              => Self::SmBios2(unsafe { (table.table as *const smbios::SmBios2EntryPoint).as_ref().unwrap() }),
			SMBIOS3_TABLE_GUID             => Self::SmBios3(unsafe { (table.table as *const smbios::SmBios3EntryPoint).as_ref().unwrap() }),
			JSON_CONFIG_DATA_TABLE_GUID    => Self::JsonConfigData(table.table as _),
			JSON_CAPSULE_DATA_TABLE_GUID   => Self::JsonCapsuleData(table.table as _),
			JSON_CAPSULE_RESULT_TABLE_GUID => Self::JsonCapsuleResult(table.table as _),
			_                              => Self::Other(table.table)
		}
	}
}

pub trait Table {
	const GUID: Guid;
}

fn to_efi_str(s: &str, buf: &mut [u16]) -> *const u16 {
	let mut len = 0;
	for (i, ch) in s.encode_utf16().enumerate() {
		buf[i] = ch;
		len += 1;
	}

	buf[len] = 0;
	buf.as_ptr()
}

fn from_efi_str(s: *const u16, buf: &mut [u8]) -> &str {
	let mut len = 0;

	loop {
		let ch = unsafe { s.add(len).read() };

		if ch == 0 || len == buf.len() {
			return unsafe { core::str::from_utf8_unchecked(&buf[..len]) }
		}

		buf[len] = ch as u8;
		len += 1;
	}
}