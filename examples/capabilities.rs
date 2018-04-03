extern crate env_logger;
extern crate firmata;
extern crate serial;

use firmata::FirmataMsg;
use firmata::connection::Connection;
use firmata::errors::*;
use std::{thread, time};

fn print_capabilities(vec: Vec<Vec<firmata::protocol::PinCapability>>) {
    println!("Pin capabilities:");
    for (i, capabilities) in vec.iter().enumerate() {
        if capabilities.len() > 0 {
            println!("\t- pin {}:", i);
            for cap in capabilities {
                println!("\t\t{:?}: resolution {:?}", cap.mode, cap.res);
            }
            println!();
        }
    }
}

fn run() -> firmata::errors::Result<()> {
    env_logger::init();

    let device = "/dev/ttyACM0";
    let mut conn = Connection::open(device)?;
    conn.resync()?;
    conn.capabilities()?;

    for _ in 0..10 {
        match conn.read() {
            Ok(FirmataMsg::CapabilityResponse(vec)) => {
                print_capabilities(vec);
                return Ok(());
            }
            Ok(_) => { },
            Err(e) => return Err(e.into()),
        }
    }
    Err(ErrorKind::UnexpectedResponse.into())
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
