extern crate env_logger;
extern crate firmata;
extern crate serial;

use firmata::connection::Connection;
use std::{thread, time};

fn run() -> firmata::errors::Result<()> {
    env_logger::init();

    let device = "/dev/ttyACM0";
    let mut conn = Connection::open(device)?;
    conn.resync()?;

    // Determine the port and port value associated with pin 49.
    let pin = 49;
    let port = pin / 8;
    let value = 1 << (pin % 8);
    let mut mask = 0xFF;

    conn.set_pin_mode(pin, firmata::protocol::PinMode::DigitalOutput)?;
    thread::sleep(time::Duration::from_millis(100));

    for _ in 0..2 {
        println!("Port {}: {:08b}", port, value & mask);
        conn.digital_port_write(port, value & mask)?;
        thread::sleep(time::Duration::from_millis(500));
        mask = !mask;
    }
    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        println!("error: {:?}", e);

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
