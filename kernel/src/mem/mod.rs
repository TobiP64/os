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

use core::{ptr::null_mut, sync::atomic::*};

const PAGE_SHIFT:      usize = 12;
const MIN_CACHE_ORDER: usize = 3;
const MAX_CACHE_ORDER: usize = 12;
const MIN_ALIGN_MASK:  usize = !(!0 << MIN_CACHE_ORDER);
const LRU_GENERATIONS: usize = 8;
const MAX_NODE_PAGES:  usize = (1 << 32) - 1;

#[derive(Debug)]
pub struct NodeDescriptor {
    pub flags:         u32,
	pub first_page:    u32,
	pub spanned_pages: u32,
	pub present_pages: u32,
	#[cfg(target_arch = "x86_64")]
	pub zone_dma24:    ZoneDescriptor,
	#[cfg(target_arch = "x86_64")]
	pub zone_dma32:    ZoneDescriptor,
	pub zone_normal:   ZoneDescriptor,
	pub pages:         [PageDescriptor; MAX_NODE_PAGES]
}

impl NodeDescriptor {
    pub fn get_ppn(&self, desc: *const PageDescriptor) -> usize {
        unsafe { desc.offset_from(self.pages.as_ptr()) as usize + self.first_page }
    }

	pub fn get_page(&self, ppn: usize) -> *mut PageDescriptor {
        &self.pages[ppn - self.first_page] as *const PageDescriptor as _
    }
}


pub struct CacheEntry {
    next: AtomicPtr<Self>
}

pub struct HeapEntry {
    next: AtomicPtr<Self>,
	len:  usize
}

unsafe impl Allocator for CacheDescriptor {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        unsafe {
            // get order
			let node  = self.node.as_mut().unwrap();
            let size  = layout.size()  + MIN_ALIGN_MASK & !MIN_ALIGN_MASK;
            let align = layout.align() + MIN_ALIGN_MASK & !MIN_ALIGN_MASK;
            let order = 64 - (size.max(align)  - 1).leading_zeros() as usize;

            crate::println!("CACHE allocating, size: {} ({}), align: {} ({}), order: {}",
							size, layout.size(), align, layout.align(), order);

            // get next order with free areas
			let (i, area) = match self.free_areas.iter()
				.enumerate()
				.skip(order - MIN_CACHE_ORDER)
				.find(|(_, p)| !p.load(Ordering::SeqCst).is_null())
			{
                Some((i, areas)) => { // area found
					let area = areas.load(Ordering::SeqCst);
                    areas.store((*area).next.load(Ordering::SeqCst), Ordering::SeqCst);

                    let page = node.get_page((PageTables::current()
						.translate(area as usize >> 12) & PPN_MASK >> PPN_SHIFT) as _);
                    (*page).refs.fetch_add(1, Ordering::SeqCst);

                    (i + MIN_CACHE_ORDER, area)
                },
				None => { // no area found, allocate new page
					let page = node.zone_normal.alloc(0);

                    if page.is_null() { return Err(AllocError); }

					(*page).owner = PageOwner { cache: self as *const _ as _ };
                    (*page).flags.store(PageType::KernelDynamic as _, Ordering::Relaxed);

                    let tables = PageTables::current();
                    let vpn = tables.iter(KERNEL_VIRT_BASE..).find(|(e, _)| !e.is_valid()).ok_or(AllocError)?;
                    tables.map(vpn, node.get_ppn(page), 1, (VALID | READ | WRITE | GLOBAL) as _, node);

                    let area = (vpn << 12) as *mut CacheEntry;
                    (*area).next = AtomicPtr::new(null_mut());

                    self.free_areas[PAGE_SHIFT - MIN_CACHE_ORDER].store(area, Ordering::SeqCst);
                    (PAGE_SHIFT, area)
                }
            };

            let end = (area as *mut u8).add(size);
            let mut end_aligned = (area as *mut u8).add(1 << i);

