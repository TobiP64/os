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

#![allow(dead_code)]
// \tASB_([a-zA-Z0-9_]*).*

use core::mem::size_of;
use self::{dwhci::Dwhci, emmc::Emmc};
use crate::mem::MemoryMap;

pub mod mailbox;
pub mod framebuffer;
pub mod gpio;
pub mod emmc;
pub mod dwhci;

pub const ID: usize = 0x0000_0000;

const CPUID_BCM2708: u32     = 0x410F_B767;
const CPUID_BCM2709: u32     = 0x410F_C075;
const CPUID_BCM2710: u32     = 0x410F_D034;
const CPUID_BCM2711: u32     = 0x0000_0000;
const PERI_SIZE:     usize   = size_of::<Peripherals>();

pub static MEMORY_MAP: MemoryMap = &[
	("SDRAM",     0x0_0000_0000..0x0_FC00_0000),
	("MAIN-PERI", 0x0_FC00_0000..0x0_FF80_0000),
	("LOCL-PERI", 0x0_FF80_0000..0x1_0000_0000),
	("SDRAM",     0x1_0000_0000..0x4_0000_0000),
	("L2-ALLOC",  0x4_0000_0000..0x4_4000_0000),
	("L2-NALLOC", 0x4_8000_0000..0x4_C000_0000),
	("PCIE",      0x6_0000_0000..0x7_FFFF_FFFF)
];

pub static INTERRUPT_MAP: &[(u32, &str)] = &[
	// private
	(26, "HP timer"),
	(27, "V timer"),
	(28, "legacy FIQ"),
	(29, "PS timer"),
	(30, "PNS timer"),
	(31, "legacy IRQ"),
	// shared
	(32, "ARM Mailbox 0"),
	(33, "ARM Mailbox 1"),
	(34, "ARM Mailbox 2"),
	(35, "ARM Mailbox 3"),
	(36, "ARM Mailbox 4"),
	(37, "ARM Mailbox 5"),
	(38, "ARM Mailbox 6"),
	(39, "ARM Mailbox 7"),
	(40, "ARM Mailbox 8"),
	(41, "ARM Mailbox 9"),
	(42, "ARM Mailbox 10"),
	(43, "ARM Mailbox 11"),
	(44, "ARM Mailbox 12"),
	(45, "ARM Mailbox 13"),
	(46, "ARM Mailbox 14"),
	(47, "ARM Mailbox 15"),
	(48, "Core 0 PMU"),
	(49, "Core 1 PMU"),
	(50, "Core 2 PMU"),
	(51, "Core 3 PMU"),
	(52, "AXIERR"),
	(53, "local timer"),
	// ARMC
	(64, "timer"),
	(65, "mailbox"),
	(66, "doorbell 0"),
	(67, "doorbell 1"),
	(68, "VPU0 halted"),
	(69, "VPU1 halted"),
	(70, "ARM address error"),
	(71, "ARM AXI error"),
	(72, "Software Interrupt 0"),
	(73, "Software Interrupt 1"),
	(74, "Software Interrupt 2"),
	(75, "Software Interrupt 3"),
	(76, "Software Interrupt 4"),
	(77, "Software Interrupt 5"),
	(78, "Software Interrupt 6"),
	(79, "Software Interrupt 7"),
	// VC
	(96, "timer 0"),
	(97, "timer 1"),
	(98, "timer 2"),
	(99, "timer 3"),
	(100, "H264 0"),
	(101, "H264 1"),
	(102, "H264 2"),
	(103, "JPEG"),
	(104, "ISP"),
	(105, "USB"),
	(106, "V3D"),
	(107, "Transposer"),
	(108, "Multicore Sync 0"),
	(109, "Multicore Sync 1"),
	(110, "Multicore Sync 2"),
	(111, "Multicore Sync 3"),
	(112, "DMA 0"),
	(113, "DMA 1"),
	(114, "DMA 2"),
	(115, "DMA 3"),
	(116, "DMA 4"),
	(117, "DMA 5"),
	(118, "DMA 6"),
	(119, "DMA 7 & 8"),
	(120, "DMA 9 & 10"),
	(121, "DMA 11"),
	(122, "DMA 12"),
	(123, "DMA 13"),
	(124, "DMA 14"),
	(125, "AUX"),
	(126, "ARM"),
	(127, "DMA 15"),
	(128, "HDMI CEC"),
	(129, "HVS"),
	(130, "RPIVID"),
	(131, "SDC"),
	(132, "DSI 0"),
	(133, "Pixel Valve 2"),
	(134, "Camera 0"),
	(135, "Camera 1"),
	(136, "HDMI 0"),
	(137, "HDMI 1"),
	(138, "Pixel Valve 3"),
	(139, "SPI/BSC Slave"),
	(140, "DSI 1"),
	(141, "Pixel Valve 0"),
	(142, "Pixel Valve 1 & 4"),
	(143, "CPR"),
	(144, "SMI"),
	(145, "GPIO 0"),
	(146, "GPIO 1"),
	(147, "GPIO "),
	(148, "GPIO 3"),
	(149, "OR of all I2C"),
	(150, "OR of all SPI"),
	(151, "PCM/I2S"),
	(152, "SDHOST"),
	(153, "OR of all PL011 UART"),
	(154, "PR of all ETH_PCIe L2"),
	(155, "VEC"),
	(156, "CPG"),
	(157, "RNG"),
	(158, "EMMC & EMMC2"),
	(159, "ETH_PCIe secure"),
	// ETH_PCIe L2
	(169, "AVS"),
	(175, "PCIE_0_INTA"),
	(176, "PCIE_0_INTB"),
	(177, "PCIE_0_INTC"),
	(178, "PCIE_0_INTD"),
	(180, "PCIE_0_MSI"),
	(189, "GENET_0_A"),
	(190, "GENET_0_B"),
	(208, "USB0_XHCI_0"),
];

