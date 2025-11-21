#![no_std]

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::{InputPin, OutputPin};

pub type PinSlice<'a, T> = &'a mut [&'a mut T];
pub type KeypadLayout<'a, T> = &'a [&'a [T]];

pub struct Pins<'a, R: InputPin, C: OutputPin> {
	rows: PinSlice<'a, R>,
	columns: PinSlice<'a, C>,
}

impl<'a, R: InputPin, C: OutputPin> Pins<'a, R, C> {
	pub fn new(rows: PinSlice<'a, R>, columns: PinSlice<'a, C>) -> Self {
		Pins { rows, columns }
	}
}


pub struct Keypad<'a, R: InputPin, C: OutputPin, T> {
	pins: Pins<'a, R, C>,
    layout: KeypadLayout<'a, T>,
}

impl<'a, R: InputPin, C: OutputPin, T> Keypad<'a, R, C, T> {
    pub fn new(pins: Pins<'a, R, C>, layout: KeypadLayout<'a, T>) -> Self {
        Self {
			pins,
            layout,
        }
    }

    pub fn read_char(&mut self, delay: &mut dyn DelayNs) -> Option<&T> {
        match self.read_index(delay) {
            Some((row, column)) => Some(&self.layout[row][column]),
            None => None,
        }
    }

    fn read_index(&mut self, delay: &mut dyn DelayNs) -> Option<(usize, usize)> {
        let rows = self.pins.rows.len();
        let cols = self.pins.columns.len();

        let mut found: Option<(usize, usize)> = None;

        for c in 0..cols {
            let _ = self.pins.columns[c].set_low();
            delay.delay_ms(1u32);

            for r in 0..rows {
                if self.pins.rows[r].is_low().unwrap_or(false) {
                    if found.is_some() {
                        // multi-key → treat as "no key"
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
