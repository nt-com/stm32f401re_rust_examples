#![no_main]
#![no_std]

// PWM Example
// PWM Output on Pin A6 (D12 on the Nucleo Board)
// nmt @ nt-com 2021

/// IMPORTS
use stm32f4::stm32f401;

use cortex_m_rt::entry;

#[allow(unused_extern_crates)] 
extern crate panic_halt; // panic handler

/// FUNCTIONS
fn delay() {
	for _i in 0..1000 {
		// do nothing
	}
}

/// MAIN
#[entry]
fn main() -> ! {

	// get peripherals of the STM32F401
	let peripherals = stm32f401::Peripherals::take().unwrap();

	// extract peripherals
	let rcc = &peripherals.RCC;
	let gpioa = &peripherals.GPIOA;
	let tim = &peripherals.TIM3;

	let mut duty_cycle : u32 = 0;

	// clock gate
	rcc.ahb1enr.write(|w| w.gpioaen().set_bit());
	rcc.apb1enr.write(|w| w.tim3en().set_bit());

	// no pull
	gpioa.pupdr.modify(|_, w| w.pupdr6().floating());
	// push pull output type
	gpioa.otyper.modify(|_, w| w.ot6().push_pull());
	// alternate function mode
	gpioa.moder.modify(|_, w| w.moder6().alternate());
	// set concrete alternate function
	// af02 is timer3 channel 1
	gpioa.afrl.modify(|_, w| w.afrl6().af2());

	// set timer frequency and initial duty cycle
	unsafe {
		tim.arr.write(|w| w.bits(0x8000)); // frequency 
		tim.ccr1.write(|w| w.bits(0x0)); // duty cycle
	}

	// clear enable to zero just to be sure.
	tim.ccmr1_output().write(|w| w.oc1ce().clear_bit());

	//  enable preload
	tim.ccmr1_output().write(|w| w.oc1pe().enabled());
	
	// set pwm mode
	tim.ccmr1_output().write(|w| w.oc1m().pwm_mode1());

	// enable output
	tim.ccer.write(|w| w.cc1e().set_bit());

	// enable auto-reload
	tim.cr1.write(|w| w.arpe().set_bit());

	// enable update generation - needed at first start
	tim.egr.write(|w| w.ug().set_bit());

	// start pwm
	tim.cr1.write(|w| w.cen().set_bit());

	loop {
		unsafe { // increase DC by 256, then start again at zero.
			tim.ccr1.write(|w| w.bits(duty_cycle)); // duty cycle
		}
		duty_cycle = (duty_cycle + 256) % 0x8000;
		delay();
	} // main loop

}