static mut PERIPHERALS: *mut Peripherals = 0 as _;

#[inline]
pub fn init() {
	unsafe {
		PERIPHERALS = match crate::arch::cpuid() {
			CPUID_BCM2708 => 0x2000_0000u32,
			CPUID_BCM2709 | CPUID_BCM2710 => 0x3F00_0000u32,
			CPUID_BCM2711 => 0xFC00_0000u32,
			_ => panic!()
		} as _;
	}
}

#[inline]
pub fn peripherals() -> &'static mut Peripherals {
	unsafe { core::mem::transmute(PERIPHERALS) }
}

trait Volatile: Sized {
	#[inline]
	fn ref_write_volatile(&mut self, val: Self) {
		unsafe { (self as *mut Self).write_volatile(val) }
	}
	
	#[inline]
	fn ref_read_volatile(&mut self) -> Self {
		unsafe { (self as *const Self).read_volatile() }
	}
}

impl<T> Volatile for T {}

#[repr(C)]
pub struct Peripherals {
	ms:     Ms,                        // 0x00_0000
	ccp2tx: Ccp2tx,                    // 0x00_1000
	ic:     Ic,                        // 0x00_2000
	timer:  SystemTimer,               // 0x00_3000 *
	txp:    Txp,                       // 0x00_4000
	jp:     Jp,                        // 0x00_5000
	mphi:   Mphi,                      // 0x00_6000
	dma:    Dma,                       // 0x00_7000 *
	nu:     Nu,                        // 0x00_8000
	sysac:  SysAc,                     // 0x00_9000
	asb:    Asb,                       // 0x00_A000
	arm:    Arm,                       // 0x00_B000
	_pad0:  [EmptyPeripheral; 0xF4],   // gap
	pm:     PowerManager,              // 0x10_0000
	cm:     ClockManager,              // 0x10_1000
	a2w:    A2w,                       // 0x10_2000
	_pad1:  EmptyPeripheral,           // 0x10_3000
	rng:    Rng,                       // 0x10_4000
	_pad2:  [EmptyPeripheral; 0xFB],   // gap
	gpio:   gpio::GpioRegisters,       // 0x20_0000 *?
	uart:   Uart,                      // 0x20_1000 **
	sh:     Sh,                        // 0x20_2000
	pcm:    Pcm,                       // 0x20_3000 *
	spi:    SerialPeripheralInterface, // 0x20_4000 **
	i2c0:   I2c,                       // 0x20_5000 *
	pv0:    PixelValve,                // 0x20_6000
	pv1:    PixelValve,                // 0x20_7000
	dpi:    DisplayParallelInterface,  // 0x20_8000
	dsi:    DisplaySerialInterface,    // 0x20_9000
	_pad3:  EmptyPeripheral,           // 0x20_A000
	tb:     Tb,                        // 0x20_B000
	pwm:    PulseWidthModulator,       // 0x20_C000 *
	prm:    Prm,                       // 0x20_D000
	te:     Te,                        // 0x20_E000
	otp:    OneTimeProgrammable,       // 0x20_F000
	slim:   Slim,                      // 0x21_0000
	cpg:    Cpg,                       // 0x21_1000
	ts:     Ts,                        // 0x21_2000
	_pad4:  EmptyPeripheral,           // 0x21_3000
	slv:    I2cSpiSlave,               // 0x21_4000
	aux:    Auxiliaries,               // 0x21_5000 *
	_pad5:  [EmptyPeripheral; 0x2A],   // gap
	ao:     AveOut,                    // 0x24_0000
	_pad6:  [EmptyPeripheral; 0xBF],   // gap
	emmc:   Emmc,                      // 0x30_0000
	_pad7:  [EmptyPeripheral; 0xFF],   // gap
	sca:    Scaler,                    // 0x40_0000
	_pad8:  [EmptyPeripheral; 0x3FF],  // gap
	cam0:   Cam,                       // 0x80_0000
	cam1:   Cam,                       // 0x80_1000
	cmi:    Cmi,                       // 0x80_2000
	_pad9:  EmptyPeripheral,           // 0x80_3000
	i2c1:   I2c,                       // 0x80_4000 *
	i2c2:   I2c,                       // 0x80_5000 *?
	vec:    Vec,                       // 0x80_6000
	pv2:    PixelValve,                // 0x80_7000
	_pad10: [EmptyPeripheral; 0x179],  // gap
	dwhci:  Dwhci,                     // 0x98_0000
	_pad11: [EmptyPeripheral; 0x47F],  // gap
	sdram:  Sdram,                     // 0xE0_0000
	_pad12: [EmptyPeripheral; 0x200]   // gap
}

