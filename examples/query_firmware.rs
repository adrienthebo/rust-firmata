extern crate env_logger;
extern crate firmata;
extern crate serial;

use firmata::FirmataMsg;
use firmata::connection::Connection;
use firmata::errors::*;
use std::{str, thread, time};

fn run() -> firmata::errors::Result<()> {
    env_logger::init();

    let device = "/dev/ttyACM0";
    let mut conn = Connection::open(device)?;
    conn.resync()?;
    conn.query_firmware()?;

    match conn.read() {
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
