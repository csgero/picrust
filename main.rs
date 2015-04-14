#![feature(lang_items)]
#![no_std]

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }

#[no_mangle]
pub extern fn main() {
	let gpio_addr: u32 = 0x20200000;
	
	unsafe {
		*((gpio_addr + 4) as *mut u32) = 1 << 18;
		*((gpio_addr + 40) as *mut u32) = 1 << 16;
	}

	loop {}
}