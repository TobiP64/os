OUTPUT_FORMAT("binary")
ENTRY(_start)

SECTIONS {
    .isr_vector {
        . = 0x1000
		KEEP(*(.text.aarch64_int_currrent_el_sp_el0_sync));
        . = 0x1080
		KEEP(*(.text.aarch64_int_currrent_el_sp_el0_irq));
        . = 0x1100
		KEEP(*(.text.aarch64_int_currrent_el_sp_el0_fiq));
        . = 0x1180
		KEEP(*(.text.aarch64_int_currrent_el_sp_el0_serror));
        . = 0x1200
		KEEP(*(.text.aarch64_int_currrent_el_sp_elx_sync));
        . = 0x1280
		KEEP(*(.text.aarch64_int_currrent_el_sp_elx_irq));
        . = 0x1300
		KEEP(*(.text.aarch64_int_currrent_el_sp_elx_fiq));
        . = 0x1380
		KEEP(*(.text.aarch64_int_currrent_el_sp_elx_serror));
        . = 0x1400
		KEEP(*(.text.aarch64_int_lower_el_aarch64_sync));
        . = 0x1480
		KEEP(*(.text.aarch64_int_lower_el_aarch64_irq));
        . = 0x1500
		KEEP(*(.text.aarch64_int_lower_el_aarch64_fiq));
        . = 0x1580
		KEEP(*(.text.aarch64_int_lower_el_aarch64_serror));
        . = 0x1600
		KEEP(*(.text.aarch64_int_lower_el_aarch32_sync));
        . = 0x1680
		KEEP(*(.text.aarch64_int_lower_el_aarch32_irq));
        . = 0x1700
		KEEP(*(.text.aarch64_int_lower_el_aarch32_fiq));
        . = 0x1780
		KEEP(*(.text.aarch64_int_lower_el_aarch32_serror));
    }

    .text : 0x1800 {
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