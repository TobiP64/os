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

#[repr(C)]
pub struct Nvme {
	pub controller_capabilities:                                  u64,
	pub version:                                                  u32,
	pub interrupt_mask_set:                                       u32,
	pub interrupt_mask_clear:                                     u32,
	pub controller_config:                                        u32,
	pub _res0:                                                    u32,
	pub controller_status:                                        u32,
	pub nvm_subsystem_reset:                                      u32,
	pub admin_queue_attributes:                                   u32,
	pub admin_submission_queue:                                   Ptr64<u8>,
	pub admin_completion_queue:                                   Ptr64<u8>,
	pub controller_memory_buffer_location:                        u32,
	pub controller_memory_buffer_size:                            u32,
	pub boot_partition_information:                               u32,
	pub boot_partition_read_select:                               u32,
	pub boot_partition_memory_buffer_location:                    Ptr64<u8>,
	pub controller_memory_buffer_status:                          u32,
	pub _res1:                                                    [u32; 0x369],
	pub persistent_memory_capabilities:                           u32,
	pub persistent_memory_region_control:                         u32,
	pub persistent_memory_region_status:                          u32,
	pub persistent_memory_region_elasticity_buffer_size:          u32,
	pub persistent_memory_region_sustained_write_throughput:      u32,
	pub persistent_memory_region_controller_memory_space_control: u32,
	pub _res2:                                                    [u32; 0x1E4],
	pub doorbells_base:                                           [u32; 0]
}