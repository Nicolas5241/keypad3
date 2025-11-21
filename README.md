A crate based on keypad2 adapted to work with matrix keypads of any size.

## Example

```rust
let rows = &mut [
    &mut pins.d7.into_pull_up_input().downgrade(),
    &mut pins.d6.into_pull_up_input().downgrade(),
    &mut pins.d5.into_pull_up_input().downgrade(),
    &mut pins.d4.into_pull_up_input().downgrade(),
];

let columns = &mut [
    &mut pins.d3.into_opendrain().downgrade(),
    &mut pins.d2.into_opendrain().downgrade(),
    &mut pins.d1.into_opendrain().downgrade(),
    &mut pins.d0.into_opendrain().downgrade(),
];

let pins = Pins::new(rows, columns);

let layout = &[
    &['1', '2', '3', 'A'],
    &['4', '5', '6', 'B'],
    &['7', '8', '9', 'C'],
    &['*', '0', '#', 'D'],
]

let mut keypad: KeypadLayout<_> = Keypad::new(pins, layout, delay);

let key = keypad.read();

if let Some(key_value) = key {
    ...
}
```
