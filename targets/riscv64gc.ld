OUTPUT_FORMAT("binary")
ENTRY(_start)

SECTIONS {
    .text : 0x1000 {
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