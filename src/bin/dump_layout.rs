// (c) 2016-2017 Productize SPRL <joost@productize.be>

extern crate kicad_parse_gen as kicad;

use std::path::PathBuf;

fn main() {
    let mut args = std::env::args();
    args.next();
    let name = args.next().unwrap();
    let name = PathBuf::from(name);
    let layout = kicad::read_layout(&name).unwrap();
    let s = kicad::layout::layout_to_string(&layout, 0).unwrap();
    println!("{}", name.display());
    println!("{}", s)
}
