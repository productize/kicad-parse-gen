// (c) 2016-2017 Joost Yervante Damad <joost@productize.be>

extern crate kicad_parse_gen as kicad;

extern crate difference;

use std::path::PathBuf;

#[test]
fn parse_and_compare() {
    let mut file_name = String::new();
    file_name.push_str(env!("CARGO_MANIFEST_DIR"));
    file_name.push_str("/tests/data/");
    file_name.push_str("usbser.kicad_pcb");
    let file_name = PathBuf::from(file_name);
    
    let content = kicad::read_file(&file_name).unwrap();
    
    let layout = kicad::read_layout(&file_name).unwrap();
    let s = kicad::layout::layout_to_string(&layout, 0).unwrap();

    // kicad::write_file("/tmp/dump.kicad_pcb", &s).unwrap();

    // very inefficient...
    let (n, d) = difference::diff(&content, &s, "\n");
    if n > 1 {
        difference::print_diff(&content, &s, "\n");
        println!("{:?}", d);
        assert_eq!(n, 1);
    }
}
