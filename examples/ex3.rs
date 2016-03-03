// (c) 2016 Joost Yervante Damad <joost@productize.be>

extern crate rustykicad;

fn main() {
    let s = rustykicad::read_layout("../examples/usbser.kicad_pcb").unwrap();
    println!("{}", s);
}
