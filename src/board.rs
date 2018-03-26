//! The local representation of a Firmata device.


use std::io;


pub struct Pin {
    pub capability: ::protocol::PinCapability,
    pub value: u16
}


pub struct Board<T: io::Read + io::Write> {
    conn: Box<T>,
}


impl<T: io::Read + io::Write> Board<T> {
    pub fn new(conn: Box<T>) -> Board<T> {
        Board { conn: conn }
    }
}