            while end_aligned > end {
                let order = 63 - end_aligned.offset_from(end).leading_zeros() as usize;
                let area = end_aligned.sub(1 << order) as *mut CacheEntry;

                loop {
                    let next = self.free_areas[order - MIN_CACHE_ORDER].load(Ordering::SeqCst);
                    (*area).next.store(next, Ordering::SeqCst);

                    if self.free_areas[order - MIN_CACHE_ORDER].compare_exchange(
                            next, area, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                        break;
                    }
                }

				end_aligned = end_aligned.sub(1 << order);
            }

			Ok(core::ptr::NonNull::new_unchecked(core::slice::from_raw_parts_mut(
                    area as _, size)))
        }
    }

	unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        let node  = self.node.as_mut().unwrap();
        let ptr   = ptr.as_ptr();
        let ppn   = PageTables::current().translate(ptr as usize >> 12) & PPN_MASK >> PPN_SHIFT;
        let page  = self.node.as_mut().unwrap().get_page(ppn as _).as_mut().unwrap();
        let _order = 64 - ((layout.size() + MIN_ALIGN_MASK & !MIN_ALIGN_MASK)  - 1).leading_zeros() as usize;

        if page.refs.fetch_sub(1, Ordering::SeqCst) == 1 {
            PageTables::current().unmap(ppn as _, 1, node);
            node.zone_normal.free(1, page);
            return;
        }



		unimplemented!()
    }
}

#[derive(Debug)]
pub struct ZoneDescriptor {
    pub flags:         u32,
	pub first_page:    u32,
	pub spanned_pages: u32,
	pub present_pages: u32,
	pub managed_pages: AtomicU32,
	pub pages:         *mut PageDescriptor,
	pub lru_scan_limit: u32,
	pub lru_swap_limit: u32,
	pub generations:   [u32; LRU_GENERATIONS],
	pub alloc_pages:   [u32; PageDescriptor::MAX_PAGE_ORDER as usize + 1],
	pub alloc_cache:   [AtomicPtr<CacheEntry>; MAX_CACHE_ORDER - MIN_CACHE_ORDER + 1],
	pub alloc_heap:    AtomicPtr<HeapEntry>,
	pub node:          *mut NodeDescriptor
}

impl ZoneDescriptor {
    pub fn init(&mut self) {
        let node = unsafe { self.node.as_mut() }.unwrap();

        for i in (self.start_ppn - node.start_ppn..self.start_ppn - node.start_ppn + self.spanned_pages)
			.step_by(1 << MAX_PAGE_ORDER).rev() {

            node.pages[i + 1..i + (1 << MAX_PAGE_ORDER)].iter_mut().for_each(
                    |page| page.flags = AtomicU32::new(PageType::Unused as u32 | PAGE_DESC_FLAG_COMPOUND));
            unsafe { self.add_compound_page(&mut node.pages[i], MAX_PAGE_ORDER); }
        }

		self.flags |= ZONE_DESC_FLAG_INITIALIZED;
    }

	unsafe fn add_compound_page(&mut self, ptr: *mut PageDescriptor, order: usize) {
        let (head, tail) = core::slice::from_raw_parts_mut(ptr, 1 << order)
			.split_first_mut().unwrap();

        *head = PageDescriptor {
            idx:   order,
			flags: AtomicU32::new(PageType::Unused as u32 | PAGE_DESC_FLAG_COMPOUND_HEAD),
			..PageDescriptor::default()
        };

        for page in tail {
            page.next = head;
        }
		self.free_areas[order].push_front(head);
    }

	pub unsafe fn reserve(&mut self, ppn: usize, len: usize) -> *mut PageDescriptor {
        if ppn < self.start_ppn || ppn + len >= self.start_ppn + self.spanned_pages {
            return null_mut();
        }

		self.managed_pages.fetch_sub(len, Ordering::SeqCst);
        let node              = self.node.as_mut().unwrap();
        let start             = &mut node.pages[ppn - node.start_ppn] as *mut PageDescriptor;
        let end               = &mut node.pages[ppn - node.start_ppn + len - 1] as *mut PageDescriptor;
        let mut start_aligned = (*start).get_head();
        let end_aligned       = (*end).get_head();
        let end               = end.add(1);
        let mut end_aligned   = end_aligned.add(1 << (*end_aligned).idx);

        // remove all (partially) included areas from the free list

		let mut page = start_aligned;
        while page < end_aligned {
            let pg = page.as_mut().unwrap();
            self.free_areas[pg.idx].remove(pg);
            page = page.add(1 << pg.idx);
        }

		// add partially excluded areas to free list

		while start_aligned < start {
            let order = (63 - start.offset_from(start_aligned).leading_zeros() as usize).min(MAX_PAGE_ORDER);
            self.add_compound_page(start_aligned, order);
            start_aligned = start_aligned.add(1 << order);
        }

		while end_aligned > end {
            let order = (63 - end_aligned.offset_from(end).leading_zeros() as usize).min(MAX_PAGE_ORDER);
            self.add_compound_page(end_aligned.sub(1 << order), order);
            end_aligned = end_aligned.sub(1 << order);
        }

		start
    }

