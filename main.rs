#![feature(lang_items)]
#![feature(globs)]
#![no_std]
 
extern crate core;
use core::prelude::*;

static GPIO_ADDR: *mut uint = 0x20200000 as *mut uint;

#[no_mangle]
pub extern fn main() {	
	unsafe {
		set_gpio_function(16, 1);
		*(GPIO_ADDR.offset(10)) = 1 << 16;
	}

	loop {}
}

unsafe fn set_gpio_function(pin: uint, function_code: uint) {
	*(GPIO_ADDR.offset((pin / 10) as int)) = function_code << ((pin % 10) * 3)
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
