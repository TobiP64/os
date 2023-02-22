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

use super::Guid;

pub type LBA = u64;

pub const SIGNATURE: u64 = 0x5452415020494645;

pub const UNUSED_ENTRY_GUID:         Guid = 0x00000000000000000000000000000000;
pub const EFI_SYSTEM_PARTITION_GUID: Guid = 0xC12A7328F81F11D2BA4B00A0C93EC93B;
pub const LEGACY_MBR_PARTITION_GUID: Guid = 0x024DEE4133E711D39D690008C781F39F;

pub const PARTITION_ENTRY_REQUIRED:             u64 = 0x1;
pub const PARTITION_ENTRY_NO_BLOCK_IO_PROTOCOL: u64 = 0x2;
pub const PARTITION_ENTRY_LEGACY_BIOS_BOOTABLE: u64 = 0x3;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct Header {
	/// Identifies EFI-compatible partition table header.
	/// This value must contain the ASCII string “EFI
	/// PART”, encoded as the 64-bit constant
	/// 0x5452415020494645.
	pub signature:                   u64,
	/// The revision number for this header. This revision
	/// value is not related to the UEFI Specification
	/// version. This header is version 1.0, so the correct
	/// value is 0x00010000.
	pub revision:                    u32,
	/// ize in bytes of the GPT Header. The
	/// HeaderSize must be greater than or equal to
	/// 92 and must be less than or equal to the logical
	/// block size.
	pub header_size:                 u32,
	/// CRC32 checksum for the GPT Header structure.
	/// This value is computed by
	/// setting this field to 0, and computing the 32-bit
	/// CRC for HeaderSize bytes.
	pub header_crc32:                u32,
	/// Must be zero.
	pub _res0:                       u32,
	/// The LBA that contains this data structure.
	pub my_lba:                      u64,
	/// LBA address of the alternate GPT Header.
	pub alternate_lba:               LBA,
	/// The first usable logical block that may be used by a
	/// partition described by a GUID Partition Entry.
	pub first_usable_lba:            LBA,
	/// The last usable logical block that may be used by a
	/// partition described by a GUID Partition Entry.
	pub last_usable_lba:             LBA,
	/// GUID that can be used to uniquely identify the
	/// disk.
	pub disk_guid:                   Guid,
	/// The starting LBA of the GUID Partition Entry array.
	pub partition_entry_lba:         LBA,
	/// The number of Partition Entries in the GUID
	/// Partition Entry array.
	pub number_of_partition_entries: u32,
	/// The size, in bytes, of each the GUID Partition Entry
	/// structures in the GUID Partition Entry array. This
	/// field shall be set to a value of 128 x 2^n where n is
	/// an integer greater than or equal to zero (e.g., 128,
	/// 256, 512, etc.).
	/// NOTE: Previous versions of this specification
	/// allowed any multiple of 8..
	pub size_of_partition_entry:     u32,
	/// The CRC32 of the GUID Partition Entry array.
	/// Starts at PartitionEntryLBA and is
	/// computed over a byte length of
	/// number_of_partition_entries *
	/// size_of_partition_entry.
	pub partition_entry_array_crc32: u32
}

impl Header {
	pub fn is_valid(&self) -> bool {
		self.signature == SIGNATURE
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct PartitionEntry {
	/// Unique ID that defines the purpose and
	/// type of this Partition. A value of zero
	/// defines that this partition entry is not
	/// being used.
	pub partition_type_guid:   Guid,
	/// GUID that is unique for every partition
	/// entry. Every partition ever created will
	/// have a unique GUID. This GUID must
	/// be assigned when the GPT Partition
	/// Entry is created. The GPT Partition
	/// Entry is created whenever the
	/// NumberOfPartitionEntrie
	/// s in the GPT Header is increased to
	/// include a larger range of addresses.
	pub unique_partition_guid: Guid,
	/// Starting LBA of the partition defined by
	/// this entry.
	pub starting_lba:          LBA,
	/// Ending LBA of the partition defined by
	/// this entry.
	pub ending_lba:            LBA,
	/// Attribute bits, all bits reserved by UEFI
	pub attributes:            u64,
	/// Null-terminated string containing a
	/// human-readable name of the partition.
	pub partition_name:        [u8; 72]
}

impl core::fmt::Debug for PartitionEntry {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		let mut i = 0;
		while i < self.partition_name.len() && self.partition_name[i] != 0 { i += 1; }
		
		f.debug_struct("PartitionEntry")
			.field("partition_type_guid", &self.partition_type_guid)
			.field("unique_partition_guid", &self.unique_partition_guid)
			.field("starting_lba", &self.starting_lba)
			.field("ending_lba", &self.ending_lba)
			.field("attributes", &self.attributes)
			.field("partition_name", unsafe { &core::str::from_utf8_unchecked(&self.partition_name[..i]) })
			.finish()
	}
}