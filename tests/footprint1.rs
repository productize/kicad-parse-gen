// (c) 2015-2017 Productize SPRL <joost@productize.be>

extern crate kicad_parse_gen as kicad;

extern crate difference;

#[test]
fn parse_and_compare() {
    let mut file_name = String::new();
    file_name.push_str(env!("CARGO_MANIFEST_DIR"));
    file_name.push_str("/tests/data/");
    file_name.push_str("SOT-23.kicad_mod");

    let content = kicad::read_file(&file_name).unwrap();

    let module = kicad::read_module(&file_name).unwrap();
    let s = kicad::footprint::module_to_string(&module, 0).unwrap();

    // very inefficient...
    let (n, _) = difference::diff(&s, &content, "\n");
    if n > 1 {
        difference::print_diff(&s, &content, "\n");
        assert_eq!(n, 1);
    }

}
