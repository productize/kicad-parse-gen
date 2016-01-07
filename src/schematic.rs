// (c) 2016 Joost Yervante Damad <joost@productize.be>

use std;
use std::fmt;
use std::fs::File;
use std::io::Read;

// get from parent
use ERes;
use err;
use read_file;

pub struct Schematic {
    libraries:Vec<String>
}

impl Schematic {
    fn new() -> Schematic {
        Schematic { libraries:vec![] }
    }
}


impl fmt::Display for Schematic {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "EESchema Schematic File Version 2")
    }
}


fn parse(s: &str) -> ERes<Schematic> {
    let sch = Schematic::new();
    Ok(sch)
}


pub fn parse_str(s: &str) -> Schematic {
    parse(s).unwrap()
}

pub fn parse_file(name: &str) -> Schematic {
    let s = read_file(name).unwrap();
    parse(&s[..]).unwrap()
}

