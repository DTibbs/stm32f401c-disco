//! Board support crate for the STM32F3DISCOVERY
//!
//! # Usage
//!
//! - Trying out the examples
//!
//! ``` text
//! $ # if you don't have the clone subcommand
//! $ cargo install cargo-clone
//!
//! $ cargo clone f4 --vers 0.0.1
//!
//! # on another terminal
//! $ openocd -f interface/stlink-v2-1.cfg -f target/stm32f4x.cfg
//!
//! # flash and debug the "Hello, world" example
//! $ cd f4
//! $ rustup target add thumbv7em-none-eabihf
//! $ cargo run --example hello
//! ```
//!
//! You'll need to have both OpenOCD and arm-none-eabi-gcc installed.
//!
//! - Building an application that depends on this crate
//!
//! To build applications (binary crates) using this crate follow [cortex-m-quickstart] instructions
//! and add this crate as a dependency in step number 6 and make sure you enable the "rt" Cargo
//! feature of this crate. Also, instead of step number 4 remove *both* the build.rs and memory.x
//! files.
//!
//! [cortex-m-quickstart]: https://docs.rs/cortex-m-quickstart/~0.3
//!
//! # Examples
//!
//! See the [examples] module.
//!
//! [examples]: examples/index.html

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

// TODO: add driver crates for LIS3x and CS43L22 here when they're ready
pub extern crate stm32f4xx_hal as hal;

//use hal::gpio::gpioa::{PA5, PA6, PA7};
//use hal::gpio::gpiob::{PB6, PB7};
//use hal::gpio::gpioe::PE3;
//use hal::gpio::{AF4, AF5, Output, PushPull};
//use hal::i2c::I2c;
//use hal::spi::Spi;
// TODO: Use I2S for the CS43L22
//use hal::stm32f4::{I2C1, SPI1};

pub mod examples;
pub mod led;

// On board LIS302DL(revB) or LIS3DSH(revC) connected to the SPI1 bus via the pins PA5, PA6, PA7 and PE3
// TODO!
//#[config(feature=revB)]
//pub type LIS302DL = lis302dl::lis302dl<Spi<SPI1, (PA5<AF5>, PA6<AF5>, PA7<AF5>)>, PE3<Output<PushPull>>>;
//#[config(feature=revC)]
//pub type LIS3DSH = lis3dsh::lis3dsh<Spi<SPI1, (PA5<AF5>, PA6<AF5>, PA7<AF5>)>, PE3<Output<PushPull>>>;


// On board CS43L22 controlled through I2C1 and processes digital signals through I2S3
// TODO!
//pub type CS43L22 = cs43l22::cs43l22<i2c<I2C1, 