//! Turns all the user LEDs on
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![no_main]

extern crate f4;
#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate panic_semihosting;

use f4::hal::prelude::*;
use f4::hal::stm32f4;
use f4::led::Leds;
use rt::ExceptionFrame;

entry!(main);

fn main() -> ! {
    let p = stm32f4::Peripherals::take().unwrap();

    let mut rcc = p.RCC.constrain();
    let gpioe = p.GPIOE.split(&mut rcc.ahb);

    let mut leds = Leds::new(gpioe);

    for led in leds.iter_mut() {
        led.on();
    }

    loop {}
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
