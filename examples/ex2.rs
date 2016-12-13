// (c) 2016 Productize SPRL <joost@productize.be>

extern crate kicad_parse_gen;

fn main() {
    let mut filename = String::new();
    filename.push_str(env!("CARGO_MANIFEST_DIR"));
    filename.push_str("/examples/hgminiUSBC.sch");
    let s = kicad_parse_gen::read_schematic(&filename).unwrap();
    println!("{}", s);
}
