#![feature(lang_items)]
#![feature(globs)]
#![no_std]
 
extern crate core;
use core::prelude::*;

static GPIO_ADDR: *mut u32 = 0x20200000 as *mut u32;

#[no_mangle]
pub extern fn main() {	
	unsafe {
		set_gpio_function(16, 1);
		set_gpio(16, Off);
	}

	loop {}
}

unsafe fn set_gpio_function(pin: u32, function_code: u32) {
	if pin > 53 || function_code > 7 {
		return
	}

	*(GPIO_ADDR.offset((pin / 10) as int)) = function_code << ((pin % 10) * 3) as uint
}

enum PinVal {
	On,
	Off
}

unsafe fn set_gpio(pin: u32, val: PinVal) {
	let baseAddress = GPIO_ADDR.offset(((pin / 0xff)) as int);
	let address = match val {
		On => baseAddress.offset(0x07),
		Off => baseAddress.offset(0x0a)
	};

	*(address) = 1u32 << (pin & 0x1F) as uint;
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
