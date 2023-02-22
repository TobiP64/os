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

pub struct Framebuffer {
	pub data:   *mut u8,
	pub width:  u32,
	pub height: u32,
	pub pitch:  u32,
	pub bpp:    u32
}

impl Framebuffer {
	#[inline]
	pub unsafe fn draw(&mut self, (x, y): (u32, u32), color: u32) {
		let pixel = (y * self.pitch + x * (self.bpp >> 3)) as isize;
		match self.bpp {
			32 => {
				self.data.offset(pixel).write_volatile((color >> 24 & 0xFF) as _);
				self.data.offset(pixel + 1).write_volatile((color >> 16 & 0xFF) as _);
				self.data.offset(pixel + 2).write_volatile((color >> 8  & 0xFF) as _);
				self.data.offset(pixel + 3).write_volatile((color & 0xFF) as _);
			}
			24 => {
				self.data.offset(pixel).write_volatile((color >> 24 & 0xFF) as _);
				self.data.offset(pixel + 1).write_volatile((color >> 16 & 0xFF) as _);
				self.data.offset(pixel + 2).write_volatile((color >> 8  & 0xFF) as _);
			}
			16 => {
				self.data.offset(pixel).write_volatile((color >> 24 & 0xFF) as _);
				self.data.offset(pixel + 1).write_volatile((color >> 16 & 0xFF) as _);
			}
			_ => ()
		}
	}
}

pub struct FramebufferTextWriter {
	framebuffer: Framebuffer,
	x:    u32,
	y:    u32,
}

impl FramebufferTextWriter {
	#[inline]
	pub fn new(framebuffer: Framebuffer) -> Self {
		Self { framebuffer, x: 0, y: 0 }
	}
	
	#[inline]
	pub fn draw_glyph(&mut self, (x, y): (u32, u32), ch: char) {
		let ch = match ch {
			ch @ '!'..='~' => ch as usize - 0x21,
			_ => 0x5E
		};
		for i in 0..0x10 {
			for j in 0..0x20 {
				let v = FONT[ch][i][j] as u32;
				unsafe { self.framebuffer.draw(
					(x + i as u32, y + j as u32), 0xFF | v << 8 | v << 16 | v << 24) }
			}
		}
	}
}

impl core::fmt::Write for FramebufferTextWriter {
	fn write_str(&mut self, s: &str) -> core::fmt::Result {
		s.chars().map(|c| self.write_char(c)).collect()
	}
	
	#[inline]
	fn write_char(&mut self, c: char) -> core::fmt::Result {
		match c {
			' ' => (),
			'\t' => self.x += 48,
			'\n' => {
				self.x = 0;
				self.y += 32 as u32;
			},
			ch => self.draw_glyph((self.x, self.y), ch)
		}
		self.x += 16;
		Ok(())
	}
}