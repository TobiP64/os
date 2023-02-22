pub const DEVICE_TREES: &[u8] = include_bytes!("../device_trees.bin");

#[naked]
#[no_mangle]
pub unsafe extern fn _start() {
    asm!(
		csrw mie, 0
		csrw mip, 0
		csrw mtvec, _start
		csrr misa, x1

	

		csrw satp, 0
		csrw medeleg, 0xFFFF
		csrw mideleg, 0xFFFF


		li x1, 0
		csrr x2, mhartid
		beq , 8
		j -4
	);

    if mhartid.read() != 0 {
        park();												// park non-boot cores
    }

	// TODO setup trap vector and sstatus, enable interrupts

	mtvec.write_ptr(crate::arch::park as *const ());		// setup interrupt handler
	mie.write(0);											// disable interrupts
	medeleg.write(0xFFFF);									// delegate all exceptions to supervisor mode
	mideleg.write(0xFFFF);									// delegate all interrupts to supervisor mode
	satp.write(0);											// disable paging
	sp.write_ptr(&_stack_end);								// set stack pointer
	fp.write_ptr(&_stack_end);								// set frame pointer
	gp.write_ptr(&_global_pointer);							// set global pointer
	llvm_asm!("csrr a0, mhartid");
    llvm_asm!("mv   a1, $0" :: "r"(DEVICE_TREE.as_ptr()) ::);
    mepc.write_ptr(main as *const ());						// set return address
	mret();													// jump to main and switch to supervisor mode
}