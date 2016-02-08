// (c) 2016 Joost Yervante Damad <joost@productize.be>

use std::fs::File;
use std::io::Read;
use std::io::Write;

extern crate rustc_serialize;

pub type ERes<T> = Result<T, String>;

pub fn err<T>(msg: &str) -> ERes<T> {
    Err(String::from(msg))
}

macro_rules! fail {
    ($expr:expr) => (
        return Err(::std::error::FromError::from_error($expr));
    )
}

macro_rules! println_err(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);

pub fn read_file(name: &str) -> ERes<String> {
    let mut f = try!(match File::open(name) {
        Ok(f) => Ok(f),
        Err(err) => Err(format!("open error in file '{}': {}", name, err))
    });
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(err) => Err(format!("read error in file '{}': {}", name, err))
    }
}

pub fn write_file(name:&str, data:&String) -> ERes<()> {
    let mut f = try!(match File::create(name) {
        Ok(f) => Ok(f),
        Err(err) => Err(format!("create error in file '{}': {}", name, err))
    });
    try!(match write!(&mut f, "{}", data) {
        Ok(f) => Ok(f),
        Err(err) => Err(format!("write error in file '{}': {}", name, err))
    });
         
    Ok(())
    
}


pub mod footprint;
pub mod schematic;
pub mod layout;
pub mod symbol_lib;
