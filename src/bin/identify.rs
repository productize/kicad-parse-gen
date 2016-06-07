// (c) 2016 Productize SPRL <joost@productize.be>

extern crate kicad_parse_gen;

fn main() { 
    let mut args = std::env::args();
    args.next();
    let name = args.next().unwrap();
    let f = kicad_parse_gen::read_kicad_file(&name, kicad_parse_gen::Expected::Any).unwrap();
    println!("found: {}", f);
}

