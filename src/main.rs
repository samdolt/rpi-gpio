extern crate peripheral_linux;
extern crate peripheral;
use peripheral::prelude::*;

use peripheral_linux::GPIO;

use std::thread::sleep;
use std::time::Duration;

fn wait() {
        sleep(Duration::from_millis(1000));
}

fn main() {

    let mut gpio = GPIO::new(26).unwrap();

    loop {
        gpio.set_to_output().unwrap();
        gpio.set_high().unwrap();
        wait();
        gpio.set_low().unwrap();
        wait();
    }
}
