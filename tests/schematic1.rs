// (c) 2016-2017 Productize SPRL <joost@productize.be>

extern crate kicad_parse_gen as kicad;

extern crate difference;

use std::path::PathBuf;

#[test]
fn parse_and_compare() {
    let mut file_name = String::new();
    file_name.push_str(env!("CARGO_MANIFEST_DIR"));
    file_name.push_str("/tests/data/");
    file_name.push_str("hgminiUSBC.sch");
    let file_name = PathBuf::from(file_name);
    
    let content = kicad::read_file(&file_name).unwrap();
    
    let s = kicad::read_schematic(&file_name).unwrap();
    let s = format!("{}", s);
    
    // very inefficient...
    let (n, _) = difference::diff(&content, &s, "\n");
    if n > 1 {
        difference::print_diff(&content, &s, "\n");
        assert_eq!(n, 1);
    }
}
