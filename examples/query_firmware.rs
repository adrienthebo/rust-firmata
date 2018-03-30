extern crate env_logger;
extern crate firmata;
extern crate serial;

use firmata::FirmataMsg;
use firmata::client;
use firmata::errors::*;
use serial::SerialPort;
use std::{str, thread, time};

fn run() -> firmata::errors::Result<()> {
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
    client::query_firmware(&mut sp).chain_err(|| "Unable to send firmware query command")?;

    match client::read(&mut sp) {
        Ok(FirmataMsg::QueryFirmware {
            major,
            minor,
            firmware_name,
        }) => {
            println!(
                "Firmware query: Firmata v{}.{} '{}'",
                major,
                minor,
                str::from_utf8(&firmware_name).unwrap()
            );
            Ok(())
        }
        Ok(_) => Err(ErrorKind::UnexpectedResponse.into()),
        Err(e) => Err(e.into()),
    }
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
