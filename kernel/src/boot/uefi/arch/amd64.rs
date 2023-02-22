use super::*;

#[no_mangle]
pub fn AMD64_init(
        system_table: &hw::uefi::SystemTable,
        framebuffer:  Option<crate::GenericFramebuffer>,
        memory_map:   hw::uefi::MemoryMap
) -> ! {
    unsafe { AMD64_BSC(data, madt); }
}

#[no_mangle]
pub unsafe fn AMD64_BSC(data: &kernel_sv::SvData, madt: &hw::acpi::MADT) {
    // check features
    if core::arch::x86_64::__cpuid(0x01).ecx & (1 << 21) == 0 {
		println!("ERROR: x2APIC not supported");
        hw::arch::park();
    }

	// setup GDT
    AMD64_GDT[0] = 0x0000000000000000;
    AMD64_GDT[0] = 0x00209A0000000000;
    AMD64_GDT[0] = 0x0000920000000000;

    // setup IDT
    AMD64_IDT[0] = entry(amd64_int_divide_by_zero, 0x8E00_0000);
    AMD64_IDT[1] = entry(amd64_int_debug, 0x8E00_0000);
    AMD64_IDT[2] = entry(amd64_int_nmi, 0x8E00_0000);
	AMD64_IDT[3] = entry(amd64_int_breakpoint, 0x8E00_0000);
	AMD64_IDT[4] = entry(amd64_int_overflow, 0x8E00_0000);
	AMD64_IDT[5] = entry(amd64_int_bound_range, 0x8E00_0000);
	AMD64_IDT[6] = entry(amd64_int_invalid_opcode, 0x8E00_0000);
	AMD64_IDT[7] = entry(amd64_int_device_not_available, 0x8E00_0000);
	AMD64_IDT[8] = entry(amd64_int_double_fault, 0x8E00_0000);
	AMD64_IDT[10] = entry(amd64_int_invalid_tss, 0x8E00_0000);
	AMD64_IDT[11] = entry(amd64_int_segment_not_present, 0x8E00_0000);
	AMD64_IDT[12] = entry(amd64_int_stack, 0x8E00_0000);
	AMD64_IDT[13] = entry(amd64_int_general_protection, 0x8E00_0000);
	AMD64_IDT[14] = entry(amd64_int_page_fault, 0x8E00_0000);
	AMD64_IDT[16] = entry(amd64_int_x87, 0x8E00_0000);
	AMD64_IDT[17] = entry(amd64_int_alignment_check, 0x8E00_0000);
	AMD64_IDT[18] = entry(amd64_int_machine_check, 0x8E00_0000);
	AMD64_IDT[19] = entry(amd64_int_simd_floating_point, 0x8E00_0000);
	AMD64_IDT[21] = entry(amd64_int_control_protection_exception, 0x8E00_0000);
	AMD64_IDT[28] = entry(amd64_int_hypervisor_injection_exception, 0x8E00_0000);
	AMD64_IDT[29] = entry(amd64_int_vmm_communication_exception, 0x8E00_0000);
	AMD64_IDT[30] = entry(amd64_int_security_exception, 0x8E00_0000);
	AMD64_IDT[32] = entry(amd64_int_apic_timer, 0x8E00_0000);
	AMD64_IDT[33] = entry(amd64_int_apic_thermal, 0x8E00_0000);
	AMD64_IDT[34] = entry(amd64_int_apic_counter, 0x8E00_0000);
	AMD64_IDT[35] = entry(amd64_int_apic_lint0, 0x8E00_0000);
	AMD64_IDT[36] = entry(amd64_int_apic_lint1, 0x8E00_0000);
	AMD64_IDT[37] = entry(amd64_int_apic_error, 0x8E00_0000);
    AMD64_IDT[38] = entry(amd64_int_apic_spurious, 0x8E00_0000);
    AMD64_IDT[39] = entry(amd64_int_ipi_hart_up, 0x8E00_0000);
    AMD64_IDT[39] = entry(amd64_int_ipi_hart_down, 0x8E00_0000);

	// get TSC frequency
    let tsc = unsafe { core::arch::x86_64::__cpuid(0x15) };
    let tsc_frq = tsc.ecx * tsc.ebx / tsc.eax;

    let trampoline_base_addr = 0x8000;

    // copy trampoline
    asm!("
		mov ecx, 512
		mov rsi, .AMD64_AP_trampoline
		mov rdi, {0}
		rep
		movs
	", in(reg) trampoline_base_addr, options(memory));

    // enable x2APIC
	hw::arch::cli();
	hw::arch::APIC_BASE.set(0x0000_0000_FEE0_0800);
    hw::arch::APIC_BASE.set(0x0000_0000_FEE0_0C00);

    'loop: for e in madt {
		let apic = if let hw::acpi::MadtInterruptController::ProcessorLocalX2Apic(v) = e {
            v
        } else {
            continue;
        };

        let hart: &mut kernel_sv::hart::Hart = ();
		hart.status = kernel_sv::hart::Hart::STATUS_PRE_BOOT;
        AP = hart;

        let dst = apic.acpi_processor_uid << 24;
        hw::arch::x2APIC_ESR.set(0);

        // assert INIT
        hw::arch::x2APIC_ICR.set(0xC500 | dst);
        while hw::arch::x2APIC_ICR.get() & (1 << 12) == 0 {}

		// deassert INIT
		hw::arch::x2APIC_ICR.set(0x8500 | dst);
        while hw::arch::x2APIC_ICR.get() & (1 << 12) == 0 {}

		// wait 10ms
		let tsc_deadline = hw::arch::rdtsc64() + tsc_frq / 100;
        while hw::arch::rdtsc64() < tsc_deadline {}

		// send STARTUP #1
		hw::arch::x2APIC_ICR.set(0x0608 | dst);
        while hw::arch::x2APIC_ICR.get() & (1 << 12) == 0 {}

		// wait 200us
		let tsc_deadline = hw::arch::rdtsc64() + tsc_frq / 5000;
        while hw::arch::rdtsc64() < tsc_deadline {
            if hart.status == kernel_sv::hart::Hart::STATUS_BOOT_COMPLETED {
                continue 'loop;
            }
        }

		// send STARTUP #2
		hw::arch::x2APIC_ICR.set(0x0608 | dst);
        while hw::arch::x2APIC_ICR.get() & (1 << 12) == 0 {}

		// wait 100ms
		let tsc_deadline = hw::arch::rdtsc64() + tsc_frq / 10;
        while hw::arch::rdtsc64() < tsc_deadline  {
            if hart.status == kernel_sv::hart::Hart::STATUS_BOOT_COMPLETED {
				continue 'loop;
            }
        }

		hart.status = kernel_sv::hart::Hart::STATUS_UNKNOWN;
    }

	AP = ??;
	AMD64_AP_trampoline_64();
}

pub const fn entry(f: fn(), options: u32) -> u128 {
    let offset = f as usize as u128;
    options << 16
	| offset & 0xFFFF
	| offset & 0xFFFF_FFFF_FFFF_0000 << 32
}