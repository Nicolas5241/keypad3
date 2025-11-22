use embedded_hal::{delay::DelayNs, digital::{InputPin, OutputPin}};

use crate::{Keypad, KeypadInterface, KeypadLayout, Pins};

impl<'a, R: InputPin, C: OutputPin, T, D: DelayNs> Keypad<'a, R, C, T, D> {
	pub fn new(pins: Pins<'a, R, C>, layout: KeypadLayout<'a, T>, delay: D) -> Self {
		Self {
			pins,
			layout,
			delay,
		}
	}

	pub fn read(&mut self) -> Option<&T> {
		match self.read_index() {
			Some((row, column)) => Some(&self.layout[row][column]),
			None => None,
		}
	}

	fn read_index(&mut self) -> Option<(usize, usize)> {
		let rows = self.pins.rows.len();
		let cols = self.pins.columns.len();

		let mut found: Option<(usize, usize)> = None;

		for c in 0..cols {
			let _ = self.pins.columns[c].set_low();
			self.delay.delay_ms(1u32);

			for r in 0..rows {
				if self.pins.rows[r].is_low().unwrap_or(false) {
					if found.is_some() {
						//treat multi presses as no key
						let _ = self.pins.columns[c].set_high();
						return None;
					}

					found = Some((r, c));
				}
			}

			let _ = self.pins.columns[c].set_high();
		}

		found
	}
}

impl<'a, R, C, L, D> KeypadInterface for Keypad<'a, R, C, L, D> where R: InputPin, C: OutputPin, D: DelayNs {
	type Key = &'a L;
	fn read(&mut self) -> Option<Self::Key> {
		match self.read_index() {
			Some((row, column)) => Some(&self.layout[row][column]),
			None => None,
		}
	}
}
