// (c) 2016-2017 Productize SPRL <joost@productize.be>

extern crate kicad_parse_gen as kicad;

extern crate difference;

use difference::Changeset;

use std::path::PathBuf;
use std::fmt::Write;

#[test]
fn parse_and_compare() {
    let mut file_name = String::new();
    file_name.push_str(env!("CARGO_MANIFEST_DIR"));
    file_name.push_str("/tests/data/");
    file_name.push_str("schematic2.sch");
    let file_name = PathBuf::from(file_name);

    let content = kicad::read_file(&file_name).unwrap();

    let s = kicad::read_schematic(&file_name).unwrap();
    let s = format!("{}", s);

    let changeset = Changeset::new(&content, &s, "\n");
    if changeset.distance > 0 {
        println!("{}", changeset);
        assert_eq!(changeset.distance, 0);
    }
}
