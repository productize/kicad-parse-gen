// (c) 2016 Joost Yervante Damad <joost@productize.be>

use std::env;
use std::path::PathBuf;

extern crate kicad_parse_gen;

fn main() {
    // TODO: find a better way to use files in examples...
    println!("{}", env::current_dir().unwrap().to_str().unwrap());
    let path = PathBuf::from("../examples/breakout-cache.lib");
    let s = kicad_parse_gen::symbol_lib::parse_file(&path).unwrap();
    println!("{}", s);
}
