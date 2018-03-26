//! Firmata API client.
//!
//! The client runs on the controlling host and treats the microcontroller
//! as a server.

use std::io;
use errors::*;
use parser;
use protocol::*;

use nom;

pub fn read<T>(conn: &mut T) -> Result<FirmataMsg>
    where T: io::Read + io::Write {
    let mut retries = 0;
    let max_retries = 5;

    let mut buf: Vec<u8> = Vec::new();

    loop {
        // Expand the buffer by one element for the read
        let len = buf.len() + 1;
        buf.resize(len, 0);

        match conn.read(&mut buf[len - 1..]) {
            Ok(_) => {
                match parser::parse(&buf[..]) {
                    Ok((_, msg)) => {
                        info!("Parse complete. Message: {:?}", msg);
                        trace!("Parsed buffer: {:?}", &buf);
                        break Ok(msg)
                    },
                    Err(nom::Err::Incomplete(_)) => {
                        debug!("Parse results incomplete, fetching more data.");
                    },
                    Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
                        println!("Parse error: {:?}", e);
                        break Err(ErrorKind::CommandFailed.into())
                    }

                }
            }
            Err(e) => match e.kind() {
                io::ErrorKind::TimedOut => {
                    if retries < max_retries {
                        retries += 1;
                        warn!("Firmata read timed out, retrying ({} of {})", retries, max_retries);
                    } else {
                        break Err("Command timed out after multiple retries".into())
                    }
                }
                _ => break Err(e).chain_err(|| "I/O error")
            }
        }
    }
}


pub fn reset<T>(conn: &mut T) -> io::Result<()>
    where T: io::Read + io::Write {
    conn.write_all(&[RESET])
}


pub fn query_firmware<T>(conn: &mut T) -> Result<FirmataMsg>
    where T: io::Read + io::Write {
    conn.write_all(&[START_SYSEX, QUERY_FIRMWARE, END_SYSEX])?;
    read(conn)
}


pub fn capabilities<T>(conn: &mut T) -> Result<FirmataMsg>
    where T: io::Read + io::Write {
    conn.write_all(&[START_SYSEX, CAPABILITY_QUERY, END_SYSEX])?;
    read(conn)
}


pub fn set_pin_mode<T>(conn: &mut T, pin: u8, mode: PinMode) -> io::Result<()>
    where T: io::Read + io::Write {
    conn.write_all(&[START_SYSEX, SET_PIN_MODE, pin, mode.into(), END_SYSEX])
}


pub fn analog_report<T>(conn: &mut T, pin: u8, state: bool) -> io::Result<()>
    where T: io::Read + io::Write {

    if pin >= 16 {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "Analog pin index >= 16"))
    } else {
        let mode: u8 = if state { 1 } else { 0 };
        conn.write_all(&[ANALOG_REPORT | pin, mode])
    }
}


pub fn digital_write<T>(conn: &mut T, pin: u8, state: bool) -> io::Result<()>
    where T: io::Read + io::Write {
    if pin >= 128 {
        Err(io::Error::new(io::ErrorKind::InvalidInput, "Digital pin index >= 128"))
    } else {
        let mode: u16 = if state { 1 } else { 0 };

        let port = pin / 8;
        let offset = pin % 8;
        let value: u16 = mode << offset;
        let lsb: u8 = (value & 0x7F) as u8;
        let msb: u8 = ((value & !0x7F) >> 7) as u8;

        conn.write_all(&[DIGITAL_MESSAGE << 4 | port, lsb, msb])
    }
}
