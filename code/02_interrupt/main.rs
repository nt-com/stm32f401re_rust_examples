#![no_main]
#![no_std]

// Interrupt Example for STM32F401RE
// Button on Pin A0
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
static G_EXTI: cortex_m::interrupt::Mutex<RefCell<Option<stm32f401::EXTI>>> =
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
	let syscfg = &peripherals.SYSCFG;
	let exti = &peripherals.EXTI;

	// if you wanna use a different pin, instead of hardcoding, just change this.
	// currently using pin0 on GPIOA
	let exti0_configuration : u8 = 0;

	// clock gate
	rcc.ahb1enr.write(|w| w.gpioaen().set_bit());
	rcc.apb2enr.write(|w| w.syscfgen().set_bit());

	// pin to output = user LED
	gpioa.moder.modify(|_, w| w.moder5().output());

	// pin to input = user button
	gpioa.moder.modify(|_, w| w.moder0().input());
	gpioa.pupdr.modify(|_, w| w.pupdr0().pull_down());

	// interrupt setup 	
	// see manual section 7 - 7.2.3
	// member bits is unsafe 
	syscfg.exticr1.write(|w| unsafe { w.exti0().bits(exti0_configuration) });

	// see manual section 10 - 10.3.1
	// exti set mask register
	exti.imr.write(|w| w.mr0().set_bit());
	// exti set trigger to rising
	exti.rtsr.write(|w| w.tr0().set_bit());

    cortex_m::interrupt::free(|cs| {
		// https://doc.rust-lang.org/std/cell/struct.RefCell.html
		// replaces the wrapped value with a new one, 
		// returning the old value, without deinitializing either one. 
		G_EXTI.borrow(cs).replace(Some(peripherals.EXTI));
		G_GPIOA.borrow(cs).replace(Some(peripherals.GPIOA));
	});

	// enable interrupt in NVIC
	unsafe { // https://docs.rs/cortex-m/0.7.1/cortex_m/peripheral/struct.NVIC.html
		cortex_m::peripheral::NVIC::unmask(stm32f401::Interrupt::EXTI0);	
	}
	
	loop {	
		delay();
	} // main loop

}

/// simple counter delay
fn delay() {
	for _i in 0..100000	{}
}

/// ISR 
#[interrupt]
fn EXTI0() {

	// static like in C
	static mut TOGGLE : bool = false;

	// execute closure f in an interrupt-free context.
	// this as also known as a "critical section".
    cortex_m::interrupt::free(|cs| {
        G_EXTI.borrow(cs) // Borrows the data for the duration of the critical section
            .borrow() 
            .as_ref() // used to do a cheap reference-to-reference conversion.
            .unwrap()
            .pr // pending request register
            .write(|w| w.pr0().clear());
    });
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



