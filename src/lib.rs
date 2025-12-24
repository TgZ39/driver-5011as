//! This library provides a very simple driver for the 5011as 7-segment display.
//! The layout for the pins is the following: https://components101.com/sites/default/files/component_pin/7-segment-display-pin-diagr_0.png
//!
//! This crate provides 2 drivers for the 5011as:
//! - [`LED5011AS`]\: Use this if all your pins have the same type (e.g. [`OutputPin<'_>`])
//! - [`GenericLED5011AS`]: Use this if your pins don't have the same type (e.g. [`OutputPin<'_>`] and `Flex<'_>` (from esp-hal))
//!
//! # Usage
//!
//! For this example I'm using an ESP32 with `esp-hal` with the [`LED5011AS`] driver.
//!
//! ```rust
//! // setup pins
//! let peripherals = esp_hal::init(config);
//!
//! let cfg = OutputConfig::default();
//!
//! let mut a = Output::new(peripherals.GPIO6, Level::Low, cfg);
//! let mut b = Output::new(peripherals.GPIO7, Level::Low, cfg);
//! let mut c = Output::new(peripherals.GPIO8, Level::Low, cfg);
//! let mut d = Output::new(peripherals.GPIO1, Level::Low, cfg);
//! let mut e = Output::new(peripherals.GPIO0, Level::Low, cfg);
//! let mut f = Output::new(peripherals.GPIO5, Level::Low, cfg);
//! let mut g = Output::new(peripherals.GPIO4, Level::Low, cfg);
//! let mut dp = Output::new(peripherals.GPIO10, Level::Low, cfg);
//!
//! // create LED5011AS
//! let mut display = LED5011AS::new(
//!    &mut a,
//!    &mut b,
//!    &mut c,
//!    &mut d,
//!    &mut e,
//!    &mut f,
//!    &mut g,
//!    &mut dp,
//! );
//!
//! // set a digit
//! display.set_digit(3)?;
//! display.set_digit(7)?;  // overrides the '3' so you don't have to clear the display inbetween
//!
//! // clear the display
//! display.clear()?;
//!
//! // write a custom figure to the display
//! display.write_byte(0b01110111)?;    // writes an 'A' to the display
//! ```
//!
//! [`OutputPin<'_>`]: embedded_hal::digital::OutputPin

#![no_std]
#![allow(clippy::too_many_arguments)]

pub use generic_driver::GenericLED5011AS;
pub use interface::LED5011ASDriver;
pub use simple_driver::LED5011AS;

mod generic_driver;
mod interface;
mod simple_driver;

const DIGITS: [u8; 10] = [
    0b0111111, 0b0000110, 0b1011011, 0b1001111, 0b1100110, 0b1101101, 0b1111101, 0b0000111,
    0b1111111, 0b1101111,
];
