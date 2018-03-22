//! Firmata API client.
//!
//! The client runs on the controlling host and treats the microcontroller
//! as a server.

use std::{io, thread, time};
use errors::*;
use parser;

use nom;

fn read<T>(conn: &mut T) -> Result<parser::FirmataMsg>
    where T: io::Read + io::Write {
    let mut retries = 5;

    loop {
        let mut buf = [0; 10];

        match conn.read(&mut buf) {
            Ok(len) => {
                match parser::parse(&buf[0..len]) {
                    Ok((_, msg)) => { break Ok(msg) },
                    Err(e) => match e {
                        nom::Err::Incomplete(_) => { /* Parsing is incomplete, read additional bytes */ },
                        _ => break Err(ErrorKind::CommandFailed.into())
                    }
                }
            }
            Err(e) => match e.kind() {
                io::ErrorKind::TimedOut => {
                    if retries > 0 {
                        retries -= 1;
                        thread::sleep(time::Duration::from_millis(100));
                    } else {
                        break Err("timeouts are hard and rust is stuff".into())
                    }
                }
                _ => break Err(e).chain_err(|| "I/O error")
            }
        }
    }
}

pub fn query_firmware<T>(conn: &mut T) -> Result<parser::FirmataMsg>
    where T: io::Read + io::Write {

    conn.write(&[parser::START_SYSEX, parser::QUERY_FIRMWARE, parser::END_SYSEX])?;

    read(conn)
}
