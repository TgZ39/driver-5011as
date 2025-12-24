//! This library provides a very simple driver for the 5011as 7-segment display.
//! The layout for the pins is the following: https://components101.com/sites/default/files/component_pin/7-segment-display-pin-diagr_0.png
//!
//! # Usage
//!
//! For this example I'm using an ESP32 with `esp-hal`.
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

#![no_std]

use core::result::Result;
use embedded_hal::digital::{Error, OutputPin, PinState};

const DIGITS: [u8; 10] = [
    0b0111111, 0b0000110, 0b1011011, 0b1001111, 0b1100110, 0b1101101, 0b1111101, 0b0000111,
    0b1111111, 0b1101111,
];

/// A driver for the 5011as 7-segment display.
pub struct LED5011AS<'a, PinA, PinB, PinC, PinD, PinE, PinF, PinG, PinDp> {
    a: &'a mut PinA,
    b: &'a mut PinB,
    c: &'a mut PinC,
    d: &'a mut PinD,
    e: &'a mut PinE,
    f: &'a mut PinF,
    g: &'a mut PinG,
    dp: &'a mut PinDp,
}

impl<'a, PinA, PinB, PinC, PinD, PinE, PinF, PinG, PinDp, E>
    LED5011AS<'a, PinA, PinB, PinC, PinD, PinE, PinF, PinG, PinDp>
where
    PinA: OutputPin<Error = E>,
    PinB: OutputPin<Error = E>,
    PinC: OutputPin<Error = E>,
    PinD: OutputPin<Error = E>,
    PinE: OutputPin<Error = E>,
    PinF: OutputPin<Error = E>,
    PinG: OutputPin<Error = E>,
    PinDp: OutputPin<Error = E>,
    E: Error,
{
    /// Creates a new driver instance for a 5011as 7-segment display using [this](https://components101.com/sites/default/files/component_pin/7-segment-display-pin-diagr_0.png) layout for the 8 pins.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let cfg = OutputConfig::default();
    ///
    /// let mut a = Output::new(peripherals.GPIO6, Level::Low, cfg);
    /// let mut b = Output::new(peripherals.GPIO7, Level::Low, cfg);
    /// let mut c = Output::new(peripherals.GPIO8, Level::Low, cfg);
    /// let mut d = Output::new(peripherals.GPIO1, Level::Low, cfg);
    /// let mut e = Output::new(peripherals.GPIO0, Level::Low, cfg);
    /// let mut f = Output::new(peripherals.GPIO5, Level::Low, cfg);
    /// let mut g = Output::new(peripherals.GPIO4, Level::Low, cfg);
    /// let mut dp = Output::new(peripherals.GPIO10, Level::Low, cfg);
    ///
    /// let mut display = LED5011AS::new(
    ///     &mut a,
    ///     &mut b,
    ///     &mut c,
    ///     &mut d,
    ///     &mut e,
    ///     &mut f,
    ///     &mut g,
    ///     &mut dp,
    /// );
    /// ```
    pub fn new(
        a: &'a mut PinA,
        b: &'a mut PinB,
        c: &'a mut PinC,
        d: &'a mut PinD,
        e: &'a mut PinE,
        f: &'a mut PinF,
        g: &'a mut PinG,
        dp: &'a mut PinDp,
    ) -> Self {
        Self {
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            dp,
        }
    }

    /// Writes a digit to the display.
    /// `dp` toggles the 'dot point'.
    ///
    /// # Examples
    ///
    /// ```rust
    /// display.set_digit(4, false)?; // writes a '4' to the display
    ///
    /// display.set_digit(1, true)?; // no .clear() required
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if the provided `digit` is not in the range from 0 to 9.
    pub fn set_digit(&mut self, digit: u8, dp: bool) -> Result<(), E> {
        assert!((0..=9).contains(&digit), "digit must be between 0 and 9");

        let digit = DIGITS[digit as usize] | (dp as u8) << 7;
        self.write_byte(digit)?;

        Ok(())
    }

    /// Clears the display.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let mut display = LED5011AS::new(/* pins */);
    ///
    /// display.set_digit(5)?;
    ///
    /// display.clear()?;
    /// ```
    pub fn clear(&mut self) -> Result<(), E> {
        self.write_byte(0b00000000)
    }

    /// Writes a custom figure to the display.
    /// If you simply want to write a digit to the display see [`set_digit`] instead.
    ///
    /// The binary representation of `data` is the following: `0b[dp][g][f][e][d][c][b][a]`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// display.write_byte(0b00000110)?;   // writes '1' to the display without dp
    /// display.write_byte(0b01101101)?;   // writes '5' to the display without dp
    /// display.write_byte(0b11101111)?;   // writes '9' to the display with dp
    /// display.write_byte(0b01110111)?;   // writes an 'A' to the display
    /// ```
    /// [`set_digit`]: Self::set_digit
    pub fn write_byte(&mut self, data: u8) -> Result<(), E> {
        for i in 0..8 {
            let bit = (data & 1 << i) >> i;
            let state = match bit {
                0 => PinState::Low,
                1 => PinState::High,
                _ => unreachable!(
                    "bit can only be 0 or 1 because the bit operations checks whether a bit of the data is set or not"
                ),
            };

            self.set_segment(i, state)?;
        }

        Ok(())
    }

    fn set_segment(&mut self, segment: u8, state: PinState) -> Result<(), E> {
        match segment {
            0 => self.a.set_state(state),
            1 => self.b.set_state(state),
            2 => self.c.set_state(state),
            3 => self.d.set_state(state),
            4 => self.e.set_state(state),
            5 => self.f.set_state(state),
            6 => self.g.set_state(state),
            7 => self.dp.set_state(state),
            _ => panic!("invalid segment"),
        }
    }
}
