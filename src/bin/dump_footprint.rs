// (c) 2016-2017 Productize SPRL <joost@productize.be>

extern crate kicad_parse_gen;

use std::path::PathBuf;

fn main() {
    let mut args = std::env::args();
    args.next();
    let name = args.next().unwrap();
    let name = PathBuf::from(name);
    let module = kicad_parse_gen::read_module(&name).unwrap();
    let s = kicad_parse_gen::footprint::module_to_string(&module, 0).unwrap();
    println!("{}", module.name);
    println!("{}", s)
}
