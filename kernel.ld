SECTIONS {
	.init 0x8000 : {
		*(.text.main)
	}
	
	.data : {
		*(.data)
	}
	
	.text : {
		*(.text.*)
	}
}
