use embedded_hal::digital::{Error, OutputPin, PinState};
use crate::interface::LED5011ASDriver;

/// A generic driver for the 5011as 7-segment display.
pub struct GenericLED5011AS<'a, PinA, PinB, PinC, PinD, PinE, PinF, PinG, PinDp> {
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
GenericLED5011AS<'a, PinA, PinB, PinC, PinD, PinE, PinF, PinG, PinDp>
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
    /// Creates a new generic driver instance for a 5011as 7-segment display using [this](https://components101.com/sites/default/files/component_pin/7-segment-display-pin-diagr_0.png) layout for the 8 pins.
    ///
    /// The advantage of using the generic driver is that the pins don't all have to be of the same type.
    /// Fox example pin a can be of type `OutputPin<'_>` while pin b can be of type `Flex<'_>`.
    /// The downside of this approach is that this struct defines 8 generic types which is cumbersome when using this driver as a field in another struct because the parent has to define these generics as well.
    ///
    /// If all your pins are the same type see [`LED5011AS`] instead.
    ///
    /// # Examples
    ///
    /// ```rust
    /// let cfg = OutputConfig::default();
    ///
    /// let mut a = Output::new(peripherals.GPIO6, Level::Low, cfg);
    /// let mut b = Output::new(peripherals.GPIO7, Level::Low, cfg).into_flex(); // other pins can be other types
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
    ///
    /// [`LED5011AS`]: crate::simple_driver::LED5011AS
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
}

impl<'a, PinA, PinB, PinC, PinD, PinE, PinF, PinG, PinDp, E> LED5011ASDriver for GenericLED5011AS<'a, PinA, PinB, PinC, PinD, PinE, PinF, PinG, PinDp>
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
    type Error = E;

    fn set_segment(&mut self, segment: u8, state: PinState) -> Result<(), Self::Error> {
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