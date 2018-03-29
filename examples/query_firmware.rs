extern crate env_logger;
extern crate firmata;
extern crate serial;

use firmata::FirmataMsg;
use firmata::client;
use serial::SerialPort;
use std::str;

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

    match client::query_firmware(&mut sp) {
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
        }
        Ok(n) => {
            println!(
                "That's odd - firmware query did not return a firmware response! ({:?})",
                n
            );
        }
        Err(e) => panic!("Firmata firmware query failed: {:?}", e),
    }
}
