// (c) 2016-2017 Productize SPRL <joost@productize.be>

extern crate kicad_parse_gen as kicad;

use std::path::Path;

fn main() {
    let mut args = std::env::args();
    args.next();
    let name = args.next().unwrap();
    let name = Path::new(&name);
    let f = kicad::read_kicad_file(name, kicad::Expected::Any).unwrap();
    println!("found: {}", f);
}
