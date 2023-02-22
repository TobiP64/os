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

use crate::arch::volatile;
use crate::dri::bcm2711::peripherals;

pub struct Mailbox;

static mut MAILBOX0: Mailbox = Mailbox;

impl Mailbox {
	#[inline]
	pub fn get() -> &'static mut Self {
		unsafe { &mut MAILBOX0 }
	}
	
	#[inline]
	pub fn is_empty(&self) -> bool {
		peripherals().arm.cores[0].mail0_status & 0x4000_0000 == 0
	}
	
	#[inline]
	pub fn is_full(&self) -> bool {
		peripherals().arm.cores[0].mail0_status & 0x8000_0000 == 0
	}
	
	#[inline]
	pub fn write(&mut self, channel: Channel, value: u32) {
		while !self.is_full() { volatile() }
		peripherals().arm.cores[0].mail1_write = value & !0xF | channel as u32;
	}
	
	#[inline]
	pub fn read(&mut self, channel: Channel) -> u32 {
		(loop {
			while self.is_empty() { volatile() }
			let value = peripherals().arm.cores[0].mail0_rw;
			if value & 0xF == channel as u32 { break value; }
		}) >> 4
	}
	
	#[inline]
	pub fn prop_tag_builder(&mut self) -> MailboxPropertyRequestBuilder {
		MailboxPropertyRequestBuilder {
			mailbox: self,
			buf:     unsafe { &mut MAILBOX_PROP_BUF.0 },
			len:     2
		}
	}
}

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Channel {
	PowerManagement = 0,
	Framebuffer     = 1,
	VirtualUart     = 2,
	Vchiq           = 3,
	Leds            = 4,
	Buttons         = 5,
	TouchScreen     = 6,
	PropTagsArmVc   = 8,
	PropTagsVcArm   = 9,
}

#[repr(align(16))]
struct MailboxPropBuffer([u32; 0x2000], usize);

static mut MAILBOX_PROP_BUF: MailboxPropBuffer = MailboxPropBuffer([0u32; 0x2000], 0);

const MAILBOX_PROP_PROCESS: u32 = 0x0000_0000;
const MAILBOX_PROP_SUCCESS: u32 = 0x8000_0000;
const MAILBOX_PROP_ERROR:   u32 = 0x8000_0001;

pub struct MailboxPropertyRequestBuilder<'a> {
	mailbox: &'a mut Mailbox,
	buf:     &'a mut [u32],
	len:     usize,
}

impl<'a> MailboxPropertyRequestBuilder<'a> {
	#[inline]
	pub fn add(mut self, tag: Tag, value: &[u32]) -> Self {
		self.buf[self.len] = tag as _;
		self.buf[self.len + 1] = (value.len() << 2) as _;
		self.buf[self.len + 2] = 0;
		self.buf[self.len + 3..self.len + 3 + value.len()].copy_from_slice(value);
		self.len += value.len() + 3;
		self
	}
	
	#[inline]
	pub fn send(self) -> MailboxPropertyResponse<'a> {
		self.buf[0] = (self.len as u32 + 1) << 2;
		self.buf[1] = MAILBOX_PROP_PROCESS;
		self.buf[self.len as usize] = Tag::End as _;
		self.mailbox.write(Channel::PropTagsArmVc, self.buf.as_ptr() as usize as u32);
		MailboxPropertyResponse {
			mailbox: self.mailbox,
			buf:     self.buf
		}
	}
}

pub struct MailboxPropertyResponse<'a> {
	mailbox: &'a mut Mailbox,
	buf:     &'a mut [u32]
}