	pub unsafe fn alloc(&mut self, order: usize) -> *mut PageDescriptor {
        debug_assert!(order < MAX_PAGE_ORDER + 1);

        // get next order with free areas
		let (i, areas) = match self.free_areas[order..].iter_mut()
			.enumerate()
			.find(|(_, p)| !p.is_empty())
		{
            None => return null_mut(),
			Some((i, p)) => (order + i, p)
        };

        let pages = core::slice::from_raw_parts_mut(areas.pop_front(), 1 << i);
        self.managed_pages.fetch_sub(1 << order, Ordering::SeqCst);

        // update page desc
		pages[0] = PageDescriptor {
            idx:   order,
			flags: AtomicU32::new(if order > 0 { PAGE_DESC_FLAG_COMPOUND_HEAD } else { 0 }),
			..PageDescriptor::default()
        };

        // if i is greater than the required order, split the area
		for i in (order..i).rev() {
            self.add_compound_page(&mut pages[1 << i], i + 1);
        }

		pages.as_mut_ptr()
    }

	pub unsafe fn free(&mut self, mut order: usize, mut pages: *mut PageDescriptor) {
        debug_assert!(order < MAX_PAGE_ORDER + 1);
        let o = order;
        let node = self.node.as_mut().unwrap();

        // clear pages to free

		let (page, tail) = core::slice::from_raw_parts_mut(pages, 1 << order)
			.split_first_mut().unwrap();

        *page = PageDescriptor {
            idx:   order,
			flags: AtomicU32::new(PageType::Unused as u32 | PAGE_DESC_FLAG_COMPOUND_HEAD),
			..PageDescriptor::default()
        };

        tail.fill(PageDescriptor {
            next:  pages,
			flags: AtomicU32::new(PageType::Unused as u32 | PAGE_DESC_FLAG_COMPOUND),
			..PageDescriptor::default()
        });

        // merge areas

		while order < MAX_PAGE_ORDER {
            // find buddy

			// if the area is aligned, buddy is the next area, otherwise its the previous area
			let buddy = if node.get_ppn(pages) & !(!0 << order + 1) == 0 {
                pages.add(1 << order)
            } else {
                pages.sub(1 << order)
            }.as_mut().unwrap();

            // check if merge is possible

			let buddy_ppn = node.get_ppn(buddy);
            let merge     = buddy_ppn >= self.start_ppn
							&& buddy_ppn < self.start_ppn + self.spanned_pages
							&& buddy.flags.load(Ordering::SeqCst) & (PAGE_DESC_FLAG_TYPE_MASK | PAGE_DESC_FLAG_COMPOUND_HEAD) == PageType::Unused as u32 | PAGE_DESC_FLAG_COMPOUND_HEAD
							&& buddy.idx == order;

            if !merge {
                break; // done, nothing to merge
            }

			// merge

			self.free_areas[order].remove(buddy);

            let lower = pages.min(buddy);
            let upper = pages.max(buddy).as_mut().unwrap();
            upper.prev  = null_mut();
            upper.owner = PageOwner { other: null_mut() };
            upper.idx   = 0;
            upper.flags.store(PageType::Unused as u32 | PAGE_DESC_FLAG_COMPOUND, Ordering::Relaxed);

            pages = lower;
            order += 1;
        }

		// add area

		self.add_compound_page(pages, order);
        self.managed_pages.fetch_add(1 << o, Ordering::SeqCst);
    }
}

