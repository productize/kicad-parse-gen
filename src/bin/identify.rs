// (c) 2016 Productize SPRL <joost@productize.be>

extern crate rustykicad;

fn main() { 
    let mut args = std::env::args();
    args.next();
    let name = args.next().unwrap();
    let f = rustykicad::read_kicad_file(&name, rustykicad::Expected::Any).unwrap();
    println!("found: {}", f);
}

