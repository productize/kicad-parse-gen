// (c) 2016 Joost Yervante Damad <joost@productize.be>

extern crate rustykicad;
use std::path::PathBuf;

fn main() {
    let f = PathBuf::from("../examples/hgminiUSBC.sch");
    let s = rustykicad::schematic::parse_file(f).unwrap();
    println!("{}", s);
}
