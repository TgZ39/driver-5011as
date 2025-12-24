use embedded_hal::digital::{Error, OutputPin, PinState};
use crate::interface::LED5011ASDriver;

pub struct LED5011AS<'a, Pin> {
    segments: [&'a mut Pin; 8]
}

impl<'a, Pin, E> LED5011AS<'a, Pin>
where
    Pin: OutputPin<Error = E>,
    E: Error
{
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
        Self { segments: [a, b, c, d, e, f, g, dp] }
    }
}

impl<'a, Pin, E> LED5011ASDriver for LED5011AS<'a, Pin>
where
    Pin: OutputPin<Error = E>,
    E: Error
{
    type Error = E;

    fn set_segment(&mut self, segment: u8, state: PinState) -> Result<(), Self::Error> {
        self.segments[segment as usize].set_state(state)
    }
}