use ::board::Board;
use ::errors::*;
use std::io;

pub trait RW: io::Read + io::Write + Send {}

impl<T> RW for T
where
    T: io::Read + io::Write + Send,
{
}