impl core::fmt::Debug for Peripherals {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		f.debug_struct("Peripherals")
			.field("ms", &(&peripherals().ms as *const _))
			.field("ccp2tx", &(&peripherals().ccp2tx as *const _))
			.field("ic", &(&peripherals().ic as *const _))
			.field("timer", &(&peripherals().timer as *const _))
			.field("txp", &(&peripherals().txp as *const _))
			.field("jp", &(&peripherals().jp as *const _))
			.field("mphi", &(&peripherals().mphi as *const _))
			.field("dma", &(&peripherals().dma as *const _))
			.field("nu", &(&peripherals().nu as *const _))
			.field("sysac", &(&peripherals().sysac as *const _))
			.field("asb", &(&peripherals().asb as *const _))
			.field("arm", &(&peripherals().arm as *const _))
			.field("pm", &(&peripherals().pm as *const _))
			.field("cm", &(&peripherals().cm as *const _))
			.field("a2w", &(&peripherals().a2w as *const _))
			.field("rng", &(&peripherals().rng as *const _))
			.field("gpio", &(&peripherals().gpio as *const _))
			.field("uart", &(&peripherals().uart as *const _))
			.field("sh", &(&peripherals().sh as *const _))
			.field("pcm", &(&peripherals().pcm as *const _))
			.field("kernel-svi", &(&peripherals().spi as *const _))
			.field("i2c0", &(&peripherals().i2c0 as *const _))
			.field("pv0", &(&peripherals().pv0 as *const _))
			.field("pv1", &(&peripherals().pv1 as *const _))
			.field("dpi", &(&peripherals().dpi as *const _))
			.field("dsi", &(&peripherals().dsi as *const _))
			.field("tb", &(&peripherals().tb as *const _))
			.field("pwm", &(&peripherals().pwm as *const _))
			.field("prm", &(&peripherals().prm as *const _))
			.field("te", &(&peripherals().te as *const _))
			.field("otp", &(&peripherals().otp as *const _))
			.field("slim", &(&peripherals().slim as *const _))
			.field("cpg", &(&peripherals().cpg as *const _))
			.field("ts", &(&peripherals().ts as *const _))
			.field("slv", &(&peripherals().slv as *const _))
			.field("aux", &(&peripherals().aux as *const _))
			.field("ao", &(&peripherals().ao as *const _))
			.field("emmc", &(&peripherals().emmc as *const _))
			.field("sca", &(&peripherals().sca as *const _))
			.field("cam0", &(&peripherals().cam0 as *const _))
			.field("cam1", &(&peripherals().cam1 as *const _))
			.field("cmi", &(&peripherals().cmi as *const _))
			.field("i2c1", &(&peripherals().i2c1 as *const _))
			.field("i2c2", &(&peripherals().i2c2 as *const _))
			.field("vec", &(&peripherals().vec as *const _))
			.field("pv2", &(&peripherals().pv2 as *const _))
			.field("usb", &(&peripherals().dwhci as *const _))
			.finish()
	}
}

unsafe impl Send for Peripherals {}
unsafe impl Sync for Peripherals {}

#[repr(C, align(0x1000))]
struct EmptyPeripheral(u32);

#[repr(C, align(0x1000))]
struct Ms {
	sema:     [u32; 32],
	status:   u32,
	irq:      [u32; 2],
	ic_set:   [u32; 2],
	ic_clr:   [u32; 2],
	mbox:     [u32; 8],
	vpu_sema: [u32; 2],
	vpu_stat: u32
}

#[repr(C, align(0x1000))]
struct Ccp2tx {
	tc:      u32,
	ts:      u32,
	tac:     u32,
	tpc:     u32,
	tsc:     u32,
	tic:     u32,
	ttc:     u32,
	tba:     u32,
	tdl:     u32,
	td:      u32,
	t_spare: u32
}

#[repr(C, align(0x1000))]
struct Ic {
	ic0: Ic_,
	_pad: [u8; 0x800 - size_of::<Ic_>()],
	ic1: Ic_
}

#[repr(C)]
struct Ic_ {
	ctrl:      u32,
	stat:      u32,
	src:       [u32; 2],
	mask:      [u32; 8],
	vaddr:     u32,
	wake_up:   u32,
	profile:   u32,
	force:     [u32; 2],
	force_set: [u32; 2],
	force_clr: [u32; 2]
}

#[repr(C, align(0x1000))]
struct Txp {
	dst_ptr: u32,
	dst_pitch: u32,
	dim:       u32,
	ctrl:      u32,
	progress:  u32,
	_pad:      u32,
	xtra:      u32
}

#[repr(C, align(0x1000))]
struct SystemTimer {
	cs:  u32,
	clo: u32,
	chi: u32,
	c0:  [u32; 3]
}

#[repr(C, align(0x1000))]
struct Jp {
	ctrl:   u32,
	icst:   u32,
	mctrl:  u32,
	dcctrl: u32,
	cba:    u32,
	ncb:    u32,
	sda:    u32,
	nsb:    u32,
	sbo:    u32,
	mop:    u32,
	haddr : u32,
	hwdata: u32,
	maddr:  u32,
	mwdata: u32,
	oaddr:  u32,
	owdata: u32,
	qaddr:  u32,
	qwdata: u32,
	qctrl:  u32,
	cba_:   [u32; 3],
	cs:     [u32; 3],
	cw:     [u32; 3]
}

#[repr(C, align(0x1000))]
struct Mphi {
	c0indda:  u32,
	c0inddb:  u32,
	c1indda:  u32,
	c1inddb:  u32,
	c0inds:   u32,
	c1inds:   u32,
	c0indcf:  u32,
	c1indcf:  u32,
	c0indfs:  u32,
	c1indfs:  u32,
	outdda:   u32,
	outddb:   u32,
	outds:    u32,
	outdfs:   u32,
	minfs:    u32,
	moutfs:   u32,
	axipriv:  u32,
	rxaxicfg: u32,
	txaxicfg: u32,
	ctrl:     u32,
	intstat:  u32,
	version:  u32,
	intctrl:  u32,
	hsindcf:  u32,
	hsinds:   u32,
	hsindda:  u32,
	hsinddb:  u32,
	hsindfs:  u32,
}

#[repr(C, align(0x1000))]
pub struct Dma {
	channels:   [DmaChannel; 14],
	_pad0:      [u32; 0x6F],
	int_status: u32,
	_pad1:      [u32; 0x3],
	enable:     u32
}

