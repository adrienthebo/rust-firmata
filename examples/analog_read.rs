extern crate firmata;
extern crate serial;
extern crate env_logger;

use std::{thread, time};
use serial::SerialPort;
use firmata::client;

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

    for pin in 0 .. 16 {
        client::set_pin_mode(&mut sp, pin + 54, firmata::protocol::PinMode::AnalogInput);
        thread::sleep(time::Duration::from_millis(100));
        client::analog_report(&mut sp, pin, true);
        thread::sleep(time::Duration::from_millis(100));
    }


    for _ in 0 .. 100 {
        thread::sleep(time::Duration::from_millis(10));
        println!("{:?}", client::read(&mut sp));
    }
}
