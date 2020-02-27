extern crate rust_gpiozero;
use rust_gpiozero::*;

fn main() {
    let mut led = LED::new(2);

    led.blink(1.0, 1.0);
}
