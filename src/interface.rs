use embedded_hal::digital::{Error, PinState};
use crate::DIGITS;

pub trait LED5011ASDriver {
    type Error: Error;

    fn set_segment(&mut self, segment: u8, state: PinState) -> Result<(), Self::Error>;

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
    fn set_digit(&mut self, digit: u8, dp: bool) -> Result<(), Self::Error> {
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
    fn clear(&mut self) -> Result<(), Self::Error> {
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
    fn write_byte(&mut self, data: u8) -> Result<(), Self::Error> {
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
}