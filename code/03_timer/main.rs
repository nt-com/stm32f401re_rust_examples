#![no_main]
#![no_std]

// Simple Timer Example STM32F401RE
// LED on Pin A5 (user LED on Nucleo Board)
// nmt @ nt-com 2021

/// IMPORTS
use core::cell::RefCell;

use stm32f4::stm32f401;
use stm32f4::stm32f401::interrupt;

use cortex_m;
use cortex_m_rt::entry;

#[allow(unused_extern_crates)] 
extern crate panic_halt; // panic handler

/// MUTEXES
// A "mutex" based on critical sections - only safe for single core systems
static G_TIM2: cortex_m::interrupt::Mutex<RefCell<Option<stm32f401::TIM2>>> =
    cortex_m::interrupt::Mutex::new(RefCell::new(None));
static G_GPIOA: cortex_m::interrupt::Mutex<RefCell<Option<stm32f401::GPIOA>>> =
    cortex_m::interrupt::Mutex::new(RefCell::new(None));

/// MAIN
#[entry]
fn main() -> ! {

	// get peripherals of the STM32F401
	let peripherals = stm32f401::Peripherals::take().unwrap();
	// get the peripherals of the Cortex M 
	//let cortex_peripherals = cortex_m::Peripherals::take().unwrap();
	// not needed, but leaving this in here, see below how to get to the NVIC	

	// extract peripherals
	let rcc = &peripherals.RCC;
	let gpioa = &peripherals.GPIOA;
	let tim = &peripherals.TIM2;

	// clock gate
	rcc.ahb1enr.write(|w| w.gpioaen().set_bit());
	rcc.apb1enr.write(|w| w.tim2en().set_bit());

	// pin to output = user LED
	gpioa.moder.modify(|_, w| w.moder5().output());

	// setup the timer
	
	// activate update interrupt
	tim.dier.write(|w| w.uie().set_bit());
	tim.cr1.write(|w| w.udis().clear_bit()); // just to be sure.

	// enable interrupt in NVIC
	unsafe { // https://docs.rs/cortex-m/0.7.1/cortex_m/peripheral/struct.NVIC.html
		cortex_m::peripheral::NVIC::unmask(stm32f401::Interrupt::TIM2);	
	}

	// auto reload register, section 13 - 13.4.12
	unsafe { tim.arr.write(|w| w.bits(0x1ffff)); }

	// leaving the prescaler as it is here.
	unsafe { tim.psc.write(|w| w.bits(0x0008)); }

	// start the timer 
	tim.cr1.write(|w| w.cen().set_bit());

    cortex_m::interrupt::free(|cs| {
		// https://doc.rust-lang.org/std/cell/struct.RefCell.html
		// replaces the wrapped value with a new one, 
		// returning the old value, without deinitializing either one. 
		G_TIM2.borrow(cs).replace(Some(peripherals.TIM2));
		G_GPIOA.borrow(cs).replace(Some(peripherals.GPIOA));
	});
    	
	loop {} // main loop

}

#[interrupt]
fn TIM2() {

	static mut TOGGLE : bool = false;

	// enable interrupt in NVIC
	cortex_m::peripheral::NVIC::unpend(stm32f401::Interrupt::TIM2);	

    cortex_m::interrupt::free(|cs| {
    G_TIM2.borrow(cs)  // Borrows the data for the duration of the critical section
		.borrow() 
		.as_ref() // used to do a cheap reference-to-reference conversion.
		.unwrap()
		.sr // pending request register
		.write(|w| w.uif().clear_bit());
    }); // clears interrupt flag.

 	// toggle the LED
	if *TOGGLE {
    cortex_m::interrupt::free(|cs| {
        G_GPIOA.borrow(cs)
            .borrow()
            .as_ref()
            .unwrap()
            .odr
            .write(|w| w.odr5().set_bit());
		*TOGGLE = false;
    }); 
	} else {
		cortex_m::interrupt::free(|cs| {
        G_GPIOA.borrow(cs)
            .borrow()
            .as_ref()
            .unwrap()
            .odr
            .write(|w| w.odr5().clear_bit());
		*TOGGLE = true;
    	});
	} // if-else

}



