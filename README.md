A crate based on keypad2 adapted to work with 4x4 matrix keypads instead.

## Example

```rust
let rows = (
    pins.d7.into_pull_up_input(),
    pins.d6.into_pull_up_input(),
    pins.d5.into_pull_up_input(),
    pins.d4.into_pull_up_input(),
);

let columns = (
    pins.d3.into_opendrain(),
    pins.d2.into_opendrain(),
    pins.d1.into_opendrain(),
    pins.d0.into_opendrain(),
);

let mut keypad = Keypad::new(rows, columns);

let key = keypad.read_char(&mut delay);
if key != ' ' {
    ...
}
```
