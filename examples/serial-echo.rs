//! Serial interface echo server
//!
//! In this example every received byte will be sent back to the sender. You can test this example
//! with serial terminal emulator like `minicom`.
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate f4;
#[macro_use(block)]
extern crate nb;
extern crate panic_halt;

use f4::hal::prelude::*;
use f4::hal::serial::{config, Serial};
use f4::hal::stm32f4::stm32f401;
use rt::ExceptionFrame;

entry!(main);

fn main() -> ! {
    let p = stm32f401::Peripherals::take().unwrap();

    //let mut flash = p.FLASH.constrain();
    let rcc = p.RCC.constrain();
    let gpioa = p.GPIOA.split();

    let clocks = rcc.cfgr.freeze();

    let tx = gpioa.pa9.into_alternate_af7();
    let rx = gpioa.pa10.into_alternate_af7();

    let config = config::Config::default()
        .baudrate(115_200.bps())
        .parity_none()
        .wordlength_8()
        .stopbits(config::StopBits::STOP1);
    let serial = Serial::usart1(p.USART1, (tx, rx), config, clocks).unwrap();
    let (mut tx, mut rx) = serial.split();
    
    loop {
        let byte = block!(rx.read()).unwrap();
        block!(tx.write(byte)).ok();
    }
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("{:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
