use std::time::Duration;
use std::thread::sleep;
use std::process;
use signals::{register_signal_handlers, stopped};

fn main() {
    register_signal_handlers();
    let duration = Duration::from_secs(1);
    println!("waiter id={}", process::id());
    while !stopped() {
        println!("waiting...");
        sleep(duration);
    }
    println!("stopped");
}