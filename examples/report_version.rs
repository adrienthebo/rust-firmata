extern crate firmata;
extern crate serial;

use serial::*;

fn main() {
    let device = "/dev/ttyACM0";
    let mut sp = serial::open(device).expect(&format!("Unable to open serial device"));

    sp.reconfigure(&|settings| {
        settings.set_baud_rate(Baud57600).unwrap();
        settings.set_char_size(Bits8);
        settings.set_parity(ParityNone);
        settings.set_stop_bits(Stop1);
        settings.set_flow_control(FlowNone);
        Ok(())
    }).expect("Unable to reconfigure serial device");
}
