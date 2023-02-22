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

use core::mem::size_of;
use super::peripherals;
use crate::arch::volatile;
use core::time::Duration;

const MAX_CHANNELS:   usize = 16;
const DATA_FIFO_SIZE: usize = 0x1000;

/// (legacy) Design Ware Host Controller Interface
#[repr(C, align(0x1000))]
pub struct Dwhci {
	core: DwhciCore,
	_pad0: [u8; 0x400 - size_of::<DwhciCore>()],
	host: DwhciHost,
	_pad1: [u8; 0xE00 - size_of::<DwhciCore>() - size_of::<DwhciHost>()],
	power: u32,
	fifo:  [DwhciFifo; MAX_CHANNELS]
}

#[repr(C)]
struct DwhciCore {
	otg_ctrl:             u32,
	otg_int:              u32,
	ahb_cfg:              u32,
	usb_cfg:              u32,
	reset:                u32,
	int_stat:             u32,
	int_mask:             u32,
	rx_stat_rd:           u32,
	rx_stat_rop:          u32,
	rx_fifo_siz:          u32,
	nper_tx_fifo_siz:     u32,
	nper_tx_stat:         u32,
	i2c_ctrl:             u32,
	phy_vendor_ctrl:      u32,
	gpio:                 u32,
	user_id:              u32,
	vendor_id:            u32,
	hw_cfg:               [u32; 4],
	lpm_cfg:              u32,
	power_down:           u32,
	dfifo_cfg:            u32,
	adp_ctrl_:            u32,
	_pad0:                [u32; 7],
	vendor_mdio_ctrl:     u32,
	vendor_mdio_data:     u32,
	vendor_vbus_drv:      u32,
	_pad1:                [u32; 0x1C],
	host_per_tx_fifo_siz: u32,
	dev:                  [DwhciCoreDev; 15]
}

#[repr(C)]
struct DwhciCoreDev {
	dev_per_tx_fifo: u32,
	dev_tx_fifo:     u32
}

#[repr(C)]
struct DwhciHost {
	cfg:                   u32,
	frm_interval:          u32,
	frm_num:               u32,
	_pad0:                 u32,
	per_tx_fifo_stat:      u32,
	all_channels_int:      u32,
	all_channels_int_mask: u32,
	frm_lst_base:          u32,
	port:                  u32,
	_pad1:                 [u32; 0x2F],
	channels:              [DwhciHostChannel; MAX_CHANNELS]
}

#[repr(C, align(0x20))]
struct DwhciHostChannel {
	character:  u32,
	split_ctrl: u32,
	int:        u32,
	int_mask:   u32,
	xfer_siz:   u32,
	dma_addr:   u32,
	_pad:       u32,
	dma_buf:    u32
}

#[repr(C, align(0x1000))]
struct DwhciFifo([u8; 0x1000]);

#[repr(u32)]
enum AhbCfg {
	GlobalInterruptMask = 0x0001,
	MaxAxiBurst         = 0x0006,
	WaitAxiWrites       = 0x0010,
	DmaEnabled          = 0x0020,
	NpTxfEmpLvl         = 0x0080,
	PTxfEmpLvl          = 0x0100,
}

#[repr(u32)]
enum UsbCfg {
	ToutCal        = 0x0000_0007,
	PhyIf          = 0x0000_0008,
	UlpiUtmiSel    = 0x0000_0010,
	FsIntf         = 0x0000_0020,
	PhySel         = 0x0000_0040,
	DdrSel         = 0x0000_0080,
	SrpCapable     = 0x0000_0100,
	HnpCapable     = 0x0000_0200,
	UsbTrdTim      = 0x0000_3c00,
	PhyLpweClkSel  = 0x0000_8000,
	OtgI2cSel      = 0x0001_0000,
	UlpiFsLs       = 0x0002_0000,
	UlpiAutoRes    = 0x0004_0000,
	UlpiClkSusM    = 0x0008_0000,
	UlpiExtVbusDrv = 0x0010_0000,
	UlpiExtVbusInd = 0x0020_0000,
	TermSelDlPulse = 0x0040_0000,
	IndComp        = 0x0080_0000,
	IndPassThru    = 0x0100_0000,
	UlpiIfProtDis  = 0x0200_0000,
	ForceHstMode   = 0x2000_0000,
	ForceDevMode   = 0x4000_0000,
	CorruptTx      = 0x8000_0000,
}

