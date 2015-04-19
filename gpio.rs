use core::prelude::*;

static GPIO_ADDR: *mut u32 = 0x20200000 as *mut u32;
pub unsafe fn set_gpio_function(pin: u32, function_code: u32) {
	if pin > 53 || function_code > 7 {
		return
	}

	*(GPIO_ADDR.offset((pin / 10) as int)) = function_code << ((pin % 10) * 3) as uint
}

pub enum PinVal {
	On,
	Off
}

pub unsafe fn set_gpio(pin: u32, val: PinVal) {
	let baseAddress = GPIO_ADDR.offset(((pin / 0xff)) as int);
	let address = match val {
		On => baseAddress.offset(0x07),
		Off => baseAddress.offset(0x0a)
	};

	*(address) = 1u32 << (pin & 0x1F) as uint;
}
