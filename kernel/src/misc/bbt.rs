use core::ptr;

const BLOCK_SIZE: usize = 64;

pub struct BinaryBlockTree<T> {
	root: *mut Block<T>,
	length: usize,
	depth:  usize
}

pub enum Block<T> {
	Branch([*mut Self; BLOCK_SIZE]),
	Leaf([T; BLOCK_SIZE])
}

impl<T> BinaryBlockTree<T> {
	pub const fn new() -> Self {
		Self {
			root:   ptr::null_mut(),
			length: 0,
			depth:  0
		}
	}
	
	pub fn get(&self, idx: usize) -> &T {
		unimplemented!()
	}
	
	pub fn get_mut(&mut self, idx: usize) -> &mut T {
		unimplemented!()
	}
	
	pub fn push_front(&mut self, v: T) {
		unimplemented!()
	}
	
	pub fn push_back(&mut self, v: T) {
		unimplemented!()
	}
	
	pub fn pop_front(&mut self) -> Option<T> {
		unimplemented!()
	}
	
	pub fn pop_back(&mut self) -> Option<T> {
		unimplemented!()
	}
	
	pub fn reserve(&mut self, additional: usize) {
		unimplemented!()
	}
	
	pub fn shrink(&mut self, new_capacity: usize) {
		unimplemented!()
	}
}