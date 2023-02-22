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

use core::ptr::null_mut;
use core::cmp::Ordering;

pub struct Tree<T> {
	root: *mut TreeNode<T>
}

pub struct TreeNode<T> {
	left:  *mut Self,
	right: *mut Self,
	data:  T
}

impl<T> Tree<T> {
	pub fn new() -> Self {
		Self { root: null_mut() }
	}
	
	pub fn find(&self, mut f: impl FnMut(&T) -> Ordering) -> Option<&T> {
		let mut root = self.root;
		
		while let Some(node) = unsafe { root.as_mut() } {
			match f(&node.data) {
				Ordering::Equal   => return Some(&node.data),
				Ordering::Less    => root = node.left,
				Ordering::Greater => root = node.right
			}
		}
		
		None
	}
}