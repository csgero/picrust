#![feature(lang_items)]
#![feature(globs)]
#![no_std]
 
extern crate core;
use core::prelude::*;

mod gpio;

#[no_mangle]
pub extern fn main() {	
	unsafe {
		gpio::set_gpio_function(16, 1);
		gpio::set_gpio(16, gpio::Off);
	}

	loop {}
}


#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }

//required for core	
#[lang = "begin_unwind"]
#[allow(unused_variable)]
extern fn begin_unwind(args: &core::fmt::Arguments,
                       file: &str,
                       line: uint) -> ! {
    loop {}
}
