// MIT Licensed:       u32,
	pub rsr_ecr: u32,
	pub _res0:   [u32; 0x4],
	pub f:       u32,
	pub _res1:   u32,
	pub ilp:     u32,
	pub ibr:     u32,
	pub fbr:     u32,
	pub lc:      u32,
	pub c:       u32,
	pub ifsl:    u32,
	pub imsc:    u32,
	pub ris:     u32,
	pub mis:     u32,
	pub icr:     u32,
	pub
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

#[repr(C)]
struct GICD {
	ctl:         u32,
	typer:       u32,
	iid:         u32,
	/// GICv3/4 only
	typer2:      u32,
	/// GICv3/4 only
	status:      u32,
	_res0:       [u32; 0x3],
	_impl0:      [u32; 0x8],
	/// GICv3/4 only
	set_spi_ns:  u32,
	/// GICv3/4 only
	_res1:       u32,
	/// GICv3/4 only
	clr_spi_ns:  u32,
	_res2:       u32,
	/// GICv3/4 only
	set_spi_s:   u32,
	_res3:       u32,
	/// GICv3/4 only
	clr_spi_s:   u32,
	_res4:       [u32; 0x9],
	igroup:      [u32; 0x1F],
	is_enable:   [u32; 0x20],
	ic_enable:   [u32; 0x20],
	is_pend:     [u32; 0x20],
	ic_pend:     [u32; 0x20],
	is_active:   [u32; 0x20],
	ic_active:   [u32; 0x20],
	i_priority:  [u32; 0xFF],
	_res5:       u32,
	i_targets:   [u32; 0xFF],
	_res6:       u32,
	i_cfg:       [u32; 0x40],
	/// GICv3/4 only
	i_grp_mod:   [u32; 0x40],
	nasc:        [u32; 0x40],
	sgi:         u32,
	_res7:       [u32; 0x3],
	c_pend_sgi:  [u32; 0x4],
	s_pend_sgi:  [u32; 0x4],
	_res8:       [u32; 0x28],
	_impl2:      [u32; 0x30],
	/// GICv3/4 only
	i_group_e:   [u32; 0x40],
	/// GICv3/4 only
	is_enable_e: [u32; 0x40],
	/// GICv3/4 only
	ic_enable_e: [u32; 0x40],
	/// GICv3/4 only
	is_pend_e:   [u32; 0x40],
	/// GICv3/4 only
	ic_pend_e:   [u32; 0x40],
	/// GICv3/4 only
	is_active_e: [u32; 0x40],
	/// GICv3/4 only
	ic_active_e: [u32; 0x40],
	_res20:      [u32; 0xE0],
	/// GICv3/4 only
	i_priority_e:[u32; 0xC0],
	_res21:      [u32; 0x320],
	/// GICv3/4 only
	i_cfg_e:     [u32; 0x40],
	_res22:      [u32; 0xC0],
	/// GICv3/4 only
	i_grp_mod_e: [u32; 0x20],
	_res23:      [u32; 0x60],
	/// GICv3/4 only
	nsac_e:      [u32; 0x20],
	_res24:      [u32; 0xAA0],
	/// GICv3/4 only
	route:       [u32; 0x7B7],
	_res25:      [u32; 0x9],
	/// GICv3/4 only
	route_e:     [u32; 0x800],
	_res26:      [u32; 0x800],
	_impl3:      [u32; 0xFF4],
	_res27:      [u32; 0xC]
}

#[repr(C)]
struct GICC {
	ctl:    u32,
	pm:     u32,
	bp:     u32,
	ia:     u32,
	eoi:    u32,
	rp:     u32,
	hppi:   u32,
	abp:    u32,
	aia:    u32,
	aeoi:   u32,
	ahppi:  u32,
	/// GICv3/4 only
	status: u32,
	_res0:  [u32; 0x3],
	_impl0: [u32; 0x24],
	ap:     [u32; 0x4],
	nsap:   [u32; 0x4],
	_res1:  [u32; 0xF],
	iid:    u32,
	di:     u32
}

pub struct GICv2 {
	gicd: *mut GICD,
	gicc: *mut GICC
}

impl GICv2 {
	pub fn new(gicd: *mut u8, gicc: *mut u8) -> Self {
		Self { gicd: gicd as _, gicc: gicc as _ }
	}
	
	pub unsafe fn init(&mut self) {
		let reg = &mut (*self.gicc).ctl as *mut u32;
		reg.write_volatile(reg.read_volatile() | 1);
	}
	
	pub unsafe fn enable(&mut self, id: usize) {
		(&mut (*self.gicd).is_enable[id / 32] as *mut u32).write_volatile(1 << (id as u32 % 32));
	}
	
	pub unsafe fn disable(&mut self, id: usize) {
		(&mut (*self.gicd).ic_enable[id / 32] as *mut u32).write_volatile(1 << (id as u32 % 32));
	}
	
	pub unsafe fn set_priority_threshold(&mut self, pri: usize) {
		(&mut (*self.gicc).pm as *mut u32).write_volatile(pri as _);
	}
	
	pub unsafe fn claim(&self) -> u32 {
		(&mut (*self.gicc).ia as *mut u32).read_volatile()
	}
	
	pub unsafe fn complete(&mut self, id: usize) {
		(&mut (*self.gicc).eoi as *mut u32).write_volatile(id as _)
	}
}