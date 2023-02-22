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

#![allow(unaligned_references)]

use core::{iter::IntoIterator, mem::size_of};
use super::*;

#[repr(transparent)]
pub struct AmlCode([u8]);

impl core::fmt::Debug for AmlCode {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("AmlCode").finish_non_exhaustive()
	}
}

pub trait TableTrait {
	fn validate<'a>(ptr: *const Self) -> Option<&'a Self>;
}

/// Root System Description Pointer
#[repr(C)]
pub struct RSDP {
	/// “RSD PTR ” (Notice that this signature must contain a trailing blank
	/// character.)
	signature:         [u8; 8],
	/// This is the checksum of the fields defined in the ACPI 1.0
	/// specification. This includes only the first 20 bytes of this table, bytes
	/// 0 to 19, including the checksum field. These bytes must sum to
	/// zero.
	checksum:          u8,
	/// An OEM-supplied string that identifies the OEM.
	oem_id:            [u8; 6],
	/// The revision of this structure. Larger revision numbers are backward
	/// compatible to lower revision numbers. The ACPI version 1.0
	/// revision number of this table is zero. The ACPI version 1.0 RSDP
	/// Structure only includes the first 20 bytes of this table, bytes 0 to 19.
	/// It does not include the Length field and beyond. The current value
	/// for this field is 2.
	revision:          u8,
	/// 32 bit physical address of the RSDT.
	rsdt:              Ptr32<RSDT>,
	/// The length of the table, in bytes, including the header, starting from
	/// offset 0. This field is used to record the size of the entire table. This
	/// field is not available in the ACPI version 1.0 RSDP Structure.
	length:            u32,
	/// 64 bit physical address of the XSDT.
	xsdt:              Ptr64<XSDT>,
	/// This is a checksum of the entire table, including both checksum fields.
	extended_checksum: u8,
	/// Reserved field
	reserved:          [u8; 3]
}

impl RSDP {
	pub fn get_rsdt(&self) -> Option<&RSDT> {
		unsafe { self.rsdt.as_ref() }
	}

	pub fn get_xsdt(&self) -> Option<Table> {
		if  self.revision < 2 {
			return None;
		}

		unsafe { (self.rsdt.as_ptr() as usize as *const DescHeader).as_ref().map(|h| h.into()) }
	}
}

impl super::Table for RSDP {
	const GUID: u128 = super::ACPI_20_TABLE_GUID;
}

impl core::fmt::Debug for RSDP {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let mut dbg = f.debug_struct("RSDP");

		dbg.field("signature", unsafe { &core::str::from_utf8_unchecked(&self.signature) })
			.field("checksum", &self.checksum)
			.field("oem_id", unsafe { &core::str::from_utf8_unchecked(&self.oem_id) })
			.field("revision", &self.revision)
			.field("rsdt", &self.get_rsdt());

		if self.revision < 2 {
			return dbg.finish();
		}

		dbg.field("xsdt", &self.get_xsdt())
			.field("extended_checksum", &self.extended_checksum)
			.finish()
	}
}

#[repr(C, packed)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GenericAddress {
	pub address_space: u8,
	pub bit_width:     u8,
	pub bit_offset:    u8,
	pub access_size:   u8,
	pub address:       u64
}

