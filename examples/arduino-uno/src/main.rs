#![no_std]
#![no_main]

use arduino_hal::{ Delay, default_serial };
use panic_halt as _;
use keypad3::{Keypad, KeypadLayout, Pins};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
	
	let delay = Delay::new();
	let mut serial = default_serial!(dp, pins, 57600);

	let keypad_row_pins = &mut [
		&mut pins.d2.into_pull_up_input().downgrade(),
		&mut pins.d3.into_pull_up_input().downgrade(),
		&mut pins.d4.into_pull_up_input().downgrade(),
		&mut pins.d5.into_pull_up_input().downgrade(),
	];

	let keypad_column_pins = &mut [
		&mut pins.d8.into_opendrain().downgrade(),
		&mut pins.d7.into_opendrain().downgrade(),
		&mut pins.d6.into_opendrain().downgrade(),
	];

	let keypad_pins = Pins::new(keypad_row_pins, keypad_column_pins);

	let keypad_layout: KeypadLayout<_> = &[
		&['1', '2', '3'],
		&['4', '5', '6'],
		&['7', '8', '9'],
		&['*', '0', '#'],
	];

	let mut keypad = Keypad::new(keypad_pins, keypad_layout, delay);

	let mut key_pressed = false;
	
    loop {
		let key = keypad.read();
		if let Some(key) = key {
			if !key_pressed {
				ufmt::uwriteln!(&mut serial, "{}", key);
				key_pressed = true;
			}
		} else {
		    key_pressed = false;
		}
    }
}
