#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::peripheral::{syst, Peripherals};

//use cortex_m::asm;
use cortex_m_rt::entry;
use stm32f4;
use stm32f4::stm32f411;

//let mut peripherals = stm32f401::Peripherals::take().unwrap();
//let gpioa = &peripherals.GPIOA;
//gpioa.odr.modify(|_, w| w.odr0().set_bit());
//

#[entry]
fn main() -> ! {
	let aa :u8 = 40;
    //asm::nop(); // To not have main optimize to abort in release mode, remove when you add code
	let peripherals = Peripherals::take().unwrap();
	let mut pp = stm32f411::Peripherals::take().unwrap();
    let mut systick = peripherals.SYST;
    systick.set_clock_source(syst::SystClkSource::Core);
    systick.set_reload(1_000);
    systick.clear_current();
    systick.enable_counter();
    while !systick.has_wrapped() {
        // Loop
    }
	let mut _bb = aa;
    loop {
	//assert_eq!(200u8.wrapping_add(55), 255);
	_bb = _bb.wrapping_add(aa);
	//asm::nop();
	_bb = _bb.wrapping_add(aa );
        // your code goes here
    }
}
