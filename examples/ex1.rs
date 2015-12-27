// (c) 2015 Joost Yervante Damad <joost@damad.be>

extern crate rustysexp;

fn main() {
    let s = rustysexp::parse_file("examples/SILABS_EFM32_QFN24.kicad_mod");
    println!("{}", s);
}
