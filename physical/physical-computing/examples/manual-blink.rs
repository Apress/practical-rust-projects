extern crate rust_gpiozero;
use rust_gpiozero::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let mut led = LED::new(2);

    loop{
        println!("on");
        led.on();

        sleep(Duration::from_secs(1));

        println!("off");
        led.off();

        sleep(Duration::from_secs(1));
    }
}
