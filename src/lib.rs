// (c) 2016 Joost Yervante Damad <joost@productize.be>

use std::fs::File;
use std::io::Read;

pub type ERes<T> = Result<T, String>;

pub fn err<T>(msg: &str) -> ERes<T> {
    Err(String::from(msg))
}

macro_rules! fail {
    ($expr:expr) => (
        return Err(::std::error::FromError::from_error($expr));
    )
}

fn read_file(name: &str) -> Result<String,std::io::Error> {
    let mut f = try!(File::open(name));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}

pub mod footprint;
pub mod schematic;
