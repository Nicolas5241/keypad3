#![no_std]

use embedded_hal::blocking::delay::DelayMs;
use embedded_hal::digital::v2::{InputPin, OutputPin};

pub struct Pins<'a, R: InputPin, C: OutputPin> {
	rows: &'a mut [R],
	columns: &'a mut [C],
}

impl<'a, R: InputPin, C: OutputPin> Pins<'a, R, C> {
	pub fn new(rows: &'a mut [R], columns: &'a mut [C]) -> Self {
		Pins { rows, columns }
	}
}

pub type KeypadLayout<'a> = &'a [&'a [char]];

pub struct Keypad<'a, R: InputPin, C: OutputPin> {
	pins: Pins<'a, R, C>,
    layout: KeypadLayout<'a>,
}

impl<'a, R: InputPin, C: OutputPin> Keypad<'a, R, C> {
    pub fn new(pins: Pins<'a, R, C>, layout: &'a [&'a [char]]) -> Self {
        Self {
			pins,
            layout,
        }
    }

    pub fn read_char(&mut self, delay: &mut dyn DelayMs<u16>) -> char {
        match self.read_index(delay) {
            Some((row, column)) => self.layout[row][column],
            None => ' ',
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
