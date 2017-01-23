// (c) 2015-2017 Productize SPRL <joost@productize.be>

extern crate kicad_parse_gen;

fn main() {
    let mut file_name = String::new();
    file_name.push_str(env!("CARGO_MANIFEST_DIR"));
    file_name.push_str("/examples/");
    file_name.push_str("SOT-23.kicad_mod");

    let module = kicad_parse_gen::read_module(&file_name).unwrap();
    let s = kicad_parse_gen::footprint::module_to_string(&module, 0).unwrap();
    println!("{}", s)
}
