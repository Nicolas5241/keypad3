#![no_std]

use core::convert::Infallible;

use embedded_hal::digital::v2::{InputPin, OutputPin};
use embedded_hal::blocking::delay::DelayMs;

pub type Rows<'a> = &'a [&'a dyn InputPin<Error = Infallible>];
pub type Columns<'a> = &'a mut [&'a mut dyn OutputPin<Error = Infallible>];

pub struct Keypad<'a> {
    rows: Rows<'a>,
    columns: Columns<'a>,
	layout: &'a[&'a[char]],
}

impl<'a> Keypad<'a> {
    pub fn new(rows: Rows<'a>, columns: Columns<'a>, layout: &'a [&'a [char]]) -> Self {
        Self { rows, columns, layout }
    }

    pub fn read_char(&mut self, delay: &mut dyn DelayMs<u16>) -> char {
        match self.read_index(delay) {
            Some(idx) => self.index_to_char(idx),
            None => ' ',
        }
    }

    //---------------------------------------------------------------------
    // NEW: return a single pressed key index (row-major)
    //
    // index = col * rows + row   (0-based)
    //
    // If 0 or >1 keys are pressed, returns None.
    //---------------------------------------------------------------------
    fn read_index(&mut self, delay: &mut dyn DelayMs<u16>) -> Option<usize> {
        let rows = self.rows.len();
        let cols = self.columns.len();

        let mut found: Option<usize> = None;

        for c in 0..cols {
            let _ = self.columns[c].set_low();
            delay.delay_ms(1u16);

            for r in 0..rows {
                if self.rows[r].is_low().unwrap_or(false) {
                    let idx = c * rows + r;

                    if found.is_some() {
                        // multi-key → treat as "no key"
                        let _ = self.columns[c].set_high();
                        return None;
                    }

                    found = Some(idx);
                }
            }

            let _ = self.columns[c].set_high();
        }

        found
    }

    //---------------------------------------------------------------------
    // NEW: table-based, but internal — NO more bitmask KEY_ constants
    //
    // We map the row-major index to char depending on keypad size.
    //---------------------------------------------------------------------
    fn index_to_char(&self, index: usize) -> char {
        let cols = self.columns.len();

		let rows_index = index / cols;
		let cols_index = index % cols;

        self.layout[rows_index][cols_index]
	}
}
