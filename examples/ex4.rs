// (c) 2016-2017 Joost Yervante Damad <joost@productize.be>

use std::path::PathBuf;

extern crate kicad_parse_gen;

fn main() {
    let mut file_name = String::new();
    file_name.push_str(env!("CARGO_MANIFEST_DIR"));
    file_name.push_str("/examples/");
    file_name.push_str("breakout-cache.lib");
    
    let path = PathBuf::from(&file_name);
    let s = kicad_parse_gen::symbol_lib::parse_file(&path).unwrap();
    println!("{}", s);
}
