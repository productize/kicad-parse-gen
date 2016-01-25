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

fn read_file(name: &str) -> ERes<String> {
    let mut f = try!(match File::open(name) {
        Ok(f) => Ok(f),
        Err(err) => Err(format!("open error in file {}: {}", name, err))
    });
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(err) => Err(format!("read error in file {}: {}", name, err))
    }
}

pub mod footprint;
pub mod schematic;