#[derive(Debug, Default)]
#[repr(align(32))]
pub struct PageDescriptor {
    /// If this page is a tail page of a compound page, this points to the head (first page desc)
	/// of the compound page
	pub prev:  i32,
	/// _pad0, if compound page tail
	pub next:  i32,
	pub lru:   i32,
	pub refs:  u32,
	#[cfg(not(feature = "map-ram"))]
	pub virt:  u32,
	/// _pad1, if compound page tail
    #[cfg(not(feature = "hypervisor"))]
	pub node:  *mut crate::mnt::Node,
    #[cfg(feature = "hypervisor")]
    pub owner: *mut crate::ctx::Context
}

impl PageDescriptor {
    pub const FLAGS_TYPE_MASK:           u32 = 0xF;
    pub const FLAGS_TYPE_UNUSABLE:       u32 = 0x0;
    pub const FLAGS_TYPE_UNUSED:         u32 = 0x1;
    pub const FLAGS_TYPE_KERNEL_STATIC:  u32 = 0x2;
    pub const FLAGS_TYPE_KERNEL_DYNAMIC: u32 = 0x3;
    pub const FLAGS_TYPE_USER:           u32 = 0x4;
    pub const FLAGS_TYPE_USER_CACHED:    u32 = 0x5;
    pub const FLAGS_TYPE_FIRMWARE:       u32 = 0x6;
    pub const FLAGS_COMPOUND:            u32 = 0x8;
    pub const FLAGS_PINNED:              u32 = 0x10;
    pub const FLAGS_LOCKED:              u32 = 0x20;
    pub const FLAGS_LOCKED_EXCLUSIVE:    u32 = 0x40;
    pub const FLAGS_ORDER_MASK:          u32 = 0xF80;
    pub const FLAGS_ORDER_SHIFT:         u32 = 7;
    pub const MAX_PAGE_ORDER:            u32 = 9;

    pub fn get_head(&mut self) -> *mut PageDescriptor {
        if self.flags.load(Ordering::Relaxed) & Self::FLAGS_COMPOUND != 0 {
            self.next
        } else {
            self
        }
    }
}

impl PageDescriptor {
    pub fn iter(self_: *mut Self) -> PageDescIter {
        PageDescIter(self_)
    }
}

pub struct PageDescIter(*mut PageDescriptor);

impl Iterator for PageDescIter {
    type Item = *mut PageDescriptor;

