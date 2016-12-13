// (c) 2016 Joost Yervante Damad <joost@productize.be>

extern crate kicad_parse_gen;

fn main() {
    let mut filename = String::new();
    filename.push_str(env!("CARGO_MANIFEST_DIR"));
    filename.push_str("/examples/usbser.kicad_pcb");
    let layout = kicad_parse_gen::read_layout(&filename).unwrap();
    let s = kicad_parse_gen::layout::layout_to_string(&layout, 0).unwrap();
    println!("{}", s);
}
