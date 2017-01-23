// (c) 2016-2017 Productize SPRL <joost@productize.be>

extern crate kicad_parse_gen;

fn main() {
    let mut file_name = String::new();
    file_name.push_str(env!("CARGO_MANIFEST_DIR"));
    file_name.push_str("/examples/");
    file_name.push_str("hgminiUSBC.sch");
    
    let s = kicad_parse_gen::read_schematic(&file_name).unwrap();
    println!("{}", s);
}
