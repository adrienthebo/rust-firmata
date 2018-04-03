//! Firmata API client.
//!
//! The client runs on the controlling host and treats the microcontroller
//! as a server.

use errors::*;
use parser;
use protocol::*;
use std::io;

use nom;

pub fn read<T>(conn: &mut T) -> Result<FirmataMsg>
where
    T: ::connection::RW,
{
    read_rt(conn, 3)
}

pub fn read_rt<T>(conn: &mut T, max_retries: usize) -> Result<FirmataMsg>
where
    T: ::connection::RW,
{
    let mut retries = 0;
    let mut buf: Vec<u8> = Vec::new();

    loop {
        // Expand the buffer by one element for the read
        let len = buf.len() + 1;
        buf.resize(len, 0);

        match conn.read(&mut buf[len - 1..]) {
            Ok(_) => match parser::parse(&buf[..]) {
                Ok((_, msg)) => {
                    debug!("Parse complete. Message: {:?}", msg);
                    trace!("Parsed buffer: {:?}", &buf);
                    break Ok(msg);
                }
                Err(nom::Err::Incomplete(_)) => {
                    debug!("Parse results incomplete, fetching more data.");
                }
                Err(nom::Err::Error(e)) | Err(nom::Err::Failure(e)) => {
                    break Err(ErrorKind::UnreadableMsg.into());
                }
            },
            Err(e) => match e.kind() {
                io::ErrorKind::TimedOut if retries < max_retries => {
                    retries += 1;
                    debug!(
                        "Firmata read timed out, retrying ({} of {})",
                        retries, max_retries
                    );
                }
                _ => break Err(Error::with_chain(e, "Firmata stream read failed")),
            },
        }
    }
}

pub fn reset<T>(conn: &mut T) -> io::Result<()>
where
    T: ::connection::RW,
{
    conn.write_all(&[RESET])
}

pub fn query_firmware<T>(conn: &mut T) -> io::Result<()>
where
    T: ::connection::RW,
{
    conn.write_all(&[START_SYSEX, QUERY_FIRMWARE, END_SYSEX])
}

pub fn capabilities<T>(conn: &mut T) -> io::Result<()>
where
    T: ::connection::RW,
{
    conn.write_all(&[START_SYSEX, CAPABILITY_QUERY, END_SYSEX])
}

pub fn set_pin_mode<T>(conn: &mut T, pin: u8, mode: PinMode) -> io::Result<()>
where
    T: ::connection::RW,
{
    conn.write_all(&[START_SYSEX, SET_PIN_MODE, pin, mode.into(), END_SYSEX])
}

pub fn analog_report<T>(conn: &mut T, pin: u8, state: bool) -> io::Result<()>
where
    T: ::connection::RW,
{
    if pin >= 16 {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Analog pin index >= 16",
        ))
    } else {
        let mode: u8 = if state { 1 } else { 0 };
        conn.write_all(&[ANALOG_REPORT | pin, mode])
    }
}

/// Write a value to a port register of the Firmata board.
pub fn digital_port_write<T>(conn: &mut T, port: u8, value: u8) -> io::Result<()>
where
    T: ::connection::RW,
{
    if port >= 16 {
        Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "port index >= 16",
        ))
    } else {
        let lsb: u8 = (value & 0x7F) as u8;
        let msb: u8 = ((value & !0x7F) >> 7) as u8;

        conn.write_all(&[DIGITAL_MESSAGE << 4 | port, lsb, msb])
    }
}

/// Resynchronize the serial connection to the Firmata device.
///
/// A firmata device can be in an arbitrary state when we initially connect,
/// and we may have a buffer of stale information that needs to be cleaned.
/// The easiest way to resolve this is to send several reset messages and drain
/// the serial buffer until we receive the ProtocolVersion message, which means
/// the device is in a known good state.
pub fn resync<T>(conn: &mut T) -> io::Result<()>
where
    T: ::connection::RW,
{
    debug!("Issuing Firmata reset");
    reset(conn)?;

    let max_retries = 5;
    for attempt in 0..max_retries {
        debug!(
            "Attempting to resync Firmata connection ({} of {})",
            attempt + 1,
            max_retries
        );
        query_firmware(conn)?;

        for _ in 0.. 30 {
            match read(conn) {
                Ok(FirmataMsg::ProtocolVersion { .. }) |
                Ok(FirmataMsg::QueryFirmware { .. }) => return Ok(()),
                Ok(m) => {
                    trace!("Discarding message {:?}", m);
                }
                Err(e) => { trace!("Serial read returned error {:?}", e); }
            }
        }
    }
    Err(io::Error::new(
        io::ErrorKind::NotConnected,
        "Could not resynchronize Firmata connection",
    ))
}
