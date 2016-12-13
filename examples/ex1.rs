// (c) 2015-2016 Productize SPRL <joost@productize.be>

extern crate kicad_parse_gen;

fn main() {
    let mut filename = String::new();
    filename.push_str(env!("CARGO_MANIFEST_DIR"));
    filename.push_str("/examples/kicad_mod/SOT-23.kicad_mod");
    let module = kicad_parse_gen::read_module(&filename).unwrap();
    let s = kicad_parse_gen::footprint::module_to_string(&module, 0).unwrap();
    println!("{}", s)
}
