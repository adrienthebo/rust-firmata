use std::io;

pub trait RW: io::Read + io::Write {}
impl<T> RW for T where T: io::Read + io::Write {}
