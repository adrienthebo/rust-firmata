extern crate env_logger;
extern crate firmata;
extern crate serial;

use firmata::FirmataMsg;
use firmata::client;
use serial::SerialPort;
use std::{str, thread, time};

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
    client::reset(&mut sp).expect("Unable to send Firmata reset command");
    thread::sleep(time::Duration::new(5, 0));

    client::read(&mut sp);

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

    // Determine the port and port value associated with pin 49.
    let pin = 49;
    let port = pin / 8;
    let value = 1 << (pin % 8);
    let mut mask = 0xFF;

    client::set_pin_mode(&mut sp, pin, firmata::protocol::PinMode::DigitalOutput)
        .expect("Unable to send pin mode change command");
    thread::sleep(time::Duration::from_millis(100));

    for _ in 0..2 {
        println!("Port {}: {:08b}", port, value & mask);
        client::digital_port_write(&mut sp, port, value & mask)
            .expect("Unable to send digital write command");
        thread::sleep(time::Duration::from_millis(500));
        mask = !mask;
    }
}