#[repr(C)]
struct DmaChannel {
	cs:              u32,
	ctrl_block_addr: u32,
	transform_info:  u32,
	src_addr:        u32,
	dst_addr:        u32,
	transfer_len:    u32,
	stride:          u32,
	next_ctrl_block: u32,
	debug:           u32,
	_pad:            [u32; 31]
}

#[repr(C, align(0x1000))]
struct Nu {
	host_io_of: u32
}

#[repr(C, align(0x1000))]
struct SysAc {
	host_priority:            u32,
	dbg_priority:             u32,
	hvsm_priority:            u32,
	v3d_priority:             u32,
	h264_priority:            u32,
	jpeg_priority:            u32,
	trans_priority:           u32,
	isp_priority:             u32,
	usb_priority:             u32,
	l2_arbiter_control:       u32,
	uc_arbiter_control:       u32,
	src_arbiter_control:      u32,
	peri_arbiter_control:     u32,
	dma_arbiter_control_uc:   u32,
	dma_arbiter_control_l2:   u32,
	dma_arbiter_control_per:  u32,
	dma_arbiter_control_lite: u32,
	dummy_status:             u32,
	dma_dreq_control:         u32,
	v3d_limiter:              u32,
}

#[repr(C, align(0x1000))]
struct Asb {
	axi_brdg_version: u32,
	cpr_ctrl:         u32,
	v3d_s_ctrl:       u32,
	v3d_m_ctrl:       u32,
	isp_s_ctrl:       u32,
	isp_m_ctrl:       u32,
	h264_s_ctrl:      u32,
	h264_m_ctrl:      u32,
}

#[repr(C, align(0x1000))]
struct Arm {
	control0:    u32,
	id_secure:   u32,
	_pad0:       [u32; 0x3C],
	translate:   u32,
	_pad1:       [u32; 0x3F],
	irq:         ArmIrq,
	_pad2:       [u32; 0x76],
	timer:       ArmTimer,
	control1:    u32,
	status:      u32,
	err_halt:    u32,
	id:          u32,
	_pad3:       [u32; 0xEC],
	cores:       [ArmCore; 4]
}

#[repr(C)]
struct ArmIrq {
	pending:  [u32; 3],
	fiq_ctrl: u32,
	enable:   [u32; 3],
	disable:  [u32; 3]
}

#[repr(u32)]
enum ArmIrqBits {
	ArmTimer           = 0x01,
	ArmMailbox         = 0x02,
	ArmDoorbell0       = 0x04,
	ArmDoorbell1       = 0x08,
	Gpu0Halted         = 0x10,
	Gpu1Halted         = 0x20,
	IllegalAccessType1 = 0x40,
	IllegalAccessType0 = 0x80
}

#[repr(C)]
struct ArmTimer {
	load:                 u32,
	value:                u32,
	control:              u32,
	irq_clear:            u32,
	raw_irq:              u32,
	masked_irq:           u32,
	reload:               u32,
	pre_divider:          u32,
	free_running_counter: u32
}

enum ArmTimerControl {
	Counter16Or32              = 0x0002,
	PreScaleMask               = 0x000C,
	PreScale16                 = 0x0004,
	PreScale256                = 0x0008,
	TimerInterruptEnabled      = 0x0020,
	TimerInterruptDisabled     = !0x0020,
	TimerEnabled               = 0x0080,
	TimerDisabled              = !0x0080,
	TimersHltInDbgHaltedMode   = 0x0100,
	TimersRunInDbgHaltedMode   = !0x0100,
	FreeRunningCounterEnabled  = 0x0200,
	FreeRunningCounterDisabled = !0x0200
}

#[repr(C, align(0x100))]
struct ArmCore {
	sem:          [u32; 8],
	_pad0:        [u32; 0x8],
	bell:         [u32; 4],
	_pad1:        [u32; 0xC],
	mail0_rw:     u32,
	_pad2:        [u32; 0x3],
	mail0_peek:   u32,
	mail0_sender: u32,
	mail0_status: u32,
	mail0_config: u32,
	mail1_write:  u32,
	_pad3:        [u32; 0x14],
	mail1_status: u32,
	_pad4:        [u32; 0x9],
	semclrdbg:    u32,
	bellclrdbg:   u32,
	_pad5:        [u32; 0x4],
	all_irqs:     u32,
	my_irqs:      u32,
}

#[repr(C, align(0x1000))]
struct PowerManager {
	gnric:     u32,
	audio:     u32,
	status:    u32,
	rstc:      u32,
	rsts:      u32,
	wdog:      u32,
	pads:      [u32; 7],
	cam0:      u32,
	cam1:      u32,
	ccp2tx:    u32,
	dsi0:      u32,
	dsi1:      u32,
	hdmi:      u32,
	usb:       u32,
	pxldo:     u32,
	pxbg:      u32,
	dft:       u32,
	smps:      u32,
	xosc:      u32,
	sparew:    u32,
	sparer:    u32,
	avs_rstdr: u32,
	avs_stat:  u32,
	avs_event: u32,
	avs_inten: u32,
	dummy:     u32,
	image:     u32,
	grafx:     u32,
	proc:      u32,
}

