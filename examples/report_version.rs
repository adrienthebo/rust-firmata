extern crate firmata;
extern crate serial;

use serial::SerialPort;

fn main() {
    let device = "/dev/ttyACM0";
    let mut sp = serial::open(device).expect(&format!("Unable to open serial device"));

    sp.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud57600).unwrap();
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    }).expect("Unable to reconfigure serial device");
}
