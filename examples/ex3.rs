// (c) 2016-2017 Joost Yervante Damad <joost@productize.be>

extern crate kicad_parse_gen;

fn main() {
    let mut file_name = String::new();
    file_name.push_str(env!("CARGO_MANIFEST_DIR"));
    file_name.push_str("/examples/");
    file_name.push_str("usbser.kicad_pcb");
    
    let layout = kicad_parse_gen::read_layout(&file_name).unwrap();
    let s = kicad_parse_gen::layout::layout_to_string(&layout, 0).unwrap();
    println!("{}", s);
}