#[repr(C, align(0x1000))]
struct ClockManager {
	gnricctl: u32,
	gnricdiv: u32,
	vpuctl:   u32,
	vpudiv:   u32,
	sysctl:   u32,
	sysdiv:   u32,
	periactl: u32,
	periadiv: u32,
	periictl: u32,
	periidiv: u32,
	h264ctl:  u32,
	h264div:  u32,
	ispctl:   u32,
	ispdiv:   u32,
	v3dctl:   u32,
	v3ddiv:   u32,
	cam0ctl:  u32,
	cam0div:  u32,
	cam1ctl:  u32,
	cam1div:  u32,
	ccp2ctl:  u32,
	ccp2div:  u32,
	dsi0ectl: u32,
	dsi0ediv: u32,
	dsi0pctl: u32,
	dsi0pdiv: u32,
	dpictl:   u32,
	dpidiv:   u32,
	gp0ctl:   u32,
	gp0div:   u32,
	gp1ctl:   u32,
	gp1div:   u32,
	gp2ctl:   u32,
	gp2div:   u32,
	hsmctl:   u32,
	hsmdiv:   u32,
	otpctl:   u32,
	otpdiv:   u32,
	pcmctl:   u32,
	pcmdiv:   u32,
	pwmctl:   u32,
	pwmdiv:   u32,
	slimctl:  u32,
	slimdiv:  u32,
	smictl:   u32,
	smidiv:   u32,
	tcntctl:  u32,
	tcntcnt:  u32,
	tecctl:   u32,
	tecdiv:   u32,
	td0ctl:   u32,
	td0div:   u32,
	td1ctl:   u32,
	td1div:   u32,
	tsensctl: u32,
	tsensdiv: u32,
	timerctl: u32,
	timerdiv: u32,
	uartctl:  u32,
	uartdiv:  u32,
	vecctl:   u32,
	vecdiv:   u32,
	osccount: u32,
	plla:     u32,
	pllc:     u32,
	plld:     u32,
	pllh:     u32,
	lock:     u32,
	event:    u32,
	inten:    u32,
	dsi0hsck: u32,
	cksm:     u32,
	oscfreqi: u32,
	oscfreqf: u32,
	plltctl:  u32,
	plltcnt0: u32,
	plltcnt1: u32,
	plltcnt2: u32,
	plltcnt3: u32,
	tdclken:  u32,
	burstctl: u32,
	burstcnt: u32,
	dsi1ectl: u32,
	dsi1ediv: u32,
	dsi1pctl: u32,
	dsi1pdiv: u32,
	dftctl:   u32,
	dftdiv:   u32,
	pllb:     u32,
	pulsectl: u32,
	pulsediv: u32,
	sdcctl:   u32,
	sdcdiv:   u32,
	armctl:   u32,
	armdiv:   u32,
	aveoctl:  u32,
	aveodiv:  u32,
	emmcctl:  u32,
	emmcdiv:  u32,
}

