//! Test the serial interface
//!
//! This example requires you to short (connect) the TX and RX pins.
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_main]
#![no_std]

extern crate cortex_m;
#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate f4;
#[macro_use(block)]
extern crate nb;
extern crate panic_halt;

use cortex_m::asm;
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
    //let gpiob = p.GPIOB.split();

    // clock configuration using the default settings (all clocks run at 8 MHz)
    //let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // TRY this alternate clock configuration (clocks run at nearly the maximum frequency)
    let clocks = rcc.cfgr.sysclk(64.mhz()).pclk1(32.mhz()).freeze();

    // The Serial API is highly generic
    // TRY the commented out, different pin configurations
    let tx = gpioa.pa9.into_alternate_af7();
    // let tx = gpiob.pb6.into_af7(&mut gpiob.moder, &mut gpiob.afrl);

    let rx = gpioa.pa10.into_alternate_af7();
    //let rx = gpiob.pb7.into_alternate_af7();

    // TRY using a different USART peripheral here
    let config = config::Config::default()
        .baudrate(9_600.bps())
        .parity_none()
        .wordlength_8()
        .stopbits(config::StopBits::STOP1);
    let serial = Serial::usart1(p.USART1, (tx, rx), config, clocks).unwrap();
    let (mut tx, mut rx) = serial.split();

    let sent = b'X';

    // The `block!` macro makes an operation block until it finishes
    // NOTE the error type is `!`
    block!(tx.write(sent)).ok();

    let received = block!(rx.read()).unwrap();

    assert_eq!(received, sent);

    // if all goes well you should reach this breakpoint
    asm::bkpt();

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
