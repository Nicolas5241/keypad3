#![no_std]

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::{InputPin, OutputPin};

pub type Rows<'a, R> = &'a [&'a R];
pub type Columns<'a, C> = &'a mut [&'a mut C];
pub type KeypadLayout<'a, T> = &'a [&'a [T]];

pub struct Pins<'a, R: InputPin, C: OutputPin> {
	rows: Rows<'a, R>,
	columns: Columns<'a, C>,
}

impl<'a, R: InputPin, C: OutputPin> Pins<'a, R, C> {
	pub fn new(rows: &'a [&'a R], columns: &'a mut [&'a mut C]) -> Self {
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

    pub fn read_char(&mut self, delay: &mut dyn DelayMs<u16>) -> Option<&T> {
        match self.read_index(delay) {
            Some((row, column)) => Some(&self.layout[row][column]),
            None => None,
        }
    }

    fn read_index(&mut self, delay: &mut dyn DelayMs<u16>) -> Option<(usize, usize)> {
        let rows = self.pins.rows.len();
        let cols = self.pins.columns.len();

        let mut found: Option<(usize, usize)> = None;

        for c in 0..cols {
            let _ = self.pins.columns[c].set_low();
            delay.delay_ms(1u16);

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
