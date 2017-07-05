// (c) 2016-2017 Productize SPRL <joost@productize.be>

extern crate kicad_parse_gen as kicad;

extern crate difference;

use difference::Changeset;

use std::path::PathBuf;

#[test]
fn parse_and_compare() {
    let mut file_name = String::new();
    file_name.push_str(env!("CARGO_MANIFEST_DIR"));
    file_name.push_str("/tests/data/");
    file_name.push_str("fp-lib-table");
    let file_name = PathBuf::from(file_name);

    let content = kicad::read_file(&file_name).unwrap();

    let s = kicad::read_fp_lib_table(&file_name).unwrap();
    let s = kicad::fp_lib_table::to_string(&s, 0).unwrap();

    let changeset = Changeset::new(&content, &s, "\n");
    if changeset.distance > 0 {
        println!("{}", changeset);
        assert_eq!(changeset.distance, 0);
    }
}
