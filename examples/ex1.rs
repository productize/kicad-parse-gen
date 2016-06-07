// (c) 2015-2016 Productize SPRL <joost@productize.be>

extern crate kicad_parse_gen;

fn main() {
    let module = kicad_parse_gen::read_module("../examples/SOT-23.kicad_mod").unwrap();
    let s = kicad_parse_gen::footprint::module_to_string(&module, 0).unwrap();
    println!("{}", s)
}
