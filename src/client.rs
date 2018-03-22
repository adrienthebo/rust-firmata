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
    let mut retries = 0;
    let max_retries = 10;

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
                        thread::sleep(time::Duration::from_millis(100));
                    } else {
                        break Err("Command timed out after multiple retries".into())
                    }
                }
                _ => break Err(e).chain_err(|| "I/O error")
            }
        }
    }
}

pub fn query_firmware<T>(conn: &mut T) -> Result<parser::FirmataMsg>
    where T: io::Read + io::Write {
    conn.write_all(&[parser::START_SYSEX, parser::QUERY_FIRMWARE, parser::END_SYSEX])?;
    read(conn)
}
