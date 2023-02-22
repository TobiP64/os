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

use crate::{println, print, dri::{arch::*, uefi::*}, mem::{PhysMemoryArea, MemoryType::*}};
use core::ptr::{null_mut, null};

static mut GOP_WRITE: Option<UefiGopFrameBuffer> = None;

#[no_mangle]
pub extern fn efi_main(image: Handle, system_table: &SystemTable) -> Status {
	let out = unsafe { (system_table.console_out as *const SimpleTextOutputProtocol as *mut SimpleTextOutputProtocol).as_mut() }.unwrap();
	unsafe { crate::std::set_out(out); }
	
	println!("\nBooting ...");
	
	system_table.console_in.reset(false);
	
	let (mut buffer_size, mut buffer) = (64, [null_mut(); 8]);
	let status = system_table.boot_services.locate_handle(
		LocateSearchType::ByProtocol, GRAPHICS_OUTPUT_PROTOCOL_GUID, null_mut(), &mut buffer_size, &mut buffer[0]);
	
	if status != STATUS_SUCCESS || buffer_size == 0 {
		println!("\nFailed to locate GOP handle (status: {}, buffer size: {})", status, buffer_size);
		crate::arch::park();
	}
	
	let mut interface = null_mut();
	let status = system_table.boot_services.handle_protocol(buffer[0], GRAPHICS_OUTPUT_PROTOCOL_GUID, &mut interface);
	
	if status != STATUS_SUCCESS {
		println!("\nFailed to handle GOP (status: {})", status);
		crate::arch::park();
	}
	
	let gop = unsafe { (interface as *const GraphicsOutputProtocol).as_ref() }.unwrap();
	let mut mode = null();
	let mut size = 0;
	let mut mode_number = 0;
	let mut mode_width = gop.mode.info.horizontal_resolution;
	let mut mode_height = gop.mode.info.vertical_resolution;
	
	for i in 0..gop.mode.max_mode {
		gop.query_mode(i, &mut size, &mut mode);
		let mode = unsafe { mode.as_ref() }.unwrap();
		
		if mode.horizontal_resolution > mode_width || mode.vertical_resolution > mode_height {
			mode_number = i;
			mode_width  = mode.horizontal_resolution;
			mode_height = mode.vertical_resolution;
		}
	}
	
	gop.set_mode(mode_number);
	unsafe {
		let fb = gop.mode.frame_buffer();
		GOP_WRITE = Some(UefiGopFrameBuffer::new(*gop.mode.info, core::slice::from_raw_parts_mut(
			fb.as_mut_ptr() as *mut u32, fb.len() / 4)));
		crate::std::set_out(GOP_WRITE.as_mut().unwrap());
	}
	
	println!("GOP mode: #{} {}x{} ({:?}) @ {:?}\n", gop.mode.mode, gop.mode.info.horizontal_resolution,
			 gop.mode.info.vertical_resolution, gop.mode.info.pixel_format, gop.mode.frame_buffer_base);
	print!("Discovering system memory                   ... ");
	
	let mut i = 0;
	let mut mem_map = [0u8; 0x1000];
	let (status, map_key, iter) = system_table.boot_services.get_memory_map(&mut mem_map).unwrap();
	let mut mem_map = [PhysMemoryArea::default(); 0x100];
	
	iter.for_each(|desc| {
		let ty = match desc.r#type {
			MemoryType::ReservedMemoryType      => Reserved,
			MemoryType::LoaderCode              => Usable,
			MemoryType::LoaderData              => Usable,
			MemoryType::BootServicesCode        => Usable,
			MemoryType::BootServicesData        => Usable,
			MemoryType::RuntimeServicesCode     => Firmware,
			MemoryType::RuntimeServicesData     => Firmware,
			MemoryType::ConventionalMemory      => Usable,
			MemoryType::UnusableMemory          => Reserved,
			MemoryType::AcpiReclaimMemory       => Firmware,
			MemoryType::AcpiMemoryNvs           => Firmware,
			MemoryType::MemoryMappedIo          => MMIO,
			MemoryType::MemoryMappedIoPortSpace => return,
			MemoryType::PalCode                 => Firmware,
			MemoryType::PersistentMemory        => Usable
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
	});
	
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
	
	print!("done\nExiting boot services                       ... ");
	system_table.boot_services.exit_boot_services(image, map_key);
	
	println!("done\n\nSYSTEM MEMORY MAP:\n");
	mem_map.iter().for_each(|a| println!("{}", a));
	
	// TODO enable paging
	
	// disable caches, enable some protection things and instructions
	CR0.set(CR0.get() | CR0::WP | CR0::CD);
	CR4.set(CR4.get() | CR4::FSGSBASE | CR4::PGE | CR4::SMEP | CR4::SMAP);
	RFLAGS.set(RFLAGS.get() | RFLAGS::AC);
	
	crate::arch::park()
}

struct UefiGopFrameBuffer {
	info:        GraphicsOutputModeInformation,
	framebuffer: &'static mut [u32],
	x:           u32,
	y:           u32
}

impl UefiGopFrameBuffer {
	pub fn new(info: GraphicsOutputModeInformation, framebuffer: &'static mut [u32]) -> Self {
		Self { info, framebuffer, x: 0, y: 0 }
	}
	
	pub fn draw_glyph(&mut self, (x, y): (u32, u32), ch: char) {
		let ch = match ch {
			ch @ '!'..='~' => ch as usize - 0x21,
			_ => 0x5E
		};
		for i in 0..0x10 {
			for j in 0..0x20 {
				let v = crate::dri::font::FONT[ch][i][j] as u32;
				unsafe { self.draw((x + i as u32, y + j as u32), 0xFF | v << 8 | v << 16 | v << 24) }
			}
		}
	}
	
	#[inline]
	pub unsafe fn draw(&mut self, (x, y): (u32, u32), color: u32) {
		let color = color.to_be();
		self.framebuffer[(y * self.info.horizontal_resolution + x) as usize] = match self.info.pixel_format {
			GraphicsPixelFormat::R8G8B8X8 => color,
			GraphicsPixelFormat::B8G8R8X8 => color & 0x0000FF00 << 16 | color & 0x00FF00FF | color & 0xFF000000 >> 16,
			_ => return
		}
	}
}

impl core::fmt::Write for UefiGopFrameBuffer {
	fn write_str(&mut self, s: &str) -> core::fmt::Result {
		s.chars().map(|c| self.write_char(c)).collect()
	}
	
	fn write_char(&mut self, c: char) -> core::fmt::Result {
		if self.y + 0x20 >= self.info.vertical_resolution {
			self.y = 0;
			self.framebuffer.fill(0);
		}
		
		match c {
			' ' => (),
			'\t' => {
				self.x += 48;
				return Ok(());
			},
			'\n' => {
				self.x = 0;
				self.y += 32;
				return Ok(());
			},
			ch => self.draw_glyph((self.x, self.y), ch)
		}
		self.x += 16;
		Ok(())
	}
}