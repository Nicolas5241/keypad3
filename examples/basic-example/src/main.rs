#![no_std]
#![no_main]

use arduino_hal::{ Delay, default_serial };
use panic_halt as _;
use keypad3::{Keypad, KeypadLayout, Pins};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
	
	let mut delay = Delay::new();
	let mut serial = default_serial!(dp, pins, 57600);

	let keypad_row_pins = & [
		&pins.d5.into_pull_up_input().downgrade(),
		&pins.d4.into_pull_up_input().downgrade(),
		&pins.d3.into_pull_up_input().downgrade(),
		&pins.d2.into_pull_up_input().downgrade(),
	];

	let keypad_column_pins = &mut [
		&mut pins.d9.into_output().downgrade(),
		&mut pins.d8.into_output().downgrade(),
		&mut pins.d7.into_output().downgrade(),
		&mut pins.d6.into_output().downgrade(),
	];

	let keypad_pins = Pins::new(keypad_row_pins, keypad_column_pins);

	let keypad_layout: KeypadLayout<_> = &[
		&['1', '2', '3', 'A'],
		&['4', '5', '6', 'B'],
		&['7', '8', '9', 'C'],
		&['*', '0', '#', 'D'],
	];

	let mut keypad = Keypad::new(keypad_pins, keypad_layout);
	
    loop {
		let key = keypad.read_char(&mut delay);
		if let Some(key) = key {
			ufmt::uwriteln!(&mut serial, "{}", key);
		}
    }
}
