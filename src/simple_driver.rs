use crate::interface::LED5011ASDriver;
use embedded_hal::digital::{Error, OutputPin, PinState};

pub struct LED5011AS<'a, Pin> {
    segments: [&'a mut Pin; 8],
}

impl<'a, Pin, E> LED5011AS<'a, Pin>
where
    Pin: OutputPin<Error = E>,
    E: Error,
{
    /// Creates a new driver instance for a 5011as 7-segment display using [this](https://components101.com/sites/default/files/component_pin/7-segment-display-pin-diagr_0.png) layout for the 8 pins.
    ///
    /// Use this driver if all your pins are the same type as seen in the example.
    /// If your pins are not of the same type see [`GenericLED5011AS`] instead.
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
    /// // create LED5011AS
    /// let mut display = LED5011AS::new(
    ///    &mut a,
    ///    &mut b,
    ///    &mut c,
    ///    &mut d,
    ///    &mut e,
    ///    &mut f,
    ///    &mut g,
    ///    &mut dp,
    /// );
    /// ```
    ///
    /// [`GenericLED5011AS`]: crate::generic_driver::GenericLED5011AS
    pub fn new(
        a: &'a mut Pin,
        b: &'a mut Pin,
        c: &'a mut Pin,
        d: &'a mut Pin,
        e: &'a mut Pin,
        f: &'a mut Pin,
        g: &'a mut Pin,
        dp: &'a mut Pin,
    ) -> Self {
        Self {
            segments: [a, b, c, d, e, f, g, dp],
        }
    }
}

impl<'a, Pin, E> LED5011ASDriver for LED5011AS<'a, Pin>
where
    Pin: OutputPin<Error = E>,
    E: Error,
{
    type Error = E;

    fn set_segment(&mut self, segment: u8, state: PinState) -> Result<(), Self::Error> {
        self.segments[segment as usize].set_state(state)
    }
}
