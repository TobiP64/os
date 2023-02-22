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

use crate::{println, print};
use core::ptr::{null_mut, null};

mod arch;
mod writer;

pub use writer::*;

#[no_mangle]
pub extern fn efi_main(image: hw::uefi::Handle, system_table: &hw::uefi::SystemTable) -> hw::uefi::Status {
    let mut framebuffer = None;
    let mut key = 0;

    let framebuffer = 'gop: {
        system_table.console_out.reset(false);

        let (mut buffer_size, mut buffer) = (64, [null_mut(); 8]);
        let status = system_table.boot_services.locate_handle(
            hw::uefi::LocateSearchType::ByProtocol,
			hw::uefi::GRAPHICS_OUTPUT_PROTOCOL_GUID,
			null_mut(),
			&mut buffer_size,
			&mut buffer[0]
		)?;

        if status != hw::uefi::STATUS_SUCCESS {
            println!("failed: system_table.boot_services.locate_handle returned status {}", status);
            break 'gop None;
        }

        let mut interface = null_mut();
        let status = system_table.boot_services.handle_protocol(buffer[0],  hw::uefi::GRAPHICS_OUTPUT_PROTOCOL_GUID, &mut interface)?;

        if status != hw::uefi::STATUS_SUCCESS {
            println!("failed: system_table.boot_services.handle_protocol returned status {}", status);
            break 'gop None;
        }

        let gop = unsafe { &*(interface as *const hw::uefi::GraphicsOutputProtocol) };
        let mut mode = null();
        let mut size = 0;
        let mut mode_number = gop.mode.mode;
        let mut mode_width  = gop.mode.info.horizontal_resolution;
        let mut mode_height = gop.mode.info.vertical_resolution;

        for i in 0..gop.mode.max_mode {
            gop.query_mode(i, &mut size, &mut mode);

            if status != hw::uefi::STATUS_SUCCESS {
                continue;
            }

            let mode = unsafe { &*mode };

            if mode.horizontal_resolution > mode_width || mode.vertical_resolution > mode_height {
                mode_number = i;
                mode_width  = mode.horizontal_resolution;
                mode_height = mode.vertical_resolution;
            }
        }

		gop.set_mode(mode_number);

        let fb = gop.mode.frame_buffer();
        let writer = GopConsoleWriter {
            info:        *gop.mode,
			framebuffer: unsafe { core::slice::from_raw_parts_mut(fb.as_mut_ptr() as *mut u32, fb.len() / 4) },
			x:           0,
			y:           0,
			c:           0xFFFFFFFF
        };

        fb.fill(0);
        unsafe { GOP_WRITER = Some(writer) };
        crate::set_out(GopConsoleWriter::static_write);

        println!("[BOOT/S1:ENV] GOP mode: #{} {}x{} ({:?})\n", gop.mode.mode, gop.mode.info.horizontal_resolution,
			 gop.mode.info.vertical_resolution, gop.mode.info.pixel_format);

        Some(crate::GenericFramebuffer {
            width:    gop.mode.info.horizontal_resolution,
            height:   gop.mode.info.vertical_resolution,
            format:   gop.mode.info.pixel_format as _,
            scanline: gop.mode.info.pixels_per_scan_line,
            ptr:      fb,
        })
    };

    print!("Exiting UEFI                                ... ");

    let mut size               = 0x1000;
    let mut map_key            = 0;
    let mut descriptor_size    = 0;
    let mut descriptor_version = 0;
    let mut buf                = [0u8; 0x1000];

    let status = system_table.boot_services.get_memory_map(
            &mut size, buf.as_ptr(), &mut map_key, &mut descriptor_size, &mut descriptor_version);

    if status != hw::uefi::STATUS_SUCCESS {
        println!("failed: system_table.boot_services.get_memory_map returned status {}", status);
        hw::arch::park();
    }

	let status = system_table.boot_services.exit_boot_services(image, map_key);

    if status != hw::uefi::STATUS_SUCCESS {
        println!("failed: system_table.boot_services.exit_boot_services returned status {}", status);
        hw::arch::park();
    }

	println!("done");

    let memory_map = hw::uefi::MemoryMap::new(buf, size, descriptor_size);

    arch::init(system_table, framebuffer, memory_map);


    pub struct SvData {
        ctx:       *mut ctx::Context, // sorted by cid
    	ctx_lru:   *mut ctx::Context,
        mnt:       TrieNode<mnt::Node>,
        harts:     Tree<hart::Hart, sort_hart_by_load>, // sorted by load
    	mem_nodes: Tree<mem::NodeDescriptor, sort_node_by_address>, // sorted by start_page
    	mem_areas: &'static mut [mem::PhysMemoryArea],
        mem_table: *mut [u64; 512],
        log_buf:   log::LogBuf
    }

}

fn init_mem_map(system_table: &hw::uefi::SystemTable, key: &mut usize, mem_map: &mut [PhysMemoryArea; 0x100]) {
    let mut i = 0;
    let mut buf = [0u8; 0x1000];
    let (status, map_key, iter) = match system_table.boot_services.get_memory_map(&mut buf) {
		Some(v) => v,
		None    => {
            println!("failed");
            hw::arch::park();
        }
    };

    for desc in iter {
        let ty = match desc.r#type {
            hw::uefi::MemoryType::ReservedMemoryType      => Reserved,
			hw::uefi::MemoryType::LoaderCode              => Usable,
			hw::uefi::MemoryType::LoaderData              => Usable,
			hw::uefi::MemoryType::BootServicesCode        => Usable,
			hw::uefi::MemoryType::BootServicesData        => Usable,
			hw::uefi::MemoryType::RuntimeServicesCode     => Firmware,
			hw::uefi::MemoryType::RuntimeServicesData     => Firmware,
			hw::uefi::MemoryType::ConventionalMemory      => Usable,
			hw::uefi::MemoryType::UnusableMemory          => Reserved,
			hw::uefi::MemoryType::AcpiReclaimMemory       => Firmware,
			hw::uefi::MemoryType::AcpiMemoryNvs           => Firmware,
			hw::uefi::MemoryType::MemoryMappedIo          => MMIO,
			hw::uefi::MemoryType::MemoryMappedIoPortSpace => return,
			hw::uefi::MemoryType::PalCode                 => Firmware,
			hw::uefi::MemoryType::PersistentMemory        => Usable
        };

        if i > 0 && mem_map[i - 1].r#type == ty && mem_map[i - 1].offset
			+ (mem_map[i - 1].length << 12) == desc.physical_start as usize {
            mem_map[i - 1].length += desc.number_of_pages as usize;
            return;
        }

		mem_map[i] = PhysMemoryArea {
            offset: desc.physical_start as _,
			length: desc.number_of_pages as _,
			r#type: ty,
			attrs:  0
        };
        i += 1;
    }

	let mem_map = &mut mem_map[..i];

    for i in 0..mem_map.len() - 1 {
        for j in 0..mem_map.len() - i - 1 {
            if mem_map[j].offset > mem_map[j + 1].offset {
                let tmp = mem_map[j + 1];
                mem_map[j + 1] = mem_map[j];
                mem_map[j] = tmp;
            }
        }
    }

	*key = map_key;
}