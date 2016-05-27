// (c) 2016 Productize SPRL <joost@productize.be>

extern crate rustykicad;

fn main() {
    let s = rustykicad::read_schematic("../examples/hgminiUSBC.sch").unwrap();
    println!("{}", s);
}
