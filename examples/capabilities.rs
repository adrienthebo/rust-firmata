extern crate firmata;
extern crate serial;
extern crate env_logger;

use serial::SerialPort;
use firmata::client;
use firmata::FirmataMsg;

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

    match client::capabilities(&mut sp) {
        Ok(FirmataMsg::CapabilityResponse(vec)) => {
            print_capabilities(vec);
        },
        Ok(n) => {
            println!("That's odd - firmware query did not return a firmware response! ({:?})", n);
        },
        Err(e) => { panic!("Firmata firmware query failed: {:?}", e) }
    }
}
