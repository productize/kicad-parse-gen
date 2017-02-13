// (c) 2016-2017 Productize SPRL <joost@productize.be>

extern crate kicad_parse_gen as kicad;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::path::PathBuf;
use std::env;
use kicad::layout::Adjust;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init().unwrap();

    let mut args = std::env::args();
    args.next();
    let name = args.next().unwrap();
    let name = PathBuf::from(name);
    let mut layout = kicad::read_layout(&name).unwrap();
    layout.adjust(100.0, 0.0);
    let s = kicad::layout::layout_to_string(&layout, 0).unwrap();
    info!("{}", name.display());
    println!("{}", s)
}
