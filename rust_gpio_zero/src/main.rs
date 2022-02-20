use rust_gpiozero::*;
use std::thread;
use std::time::Duration;
fn main() {
    let led = LED::new(14);
    for i in 0..5 {
        led.on();
        println!("{}", i);
        thread::sleep(Duration::from_secs(1));
        led.off();
        thread::sleep(Duration::from_secs(1));
    }
    println!("Heloo RPI");
}
