extern crate rust_gpiozero;
use rust_gpiozero::*;
use std::time::{Duration, Instant};

fn main() {
    let mut led = LED::new(2);
    let mut button = Button::new(4);

    let mut last_clicked = Instant::now();
    loop{
        println!("wait for button");
        button.wait_for_press(None);

        if last_clicked.elapsed() < Duration::new(1, 0) {
            continue
        }

        // Make the led switch on
        println!("button pressed!");
        led.toggle();
        last_clicked = Instant::now()
    }
}