#[repr(u32)]
enum Reset {
	CoreSoftReset = 0x0000_0001,
	HostSoftReset = 0x0000_0002,
	FrmCntrReset  = 0x0000_0004,
	IntTknQFlush  = 0x0000_0008,
	RxFifoFlush   = 0x0000_0010,
	TxFifoFlush   = 0x0000_0020,
	TxFifoNum     = 0x0000_07c0,
	DmaReq        = 0x4000_0000,
	AhbIdle       = 0x8000_0000,
}

#[repr(u32)]
enum IntMask {
	CurrentMode      = 0x0000_0001,
	ModeMismatch     = 0x0000_0002,
	OtgInt           = 0x0000_0004,
	Soft             = 0x0000_0008,
	RxFifoLevel      = 0x0000_0010,
	NpTxFifoEmp      = 0x0000_0020,
	GinNNakEff       = 0x0000_0040,
	GoutNakEff       = 0x0000_0080,
	UlpiCkint        = 0x0000_0100,
	I2cInt           = 0x0000_0200,
	ErlySusp         = 0x0000_0400,
	UsbSusp          = 0x0000_0800,
	UsbReset         = 0x0000_1000,
	EnumDone         = 0x0000_2000,
	IsoOutDrop       = 0x0000_4000,
	Eopf             = 0x0000_8000,
	EpMismatch       = 0x0002_0000,
	IepInt           = 0x0004_0000,
	OepInt           = 0x0008_0000,
	IncompleteIsoIn  = 0x0010_0000,
	IncompleteIsoOut = 0x0020_0000,
	FetSusp          = 0x0040_0000,
	PtrInt           = 0x0100_0000,
	HchInt           = 0x0200_0000,
	PTxFifoEmp       = 0x0400_0000,
	ConIdStsChng     = 0x1000_0000,
	DisconnectInt    = 0x2000_0000,
	SessReqInt       = 0x4000_0000,
	WakeUpInt        = 0x8000_0000
}

#[repr(u32)]
enum HwCfg1 {
	Mode           = 0x0007,
	Arch           = 0x0018,
	SinglePoint    = 0x0020,
	HsPhyInterface = 0x00c0,
	FsPhyInterface = 0x0300,
}

#[repr(u32)]
enum HwCfg1HsPhyType {
	NotSupported = 0 << 6,
	Utmi         = 1 << 6,
	Ulpi         = 2 << 6,
	UtmiUlpi     = 3 << 6
}

#[repr(u32)]
enum HwCfg1FsPhyType {
	Dedicated = 1 << 8
}

#[repr(u32)]
enum Config {
	LsPhyClkSel = 0x3,
	LsSupp      = 0x4
}

#[repr(u32)]
enum ConfigLsPhyClkSel {
	Sel3060Mhz = 0x0,
	Sel48Mhz   = 0x1,
	Sel6Mhz    = 0x2
}

#[repr(u32)]
enum Power {
	StopPClock      = 0x1,
	GateHClock      = 0x2,
	PowerClmp       = 0x4,
	ResetPdwnModule = 0x8
}

