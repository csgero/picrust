#![feature(lang_items)]
#![feature(globs)]
#![no_std]
 
extern crate core;
use core::prelude::*;

#[no_mangle]
pub extern fn main() {
	let gpio_addr = 0x20200000 as *mut u32;
	
	unsafe {
		*(gpio_addr.offset(1)) = 1 << 18;
		*(gpio_addr.offset(10)) = 1 << 16;
	}

	loop {}
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }

//required for core	
#[lang = "begin_unwind"]
extern fn begin_unwind(args: &core::fmt::Arguments,
                       file: &str,
                       line: uint) -> ! {
    loop {}
}
