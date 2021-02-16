/// ADC Single Conversion - Blocking Mode
/// LED on Pin A5
/// ADC on Pin A0
/// nt-com

#![no_main]
#![no_std]

use stm32f4::stm32f401;
use cortex_m_rt::entry;

#[allow(unused_extern_crates)] 
extern crate panic_halt; // panic handler

fn delay() {
	for _i in 0..10000 {
		// do nothing.
	}
}

#[entry]
fn main() -> ! {

	// value measured by the ADC 
	let mut adc_measure : u16;

	// get peripherals
	let peripherals = stm32f401::Peripherals::take().unwrap();

	// unwrapped peripherals we need.
	let rcc = &peripherals.RCC;
	let gpioa = &peripherals.GPIOA;
	let adc = &peripherals.ADC1;

	// clock gate, GPIOA and ADC1
	rcc.ahb1enr.write(|w| w.gpioaen().set_bit());
	rcc.apb2enr.write(|w| w.adc1en().set_bit());

	// pin to ADC analog in
	gpioa.moder.modify(|_, w| w.moder0().analog());
	// pin to output 
	gpioa.moder.modify(|_, w| w.moder5().output());
	
	// adc on
	adc.cr2.modify(|_,w| w.adon().set_bit());

	loop {	// toggle
	
		// start a single conversion 
		adc.cr2.modify(|_,w| w.swstart().start());
		// wait until end of conversion
		while adc.sr.read().eoc().is_not_complete() {}
		// read the measurement value when done, resets EOC
		adc_measure = adc.dr.read().data().bits();
		// set LED if measurement larger than the value given ~ 0.5V
		if adc_measure > 0x200 {
			gpioa.odr.modify(|_, w| w.odr5().set_bit());
		} else { // LED off otherwise
			gpioa.odr.modify(|_, w| w.odr5().clear_bit());
		}
		delay(); // wait a little bit before the next conversion

	} // main loop

} // fn main
