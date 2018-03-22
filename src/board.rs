use std::io;

pub struct Board<T: io::Read + io::Write> {
    conn: Box<T>,
}

impl<T: io::Read + io::Write> Board<T> {
    pub fn new(conn: Box<T>) -> io::Result<Board<T>> {
        Ok(Board { conn: conn })
    }
}
