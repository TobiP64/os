#![warn(clippy::all)]

mod dt_aarch64;
mod dt_riscv64;
mod emb_aarch64;
mod emb_riscv64;
mod uefi;

pub struct GenericFramebuffer {
    pub width:    u32,
	pub height:   u32,
	pub format:   u32,
	pub scanline: u32,
	pub ptr:      *mut [u8]
}

extern "C" {
    pub static HV_DATA: kernel::HvDuata;
    pub static SV_DATA: kernel::SvDuata;
}

#[link_section = ".data"]
static mut OUT: fn(&str) = dummy_out;

fn dummy_out(c: char) {}

pub fn set_out(out: fn(&str)) {
    unsafe { OUT = out; }
}

pub struct OutWriter;

impl core::fmt::Write for OutWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        (unsafe { OUT })(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({ core::write!($crate::OutWriter, $($arg)* ).unwrap_or(()) });
}

#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => ({ core::writeln!($crate::OutWriter, $($arg)* ).unwrap_or(()) });
}

#[macro_export]
macro_rules! eprint {
    ($($arg:tt)*) => ({ core::write!($crate::OutWriter, $($arg)* ).unwrap_or(()) });
}

#[macro_export]
macro_rules! eprintln {
    ($($arg:tt)*) => ({ core::writeln!($crate::OutWriter, $($arg)* ).unwrap_or(()) });
}