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

use crate::dri::bcm2711::peripherals;

#[repr(C, align(0x1000))]
pub struct Emmc {
	arg2:              u32,
	block_size_count:  u32,
	arg1:              u32,
	cmd_tf_mode:       u32,
	resp:              [u32; 4],
	data:              u32,
	stat:              u32,
	ctrl:              [u32; 2],
	int:               u32,
	int_mask:          u32,
	int_enable:        u32,
	ctrl2:             u32,
	int_force:         u32,
	boot_timeout:      u32,
	dbg_sel:           u32,
	ext_fifo_cfg:      u32,
	ext_fifo_enable:   u32,
	tune_step:         u32,
	tune_step_std:     u32,
	tune_step_ddr:     u32,
	spi_int_spt:       u32,
	slot_int_stat_ver: u32
}

#[repr(u32)]
enum EmmcStat {
	CmdInhibit    = 0x0000_0001,
	DataInhibit   = 0x0000_0002,
	DataActive    = 0x0000_0004,
	RetuningReq   = 0x0000_0008,
	WriteTransfer = 0x0000_0100,
	ReadTransfer  = 0x0000_0200,
	NewWriteData  = 0x0000_0400,
	NewReadData   = 0x0000_0800,
	CardInsert    = 0x0001_0000,
	CardStable    = 0x0002_0000,
	CardDetect    = 0x0004_0000,
	WriteProtect  = 0x0008_0000,
	DataLevel0    = 0x00F0_0000,
	CmdLevel      = 0x0100_0000,
	DataLevel1    = 0x1E00_0000
}

#[repr(u32)]
enum EmmcCtrl0 {
	HostCtrlLed      = 0x0000_0001,
	HostCtrlDWidth   = 0x0000_0002,
	HostCtrlHsEnable = 0x0000_0004,
	HostCtrlDma      = 0x0000_0018,
	HostCtrl8Bit     = 0x0000_0020,
	HostCtrlCreddet  = 0x0000_0040,
	HostCtrlCreddetS = 0x0000_0080,
	PowerCtrlOn      = 0x0000_0100,
	PowerCtrlSdVolts = 0x0000_0E00,
	PowerCtrlHwrst   = 0x0000_1000,
	GapStop          = 0x0001_0000,
	GapRestart       = 0x0002_0000,
	ReadWaitEnable   = 0x0004_0000,
	GapIen           = 0x0008_0000,
	SpiMode          = 0x0010_0000,
	BootEnable       = 0x0020_0000,
	AltBootEnable    = 0x0040_0000,
	WakeOnIntEnable  = 0x0100_0000,
	WakeOnInsEnable  = 0x0200_0000,
	WakeOnRemEnable  = 0x0400_0000,
}

#[repr(u32)]
enum EmmcCtrl1 {
	ClockIntLen  = 0x0000_0001,
	ClockStable  = 0x0000_0002,
	ClockEnable  = 0x0000_0004,
	ClockGenSel  = 0x0000_0020,
	ClockFreqMs2 = 0x0000_00C0,
	ClockFreq8   = 0x0000_FF00,
	SrstHc       = 0x0100_0000,
	SrstCmd      = 0x0200_0000,
	SrstData     = 0x0400_0000,
}

#[repr(u32)]
enum EmmcCtrl2 {
	AcNoxErr   = 0x0000_0001,
	AcToErr    = 0x0000_0002,
	AcCrcErr   = 0x0000_0004,
	AcEndErr   = 0x0000_0008,
	AcBadErr   = 0x0000_0010,
	Notc12Err  = 0x0000_0080,
	UhsMode    = 0x0007_0000,
	SigType    = 0x0008_0000,
	DrvType    = 0x0030_0000,
	TuneOn     = 0x0040_0000,
	Tuned      = 0x0080_0000,
	EnableAint = 0x4000_0000,
	EnablePsv  = 0x8000_0000,
}

#[repr(u32)]
enum EmmcInterrupts {
	CmdDone       = 0x0000_0001,
	DataDone      = 0x0000_0002,
	BlockGap      = 0x0000_0004,
	Dma           = 0x0000_0008,
	WriteReady    = 0x0000_0010,
	ReadReady     = 0x0000_0020,
	CardIn        = 0x0000_0040,
	CardOut       = 0x0000_0080,
	Card          = 0x0000_0100,
	IntA          = 0x0000_0200,
	IntB          = 0x0000_0400,
	IntC          = 0x0000_0800,
	Retune        = 0x0000_1000,
	BootAck       = 0x0000_2000,
	EndBoot       = 0x0000_4000,
	Error         = 0x0000_8000,
	CtoErr        = 0x0001_0000,
	CcrcErr       = 0x0002_0000,
	CendErr       = 0x0004_0000,
	CbadErr       = 0x0008_0000,
	DtoErr        = 0x0010_0000,
	DcrcErr       = 0x0020_0000,
	DendErr       = 0x0040_0000,
	SdOffErr      = 0x0080_0000,
	ACmdErr       = 0x0100_0000,
	ADmaErr       = 0x0200_0000,
	TuneErr       = 0x0400_0000,
	DmaErr        = 0x1000_0000,
	AtaErr        = 0x2000_0000,
	OemErr        = 0xC000_0000
}

pub struct SdCard {
	block_size: usize
}

enum SdCardType {
	Mmc,
	Type1,
	Type2Sc,
	Type2Hc
}

impl SdCard {
	pub fn get() -> Self {
		Self { block_size: 512 }
	}
	
	fn read(&mut self, off: usize, buf: &mut [u32]) -> Result<(), ()> {
		if buf.as_ptr() as usize & 0x3 != 0 {
			panic!("unaligned buffer")
		}
		
		let blocks = buf.len() >> 7;
		let mut buf = unsafe { core::slice::from_raw_parts_mut::<u32>(
			buf.as_mut_ptr() as _, buf.len() & 0x1FF >> 2) };
		
		while !buf.is_empty() {
			// TODO wait for interrupt
			
			// write single block
			for _ in 0..128 {
				buf[0] = peripherals().emmc.data;
				buf = &mut buf[..1];
			}
		}
		Ok(())
	}
	
	fn write(&mut self, off: usize, buf: &[u32]) -> Result<(), ()> {
		unimplemented!()
	}
}