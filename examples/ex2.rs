// (c) 2016 Joost Yervante Damad <joost@productize.be>

extern crate rustykicad;

fn main() {
    let s = rustykicad::schematic::parse_file("../examples/hgminiUSBC.sch");
    println!("{}", s);
}
