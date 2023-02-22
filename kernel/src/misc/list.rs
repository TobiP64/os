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

use core::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use alloc::boxed::Box;
use crate::misc::std::alloc::Allocator;
use alloc::alloc::Global;
use core::ptr::null_mut;

#[repr(C)]
pub struct LinkedList<T, A: Allocator = Global> {
	head:  AtomicPtr<Node<T>>,
	tail:  AtomicPtr<Node<T>>,
	len:   AtomicUsize,
	alloc: A
}

impl<T> LinkedList<T> {
	pub const fn new() -> Self {
		Self::new_in(Global)
	}
}

impl<T, A: Allocator> LinkedList<T, A> {
	pub const fn new_in(alloc: A) -> Self {
		Self {
			head: AtomicPtr::new(null_mut()),
			tail: AtomicPtr::new(null_mut()),
			len:  AtomicUsize::new(0),
			alloc
		}
	}
	
	pub fn is_empty(&self) -> bool {
		self.len.load(Ordering::SeqCst) == 0
	}
	
	pub fn front(&self) -> Option<&T> {
		(!self.is_empty()).then(|| &unsafe { self.head
			.load(Ordering::SeqCst).as_ref() }.unwrap().data)
	}
	
	pub fn front_mut(&self) -> Option<&mut T> {
		(!self.is_empty()).then(|| &mut unsafe { self.head
			.load(Ordering::SeqCst).as_mut() }.unwrap().data)
	}
	
	pub fn push_front(&self, v: T) -> &mut Node<T> {
		let mut node = Box::new(Node {
			next: AtomicPtr::default(),
			prev: AtomicPtr::default(),
			data: v
		});
		
		let ptr = &mut *node as *mut Node<T>;
		let head = self.head.swap(ptr, Ordering::SeqCst);
		node.next.store(head, Ordering::SeqCst);
		node.prev.store(unsafe { (*head).prev.swap(ptr, Ordering::SeqCst) }, Ordering::SeqCst);
		self.len.fetch_add(1, Ordering::SeqCst);
		Box::leak(node)
	}
	
	pub fn pop_front(&self) -> Option<T> {
		let node = loop {
			let node = unsafe { self.head.load(Ordering::SeqCst).as_mut() }?;
			
			if self.head.compare_exchange(node, node.next.load(Ordering::SeqCst), Ordering::SeqCst, Ordering::SeqCst).is_ok() {
				break node;
			}
		};
		
		loop {
			if let Some(next) = unsafe { node.next.load(Ordering::SeqCst).as_mut() } {
				if next.prev.compare_exchange(node, node.prev.load(Ordering::SeqCst), Ordering::SeqCst, Ordering::SeqCst).is_ok() {
					break;
				}
			} else {
				break;
			}
		}
		
		loop {
			if let Some(prev) = unsafe { node.prev.load(Ordering::SeqCst).as_mut() } {
				if prev.next.compare_exchange(node, node.next.load(Ordering::SeqCst), Ordering::SeqCst, Ordering::SeqCst).is_ok() {
					break;
				}
			} else {
				break;
			}
		}
		
		let node = unsafe { Box::from_raw(node) };
		Some(node.data)
	}
	
	pub fn rotate_right(&self, k: usize) {
		for _ in 0..k {
			loop {
				if let Some(head) = unsafe { self.head.load(Ordering::SeqCst).as_mut() } {
					if self.tail.compare_exchange(
						head, head.next.load(Ordering::SeqCst), Ordering::SeqCst, Ordering::SeqCst).is_ok() {
						break;
					}
				} else {
					break;
				}
			}
			
			loop {
				if let Some(tail) = unsafe { self.tail.load(Ordering::SeqCst).as_mut() } {
					if self.tail.compare_exchange(
						tail, tail.next.load(Ordering::SeqCst), Ordering::SeqCst, Ordering::SeqCst).is_ok() {
						break;
					}
				} else {
					break;
				}
			}
		}
	}
}

#[repr(C)]
pub struct Node<T> {
	next: AtomicPtr<Self>,
	prev: AtomicPtr<Self>,
	data: T
}

impl<T> core::ops::Deref for Node<T> {
	type Target = T;
	
	fn deref(&self) -> &Self::Target {
		&self.data
	}
}

impl<T> core::ops::DerefMut for Node<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.data
	}
}
