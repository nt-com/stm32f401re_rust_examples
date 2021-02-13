
#![no_main]
#![no_std]

use stm32f4::stm32f401;
use cortex_m_rt::entry;

#[allow(unused_extern_crates)] 
extern crate panic_halt; // panic handler

fn delay() {
	for _i in 0..25000 {
		// do nothing
	} 
}

#[entry]
fn main() -> ! {

	// get peripherals
	let peripherals = stm32f401::Peripherals::take().unwrap();

	// unwrapped peripherals we need.
	let rcc = &peripherals.RCC;
	let gpioa = &peripherals.GPIOA;

	let mut read_value : bool = true;

	// clock gate, GPIOA and GPIOC
	rcc.ahb1enr.write(|w| w.gpioaen().set_bit());

	// pin to output = user LED
	gpioa.moder.modify(|_, w| w.moder5().output());
	// pin to input = user button
	gpioa.moder.modify(|_, w| w.moder0().input());
	gpioa.pupdr.modify(|_, w| w.pupdr0().pull_up());
	
	loop {	// toggle
		if gpioa.idr.read().idr0().bit_is_set() {
			gpioa.odr.modify(|_, w| w.odr5().set_bit());
		} else {
			gpioa.odr.modify(|_, w| w.odr5().clear_bit());
		}
		delay();
	} // main loop

} // fn main
