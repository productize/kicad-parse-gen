// (c) 2016-2017 Joost Yervante Damad <joost@productize.be>

extern crate kicad_parse_gen as kicad;

extern crate difference;

#[test]
fn parse_and_compare() {
    let mut file_name = String::new();
    file_name.push_str(env!("CARGO_MANIFEST_DIR"));
    file_name.push_str("/tests/data/");
    file_name.push_str("breakout-cache.lib");
    
    let content = kicad::read_file(&file_name).unwrap();
    
    let s = kicad::read_symbol_lib(&file_name).unwrap();
    let s = format!("{}", s);

    // very inefficient...
    let (n, _) = difference::diff(&content, &s, "\n");
    if n > 1 {
        difference::print_diff(&content, &s, "\n");
        assert_eq!(n, 1);
    }
}
