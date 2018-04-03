#[macro_use] extern crate log;
extern crate env_logger;
extern crate firmata;
extern crate serial;

use firmata::errors::*;
use firmata::connection::Connection;
use std::{thread, time};

fn run() -> Result<()> {
    env_logger::init();

    let device = "/dev/ttyACM0";
    let mut conn = Connection::open(device)?;
    conn.resync()?;

    for apin in 0..16 {
        let dpin = apin + 54;
        info!("Enabling analog reporting on pin {} (A{})", dpin, apin);
        conn.set_pin_mode(dpin, firmata::protocol::PinMode::AnalogInput)?;
        conn.analog_report(apin, true)?;
    }

    info!("Monitoring serial interface for analog reads.");
    for _ in 0..100 {
        thread::sleep(time::Duration::from_millis(10));
        println!("{:?}", conn.read());
    }
    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}
