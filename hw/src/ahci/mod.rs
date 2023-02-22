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

use crate::Ptr64;

/// Supports 64-bit Addressing
pub const CAP_S64A:  u32 = 1 << 31;
/// Supports Native Command Queuing
pub const CAP_SNCQ:  u32 = 1 << 30;
/// Supports SNotification Register
pub const CAP_SSNTF: u32 = 1 << 29;
/// Supports Mechanical Presence Switch
pub const CAP_SMPS:  u32 = 1 << 28;
/// Supports Staggered Spin-up
pub const CAP_SSS:   u32 = 1 << 27;
/// Supports Aggressive Link Power Management
pub const CAP_SALP:  u32 = 1 << 26;
/// Supports Activity LED
pub const CAP_SAL:   u32 = 1 << 25;
/// Supports Command List Override
pub const CAP_SCLO:  u32 = 1 << 24;
/// Interface Speed Support
pub const CAP_ISS_MASK: u32 = 0b1111 << 18;
/// Gen 1 (1.5 Gbps)
pub const CAP_ISS_GEN1: u32 = 0b0001 << 18;
/// Gen 2 (3 Gbps)
pub const CAP_ISS_GEN2: u32 = 0b0010 << 18;
/// Gen 3 (6 Gbps)
pub const CAP_ISS_GEN3: u32 = 0b0011 << 18;
/// Supports AHCI mode only
pub const CAP_SAM:      u32 = 1 << 18;
/// Supports Port Multiplier
pub const CAP_SPM:      u32 = 1 << 17;
/// FIS-based Switching Supported
pub const CAP_FBSS:     u32 = 1 << 16;
/// PIO Multiple DRQ Block
pub const CAP_PMD:      u32 = 1 << 15;
/// Slumber State Capable
pub const CAP_SSC:      u32 = 1 << 14;
/// Partial State Capable
pub const CAP_PSC:      u32 = 1 << 13;
/// Number of Command Slots
pub const CAP_NCS_MASK:  u32 = 0b1111 << CAP_NCS_SHIFT;
pub const CAP_NCS_SHIFT: u32 = 8;
/// Command Completion Coalescing Supported
pub const CAP_CCCS:     u32 = 1 << 7;
/// Enclosure Management Supported
pub const CAP_EMS:      u32 = 1 << 6;
/// Supports External SATA
pub const CAP_SXS:      u32 = 1 << 5;
/// Number of Ports
pub const CAP_NP_MASK:  u32 = 0b1111;

pub const PRD_INFO_INT_ON_COMPLETION: u32 = 1 << 31;
pub const PRD_INFO_DBC_MASK:          u32 = !(!0 << 22);

#[repr(C)]
pub struct Hba {
	pub generic_gost_control:     HbaGenericHostControl,
	pub _res0:                    [u32; 0x1D],
	pub vendor_specific_regisers: [u32; 0x18],
	pub ports:                    [HbaPort; 32]
}

#[repr(C)]
pub struct HbaGenericHostControl {
	pub host_capabilities:   u32,
	pub global_host_control: u32,
	pub interrupt_status:    u32,
	pub ports_implemented:   u32,
	pub version:             u32,
	pub ccc_control:         u32,
	pub ccc_ports:           u32,
	pub em_location:         u32,
	pub em_control:          u32,
	pub host_capabilities_2: u32,
	pub bohc:                u32
}

#[repr(C)]
pub struct HbaPort {
	pub command_list_bar: Ptr64<[AhciCommandHeader; 32]>,
	pub fis_bar:          Ptr64<u8>,
	pub interrupt_status: u32,
	pub interrupt_enable: u32,
	pub command_status:   u32,
	pub _res0:            u32,
	pub task_file_data:   u32,
	pub signature:        u32,
	pub sata_status:      u32,
	pub sata_control:     u32,
	pub sata_error:       u32
}

#[repr(C)]
pub struct AhciCommandHeader {
	pub prd_table_length:  u16,
	pub flags:             u16,
	pub prs_byte_count:    u32,
	pub ctd_base_address:  u32,
	pub ctd_base_address2: u32,
	pub _res0:             [u32; 4]
}

#[repr(C)]
pub struct AhciCommandTable {
	pub command_fis:   [u32; 16],
	pub atapi_command: [u32; 4],
	pub _res0:         [u32; 12],
	pub prd_table:     [AhciPrd; 65535]
}

#[repr(C)]
pub struct AhciPrd {
	pub data_base_addr0: u32,
	pub data_base_addr1: u32,
	pub _res0:           u32,
	pub descriptor_info: u32
}