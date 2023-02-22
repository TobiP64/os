
use crate::dri::device_tree::FdtHeader;
use crate::arch::park;

#[naked]
#[no_mangle]
pub extern fn _start(x0: usize, machine_type: usize, fdt: &FdtHeader) -> ! {
	park()
}