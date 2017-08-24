// (c) 2017 Productize SPRL <joost@productize.be>

extern crate kicad_parse_gen as kicad;
#[macro_use]
extern crate log;
extern crate env_logger;

use std::path::PathBuf;
use std::env;

use kicad::klc::KLCCheck;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init().unwrap();

    let mut args = std::env::args();
    args.next();
    let name = args.next().unwrap();
    let name = PathBuf::from(name);
    let module = kicad::read_module(&name).unwrap();
    let config = kicad::klc::Config::klc();
    let checkres = module.check(&config);
    if !checkres.is_empty() {
        info!("Module {}", module.name);
        for v in checkres {
            v.dump_on_logger(1);
        }
    }
}