#[repr(C, align(0x1000))]
struct A2w {
	plla_dig:      [u32; 4],
	plla_ana:      [u32; 4],
	pllc_dig:      [u32; 4],
	pllc_ana:      [u32; 4],
	plld_dig:      [u32; 4],
	plld_ana:      [u32; 4],
	pllh_dig:      [u32; 4],
	pllh_ana:      [u32; 4],
	hdmi_ctl:      [u32; 4],
	xosc:          [u32; 2],
	smps_ctla:     [u32; 3],
	smps_ctlb:     [u32; 3],
	smps_ctlc:     [u32; 4],
	smps_ldo:      [u32; 2],
	pllb_dig:      [u32; 4],
	pllb_ana:      [u32; 4],
	plla_ctrl:     u32,
	plla_ana_sscs: u32,
	pllc_ctrl:     u32,
	pllc_ana_sscs: u32,
	plld_ctrl:     u32,
	plld_ana_sscs: u32,
	pllh_ctrl:     u32,
	hdmi_ctl_rcal: u32,
	xosc_ctrl:     u32,
	smps_a_mode:   u32,
	smps_b_stat:   u32,
	smps_c_clk:    u32,
	smps_l_spv:    u32,
	pllb_ctrl:     u32,
	pllb_ana_sscs: u32,
	plla_frac:     u32,
	plla_ana_sscl: u32,
	pllc_frac:     u32,
	pllc_ana_sscl: u32,
	plld_frac:     u32,
	plld_ana_sscl: u32,
	pllh_frac:     u32,
	hdmi_ctl_hfen: u32,
	xosc_cpr:      u32,
	smps_a_volts:  u32,
	smps_c_ctl:    u32,
	smps_l_spa:    u32,
	pllb_frac:     u32,
	pllb_ana_sscl: u32,
	plla_dsi0:     u32,
	plla_ana_kaip: u32,
	pllc_core2:    u32,
	pllc_ana_kaip: u32,
	plld_dsi0:     u32,
	plld_ana_kaip: u32,
	pllh_aux:      u32,
	pllh_ana_kaip: u32,
	xosc_bias:     u32,
	smps_a_gain:   u32,
	smps_l_scv:    u32,
	pllb_arm:      u32,
	pllb_ana_kaip: u32,
	plla_core:     u32,
	plla_ana_stat: u32,
	pllc_core1:    u32,
	pllc_ana_stat: u32,
	plld_core:     u32,
	plld_ana_stat: u32,
	pllh_rcal:     u32,
	xosc_pwr:      u32,
	smps_l_sca:    u32,
	pllb_sp0:      u32,
	pllb_ana_stat: u32,
	plla_per:      u32,
	plla_ana_sctl: u32,
	pllc_per:      u32,
	pllc_ana_sctl: u32,
	plld_per:      u32,
	plld_ana_sctl: u32,
	pllh_pix:      u32,
	pllh_ana_sctl: u32,
	smps_l_siv:    u32,
	pllb_sp1:      u32,
	pllb_ana_sctl: u32,
	plla_ccp2:     u32,
	plla_ana_vco:  u32,
	pllc_core0:    u32,
	pllc_ana_vco:  u32,
	plld_dsi1:     u32,
	plld_ana_vco:  u32,
	pllh_ana_stat: u32,
	pllh_ana_vco:  u32,
	smps_l_sia:    u32,
	pllb_sp2:      u32,
	pllb_ana_vco:  u32,
	plla_digr:     [u32; 4],
	plla_anar:     [u32; 4],
	pllc_digr:     [u32; 4],
	pllc_anar:     [u32; 4],
	plld_digr:     [u32; 4],
	plld_anar:     [u32; 4],
	pllh_digr:     [u32; 4],
	pllh_anar:     [u32; 4],
	hdmi_ctlr:     [u32; 4],
	xoscr:         [u32; 2],
	smps_ctlar:    [u32; 3],
	smps_ctlbr:    [u32; 3],
	smps_ctlcr:    [u32; 4],
	smps_ldor:     [u32; 2],
	pllb_digr:     [u32; 4],
	pllb_anar:     [u32; 4],
	plla_ctrlr:     u32,
	plla_ana_sscsr: u32,
	pllc_ctrlr:     u32,
	pllc_ana_sscsr: u32,
	plld_ctrlr:     u32,
	plld_ana_sscsr: u32,
	pllh_ctrlr:     u32,
	hdmi_ctl_rcalr: u32,
	xosc_ctrlr:     u32,
	smps_a_moder:   u32,
	smps_b_statr:   u32,
	smps_c_clkr:    u32,
	smps_l_spvr:    u32,
	pllb_ctrlr:     u32,
	pllb_ana_sscsr: u32,
	plla_fracr:     u32,
	plla_ana_ssclr: u32,
	pllc_fracr:     u32,
	pllc_ana_ssclr: u32,
	plld_fracr:     u32,
	plld_ana_ssclr: u32,
	pllh_fracr:     u32,
	hdmi_ctl_hfenr: u32,
	xosc_cprr:      u32,
	smps_a_voltsr:  u32,
	smps_c_ctlr:    u32,
	smps_l_spar:    u32,
	pllb_fracr:     u32,
	pllb_ana_ssclr: u32,
	plla_dsi0r:     u32,
	plla_ana_kaipr: u32,
	pllc_core2r:    u32,
	pllc_ana_kaipr: u32,
	plld_dsi0r:     u32,
	plld_ana_kaipr: u32,
	pllh_auxr:      u32,
	pllh_ana_kaipr: u32,
	xosc_biasr:     u32,
	smps_a_gainr:   u32,
	smps_l_scvr:    u32,
	pllb_armr:      u32,
	pllb_ana_kaipr: u32,
	plla_corer:     u32,
	plla_ana_statr: u32,
	pllc_core1r:    u32,
	pllc_ana_statr: u32,
	plld_corer:     u32,
	plld_ana_statr: u32,
	pllh_rcalr:     u32,
	xosc_pwrr:      u32,
	smps_l_scar:    u32,
	pllb_sp0r:      u32,
	pllb_ana_statr: u32,
	plla_perr:      u32,
	plla_ana_sctlr: u32,
	pllc_perr:      u32,
	pllc_ana_sctlr: u32,
	plld_perr:      u32,
	plld_ana_sctlr: u32,
	pllh_pixr:      u32,
	pllh_ana_sctlr: u32,
	smps_l_sivr:    u32,
	pllb_sp1r:      u32,
	pllb_ana_sctlr: u32,
	plla_ccp2r:     u32,
	plla_ana_vcor:  u32,
	pllc_core0r:    u32,
	pllc_ana_vcor:  u32,
	plld_dsi1r:     u32,
	plld_ana_vcor:  u32,
	pllh_ana_statr: u32,
	pllh_ana_vcor:  u32,
	smps_l_siar:    u32,
	pllb_sp2r:      u32,
	pllb_ana_vcor:  u32,
	plla_multi:     u32,
	plla_ana_multi: u32,
	pllc_multi:     u32,
	pllc_ana_multi: u32,
	plld_multi:     u32,
	plld_ana_multi: u32,
	pllh_multi:     u32,
	pllh_ana_multi: u32,
	hdmi_ctl_multi: u32,
	xosc_multi:     u32,
	smps_a_multi:   u32,
	smps_b_multi:   u32,
	smps_c_multi:   u32,
	smps_l_multi:   u32,
	pllb_multi:     u32,
	pllb_ana_multi: u32,
}

#[repr(C, align(0x1000))]
struct Rng {
	ctrl:         u32,
	status:       u32,
	data:         u32,
	ff_threshold: u32,
	int_mask:     u32
}

#[repr(C, align(0x1000))]
struct Uart {
	dr:     u32,
	rsrecr: u32,
	_pad0:  [u32; 0x4],
	fr:     u32,
	ilpr:   u32,
	ibrd:   u32,
	fbrd:   u32,
	lcrh:   u32,
	cr:     u32,
	ifls:   u32,
	imsc:   u32,
	ris:    u32,
	mis:    u32,
	icr:    u32,
	dmacr:  u32,
	_pad1:  [u32; 0xD],
	itcr:   u32,
	itip:   u32,
	itop:   u32,
	tdr:    u32
}

#[repr(C, align(0x1000))]
pub struct Sh {
	cmd:  u32,
	arg:  u32,
	tout: u32,
	cdiv: u32,
	rsp:  [u32; 4],
	hsts: u32,
	vdd:  u32,
	edm:  u32,
	hcfg: u32,
	hbct: u32,
	data: u32,
	hblc: u32
}

#[repr(C, align(0x1000))]
pub struct Pcm {
	cd:      u32,
	fifo:    u32,
	mode:    u32,
	rxc:     u32,
	txc:     u32,
	dreq:    u32,
	int_en:  u32,
	int_stc: u32,
	gray:    u32
}

#[repr(C, align(0x1000))]
pub struct SerialPeripheralInterface {
	cs:   u32,
	fifo: u32,
	clk:  u32,
	dlen: u32,
	ltoh: u32,
	dc:   u32
}

