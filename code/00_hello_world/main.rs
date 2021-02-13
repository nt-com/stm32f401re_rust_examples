
#![no_main]
#![no_std]

use stm32f4::stm32f401;
use cortex_m_rt::entry;

#[allow(unused_extern_crates)] 
extern crate panic_halt; // panic handler

fn delay() {
	for _i in 0..25000 {
		// do nothing.
	}
}

#[entry]
fn main() -> ! {

	// get peripherals
	let mut peripherals = stm32f401::Peripherals::take().unwrap();

	let rcc = &peripherals.RCC;
	let gpioa = &peripherals.GPIOA;

	// clock gate
	rcc.ahb1enr.write(|w| w.gpioaen().set_bit());
	// pin to output
	gpioa.moder.modify(|_, w| w.moder5().output());

	loop {	// toggle
		gpioa.odr.modify(|_, w| w.odr5().set_bit());
		delay();
		gpioa.odr.modify(|_, w| w.odr5().clear_bit());
		delay();
	} // main loop

}
