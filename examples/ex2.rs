// (c) 2016 Productize SPRL <joost@productize.be>

extern crate kicad_parse_gen;

fn main() {
    let s = kicad_parse_gen::read_schematic("../examples/hgminiUSBC.sch").unwrap();
    println!("{}", s);
}