#[repr(u32)]
enum HostPort {
	Connect            = 0x0000_0001,
	ConnectChanged     = 0x0000_0002,
	Enable             = 0x0000_0004,
	EnableChanged      = 0x0000_0008,
	OverCurrent        = 0x0000_0010,
	OverCurrentChanged = 0x0000_0020,
	Reset              = 0x0000_0100,
	Power              = 0x0000_1000,
	Speed              = 0x0006_0000,
	SpeedHigh          = !0x0006_0000,
	SpeedFull          = 0x0002_0000,
	SpeedLow           = 0x0004_0000,
	DefaultMask = Self::ConnectChanged as u32
		| Self::Enable as u32
		| Self::EnableChanged as u32
		| Self::OverCurrentChanged as u32
}



enum Error {
	UnknownVendor(u32),
	PowerOnFailed,
	CoreInitFailed,
	HostInitFailed,
	RootPortEnableFailed
}

#[repr(u32)]
enum DeviceId {
	SdCard = 0x0,
	UsbHcd = 0x3
}

fn init(set_int: fn(fn(*mut u8), *mut u8)) -> core::result::Result<(), Error> {
	use super::mailbox::*;
	
	unsafe { crate::arch::data_mem_barrier(); }
	
	let dwhci = &mut peripherals().dwhci;
	
	// check vendor id
	
	if dwhci.core.vendor_id != 0x4F54280A {
		return Err(Error::UnknownVendor(dwhci.core.vendor_id));
	}
	
	// set power state
	
	let response = Mailbox::get()
		.prop_tag_builder()
		.add(Tag::SetPowerState, &[DeviceId::UsbHcd as u32, PowerState::On as u32 | PowerState::Wait as u32])
		.send()
		.receive()
		.map_err(|_| Error::PowerOnFailed)?;
	
	let tag = response.get(Tag::SetPowerState).unwrap();
	
	if tag[1] & PowerStateResponse::DeviceDoesNotExist as u32 != 0
		|| tag[1] & PowerStateResponse::On as u32 == 0 {
		return  Err(Error::PowerOnFailed);
	}
	
	// disable interrupts
	
	dwhci.core.ahb_cfg &= !(AhbCfg::GlobalInterruptMask as u32);
	set_int(interrupt_handler, 0 as _);
	
	// init core
	
	dwhci.core.usb_cfg &= !(UsbCfg::UlpiExtVbusDrv as u32 | UsbCfg::TermSelDlPulse as u32);
	
	// reset device
	
	if !wait(|| dwhci.core.reset & Reset::AhbIdle as u32 != 0, 100) {
		return Err(Error::CoreInitFailed);
	}
	
	dwhci.core.reset |= Reset::CoreSoftReset as u32;
	
	if !wait(|| dwhci.core.reset & Reset::CoreSoftReset as u32 == 0, 10) {
		return Err(Error::CoreInitFailed);
	}
	
	sleep(Duration::from_millis(100));
	
	// init core
	
	dwhci.core.usb_cfg &= !(UsbCfg::UlpiUtmiSel as u32 | UsbCfg::PhyIf as u32);
	
	if dwhci.core.hw_cfg[1] & HwCfg1::HsPhyInterface as u32 == HwCfg1HsPhyType::Ulpi as u32
		&& dwhci.core.hw_cfg[1] & HwCfg1::FsPhyInterface as u32 == HwCfg1FsPhyType::Dedicated as u32 {
		dwhci.core.usb_cfg |= UsbCfg::UlpiFsLs as u32 | UsbCfg::UlpiClkSusM as u32;
	} else {
		dwhci.core.usb_cfg &= !(UsbCfg::UlpiFsLs as u32 | UsbCfg::UlpiClkSusM as u32);
	}
	
	dwhci.core.ahb_cfg |= AhbCfg::DmaEnabled as u32 | AhbCfg::WaitAxiWrites as u32;
	dwhci.core.ahb_cfg &= !(AhbCfg::MaxAxiBurst as u32);
	dwhci.core.usb_cfg &= !(UsbCfg::HnpCapable as u32 | UsbCfg::SrpCapable as u32);
	
	// enable interrupts
	
	dwhci.core.int_stat = !0;
	dwhci.core.ahb_cfg |= AhbCfg::GlobalInterruptMask as u32;
	
	// init host
	
	dwhci.power = 0;
	dwhci.host.cfg &= !(Config::LsPhyClkSel as u32);
	
	dwhci.host.cfg |= if dwhci.core.hw_cfg[1] & HwCfg1::HsPhyInterface as u32 == HwCfg1HsPhyType::Ulpi as u32
		&& dwhci.core.hw_cfg[1] & HwCfg1::FsPhyInterface as u32 == HwCfg1FsPhyType::Dedicated as u32
		&& dwhci.core.usb_cfg & UsbCfg::UlpiFsLs as u32 != 0 {
		ConfigLsPhyClkSel::Sel48Mhz as u32
	} else {
		ConfigLsPhyClkSel::Sel3060Mhz as u32
	};
	
	// flush tx fifo
	
	dwhci.core.reset = Reset::TxFifoFlush as u32 | 0x10 << 6;
	if wait(|| dwhci.core.reset & Reset::TxFifoFlush as u32 == 0, 10) {
		sleep(Duration::from_millis(1));
	}
	
	// flush rx fifo
	
	dwhci.core.reset = Reset::RxFifoFlush as u32;
	if wait(|| dwhci.core.reset & Reset::RxFifoFlush as u32 == 0, 10) {
		sleep(Duration::from_millis(1));
	}
	
	// host port power
	
	if dwhci.host.port & HostPort::Power as u32 == 0 {
		dwhci.host.port |= HostPort::Power as u32;
	}
	
	// enable interrupts
	
	dwhci.core.int_mask = 0;
	dwhci.core.int_stat = !0;
	dwhci.core.int_mask |= IntMask::HchInt as u32;
	
	// enable root port
	
	if !wait(|| dwhci.host.port & HostPort::Connect as u32 != 0, 20) {
		return Err(Error::RootPortEnableFailed);
	}
	
	sleep(Duration::from_millis(100));
	dwhci.host.port = dwhci.host.port & !(HostPort::DefaultMask as u32) | HostPort::Reset as u32;
	sleep(Duration::from_millis(50));
	dwhci.host.port &= !(HostPort::DefaultMask as u32 | HostPort::Reset as u32);
	sleep(Duration::from_millis(10));
	
	// init root port
	
	/*match dwhci.host.port & HostPort::Speed as u32 {
	
	}*/
	
	unsafe { crate::arch::data_mem_barrier(); }
	
	Ok(())
}

