use embedded_hal::{delay::DelayNs, digital::{InputPin, OutputPin}};

pub type PinSlice<'a, T> = &'a mut [&'a mut T];
pub type KeypadLayout<'a, T> = &'a [&'a [T]];

pub struct Pins<'a, R: InputPin, C: OutputPin> {
	pub rows: PinSlice<'a, R>,
	pub columns: PinSlice<'a, C>,
}

pub struct Keypad<'a, R: InputPin, C: OutputPin, T, D: DelayNs> {
	pub pins: Pins<'a, R, C>,
	pub layout: KeypadLayout<'a, T>,
	pub delay: D,
}
