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

use crate::{
	*,
	arch::*,
	mem::MemoryType::Usable,
	utils::{get_mem_unit, NoDbg},
	spi::SYSCALL_TABLE
};
use core::{mem::size_of, sync::atomic::{AtomicUsize, AtomicU32}};
use crate::sched::handle_timer_interrupt;
use hw::device_tree::*;
use hw::uart_ns16550a::Ns16550a;

//#[link_section = ".init"]
//static INIT_RAM_FS: &[u8] = include_bytes!("../target/initramfs.img");

#[no_mangle]
pub extern fn _start(hartid: usize, fdt: &FdtHeader) -> ! {
	if hartid != 0 {
		park();													// park non-boot cores
	}
	
	unsafe {
		stvec.write_ptr(park as *const ());						// setup interrupt handler
		sp.write(0x100000000);									// set stack pointer
		gp.write_ptr(&_global_pointer);							// set global pointer
		
		let uart = (0x1000_0000 as *mut Ns16550a).as_mut().unwrap();
		uart.init();
		crate::std::set_out(uart);
	}
	
	println!("\nBooting ...");
	_init(hartid, fdt)
}

#[no_mangle]
fn _init(hartid: usize, fdt: &FdtHeader) -> ! {
	stvec.write_ptr(exceptions::handle_early_exception as *const ());
	
	print!("\nDiscovering system memory               ... ");
	
	let harts = fdt.get(&["", "cpus", "cpu"]).count();
	let _reserved = fdt.memory_reservation_block_slice();
	let mut i = 1;
	let mut mem_map = [PhysMemoryArea::default(); 0x100];
	
	if let Some(FdtStructureToken::Prop { name: _, value }) = fdt.get(&["", "memory", "reg"]).next() {
		for (address, length) in value.as_slice::<(u64, u64)>().unwrap() {
			mem_map[i].offset = address.to_be() as _;
			mem_map[i].length = length.to_be() as usize >> 12;
			i += 1;
		}
	} else {
		panic!("Failed to get memory map from device tree");
	}
	
	// HACK: Add SBI reserved area manually, because for some reason the reservation block is empty
	crate::misc::qemu::riscv_sbi_fix_memory_map(&mut mem_map);
	
	// BOOT ENV INDEPENDENT CODE -------------------------------------------------------------------
	
	print!("done\nSetting up virtual memory               ... ");
	
	let mem_map           = &mut mem_map[..i];
	let total_memory      = mem_map.iter().map(|area| area.length).sum::<usize>();
	let phys_text_addr    = unsafe { &_text_start   } as *const _ as usize;
	let phys_rodata_addr  = unsafe { &_rodata_start } as *const _ as usize;
	let phys_data_addr    = unsafe { &_data_start   } as *const _ as usize;
	let phys_bss_addr     = unsafe { &_bss_start    } as *const _ as usize;
	let phys_kernel_end   = unsafe { &_kernel_end   } as *const _ as usize;
	let text_len          = phys_rodata_addr - phys_text_addr;
	let rodata_len        = phys_data_addr   - phys_rodata_addr;
	let data_len          = phys_bss_addr    - phys_data_addr;
	let bss_len           = phys_kernel_end  - phys_bss_addr;
	let _log_len          = DEFAULT_LOG_BUF_LEN;
	let mmap_len          = size_of::<NodeDescriptor>() + size_of::<PageDescriptor>() * total_memory + 0xFFF >> 12 << 12;
	let _smp_len          = size_of::<HartKernelData>() + 0xFFF >> 12 << 12;
	let virt_text_addr    = KERNEL_VIRT_BASE;
	let virt_rodata_addr  = virt_text_addr    + text_len;
	let virt_data_addr    = virt_rodata_addr  + rodata_len;
	let virt_bss_addr     = virt_data_addr    + data_len;
	let virt_mmap_addr    = virt_bss_addr     + bss_len;
	let virt_kernel_end   = virt_mmap_addr    + mmap_len;
	let kernel_pages      = (virt_kernel_end - KERNEL_VIRT_BASE) >> 12;
	let page_tables_size  = 1 + (kernel_pages + 0x3FFFF >> 18) + (kernel_pages + 0x1FF >> 9) + kernel_pages;
	let page_tables_base  = mem_map.iter()
		.find(|area| area.length - (phys_kernel_end as isize - area.offset as isize >> 12).max(0) as usize >= page_tables_size)
		.expect("failed to alloc page table memory")
		.offset.max(phys_kernel_end);
	
	// setup initial page tables
	
	// level 0
	
	let page_tables = unsafe { core::slice::from_raw_parts_mut(page_tables_base as *mut u64, (page_tables_size << 12) / 8) };
	let entries     = 1 << 9;
	let tables      = 1;
	let base        = page_tables[entries..].as_mut_ptr() as u64 >> 12;
	
	(0..256).for_each(|i| page_tables[i] = VALID | READ | WRITE | EXEC | GLOBAL | ((i as u64) << 37));
	page_tables[256] = VALID | GLOBAL | (base << PPN_SHIFT);
	page_tables[257..512].fill(0);
	
	// level 1
	
	let page_tables = &mut page_tables[entries..];
	let entries     = tables << 9;
	let tables      = kernel_pages + 0x3FFFF >> 18;
	let base        = page_tables[entries..].as_mut_ptr() as u64 >> 12;
	
	(0..tables).for_each(|i| page_tables[i] = VALID | GLOBAL | (base + (i as u64) << PPN_SHIFT));
	page_tables[tables..entries].fill(0);
	
	// level 2
	
	let page_tables = &mut page_tables[entries..];
	let entries     = tables << 9;
	let tables      = kernel_pages + 0x1FF >> 9;
	let base        = page_tables[entries..].as_mut_ptr() as u64 >> 12;
	
	(0..tables).for_each(|i| page_tables[i] = VALID | GLOBAL | (base + (i as u64) << PPN_SHIFT));
	page_tables[tables..entries].fill(0);
	
	// level 3
	
	let page_tables = &mut page_tables[entries..];
	let entries     = tables << 9;
	let tables      = kernel_pages;
	let base        = phys_text_addr as u64 >> 12;
	
	(0..virt_rodata_addr - KERNEL_VIRT_BASE >> 12).for_each(|i|
		page_tables[i] = VALID | GLOBAL | EXEC | (base + (i as u64) << PPN_SHIFT));
	(virt_rodata_addr - KERNEL_VIRT_BASE >> 12..virt_data_addr - KERNEL_VIRT_BASE >> 12).for_each(|i|
		page_tables[i] = VALID | GLOBAL | READ | (base + (i as u64) << PPN_SHIFT));
	(virt_data_addr - KERNEL_VIRT_BASE >> 12..virt_bss_addr - KERNEL_VIRT_BASE >> 12).for_each(|i|
		page_tables[i] = VALID | GLOBAL | READ | WRITE | (base + (i as u64) << PPN_SHIFT));
	
	//let mut i = virt_bss_addr - KERNEL_VIRT_BASE >> 12;
	let mut area = 0;
	let mut offset = page_tables_base + (page_tables_size << 12);
	
	/*for area in memory_areas {
		let offset = area.offset;
		while offset < area.offset + (area.length << 12) {
			let min_offset;
			let min_end;
			
			for (off, pages) in &reserved[..reserved_i] {
			
			}
		}
	}*/
	
	(virt_bss_addr - KERNEL_VIRT_BASE >> 12..virt_kernel_end - KERNEL_VIRT_BASE >> 12).for_each(|i| {
		let page = loop {
			if mem_map[area].r#type != Usable {
			
			} else if mem_map[area].length - (offset - mem_map[area].offset >> 12) > 0 {
				offset += 0x1000;
				break offset - 1 >> 12
			} else if area >= mem_map.len() - 1 {
				panic!("failed to allocate memory for heap");
			}
			
			area += 1;
		} as u64;
		
		page_tables[i] = VALID | GLOBAL | READ | WRITE | (page << PPN_SHIFT)
	});
	
	page_tables[tables..entries].fill(0);
	
	// load page tables, jump to virtual kernel
	
	unsafe {
		satp.write((page_tables_base as u64 >> 12) | satp::MODE_SV48);
		sfence_vma2();
		print!("done\nJumping to kernel space                 ... ");
		jump_to_kernel_space();
	}
	
	print!("done\nSetting up memory management            ... ");
	
	// zero out heap
	unsafe { core::slice::from_raw_parts_mut(
		virt_bss_addr as *mut usize,
		(virt_kernel_end - virt_bss_addr) / size_of::<usize>()
	) }.fill(0);
	
	let mut global = unsafe { &mut GLOBAL_DATA };
	global.ttb_id = ((page_tables_base as u64 >> 12) | satp::MODE_SV48) as usize;
	global.mem_nodes = unsafe { core::slice::from_raw_parts_mut(virt_mmap_addr as *mut NodeDescriptor, 1) };
	global.mem_nodes[0] = NodeDescriptor {
		mem_map:     unsafe { NoDbg(core::slice::from_raw_parts_mut(
			(virt_mmap_addr + size_of::<NodeDescriptor>()) as *mut _, total_memory)) },
		zone_normal: ZoneDescriptor {
			flags:         0,
			start_ppn:     mem_map[0].offset >> 12,
			spanned_pages: total_memory,
			present_pages: total_memory,
			managed_pages: AtomicUsize::new(total_memory),
			free_areas:    Default::default(),
			node:          &mut global.mem_nodes[0]
		},
		flags:         0,
		start_ppn:     mem_map[0].offset >> 12,
		spanned_pages: total_memory,
		present_pages: total_memory
	};
	global.cache.node = &mut global.mem_nodes[0];
	
	unsafe {
		global.mem_nodes[0].zone_normal.init();
		global.mem_nodes[0].zone_normal.reserve(global.mem_nodes[0].start_ppn, 0x200000 >> 12);
		global.mem_nodes[0].zone_normal.reserve(page_tables_base >> 12, page_tables_size);
		
		let (mut ppn, mut len) = (((page_tables[0] & PPN_MASK) >> PPN_SHIFT) as usize, 0);
		
		for entry in &page_tables[..tables] {
			let ppn0 = ((entry & PPN_MASK) >> PPN_SHIFT) as usize;
			
			if ppn0 == ppn + len {
				len += 1;
				continue;
			}
			
			let page = global.mem_nodes[0].zone_normal.reserve(ppn, len);
			for page in core::slice::from_raw_parts_mut(page, len) {
				page.flags = AtomicU32::new(PageType::Kernel as _);
			}
			
			ppn = ppn0;
			len = 1;
		}
		
		let page = global.mem_nodes[0].zone_normal.reserve(ppn, len);
		for page in core::slice::from_raw_parts_mut(page, len) {
			page.flags = AtomicU32::new(PageType::Kernel as _);
		}
	}
	
	print!("done\nSetting up init task                    ... ");
	
	let task0 = global.tasks.push_front(Task {
		flags:         FLAG_KERNEL_TASK,
		syscall_table: &SYSCALL_TABLE,
		sched_hart:    hartid,
		..Task::default()
	});
	
	// set x0 to fdt
	unsafe {
		*(&mut task0.core_img as *mut CoreImage as *mut *const FdtHeader) = fdt;
	}
	
	global.hart_data[hartid].queue.push_front(task0);
	
	print!("done\nSetting up interrupts                   ... ");
	
	stvec.write_ptr(exceptions::handle_exception as *const ());
	
	println!("done\nInitialization sequence completed.\n");
	
	// TODO log buffer
	// TODO vfs
	
	// 1. init cpus/scheduler
	// 2. mount initfs
	// 3. load drivers + init (how to init drivers?)
	// 4. jump to init (no stack setup, init sets up itself)
	
	// print debug info
	
	println!("KERNEL ONLINE\n");
	
	let (tunit, tmem) = get_mem_unit(total_memory << 12);
	let (kunit, kmem) = get_mem_unit(kernel_pages + page_tables_size << 12);
	//println!("\nDEVICE TREE:\n\n{:#?}\n", fdt);
	println!("Number of harts:       {:6}, booting #{}", harts, hartid);
	println!("Page size:             {:6}", 1 << 12);
	println!("Total usable memory: {:6}{} ({} pages)", tmem, tunit, total_memory);
	println!("Memory used:         {:6}{} ({} pages)", kmem, kunit, kernel_pages + page_tables_size);
	println!("\nPHYSICAL MEMORY MAP:\n");
	mem_map.iter().for_each(|a| println!("{}", a));
	println!("\nVIRTUAL MEMORY MAP:\n");
	println!("VIRT START         VIRT END           PHYS START         PHYS END         SIZE              PAGES NAME");
	print_range("ID",            0,                0,                1usize << 47);
	print_range("PAGE TABLES",   page_tables_base, page_tables_base, page_tables_size << 12);
	print_range("KERNEL TEXT",   virt_text_addr,   phys_text_addr,   text_len);
	print_range("KERNEL RODATA", virt_rodata_addr, phys_rodata_addr, rodata_len);
	print_range("KERNEL DATA",   virt_data_addr,   phys_data_addr,   data_len);
	print_range("KERNEL BSS",    virt_bss_addr,    phys_bss_addr,    bss_len);
	print_range("KERNEL MMAP",   virt_mmap_addr,   0,                mmap_len);
	println!();
	print_kernel_space(&page_tables[..tables]);
	
	unsafe { handle_timer_interrupt(task0) }
}

