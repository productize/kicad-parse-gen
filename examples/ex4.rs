// (c) 2016 Joost Yervante Damad <joost@productize.be>

use std::path::PathBuf;

extern crate kicad_parse_gen;

fn main() {
    let mut filename = String::new();
    filename.push_str(env!("CARGO_MANIFEST_DIR"));
    filename.push_str("/examples/breakout-cache.lib");
    let path = PathBuf::from(&filename);
    let s = kicad_parse_gen::symbol_lib::parse_file(&path).unwrap();
    println!("{}", s);
}