    fn next(&mut self) -> Option<Self::Item> {
        (!self.0.is_null()).then(|| {
            let page = self.0;
            self.0 = unsafe { (*self.0).next };
            page
        })
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct PhysMemoryArea {
    /// Length of the area, in pages (max length = 16 TiB for 4K pages)
	pub length: u32,
	pub flags:  u32, // RWX + cache stuff
	pub node:   *mut NodeDescriptor
}

impl PhysMemoryArea {
    pub const FLAGS_TYPE_MASK:     usize = 0b11;
    pub const FLAGS_TYPE_UNUSABLE: usize = 0b00;
    pub const FLAGS_TYPE_USABLE:   usize = 0b01;
    pub const FLAGS_TYPE_FIRMWARE: usize = 0b10;
    pub const FLAGS_TYPE_MMIO:     usize = 0b11;
    pub const FLAGS_PERSISTENT:    usize = 1 << 2;
}

#[derive(Copy, Clone, Debug, Default)]
pub struct VirtMemoryArea {
    pub addr:   u32,
	pub offset: u32,
	pub length: u32,
	pub flags:  u32,
	pub rd:     *mut crate::ctx::ResourceDescriptor,
	pub pages:  *mut PageDescriptor,
}

impl VirtMemoryArea {
    pub const FLAGS_READ:  usize = 0x1;
    pub const FLAGS_WRITE: usize = 0x2;
    pub const FLAGS_EXEC:  usize = 0x4;
    pub const FLAGS_USER:  usize = 0x8;
    pub const FLAGS_STACK: usize = 0x10;

    pub fn cmp(&self, addr: usize) -> core::cmp::Ordering {
        if addr < self.addr {
            Less
        } else if addr > self.addr + self.length {
            Greater
        } else {
            Equal
        }
    }
}

#[no_mangle]
pub fn handle_page_fault(ctx: &ctx::Context, addr: usize, attr: usize) {
    unsafe { llvm_asm!("uret"::::"volatile"); }
	let addr = addr as usize >> PAGE_SHIFT;

    let area = match task.mem_areas.find(|area| area.cmp(addr)) {
        Some(v) => v,
		None    => return
    };

    if area.flags & attr != attr {
        return;
    }

	let mut tables = PageTables::current();
    let entry = tables.translate(addr as usize >> 12);

    if attr & AREA_FLAG_WRITE != 0 && entry & WRITE == 0 {
        let node = unsafe { &mut GLOBAL_DATA.mem_nodes[0] };
        let old_ppn = entry & PPN_MASK >> PPN_SHIFT;
        let old_page = unsafe { node.get_page(old_ppn as _).as_mut().unwrap() };

        let ppn = if old_page.refs.fetch_sub(1, Ordering::SeqCst) == 1 {
            old_page.refs.store(1, Ordering::SeqCst);
            old_ppn as usize
        } else {
            let page = unsafe { node.zone_normal.alloc(0).as_mut().unwrap() };
            let ppn = node.get_ppn(page);
            unsafe { ((old_ppn << 12) as *mut usize).copy_to_nonoverlapping(
                    (ppn << 12) as *mut usize, (1 << PAGE_SHIFT) / core::mem::size_of::<usize>()); }
			ppn
        };

        tables.map(addr as usize >> 12, ppn, 1, (entry | WRITE) as _, node);

        unsafe { eret() }
    } else if entry & VALID == 0 {
        // TODO if page is not present, load and block task
		unsafe { task.block(); }
    } else {
        panic!("unrecoverable page fault");
    }
}

//#[cfg(test)]
pub mod test {
	use super::*;
	use crate::utils::NoDbg;
	use core::ptr::null_mut;

	//#[test]
	pub fn buddy_alloc() {
		let mut mem_map = unsafe { core::mem::zeroed::<[PageDescriptor; 512]>() };
		let (head, tail) = mem_map.split_first_mut().unwrap();

		head.idx = 9;
		head.flags = AtomicU32::new(PAGE_DESC_FLAG_COMPOUND_HEAD);

		for page in tail {
			page.next = head;
			page.flags = AtomicU32::new(PAGE_DESC_FLAG_COMPOUND);
		}

		let mut node = NodeDescriptor {
			pages:       NoDbg(unsafe { (&mut mem_map as *mut [PageDescriptor]).as_mut().unwrap() }),
			zone_normal:   ZoneDescriptor {
				flags:         ZONE_DESC_FLAG_INITIALIZED,
				start_ppn:     0,
				spanned_pages: 512,
				present_pages: 512,
				managed_pages: AtomicUsize::new(512),
				free_areas:    Default::default(),
				node:          null_mut()
			},
			flags:         0,
			start_ppn:     0,
			spanned_pages: 512,
			present_pages: 512
		};

		node.zone_normal.node = &mut node;
		node.zone_normal.free_areas[9] = ZoneFreeArea {
			len: AtomicUsize::new(1),
			next: &mut mem_map[0],
			prev: &mut mem_map[0]
		};

		crate::println!("{}", NodesDisplay::new(&node));

		unsafe { node.zone_normal.reserve(1, 1); }
		crate::println!("{}", NodesDisplay::new(&node));
		unsafe { node.zone_normal.reserve(2, 1); }
		crate::println!("{}", NodesDisplay::new(&node));

		/*let page0 = unsafe { node.zone_normal.alloc(1).as_mut().unwrap() };
		crate::println!("{}", NodesDisplay::new(&node));

		let page1 = unsafe { node.zone_normal.alloc(0).as_mut().unwrap() };
		crate::println!("{}", NodesDisplay::new(&node));

		unsafe { node.zone_normal.free(1, page0); };
		crate::println!("{}", NodesDisplay::new(&node));

		unsafe { node.zone_normal.free(0, page1); };
		crate::println!("{}", NodesDisplay::new(&node));*/
	}
}