impl core::fmt::Debug for GenericAddress {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "{};{};{};{};{:#018X}", self.address_space, self.bit_width, self.bit_offset, self.access_size, { self.address })
	}
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum AddressSpace {
	SystemMemorySpace,
	SystemIoSpace,
	PciConfigurationSpace,
	EmbeddedController,
	SmBus,
	PlatformCommunicationsChannel = 0xA,
	FunctionalFixedHardware = 0x7F
}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct DescHeader {
	/// The ASCII string representation of the table identifier. Notice that if
	/// OSPM finds a signature in a table that is not listed in Table 5-30 ,
	/// OSPM ignores the entire table (it is not loaded into ACPI
	/// namespace); OSPM ignores the table even though the values in the
	/// Length and Checksum fields are correct.
	signature:         u32,
	/// The length of the table, in bytes, including the header, starting from
	/// offset 0. This field is used to record the size of the entire table.
	length:            u32,
	/// The revision of the structure corresponding to the signature field for
	/// this table. Larger revision numbers are backward compatible to
	/// lower revision numbers with the same signature.
	revision:          u8,
	/// The entire table, including the checksum field, must add to zero to
	/// be considered valid.
	checksum:          u8,
	/// An OEM-supplied string that identifies the OEM.
	oem_id:            [u8; 6],
	/// An OEM-supplied string that the OEM uses to identify the particular
	/// data table. This field is particularly useful when defining a definition
	/// block to distinguish definition block functions. The OEM assigns
	/// each dissimilar table a new OEM Table ID.
	oem_table_id:      [u8; 8],
	/// An OEM-supplied revision number. Larger numbers are assumed to
	/// be newer revisions.
	oem_revision:      u32,
	/// Vendor ID of utility that created the table. For tables containing
	/// Definition Blocks, this is the ID for the ASL Compiler.
	creator_id:        u32,
	/// Revision of utility that created the table. For tables containing
	/// Definition Blocks, this is the revision for the ASL Compiler.
	creator_revision:  u32
}

impl<'a> Into<Table<'a>> for &'a DescHeader {
	fn into(self) -> Table<'a> {
		use Table::*;
		let ptr = self as *const DescHeader;
		unsafe {
			match &self.signature.to_ne_bytes() {
				b"APIC" => Apic((ptr as *const MADT).as_ref().unwrap()),
				b"BERT" => Bert((ptr as *const BERT).as_ref().unwrap()),
				b"BGRT" => Bgrt((ptr as *const BGRT).as_ref().unwrap()),
				b"CPEP" => Cpep((ptr as *const CPEP).as_ref().unwrap()),
				b"ECDT" => Ecdt((ptr as *const ECDT).as_ref().unwrap()),
				b"FACP" => Fadt((ptr as *const FADT).as_ref().unwrap()),
				b"FACS" => Facs((ptr as *const FACS).as_ref().unwrap()),
				b"RSDT" => Rsdt((ptr as *const RSDT).as_ref().unwrap()),
				b"SBST" => Sbst((ptr as *const SBST).as_ref().unwrap()),
				b"SLIT" => Slit((ptr as *const SLIT).as_ref().unwrap()),
				b"SRAT" => Srat((ptr as *const SRAT).as_ref().unwrap()),
				b"XSDT" => Xsdt((ptr as *const XSDT).as_ref().unwrap()),
				b"HPET" => Hpet((ptr as *const HPET).as_ref().unwrap()),
				b"MCFG" => Mcfg((ptr as *const MCFG).as_ref().unwrap()),
				b"BOOT" => Boot,
				[b'O', b'E', b'M', _] => OemX(self),
				_       => Other(self)
			}
		}
	}
}

impl core::fmt::Debug for DescHeader {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("DescHeader")
			.field("signature", unsafe { &core::str::from_utf8_unchecked(&self.signature.to_le_bytes()) })
			.field("length", &self.length)
			.field("revision", &self.revision)
			.field("checksum", &self.checksum)
			.field("oem_id", unsafe { &core::str::from_utf8_unchecked(&self.oem_id) })
			.field("oem_table_id", unsafe { &core::str::from_utf8_unchecked(&self.oem_table_id) })
			.field("oem_revision", &self.oem_revision)
			.field("creator_id", &self.creator_id)
			.field("creator_revision", &self.creator_revision)
			.finish()
	}
}

