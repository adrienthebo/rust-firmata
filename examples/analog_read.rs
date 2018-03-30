extern crate env_logger;
extern crate firmata;
extern crate serial;

use firmata::client;
use firmata::errors::*;
use serial::SerialPort;
use std::{thread, time};

fn run() -> Result<()> {
    env_logger::init();

    let device = "/dev/ttyACM0";
    let mut sp = serial::open(device).expect("Unable to open serial device");

    sp.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud57600).unwrap();
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    }).expect("Unable to reconfigure serial device");

    client::resync(&mut sp).chain_err(|| "Unable to resynchronize Firmata connection")?;
    thread::sleep(time::Duration::from_millis(100));

    for pin in 0..16 {
        client::set_pin_mode(&mut sp, pin + 54, firmata::protocol::PinMode::AnalogInput)
            .chain_err(|| format!("Unable to set pin mode on pin {}", pin))?;
        thread::sleep(time::Duration::from_millis(100));
        client::analog_report(&mut sp, pin, true)
            .chain_err(|| format!("Unable to enable analog reporting on pin {}", pin))?;
        thread::sleep(time::Duration::from_millis(100));
    }

    for _ in 0..100 {
        thread::sleep(time::Duration::from_millis(10));
        println!("{:?}", client::read(&mut sp));
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