#[repr(C, align(0x1000))]
pub struct I2c {
	ctrl: u32,
	stat: u32,
	dlen: u32,
	slad: u32,
	fifo: u32,
	div:  u32,
	del:  u32,
	clkt: u32
}

#[repr(C, align(0x1000))]
pub struct PixelValve {
	c:            u32,
	vc:           u32,
	vsyncd_even:  u32,
	horz_a:       u32,
	horz_b:       u32,
	vert_a:       u32,
	vert_b:       u32,
	vert_a_even:  u32,
	vert_b_even:  u32,
	int_enable:   u32,
	int_status:   u32,
	status:       u32,
	dsi_hact_act: u32
}

#[repr(C, align(0x1000))]
pub struct DisplayParallelInterface {
	c: u32,
}

#[repr(C, align(0x1000))]
pub struct DisplaySerialInterface {
	ctrl:          u32,
	cmd_pktc:      u32,
	cmd_pkth:      u32,
	rx1_pkth:      u32,
	rx2_pkth:      u32,
	cmd_data_fifo: u32,
	disp0_ctrl:    u32,
	disp1_ctrl:    u32,
	pix_fifo:      u32,
	int_status:    u32,
	int_enable:    u32,
	status:        u32,
	hstx_to_cnt:   u32,
	lprx_to_cnt:   u32,
	ta_to_cnt:     u32,
	pr_to_cnt:     u32,
	phy_ctrl:      u32,
	hs_clt0:       u32,
	hs_clt1:       u32,
	hs_clt2:       u32,
	hs_dtl3:       u32,
	hs_dtl4:       u32,
	hs_dtl5:       u32,
	lp_dtl6:       u32,
	lp_dtl7:       u32,
	afec:          [u32; 2],
	tst_sel:       u32,
	tst_mon:       u32,
}
#[repr(C, align(0x1000))]
pub struct Tb {
	task:             u32,
	task_param:       [u32; 3],
	_pad0:            [u32; 0x1C],
	task_status:      u32,
	task_rxdata:      [u32; 2],
	_pad1:            [u32; 0x19],
	task_txtclr:      u32,
	_pad2:            [u32; 0x03],
	hdmi:             u32,
	_pad3:            [u32; 0x3F],
	pcm:              u32,
	_pad4:            [u32; 0x3F],
	host:             u32,
	_pad5:            [u32; 0x3F],
	printer_ctrl:     u32,
	printer_data:     u32,
	_pad6:            [u32; 0x3E],
	boot_addr:        u32,
	boot_opt:         u32,
	boot_secure_mode: u32,
	boot_status:      u32,
	_pad7:            [u32; 0xBC],
	jtb_config:       u32,
	jtb_tms:          u32,
	jtb_tdi:          u32,
	jtb_tdo:          u32,
	jtb_bitcnt:       u32,
	jtb_porten:       u32
}

#[repr(C, align(0x1000))]
pub struct PulseWidthModulator {
	ctrl: u32,
	stat: u32,
	dmac: u32,
	rng1: u32,
	dat1: u32,
	fif1: u32,
	rng2: u32,
	dat2: u32
}

#[repr(C, align(0x1000))]
pub struct Prm {
	cs:  u32,
	cv:  u32,
	scc: u32
}

#[repr(C, align(0x1000))]
pub struct Te([TeEntry; 3]);

#[repr(C, packed)]
struct TeEntry {
	c:        u32,
	vs_width: u32,
	timer:    u32
}

#[repr(C, align(0x1000))]
pub struct OneTimeProgrammable {
	boot_mode:       u32,
	config:          u32,
	ctrl_lo:         u32,
	ctrl_hi:         u32,
	status:          u32,
	bitsel:          u32,
	data:            u32,
	addr:            u32,
	write_data_read: u32,
	init_status:     u32
}

#[repr(C, align(0x1000))]
pub struct Slim {
	con:         u32,
	con2:        u32,
	status:      u32,
	fs:          u32,
	ea:          [u32; 2],
	dma_mc_rx:   u32,
	dma_mc_tx:   u32,
	dma_dc:      [u32; 10],
	dma_mc_con:  u32,
	dma_mc_stat: u32,
	dma_dc_stat: [u32; 2],
	_pad0:       [u32; 0x1B],
	mc_in_con:   u32,
	mc_out_con:  u32,
	mc_out_stat: u32,
	_pad1:       [u32; 0x36],
	dcc:         [Dcc; 10]
}

#[repr(C, align(0x20))]
pub struct Dcc {
	pa:   [u32; 2],
	con:  u32,
	stat: u32,
	prot: u32
}

#[repr(C, align(0x1000))]
pub struct Cpg {
	config:     u32,
	int_status: u32,
	trigger:    u32,
	params:     [u32; 4],
	_pad:       [u32; 0x8],
	debug:      [u32; 4]
}

#[repr(C, align(0x1000))]
struct Ts {
	tsensctl:  u32,
	tsensstat: u32
}

#[repr(C, align(0x1000))]
struct I2cSpiSlave {
	dr:      u32,
	rsr:     u32,
	slv:     u32,
	cr:      u32,
	fr:      u32,
	ifls:    u32,
	imsc:    u32,
	ris:     u32,
	mis:     u32,
	icr:     u32,
	dmacr:   u32,
	tdr:     u32,
	gpustat: u32,
	hctrl:   u32,
	dbg1:    u32,
	dbg2:    u32
}