#[derive(Debug, Copy, Clone)]
pub enum Table<'a> {
	Apic(&'a MADT),
	Bert(&'a BERT),
	Bgrt(&'a BGRT),
	Cpep(&'a CPEP),
	Dsdt(&'a DSDT),
	Ecdt(&'a ECDT),
	//Einj(&'a EINJ),
	//Erst(&'a ERST),
	Fadt(&'a FADT),
	Facs(&'a FACS),
	//Fpdt(&'a FPDT),
	//Gtdt(&'a GTDT),
	//Hest(&'a HEST),
	//Msct(&'a MSCT),
	//Mpst(&'a MPST),
	//Nfit(&'a NFIT),
	OemX(&'a DescHeader),
	//Pcct(&'a PCCT),
	//Pmtt(&'a PMTT),
	//Psdt(&'a PSDT),
	//Rasf(&'a RASF),
	Rsdt(&'a RSDT),
	Sbst(&'a SBST),
	//Sdev(&'a SDEV),
	Slit(&'a SLIT),
	Srat(&'a SRAT),
	//Ssdt(&'a SSDT),
	Xsdt(&'a XSDT),
	Boot,
	//Csrt(&'a CSRT),
	//Dbg2(&'a DBG2),
	//Dbgp(&'a DBGP),
	//Dmar(&'a DMAR),
	//Dppt(&'a DPPT),
	//Drtm(&'a DRTM),
	//Etdt(&'a ETDT),
	Hpet(&'a HPET),
	//Ibft(&'a IBFT),
	//Iort(&'a IORT),
	//Ivrs(&'a IVRS),
	//Lpit(&'a LPIT),
	Mcfg(&'a MCFG),
	//Mchi(&'a MCHI),
	//Msdm(&'a MSDM),
	//Sdei(&'a SDEI),
	//Slic(&'a SLIC),
	//Spcr(&'a SPCR),
	//Spmi(&'a SPMI),
	//Stao(&'a STAO),
	//Tcpa(&'a TCPA),
	//Tpm2(&'a TPM2),
	//Uefi(&'a Uefi),
	//Waet(&'a WAET),
	//Wdat(&'a WDAT),
	//Wdrt(&'a WDRT),
	//Wpbt(&'a WPBT),
	//Wsmt(&'a WSMT),
	//Xenv(&'a XENV),
	Other(&'a DescHeader)
}

#[repr(C)]
pub struct RSDT {
	pub header: DescHeader,
	entries:    [u32; 0x1000]
}

impl<'a> IntoIterator for &'a RSDT {
	type Item     = <RsdtEntryIter<'a> as Iterator>::Item;
	type IntoIter = RsdtEntryIter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		RsdtEntryIter(self.entries[..(self.header.length as usize
			- size_of::<DescHeader>()) / 4].iter())
	}
}

#[derive(Clone)]
pub struct RsdtEntryIter<'a>(<&'a [u32] as IntoIterator>::IntoIter);

impl<'a> Iterator for RsdtEntryIter<'a> {
	type Item = Table<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|ptr| unsafe { (*ptr as usize as *const DescHeader)
			.as_ref().unwrap().into() })
	}
}

impl core::fmt::Debug for RsdtEntryIter<'_> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_list().entries(self.clone()).finish()
	}
}

impl core::fmt::Debug for RSDT {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("RSDT")
			.field("header", &self.header)
			.field("entries", &self.into_iter())
			.finish()
	}
}

#[repr(C, packed)]
pub struct XSDT {
	pub header: DescHeader,
	entries:    [u64; 0x1000]
}

impl<'a> IntoIterator for &'a XSDT {
	type Item     = <XsdtEntryIter<'a> as Iterator>::Item;
	type IntoIter = XsdtEntryIter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		XsdtEntryIter(self.entries[..(self.header.length as usize
			- size_of::<DescHeader>()) / 8].iter())
	}
}

#[derive(Clone)]
pub struct XsdtEntryIter<'a>(<&'a [u64] as IntoIterator>::IntoIter);

impl<'a> Iterator for XsdtEntryIter<'a> {
	type Item = Table<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next().map(|ptr| unsafe { (*ptr as usize as *const DescHeader).as_ref().unwrap().into() })
	}
}

impl core::fmt::Debug for XsdtEntryIter<'_> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_list().entries(self.clone()).finish()
	}
}

impl core::fmt::Debug for XSDT {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("XSDT")
			.field("header", &{ self.header })
			.field("entries", &self.into_iter())
			.finish()
	}
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct FADT {
	pub header:               DescHeader,
	pub firmware_ctrl:        u32,
	pub dsdt:                 Ptr32<DSDT>,
	pub _res0:                u8,
	pub preferred_pm_profile: u8,
	pub sci_int:              u16,
	pub smi_cmd:              u32,
	pub acpi_enable:          u8,
	pub acpi_disable:         u8,
	pub s4bios_req:           u8,
	pub pstate_cnt:           u8,
	pub pm1a_evt_blk:         u32,
	pub pm1b_evt_blk:         u32,
	pub pm1a_cnt_blk:         u32,
	pub pm1b_cnt_blk:         u32,
	pub pm2_cnt_blk:          u32,
	pub pm_tmr_blk:           u32,
	pub cpe0_blk:             u32,
	pub cpe1_blk:             u32,
	pub pm1_evt_len:          u8,
	pub pm1_cnt_len:          u8,
	pub pm2_cnt_len:          u8,
	pub pm_tmr_lem:           u8,
	pub gpe0_blk_len:         u8,
	pub gpe1_blk_len:         u8,
	pub gpe1_base:            u8,
	pub cst_cnt:              u8,
	pub p_lvl2_lat:           u16,
	pub p_lvl3_lat:           u16,
	pub flush_size:           u16,
	pub flush_stride:         u16,
	pub duty_offset:          u8,
	pub duty_width:           u8,
	pub day_alrm:             u8,
	pub mon_alrm:             u8,
	pub century:              u8,
	pub iapc_boot_arch:       u16,
	pub _res1:                u8,
	pub flags:                u32,
	pub reset_reg:            GenericAddress,
	pub reset_value:          u8,
	pub arm_boot_arch:        u16,
	pub fadt_minor_version:   u8,
	pub x_firmware_ctrl:      u64,
	pub x_dsdt:               Ptr64<DSDT>,
	pub x_pm1a_evt_blk:       GenericAddress,
	pub x_pm1b_evt_blk:       GenericAddress,
	pub x_pm1a_cnt_blk:       GenericAddress,
	pub x_pm1b_cnt_blk:       GenericAddress,
	pub x_pm2_cnt_blk:        GenericAddress,
	pub x_pm_tmr_blk:         GenericAddress,
	pub x_gpe0_blk:           GenericAddress,
	pub x_gpe1_blk:           GenericAddress,
	pub sleep_control_reg:    GenericAddress,
	pub sleep_status_reg:     GenericAddress,
	pub hypervisor_vendor_id: u64
}

impl FADT {
	pub fn firmware_ctrl(&self) -> Option<&FACS> {
		unsafe {
			if self.firmware_ctrl != 0 {
				self.firmware_ctrl as usize as *const FACS
			} else if self.x_firmware_ctrl != 0 && self.header.revision >= 2 {
				self.x_firmware_ctrl as usize as *const FACS
			} else {
				return None
			}.as_ref()
		}
	}

	pub fn dsdt(&self) -> Option<&DSDT> {
		unsafe {
			if self.dsdt.0 != 0 {
				self.dsdt.as_ref()
			} else if self.x_dsdt.0 != 0 && self.header.revision >= 2 {
				self.x_dsdt.as_ref()
			} else {
				None
			}
		}
	}
}

#[repr(C)]
pub struct MADT {
	pub header:                          DescHeader,
	pub local_interrupt_controller_addr: Ptr32<()>,
	pub flags:                           u32,
	pub interrupt_controllers:           [u8; 0x1000]
}

impl<'a> IntoIterator for &'a MADT {
	type Item     = <MadtEntryIter<'a> as Iterator>::Item;
	type IntoIter = MadtEntryIter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		MadtEntryIter(&self.interrupt_controllers[..(self.header.length as usize
			- size_of::<DescHeader>() - 8)])
	}
}

impl core::fmt::Debug for MADT {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("MADT")
			.field("header", &self.header)
			.field("local_interrupt_controller_addr", &self.local_interrupt_controller_addr)
			.field("flags", &DbgHex(self.flags))
			.field("interrupt_controllers", &self.into_iter())
			.finish()
	}
}

#[derive(Clone)]
pub struct MadtEntryIter<'a>(&'a [u8]);

impl<'a> Iterator for MadtEntryIter<'a> {
	type Item = &'a MadtInterruptController;

	fn next(&mut self) -> Option<Self::Item> {
		if self.0.is_empty() {
			return None;
		}

		let v = unsafe { (self.0.as_ptr() as *const MadtInterruptController).as_ref() }.unwrap();
		self.0 = &self.0[unsafe { *self.0.as_ptr().add(1) } as usize..];
		Some(v)
	}
}

impl core::fmt::Debug for MadtEntryIter<'_> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_list().entries(self.clone()).finish()
	}
}

