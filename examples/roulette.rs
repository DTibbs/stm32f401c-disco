//! A LED roulette!
#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![no_main]

#[macro_use(entry, exception)]
extern crate cortex_m_rt as rt;
extern crate cortex_m;
extern crate f4;
extern crate panic_halt;

use f4::hal::delay::Delay;
use f4::hal::prelude::*;
use f4::hal::stm32f4::stm32f401;
use f4::led::Leds;
use rt::ExceptionFrame;

entry!(main);

fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f401::Peripherals::take().unwrap();

    //let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();
    let gpiod = dp.GPIOD.split();

    // clock configuration using the default settings (all clocks run at 8 MHz)
    //let clocks = rcc.cfgr.freeze(&mut flash.acr);
    // TRY this alternate clock configuration (all clocks run at 16 MHz)
    let clocks = rcc.cfgr.sysclk(16.mhz()).freeze();

    let mut leds = Leds::new(gpiod);
    let mut delay = Delay::new(cp.SYST, clocks);

    let n = leds.len();
    loop {
        for curr in 0..n {
            let next = (curr + 1) % n;
            leds[curr].off();
            leds[next].on();

            delay.delay_ms(100_u8);
        }
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