#[repr(C, align(0x1000))]
struct Auxiliaries {
	irq:           u32,
	enables:       u32,
	_pad0:         [u32; 0xE],
	muart_io:      u32,
	muart_ier:     u32,
	muart_iir:     u32,
	muart_lcr:     u32,
	muart_mcr:     u32,
	muart_lsr:     u32,
	muart_msr:     u32,
	muart_scratch: u32,
	muart_ctrl:    u32,
	muart_stat:    u32,
	_pad1:         [u32; 0x5],
	spi1_ctrl:     [u32; 2],
	spi1_stat:     u32,
	spi1_data:     u32,
	spi1_peek:     u32,
	_pad2:         [u32; 0xA],
	spi2_ctrl:     [u32; 2],
	spi2_stat:     u32,
	spi2_data:     u32,
	spi2_peek:     u32,
}



#[repr(C, align(0x1000))]
struct AveOut {
	ctrl:     u32,
	status:   u32,
	offset:   u32,
	y_coeff:  u32,
	cb_coeff: u32,
	cr_coeff: u32,
	block_id: u32,
}

#[repr(C, align(0x1000))]
struct Scaler {
	ctrl:             u32,
	status:           u32,
	id:               u32,
	alt_control:      u32,
	profile:          u32,
	dither:           u32,
	dispeoln:         u32,
	disp_list:        [u32; 3],
	disp_list_status: u32,
	displact:         [u32; 3],
	dispctrl0:        u32,
	dispbkgnd0:       u32,
	dispstat0:        u32,
	dispbase0:        u32,
	dispctrl1:        u32,
	dispbkgnd1:       u32,
	dispstat1:        u32,
	dispbase1:        u32,
	dispctrl2:        u32,
	dispbkgnd2:       u32,
	dispstat2:        u32,
	dispbase2:        u32,
	dispalpha2:       u32,
	gam_address:      u32,
	dispgamadr:       u32,
	oledoffs:         u32,
	oledcoef:         [u32; 3],
	dispslave:        [u32; 3],
	gam_data:         u32,
	dispgamdat:       u32,
}

#[repr(C, align(0x1000))]
struct Cam {
	ctl:   u32,
	sta:   u32,
	ana:   u32,
	pri:   u32,
	clk:   u32,
	clt:   u32,
	dat:   [u32; 4],
	dlt:   u32,
	cmp:   [u32; 2],
	cap:   [u32; 2],
	_pad0: [u32; 0x2D],
	dbg:   [u32; 4],
	ictl:  u32,
	ista:  u32,
	idi0:  u32,
	ipipe: u32,
	ibsa0: u32,
	ibea0: u32,
	ibls:  u32,
	ibwp:  u32,
	ihwin: u32,
	ihsta: u32,
	ivwin: u32,
	ivsta: u32,
	icc:   u32,
	ics:   u32,
	idc:   u32,
	idpo:  u32,
	idca:  u32,
	idcd:  u32,
	ids:   u32,
	dcs:   u32,
	dbsa0: u32,
	dbea0: u32,
	dbwp:  u32,
	dbctl: u32,
	ibsa1: u32,
	ibea1: u32,
	idi1:  u32,
	dbsa1: u32,
	dbea1: u32,
	_pad1: [u32; 0x3A],
	misc:  u32
}

#[repr(C, align(0x1000))]
struct Cmi {
	cam0:      u32,
	cam1:      u32,
	_pad0:     u32,
	cam_test:  u32,
	_pad1:     u32,
	usb_ctl:   u32
}

#[repr(C, align(0x1000))]
struct Vec {
	cgmsae_reset: u32,
	cgmsae_top_control: u32,
	cgmsae_bot_control: u32,
	cgmsae_top_format: u32,
	cgmsae_bot_format: u32,
	cgmsae_top_data: u32,
	cgmsae_bot_data: u32,
	cgmsae_revid: u32,
	enc_revid: u32,
	enc_primary_control: u32,
	wse_reset: u32,
	wse_control: u32,
	wse_wss_data: u32,
	wse_vps_data_1: u32,
	wse_vps_control: u32,
	revid: u32,
	config0: u32,
	schph: u32,
	soft_reset: u32,
	cps01_cps23: u32,
	cps45_cps67: u32,
	cps89_cps1011: u32,
	cps1213_cps1415: u32,
	cps1617_cps1819: u32,
	cps2021_cps2223: u32,
	cps2425_cps2627: u32,
	cps2829_cps3031: u32,
	cps32_cpc: u32,
	clmp0_start: u32,
	clmp0_end: u32,
	freq3_2: u32,
	freq1_0: u32,
	config1: u32,
	config2: u32,
	interrupt_control: u32,
	interrupt_status: u32,
	fcw_secam_b: u32,
	secam_gain_val: u32,
	config3: u32,
	config4: u32,
	status0: u32,
	mask0: u32,
	cfg: u32,
	dac_test: u32,
	dac_config: u32,
	dac_misc: u32,
}

#[repr(C, align(0x1000))]
struct Sdram {
	cs:      u32,
	sa:      u32,
	sb:      u32,
	sc:      u32,
	pt2:     u32,
	pt1:     u32,
	idl:     u32,
	rtc:     u32,
	wtc:     u32,
	rdc:     u32,
	wdc:     u32,
	rac:     u32,
	cyc:     u32,
	cmd:     u32,
	dat:     u32,
	secsrt0: u32,
	secend0: u32,
	secsrt1: u32,
	secend1: u32,
	secsrt2: u32,
	secend2: u32,
	secsrt3: u32,
	secend3: u32,
	phyc:    u32,
	mrt:     u32,
	tmc:     u32,
	rwc:     u32,
	vad:     u32,
	vin:     u32,
	mr:      u32,
	sd:      u32,
	se:      u32,
	ver:     u32,
	stall:   u32,
	reord:   u32,
	lac:     u32,
	pre:     u32,
	sf:      u32,
	carcrc:  u32,
	dmrcrc:  [u32; 2],
	dqrcrc:  [u32; 16],
	dqlcrc:  [u32; 16]
}