#[repr(C, packed)]
#[derive(Copy, Clone, Default, Debug)]
pub struct MadtProcessorLocalApic {
	pub length:                    u8,
	pub acpi_processor_uid:        u8,
	pub apic_id:                   u8,
	pub flags:                     u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Default, Debug)]
pub struct MadtIoApic {
	pub length:                    u8,
	pub io_apic_id:                u8,
	pub _res0:                     u8,
	pub io_apic_addr:              Ptr32<()>,
	pub global_system_int_base:    u32
}

#[repr(C, packed)]
#[derive(Copy, Clone, Default, Debug)]
pub struct MadtInterruptSourceOverride {
	pub length:                    u8,
	pub bus:                       u8,
	pub source:                    u8,
	pub global_system_int:         u32,
	pub flags:                     u16
}

#[repr(C, packed)]
#[derive(Copy, Clone, Default, Debug)]
pub struct MadtNonMaskableInterruptSource {
	pub length:                    u8,
	pub flags:                     u16,
	pub global_system_int:         u32,
}

#[repr(C, packed)]
#[derive(Copy, Clone, Default, Debug)]
pub struct MadtLocalApicNmi {
	pub length:                    u8,
	pub acpi_processor_uid:        u8,
	pub flags:                     u16,
	pub local_apic_lint:           u8
}