impl MailboxPropertyResponse<'_> {
	#[inline]
	pub fn receive(self) -> Result<Self, ()> {
		self.mailbox.read(Channel::PropTagsArmVc);
		match self.buf[1] {
			MAILBOX_PROP_SUCCESS => Ok(self),
			_ => Err(())
		}
	}
	
	#[inline]
	pub fn get(&self, tag: Tag) -> Option<&[u32]> {
		let mut buf = &self.buf[2..(self.buf[0] >> 2) as usize];
		while !buf.is_empty() {
			if buf[0] != tag as u32 {
				buf = &buf[((buf[1] >> 2) + 3) as usize..];
			} else if buf[0] == Tag::End as u32 {
				return None
			} else {
				return Some(&buf[3..3 + ((buf[2] & 0xFFFF) >> 2) as usize])
			}
		}
		None
	}
}

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Tag {
	End                          = 0x0_0000,
	GetFirmwareRevision          = 0x0_0001,
	GetBoardModel                = 0x1_0001,
	GetBoardRevision             = 0x1_0002,
	GetBoardMacAddress           = 0x1_0003,
	GetBoardSerial               = 0x1_0004,
	GetArmMemory                 = 0x1_0005,
	GetVcMemory                  = 0x1_0006,
	GetClocks                    = 0x1_0007,
	GetCommandLine               = 0x5_0001,
	GetDmaChannels               = 0x6_0001,
	GetPowerState                = 0x2_0001,
	GetTiming                    = 0x2_0002,
	SetPowerState                = 0x2_8001,
	GetClockState                = 0x3_0001,
	SetClockState                = 0x3_8001,
	GetClockRate                 = 0x3_0002,
	SetClockRate                 = 0x3_8002,
	GetMaxClockRate              = 0x3_0004,
	GetMinClockRate              = 0x3_0007,
	GetTurbo                     = 0x3_0009,
	SetTurbo                     = 0x3_8009,
	GetVoltage                   = 0x3_0003,
	SetVoltage                   = 0x3_8003,
	GetMaxVoltage                = 0x3_0005,
	GetMinVoltage                = 0x3_0008,
	GetTemperature               = 0x3_0006,
	GetMaxTemperature            = 0x3_000A,
	AllocateMemory               = 0x3_000C,
	LockMemory                   = 0x3_000D,
	UnlockMemory                 = 0x3_000E,
	ReleaseMemory                = 0x3_000F,
	ExecuteCode                  = 0x3_0010,
	GetDispmanxResourceMemHandle = 0x3_0014,
	GetEdidBlock                 = 0x3_0020,
	AllocateBuffer               = 0x4_0001,
	ReleaseBuffer                = 0x4_8001,
	BlankScreen                  = 0x4_0002,
	GetPhysicalResolution        = 0x4_0003,
	TestPhysicalResolution       = 0x4_4003,
	SetPhysicalResolution        = 0x4_8003,
	GetVirtualResolution         = 0x4_0004,
	TestVirtualResolution        = 0x4_4004,
	SetVirtualResolution         = 0x4_8004,
	GetDepth                     = 0x4_0005,
	TestDepth                    = 0x4_4005,
	SetDepth                     = 0x4_8005,
	GetPixelOrder                = 0x4_0006,
	SetPixelOrder                = 0x4_8006,
	GetAlphaMode                 = 0x4_0007,
	SetAlphaMode                 = 0x4_4007,
	GetPitch                     = 0x4_0008,
	GetVirtualOffset             = 0x4_0009,
	TestVirtualOffset            = 0x4_4009,
	SetVirtualOffset             = 0x4_8009,
	GetOverScan                  = 0x4_000A,
	TestOverScan                 = 0x4_400A,
	SetOverScan                  = 0x4_800A,
	GetPalette                   = 0x4_000B,
	TestPalette                  = 0x4_400B,
	SetPalette                   = 0x4_800B,
	SetCursorInfo                = 0x0_8010,
	GetCursorState               = 0x0_8011
}

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PowerState {
	On   = 0x1,
	Wait = 0x2
}

#[repr(u32)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum PowerStateResponse {
	On                 = 0x1,
	DeviceDoesNotExist = 0x2
}