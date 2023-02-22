OUTPUT_FORMAT("binary")
ENTRY(_start)

SECTIONS {
    .AMD64_AP_trampoline : 0x1000 {
        KEEP(*(.AMD64_AP_trampoline.AMD64_AP_trampoline_start));
		*(.AMD64_AP_trampoline .AMD64_AP_trampoline.*)
	}

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