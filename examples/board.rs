extern crate env_logger;
extern crate firmata;
extern crate serial;

use firmata::errors::*;
use firmata::protocol::PinMode;
use firmata::connection::Connection;
use std::{thread, time};

fn run() -> Result<()> {
    env_logger::init();

    let device = "/dev/ttyACM0";
    let mut conn = Connection::open(device)?;
    conn.resync()?;

    let power_en = 49;
    let port = power_en / 8;
    let port_mode = 1 << (power_en % 8);

    let br_sens_b = 68;

    conn.set_pin_mode(power_en, PinMode::DigitalOutput)?;
    conn.digital_port_write(port, port_mode)?;
    conn.set_pin_mode(br_sens_b, PinMode::AnalogInput)?;
    conn.analog_report(14, true)?;
    thread::sleep(time::Duration::from_millis(100));

    for _ in 0..100 {
        let mut ctr = 0;
        while let Ok(_) = conn.update() {
            ctr += 1;
            if ctr > 5 {
                break;
            }
        }
        let ref pins = conn.board().unwrap().pins;
        println!("Analog value: {:?}", pins);
    }

    conn.digital_port_write(port, 0)?;
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
