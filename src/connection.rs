use ::board::Board;
use ::errors::*;
use std::io;

use serial_unix;
use serial_core as serial;
use serial_core::prelude::*;

const SERIAL_SETTINGS: serial::PortSettings = serial::PortSettings {
    baud_rate: serial::Baud57600,
    char_size: serial::Bits8,
    parity: serial::ParityNone,
    stop_bits: serial::Stop1,
    flow_control: serial::FlowNone,
};

pub trait RW: io::Read + io::Write + Send {}

impl<T> RW for T
where
    T: io::Read + io::Write + Send,
{
}

pub enum Connection<T>
where
    T: RW
{
    Open { inner: T, board: Board },
    Closed
}

impl<T> Connection<T>
where
    T: RW
{
    pub fn new(inner: T) -> Result<Self>
    where T: RW
    {
        Ok(Connection::Open { inner, board: Board::default() })
    }

    pub fn board(&self) -> Option<&Board> {
        match *self {
            Connection::Open { ref board, .. } => Some(board),
            Connection::Closed => None
        }
    }

    pub fn conn(&self) -> Option<&RW> {
        match *self {
            Connection::Open { ref inner, .. } => Some(inner),
            Connection::Closed => None
        }
    }

    pub fn conn_mut(&mut self) -> Option<&mut RW> {
        match *self {
            Connection::Open { ref mut inner, .. } => Some(inner),
            Connection::Closed => None
        }
    }

    pub fn resync(&mut self) -> Result<()> {
        match *self {
            Connection::Open { ref mut inner, .. } => {
                ::client::resync(inner)
                    .map_err(|e| e.into())
            },
            Connection::Closed => Err(ErrorKind::ConnectionClosed.into())
        }
    }

    pub fn set_pin_mode(&mut self, port: u8, mode: ::protocol::PinMode) -> Result<()> {
        match *self {
            Connection::Open { ref mut inner, .. } => {
                ::client::set_pin_mode(inner, port, mode)
                    .map_err(|e| e.into())
            },
            Connection::Closed => Err(ErrorKind::ConnectionClosed.into())
        }
    }

    pub fn digital_port_write(&mut self, port: u8, value: u8) -> Result<()> {
        match *self {
            Connection::Open { ref mut inner, .. } => {
                ::client::digital_port_write(inner, port, value)
                    .map_err(|e| e.into())
            },
            Connection::Closed => Err(ErrorKind::ConnectionClosed.into())
        }
    }

    pub fn analog_report(&mut self, pin: u8, state: bool) -> Result<()> {
        match *self {
            Connection::Open { ref mut inner, .. } => {
                ::client::analog_report(inner, pin, state)
                    .map_err(|e| e.into())
            },
            Connection::Closed => Err(ErrorKind::ConnectionClosed.into())
        }
    }

    pub fn query_firmware(&mut self) -> Result<()> {
        match *self {
            Connection::Open { ref mut inner, .. } => {
                ::client::query_firmware(inner)
                    .map_err(|e| e.into())
            },
            Connection::Closed => Err(ErrorKind::ConnectionClosed.into())
        }
    }

    pub fn capabilities(&mut self) -> Result<()> {
        match *self {
            Connection::Open { ref mut inner, .. } => {
                ::client::capabilities(inner)
                    .map_err(|e| e.into())
            },
            Connection::Closed => Err(ErrorKind::ConnectionClosed.into())
        }
    }

    pub fn read(&mut self) -> Result<::protocol::FirmataMsg> {
        match *self {
            Connection::Open { ref mut inner, .. } => {
                ::client::read(inner)
            },
            Connection::Closed => Err(ErrorKind::ConnectionClosed.into())
        }
    }
}

impl Connection<serial_unix::TTYPort>
{
    pub fn open(path: &str) -> Result<Self> {
        use ::std::path::Path;
        serial_unix::TTYPort::open(Path::new(path))
            .and_then(|mut inner| {
                inner.configure(&SERIAL_SETTINGS)?;
                Ok(Connection::Open { inner, board: Board::default() })
            })
            .map_err(|err| err.into())
    }
}
