//! On-board user LEDs

use core::ops;

use hal::prelude::*;

use hal::gpio::gpiod::{self, PD12, PD13, PD14, PD15, PD};
use hal::gpio::{Output, PushPull};

/// User Orange LED
pub type LD3 = PD13<Output<PushPull>>;

/// User Green LED
pub type LD4 = PD12<Output<PushPull>>;

/// User Red LED
pub type LD5 = PD14<Output<PushPull>>;

/// User Blue LED
pub type LD6 = PD15<Output<PushPull>>;

/// Array of all the user LEDs on the board
pub struct Leds {
    leds: [Led; 4],
}

impl Leds {
    /// Initializes all the user LEDs
    pub fn new(gpiod: gpiod::Parts) -> Self {
        let ld3 = gpiod
            .pd13
            .into_push_pull_output();
        let ld4 = gpiod
            .pd12
            .into_push_pull_output();
        let ld5 = gpiod
            .pd14
            .into_push_pull_output();
        let ld6 = gpiod
            .pd15
            .into_push_pull_output();

        Leds {
            leds: [
                ld3.into(),
                ld4.into(),
                ld5.into(),
                ld6.into(),
            ],
        }
    }
}

impl ops::Deref for Leds {
    type Target = [Led];

    fn deref(&self) -> &[Led] {
        &self.leds
    }
}

impl ops::DerefMut for Leds {
    fn deref_mut(&mut self) -> &mut [Led] {
        &mut self.leds
    }
}

impl ops::Index<usize> for Leds {
    type Output = Led;

    fn index(&self, i: usize) -> &Led {
        &self.leds[i]
    }
}

impl ops::IndexMut<usize> for Leds {
    fn index_mut(&mut self, i: usize) -> &mut Led {
        &mut self.leds[i]
    }
}

/// One of the on-board user LEDs
pub struct Led {
    pd: PD<Output<PushPull>>,
}

macro_rules! ctor {
    ($($ld:ident),+) => {
        $(
            impl Into<Led> for $ld {
                fn into(self) -> Led {
                    Led {
                        pd: self.downgrade(),
                    }
                }
            }
        )+
    }
}

ctor!(LD3, LD4, LD5, LD6);

impl Led {
    /// Turns the LED off
    pub fn off(&mut self) {
        self.pd.set_low()
    }

    /// Turns the LED on
    pub fn on(&mut self) {
        self.pd.set_high()
    }
}
