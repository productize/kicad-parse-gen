// (c) 2015-2016 Joost Yervante Damad <joost@productize.be>

extern crate rustykicad;

fn main() {
    let s = rustykicad::footprint::parse_file("../examples/SILABS_EFM32_QFN24.kicad_mod");
    println!("{}", s);
}
