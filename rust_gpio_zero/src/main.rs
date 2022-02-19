use rust_gpiozero::*;
use std::thread;
use std::time::Duration;
fn main() {
let led = LED::new(17);
for _ in 0..5{
    led.on();
    thread::sleep(Duration::from_secs(1));
    led.off();
    thread::sleep(Duration::from_secs(1));
}
println!("Heloo RPI");
}
