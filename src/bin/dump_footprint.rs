// (c) 2016 Productize SPRL <joost@productize.be>

extern crate kicad_parse_gen;

fn main() { 
    let mut args = std::env::args();
    args.next();
    let name = args.next().unwrap();
    let module = kicad_parse_gen::read_module(&name).unwrap();
    let s = kicad_parse_gen::footprint::module_to_string(&module, 0).unwrap();
    println!("{}", module.name);
    println!("{}", s)
}

