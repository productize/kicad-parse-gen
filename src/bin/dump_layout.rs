// (c) 2016 Productize SPRL <joost@productize.be>

extern crate kicad_parse_gen;

fn main() {
    let mut args = std::env::args();
    args.next();
    let name = args.next().unwrap();
    let layout = kicad_parse_gen::read_layout(&name).unwrap();
    let s = kicad_parse_gen::layout::layout_to_string(&layout, 0).unwrap();
    println!("{}", name);
    println!("{}", s)
}
