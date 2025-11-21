use embedded_hal::digital::{InputPin, OutputPin};

use crate::{PinSlice, Pins};

impl<'a, R: InputPin, C: OutputPin> Pins<'a, R, C> {
	pub fn new(rows: PinSlice<'a, R>, columns: PinSlice<'a, C>) -> Self {
		Pins { rows, columns }
	}
}
