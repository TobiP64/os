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

use alloc::{boxed::Box, vec::Vec, string::String};

pub type Trie<T> = Option<Box<TrieNode<T>>>;

#[derive(Clone, Debug)]
pub struct TrieNode<T> {
    key:      String,
	val:      Option<Box<T>>,
	parent:   *mut TrieNode<T>,
	children: Vec<TrieNode<T>>
}

impl<T> TrieNode<T> {
	pub const fn const_default() -> Self {
		Self::new('\0')
	}

	pub const fn new(ch: char) -> Self {
		Self { ch, children: Vec::new(), value: None }
	}

	pub fn insert(&mut self, path: &str, node: T) -> Option<Box<T>> {
		path.chars().fold(self, |node, ch| {
			if !node.children.iter_mut().any(|node| node.ch == ch) {
				node.children.push(TrieNode::new(ch));
			}

			node.children.iter_mut().find(|node| node.ch == ch).unwrap()
		}).value.replace(Box::new(node))
	}

	pub fn remove(&mut self, path: &str) /*-> Option<T>*/ {
		self._remove(path.chars());
	}

	fn _remove(&mut self, mut chars: impl IntoIterator<Item = char>) -> bool {
		let mut chars = chars.into_iter();
		if let Some(ch) = chars.next() {
			if let Some(i) = self.children.iter_mut()
				.enumerate()
				.find(|(_, node)| node.ch == ch)
				.and_then(|(i, node)| node._remove(chars).then_some(i)) {
				self.children.remove(i);
			}
		} else {
			self.value = None
		}

		self.value.is_none() && self.children.is_empty()
	}

	pub fn get(&self, path: &str) -> Option<&T> {
		path.chars()
			.try_fold(self, |node, ch| node.children.iter().find(|node| node.ch == ch))
			.and_then(|node| node.value.as_deref())
	}
}

impl<T> Default for TrieNode<T> {
	fn default() -> Self {
		Self::new('\0')
	}
}