#[repr(C, packed)]
#[derive(Copy, Clone, Default, Debug)]
pub struct MadtLocalApicAddressOverride {
	pub length:                    u8,
	pub _res0:                     u16,
	pub local_apic_addr:           u64
}

#[repr(C, packed)]
#[derive(Copy, Clone, Default, Debug)]
pub struct MadtIoSapic {
	pub length:                    u8,
	pub io_apic_id:                u8,
	pub _res0:                     u8,
	pub global_system_int_base:    u32,
	pub io_sapic_addr:             u64
}

#[repr(C, packed)]
pub struct MadtLocalSapic {
	pub length:                    u8,
	pub acpi_processor_id:         u8,
	pub local_sapic_id:            u8,
	pub local_sapic_sid:           u8,
	pub _res0:                     [u8; 3],
	pub flags:                     u32,
	pub acpi_processor_uid_val:    u32,
	pub acpi_processor_uid_str:    [u8; 0x1000]
}

impl core::fmt::Debug for MadtLocalSapic {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let mut i = 0;
		while self.acpi_processor_uid_str[i] != 0 { i += 1; }

		f.debug_struct("MadtLocalSapic")
			.field("length", &self.length)
			.field("acpi_processor_id", &self.acpi_processor_id)
			.field("local_sapic_id", &self.local_sapic_id)
			.field("local_sapic_sid", &self.local_sapic_sid)
			.field("flags", &{ self.flags })
			.field("acpi_processor_uid_val", &{ self.acpi_processor_uid_val })
			.field("acpi_processor_uid_str", unsafe { &core::str::from_utf8_unchecked(&self.acpi_processor_uid_str[..i]) })
			.finish()
	}
}

#[repr(C, packed)]
#[derive(Copy, Clone, Default, Debug)]
pub struct MadtPlatformInterruptSource {
	pub length:                    u8,
	pub flags:                     u16,
	pub interrupt_type:            u8,
	pub processor_id:              u8,
	pub processor_eid:             u8,
	pub io_sapic_vector:           u8,
	pub global_system_int:         u8,
	pub platform_int_src_flags:    u32
}

#[repr(C, packed)]
#[derive(Copy, Clone, Default, Debug)]
pub struct MadtProcessorLocalX2Apic {
	pub length:                    u8,
	pub _res0:                     u16,
	pub x2_apic:                   u32,
	pub flags:                     u32,
	pub acpi_processor_uid:        u32
}

#[repr(C, packed)]
#[derive(Copy, Clone, Default, Debug)]
pub struct MadtLocalX2ApicNmi  {
	pub length:                    u8,
	pub flags:                     u16,
	pub acpi_processor_uid:        u32,
	pub local_x2_apic_lint:        u8,
	pub _res0:                     [u8; 3]
}

