extern crate firmata;
extern crate serial;
extern crate env_logger;

use std::str;
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

    let msg = client::query_firmware(&mut sp).expect("Firmata firmware query failed");

    if let FirmataMsg::QueryFirmware { major, minor, firmware_name } = msg {
        println!("Firmware query: Firmata v{}.{} '{}'",
                 major,
                 minor,
                 str::from_utf8(&firmware_name).unwrap());
    } else {
        println!("How the what the - {:?}", msg);
    }
}