fn print_range(name: &str, virt_start: usize, phys_start: usize, size: usize) {
	if size >> 12 > 512 * 512 {
		for i in 9..64 {
			if size >> i + 12 == 0 {
				println!("{:016X} - {:016X} @ {:016X} - {:016X} {:016X}   2^{:2} {}", virt_start, virt_start + size, phys_start, phys_start + size, size, i - 1, name);
				return;
			}
		}
	} else {
		println!("{:016X} - {:016X} @ {:016X} - {:016X} {:016X} {:6} {}", virt_start, virt_start + size, phys_start, phys_start + size, size, size >> 12, name);
	}
}

fn print_kernel_space(page_tables: &[u64]) {
	let (mut ppn, mut attr, mut idx, mut len) = (
		((page_tables[0] & PPN_MASK) >> PPN_SHIFT) as usize,
		page_tables[0] & (VALID | GLOBAL | USER | READ | WRITE | EXEC),
		0, 0
	);
	
	for (i, entry) in page_tables.iter().enumerate() {
		let ppn0 = ((entry & PPN_MASK) >> PPN_SHIFT) as usize;
		let attr0 = entry & (VALID | GLOBAL | USER | READ | WRITE | EXEC);
		if ppn0 != ppn + len || attr != attr0 {
			println!(
				"{:016X} - {:016X} @ {:016X} - {:016X} {:016X} {:6} {}{}{}{}{}{}",
				KERNEL_VIRT_BASE + (idx << 12), KERNEL_VIRT_BASE + (idx + len << 12),
				ppn << 12, ppn + len << 12, len << 12, len,
				if attr & VALID  != 0 { 'P' } else { '-' },
				if attr & GLOBAL != 0 { 'G' } else { '-' },
				if attr & USER   != 0 { 'U' } else { '-' },
				if attr & READ   != 0 { 'R' } else { '-' },
				if attr & WRITE  != 0 { 'W' } else { '-' },
				if attr & EXEC   != 0 { 'X' } else { '-' },
			);
			
			ppn = ppn0;
			attr = attr0;
			idx = i;
			len = 1;
		} else {
			len += 1;
		}
	}
	
	println!(
		"{:016X} - {:016X} @ {:016X} - {:016X} {:016X} {:6} {}{}{}{}{}{}",
		KERNEL_VIRT_BASE + (idx << 12), KERNEL_VIRT_BASE + (idx + len << 12),
		ppn << 12, ppn + len << 12, len << 12, 12,
		if attr & VALID  != 0 { 'P' } else { '-' },
		if attr & GLOBAL != 0 { 'G' } else { '-' },
		if attr & USER   != 0 { 'U' } else { '-' },
		if attr & READ   != 0 { 'R' } else { '-' },
		if attr & WRITE  != 0 { 'W' } else { '-' },
		if attr & EXEC   != 0 { 'X' } else { '-' },
	);
}