#[repr(C, packed)]
#[derive(Copy, Clone, Default, Debug)]
pub struct MadtGicCpuInterface {
	pub length:                    u8,
	pub _res0:                     u16,
	pub cpu_interface_number:      u32,
	pub acpi_processor_uid:        u32,
	pub flags:                     u32,
	pub parking_protocol_version:  u32,
	pub performance_int_gisv:      u32,
	pub parked_addr:               u64,
	pub physical_base_addr:        u64,
	pub gicv:                      u64,
	pub gich:                      u64,
	pub vgic_maintenance_int:      u32,
	pub gicr_base_addr:            u64,
	pub mpidr:                     u64,
	pub processor_power_efficiency_class: u8,
	pub _res1:                     [u8; 3]
}

#[repr(C, packed)]
#[derive(Copy, Clone, Default, Debug)]
pub struct MadtGicDistributorInterface {
	pub length:                    u8,
	pub _res0:                     u16,
	pub gic_id:                    u32,
	pub physical_base_addr:        u64,
	pub system_vector_base:        u32,
	pub gic_version:               u8,
	pub _res1:                     [u8; 3]
}

#[repr(C, packed)]
#[derive(Copy, Clone, Default, Debug)]
pub struct MadtGicMsiFrame {
	pub length:                    u8,
	pub _res0:                     u16,
	pub gic_msi_frame_id:          u32,
	pub physical_base_addr:        u64,
	pub flags:                     u32,
	pub spi_count:                 u16,
	pub spi_base:                  u16
}

#[repr(C, packed)]
#[derive(Copy, Clone, Default, Debug)]
pub struct MadtGicRedistributor {
	pub length:                    u8,
	pub _res0:                     u16,
	pub discovery_range_base_addr: u64,
	pub discovery_range_length:    u32
}

#[repr(C, packed)]
#[derive(Copy, Clone, Default, Debug)]
pub struct MadtGicInterruptTranslationService {
	pub length:                    u8,
	pub _res0:                     u16,
	pub gic_its_id:                u32,
	pub physical_base_addr:        u64,
	pub _res1:                     u32
}

