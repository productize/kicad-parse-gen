// (c) 2016 Joost Yervante Damad <joost@productize.be>

extern crate rustykicad;

fn main() {
    let layout = rustykicad::read_layout("../examples/usbser.kicad_pcb").unwrap();
    let s = rustykicad::layout::layout_to_string(&layout, 0).unwrap();
    println!("{}", s);
}
