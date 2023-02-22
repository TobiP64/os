
use super::*;

pub mod vendors;

pub type PciEcam = [[[[u8; 0x1000]; 8]; 32]; 256];

#[repr(C)]
#[derive(Copy, Clone)]
pub union ConfigurationSpaceHeader {
	common: ConfigurationSpaceHeaderCommon,
	type0:  ConfigurationSpaceHeaderType0,
	type1:  ConfigurationSpaceHeaderType1,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ConfigurationSpaceHeaderCommon {
	pub vendor_id:   u16,
	pub device_id:   u16,
	pub command:     u16,
	pub status:      u16,
	pub revision_id: u8,
	pub prog_if:     u8,
	pub subclass:    u8,
	pub class_code:  u8,
	pub _pad0:       u8,
	pub _pad1:       u8,
	pub header_type: u8,
	pub bist:        u8,
	pub _pad2:       [u32; 9],
	pub caps_ptr:    u8,
	pub _pad3:       [u8; 3],
	pub _pad4:       [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ConfigurationSpaceHeaderType0 {
	pub vendor_id:   u16,
	pub device_id:   u16,
	pub command:     u16,
	pub status:      u16,
	pub revision_id: u8,
	pub prog_if:     u8,
	pub subclass:    u8,
	pub class_code:  u8,
	pub _pad0:       u8,
	pub _pad1:       u8,
	pub header_type: u8,
	pub bist:        u8,
	pub bar0:        u64,
	pub bar1:        u64,
	pub bar2:        u64,
	pub cardbus_cis_ptr: u32,
	pub subsys_vendor_id: u16,
	pub subsys_id:   u16,
	pub  ext_rom_base_addr: u32,
	pub _pad3:       [u8; 3],
	pub _pad4:       [u32; 2]
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ConfigurationSpaceHeaderType1 {
	pub vendor_id:                   u16,
	pub device_id:                   u16,
	pub command:                     u16,
	pub status:                      u16,
	pub revision_id:                 u8,
	pub prog_if:                     u8,
	pub subclass:                    u8,
	pub class_code:                  u8,
	pub _pad0:                       u8,
	pub _pad1:                       u8,
	pub header_type:                 u8,
	pub bist:                        u8,
	pub bar0:                        u64,
	pub primary_bus_number:          u8,
	pub secondary_bus_number:        u8,
	pub subordinate_bus_number:      u8,
	pub _pad2:                       u8,
	pub io_base:                     u8,
	pub io_limit:                    u8,
	pub secondary_status:            u16,
	pub memory_base:                 u16,
	pub memory_limit:                u16,
	pub prefetchable_memory_base:    u16,
	pub prefetchable_memory_limit:   u16,
	pub prefetchable_memory_base_u:  u32,
	pub prefetchable_memory_limit_u: u32,
	pub io_base_u:                   u16,
	pub io_limit_u:                  u16,
	pub caps_ptr:                    u8,
	pub _pad3:                       [u8; 3],
	pub bridge_control:              u16,
	pub _pad4:                       [u8; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union Capability {
	pub header:        CapabilityHeader,
	pub msi32:         MsiCapability32,
	pub msi32vectored: MsiCapability32Vectored,
	pub msi64:         MsiCapability64,
	pub msi64vectored: MsiCapability64Vectored,
	
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CapabilityHeader {
	pub id:  u8,
	pub ptr: u8
}

impl CapabilityHeader {
	const ID_MSI: u8 = 0x05;
	const ID_MSI_X: u8 = 0x11;
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MsiCapability32 {
	pub _pad0:           u16,
	pub message_control: u16,
	pub message_address: u32,
	pub message_data:    u16
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MsiCapability32Vectored {
	pub _pad0:           u16,
	pub message_control: MsiCapabilityMessageControl,
	pub message_address: u32,
	pub message_data:    u16,
	pub _pad1:           u16,
	pub mask_bits:       u32,
	pub pending_bits:    u32
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MsiCapability64 {
	pub _pad0:           u16,
	pub message_control: MsiCapabilityMessageControl,
	pub message_address: u64,
	pub message_data:    u16
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MsiCapability64Vectored {
	pub _pad0:           u16,
	pub message_control: MsiCapabilityMessageControl,
	pub message_address: u64,
	pub message_data:    u16,
	pub _pad1:           u16,
	pub mask_bits:       u32,
	pub pending_bits:    u32
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MsiCapabilityMessageControl(u16);

impl MsiCapabilityMessageControl {
	define_bits!(
		RW 0,    get_msi_enable, set_msi_enable;
		RO 1..3, get_multiple_message_capable, set_multiple_message_capable;
		RW 4..6, get_multiple_message_enable, set_multiple_message_enable;
		RO 7,    get_64bit_address_capable;
		RO 8,    get_per_vector_masking_capable;
	);
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MsiXCapability {
	pub _pad0:           u16,
	pub message_control: MsiXCapabilityMessageControl,
	pub table_offset:    u32,
	pub pba_offset:      u32
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MsiXCapabilityMessageControl(u16);

impl MsiXCapabilityMessageControl {
	define_bits!(
		RO 0..10, get_table_size;
		RW 14,    get_function_mask, set_function_mask;
		RW 15,    get_msi_x_enable, set_msi_x_enable;
	);
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MsiXTableEntry {
	pub message_address: u64,
	pub message_data:    u32,
	pub vector_control:  u32
}
