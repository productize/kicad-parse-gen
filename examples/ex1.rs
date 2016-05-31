// (c) 2015-2016 Productize SPRL <joost@productize.be>

extern crate rustykicad;

fn main() {
    let module = rustykicad::read_module("../examples/SOT-23.kicad_mod").unwrap();
    let s = rustykicad::footprint::module_to_string(&module).unwrap();
    println!("{}", s)
}
