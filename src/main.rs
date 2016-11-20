extern crate peripheral_linux;
extern crate peripheral;
extern crate spidev;

use peripheral::prelude::*;

use peripheral_linux::GPIO;

use std::thread::sleep;
use std::time::Duration;
use std::io::Write;

use spidev::{Spidev, SpidevOptions, SpidevTransfer, SPI_MODE_0};

fn wait() {
    sleep(Duration::from_millis(100));
}

fn main() {

    let mut out_en = GPIO::new(25).unwrap();

    let mut spi = Spidev::open("/dev/spidev0.1").unwrap();
    let mut options = SpidevOptions::new();
    options.bits_per_word(8);
    options.max_speed_hz(1_000);
    options.mode(SPI_MODE_0);
    spi.configure(&options).unwrap();

    out_en.set_to_output().unwrap();
    out_en.set_high().unwrap();

    wait();



    loop {
        spi.write(&[0xF0]).unwrap();
    }
}