fn init_usb_device() {

}

struct UsbDevice;

impl UsbDevice {
	fn init() -> Self {
		Self
	}
}

struct DwhciDevice {
	channel_mask: usize,
	channels_max: usize
}

impl DwhciDevice {
	fn submit_blocking_request(&self) {
		unsafe { crate::arch::data_mem_barrier(); }
		
		unsafe { crate::arch::data_mem_barrier(); }
	}
	
	fn transfer_stage(&mut  self) {
		// allocate channel
		let channel = self.alloc_channel().unwrap();
		
		// enable interrupts
		peripherals().dwhci.host.all_channels_int_mask |= 1 << channel as u32;
		
		// start transaction
		self.start_transaction(&TransferStageData {});
	}
	
	fn alloc_channel(&mut self) -> Option<usize> {
		for i in 0..self.channels_max {
			if self.channel_mask & (1 << i) == 0 {
				self.channel_mask |= 1 << i;
				return Some(i);
			}
		}
		None
	}
	
	fn start_transaction(&self, data: &TransferStageData) {
	
	}
}

struct TransferStageData {

}

fn interrupt_handler(state: *mut u8) {

}

#[inline]
fn wait(condition: impl Fn() -> bool, mut timeout: u32) -> bool {
	while !condition() && timeout > 0 {
		timeout -= 1;
		sleep(Duration::from_millis(1));
	}
	timeout != 0
}

fn sleep(d: Duration) {
	for _ in 0..d.as_nanos() { volatile() }
}