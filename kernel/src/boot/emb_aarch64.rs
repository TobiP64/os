
pub const DEVICE_TREE: &[u8] = include_bytes!("../device_tree.dtb");

#[naked]
#[no_mangle]
pub unsafe extern fn _start() -> ! {
    if MPIDR_EL1.read() & 0xFF != 0 {						// check core id

		park();												// park non-boot cores

    } else if CurrentEL.read() != 0b1000 {					// executing in EL1, no EL transition required

		set_sp(0x100000000);								// setup stack
		main();												// jump to main

    } else {												// executing in EL2, transition to EL1 required
		//               A IF   EL1h
		SPSR_EL2.write(0b1_1100_0101);						// disable interrupts, set target EL
		SP_EL1.write(0x100000000);							// setup stack
		ELR_EL2.write_ptr(main as *const ());				// write return address
		eret();												// jump to main and switch to EL1
    }
}