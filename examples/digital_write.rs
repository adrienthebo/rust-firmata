extern crate firmata;
extern crate serial;
extern crate env_logger;

use std::{str, thread, time};
use serial::SerialPort;
use firmata::client;
use firmata::FirmataMsg;

fn main() {
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


    println!("Resetting device and delaying for 5 seconds.");
    client::reset(&mut sp);
    thread::sleep(time::Duration::new(5, 0));

    client::read(&mut sp);

    match client::query_firmware(&mut sp) {
        Ok(FirmataMsg::QueryFirmware { major, minor, firmware_name }) => {
            println!("Firmware query: Firmata v{}.{} '{}'",
                     major,
                     minor,
                     str::from_utf8(&firmware_name).unwrap());
        },
        Ok(n) => {
            println!("That's odd - firmware query did not return a firmware response! ({:?})", n);
        },
        Err(e) => { panic!("Firmata firmware query failed: {:?}", e) }
    }

    let pin = 49;
    let mut state = true;

    client::set_pin_mode(&mut sp, pin, firmata::protocol::PinMode::DigitalOutput);
    thread::sleep(time::Duration::from_millis(100));

    for _ in 0..6  {
        println!("{}: {}", pin, state);
        client::digital_write(&mut sp, pin, state);
        thread::sleep(time::Duration::from_millis(500));
        state = !state;
    }
}
