// (c) 2016 Joost Yervante Damad <joost@productize.be>

extern crate kicad_parse_gen;

fn main() {
    let layout = kicad_parse_gen::read_layout("../examples/usbser.kicad_pcb").unwrap();
    let s = kicad_parse_gen::layout::layout_to_string(&layout, 0).unwrap();
    println!("{}", s);
}
