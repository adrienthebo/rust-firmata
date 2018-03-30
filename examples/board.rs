extern crate firmata;
extern crate serial;
extern crate env_logger;

use firmata::errors::*;
use firmata::{client, protocol, Board};
use serial::SerialPort;
use std::{thread, time};

fn update_board<T: SerialPort>(conn: &mut T, board: &mut Board) {
    let mut ctr = 0;
    while let Ok(msg) = client::read(conn) {
        board.update(msg);
        ctr += 1;
        if ctr > 5 {
            break;
        }
    }
}

fn run() -> Result<()> {
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

    let mut board = Board::default();

    client::resync(&mut sp).chain_err(|| "Unable to resynchronize Firmata connection")?;
    thread::sleep(time::Duration::from_millis(100));

    let power_en = 49;
    let port = power_en / 8;
    let port_mode = 1 << (power_en % 8);

    let br_sens_b = 68;

    client::set_pin_mode(&mut sp, power_en, protocol::PinMode::DigitalOutput)?;
    thread::sleep(time::Duration::from_millis(20));
    client::digital_port_write(&mut sp, port, port_mode)
        .chain_err(|| "Unable to enable system power")?;

    client::set_pin_mode(&mut sp, br_sens_b, firmata::protocol::PinMode::AnalogInput)?;
    thread::sleep(time::Duration::from_millis(100));
    client::analog_report(&mut sp, 14, true)?;
    thread::sleep(time::Duration::from_millis(100));

    for _ in 0..100 {
        update_board(&mut sp, &mut board);
        let ref pins = board.pins;
        println!("Analog value: {:?}", pins);
    }

    client::digital_port_write(&mut sp, port, 0)
        .chain_err(|| "Unable to disable system power")?;
    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

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