#[repr(u8)]
#[derive(Debug)]
pub enum MadtInterruptController {
	ProcessorLocalApic(MadtProcessorLocalApic),
	IoApic(MadtIoApic),
	InterruptSourceOverride(MadtInterruptSourceOverride),
	NonMaskableInterruptSource(MadtNonMaskableInterruptSource),
	LocalApicNmi(MadtLocalApicNmi),
	LocalApicAddressOverride(MadtLocalApicAddressOverride),
	IoSapic(MadtIoSapic),
	LocalSapic(MadtLocalSapic),
	PlatformInterruptSource(MadtPlatformInterruptSource),
	ProcessorLocalX2Apic(MadtProcessorLocalX2Apic),
	LocalX2ApicNmi(MadtLocalX2ApicNmi),
	GicCpuInterface(MadtGicCpuInterface),
	GicDistributorInterface(MadtGicDistributorInterface),
	GicMsiFrame(MadtGicMsiFrame),
	GicRedistributor(MadtGicRedistributor),
	GicInterruptTranslationService(MadtGicInterruptTranslationService),
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct BERT {
	pub header:                   DescHeader,
	pub boot_error_region_length: u32,
	pub boot_error_region:        Ptr64<()>
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct BGRT {
	pub header:         DescHeader,
	pub version:        u16,
	pub status:         u8,
	pub image_type:     u8,
	pub image_address:  Ptr64<()>,
	pub image_offset_x: u32,
	pub image_offset_y: u32
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct SBST {
	pub header:                DescHeader,
	pub warning_energy_level:  u32,
	pub low_energy_level:      u32,
	pub critical_energy_level: u32
}

#[repr(C)]
pub struct ECDT {
	pub header:     DescHeader,
	pub ec_control: GenericAddress,
	pub ec_data:    GenericAddress,
	pub uid:        u32,
	pub gpe_bit:    u8,
	pub ec_id:      [u8; 0x1000]
}

impl ECDT {
	pub fn get_ec_id(&self) -> &str {
		let mut i = 0;
		while self.ec_id[i] != 0 { i += 1; }
		unsafe { core::str::from_utf8_unchecked(&self.ec_id[..i]) }
	}
}

impl core::fmt::Debug for ECDT {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("ECDT")
			.field("header", &self.header)
			.field("ec_control", &self.ec_control)
			.field("ec_data", &self.ec_data)
			.field("uid", &self.uid)
			.field("gpe_bit", &self.gpe_bit)
			.field("ec_id", &self.get_ec_id())
			.finish()
	}
}

#[repr(C)]
pub struct SRAT {
	pub header:    DescHeader,
	_res0:         u32,
	_res1:         u64,
	static_resource_allocations: [StaticResourceAllocation; 0x1000]
}

impl core::fmt::Debug for SRAT {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("SRAT")
			.field("header", &self.header)
			.finish()
	}
}

pub enum StaticResourceAllocation {
	ProcessorLocalApicSapicAffinity {

	},
	MemoryAffinity {

	},
	ProcessorLocalX2ApicAffinity {

	},
	GiccAffinity {

	},
	GicInterruptTranslationServiceAffinity {

	}
}

#[repr(C)]
pub struct SLIT {
	pub header:  DescHeader,
	pub entries: [u8; 0x1000_0000]
}

impl core::fmt::Debug for SLIT {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("SLIT").field("header", &self.header).finish()
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct CPEP {
	pub header:  DescHeader,
	reserved:    u64,
	entries:     [CpepProcessorStructure; 0x1000]
}

impl CPEP {
	pub fn processor_structures(&self) -> &[CpepProcessorStructure] {
		&self.entries[..(self.header.length as usize - size_of::<DescHeader>() - 8)
			/ size_of::<CpepProcessorStructure>()]
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct CpepProcessorStructure {
	pub r#type:           u8,
	pub length:           u8,
	pub processor_id:     u8,
	pub processor_eid:    u8,
	pub polling_interval: u32
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct FACS {
	pub header:  DescHeader,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct DSDT {
	pub header:       DescHeader
}

impl DSDT {
	pub fn definition_block(&self) -> &AmlCode {
		unsafe  {
			(core::slice::from_raw_parts(
				(self as *const Self).add(1) as *const u8,
				self.header.length as usize - size_of::<DescHeader>()
			) as *const [u8] as *const AmlCode).as_ref().unwrap()
		}
	}
}

#[repr(C, packed)]
#[derive(Copy, Clone, Debug)]
pub struct HPET {
	pub header:               DescHeader,
	pub event_timer_block_id: u32,
	pub base_address:         GenericAddress,
	pub hpet_number:          u8,
	pub main_counter_minimum: u16,
	pub page_protecton_attr:  u8
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MCFG {
	pub header:  DescHeader,
	reserved:    u64,
	entries:     [McfgEntry; 0x1000]
}

impl<'a> IntoIterator for &'a MCFG {
	type Item     = <McfgEntryIter<'a> as Iterator>::Item;
	type IntoIter = McfgEntryIter<'a>;

	fn into_iter(self) -> Self::IntoIter {
		McfgEntryIter(self.entries[..(self.header.length as usize
			- size_of::<DescHeader>() - 8) / size_of::<McfgEntry>()].iter())
	}
}

impl core::fmt::Debug for MCFG {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("MCFG")
			.field("header", &self.header)
			.field("entries", &self.into_iter())
			.finish()
	}
}

#[derive(Clone)]
pub struct McfgEntryIter<'a>(<&'a [McfgEntry] as IntoIterator>::IntoIter);

impl<'a> Iterator for McfgEntryIter<'a> {
	type Item = &'a McfgEntry;

	fn next(&mut self) -> Option<Self::Item> {
		self.0.next()
	}
}

impl core::fmt::Debug for McfgEntryIter<'_> {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_list().entries(self.clone()).finish()
	}
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct McfgEntry {
	pub address:          Ptr64<()>,
	pub pci_segment:      u16,
	pub start_bus_number: u8,
	pub end_bus_number:   u8,
	reserved:             u32,
}