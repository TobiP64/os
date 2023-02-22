OUTPUT_FORMAT("binary")
ENTRY(_start)

SECTIONS {
    .isr_vector {
        . = 0x40200000
		KEEP(*(.text.aarch64_int_currrent_el_sp_el0_sync));
        . = 0x40200080
		KEEP(*(.text.aarch64_int_currrent_el_sp_el0_irq));
        . = 0x40200100
		KEEP(*(.text.aarch64_int_currrent_el_sp_el0_fiq));
        . = 0x40200180
		KEEP(*(.text.aarch64_int_currrent_el_sp_el0_serror));
        . = 0x40200200
		KEEP(*(.text.aarch64_int_currrent_el_sp_elx_sync));
        . = 0x40200280
		KEEP(*(.text.aarch64_int_currrent_el_sp_elx_irq));
        . = 0x40200300
		KEEP(*(.text.aarch64_int_currrent_el_sp_elx_fiq));
        . = 0x40200380
		KEEP(*(.text.aarch64_int_currrent_el_sp_elx_serror));
        . = 0x40200400
		KEEP(*(.text.aarch64_int_lower_el_aarch64_sync));
        . = 0x40200480
		KEEP(*(.text.aarch64_int_lower_el_aarch64_irq));
        . = 0x40200500
		KEEP(*(.text.aarch64_int_lower_el_aarch64_fiq));
        . = 0x40200580
		KEEP(*(.text.aarch64_int_lower_el_aarch64_serror));
        . = 0x40200600
		KEEP(*(.text.aarch64_int_lower_el_aarch32_sync));
        . = 0x40200680
		KEEP(*(.text.aarch64_int_lower_el_aarch32_irq));
        . = 0x40200700
		KEEP(*(.text.aarch64_int_lower_el_aarch32_fiq));
        . = 0x40200780
		KEEP(*(.text.aarch64_int_lower_el_aarch32_serror));
    }

    .text : 0x40200800 {
		KEEP(*(.text._start));
		*(.text .text.*)
	}

	.rodata : ALIGN(4096) {
		*(.rodata .rodata.*)
	}

	.data : ALIGN(4096) {
		*(.sdata .sdata.*)
		*(.data .data.*)
	}

	.bss : ALIGN(4096) {
		*(.sbss .sbss.*)
		*(.bss .bss.*)
	}

	/DISCARD/ : {
		*(.comment .comment.*)
		*(.ARM.exidx .ARM.exidx.*)
	}
}