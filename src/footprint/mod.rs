// (c) 2015-2016 Joost Yervante Damad <joost@productize.be>

// extension: .kicad_mod
// format: new-style

use std::fmt;
use std::result;

// get from parent
use Result;
use str_error;
use symbolic_expressions;
use symbolic_expressions::IntoSexp;
use formatter::KicadFormatter;
use FromSexp;

//pub use footprint;
pub use footprint::data::*;
pub use footprint::de::wrap;

/// convert a Kicad Module (footprint) to a String
pub fn module_to_string(module:&Module, indent_level:i64) -> Result<String> {
    let formatter = KicadFormatter::new(indent_level);
    symbolic_expressions::ser::to_string_with_formatter(&module.into_sexp(), formatter).map_err(From::from)
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        try!(match self.side {
            LayerSide::Front => write!(f, "F."),
            LayerSide::Back  => write!(f, "B."),
            LayerSide::Dwgs  => write!(f, "Dwgs."),
            LayerSide::Cmts  => write!(f, "Cmts."),
            LayerSide::Eco1  => write!(f, "Eco1."),
            LayerSide::Eco2  => write!(f, "Eco2."),
            LayerSide::Edge  => write!(f, "Edge."),
            LayerSide::In1   => write!(f, "In1."),
            LayerSide::In2   => write!(f, "In2."),
            LayerSide::Both  => write!(f, "*."),
            LayerSide::None  => Ok(()),
        });
        match self.t {
            LayerType::Cu    => write!(f,"Cu"),
            LayerType::Paste => write!(f,"Paste"),
            LayerType::Mask  => write!(f,"Mask"),
            LayerType::SilkS => write!(f,"SilkS"),
            LayerType::User  => write!(f,"User"),
            LayerType::Adhes => write!(f,"Adhes"),
            LayerType::Cuts  => write!(f,"Cuts"),
            LayerType::CrtYd => write!(f,"CrtYd"),
            LayerType::Fab   => write!(f,"Fab"),
            LayerType::Margin => write!(f,"Margin"),
            LayerType::Other(ref x) => write!(f, "{}", x),
        }
    }
}

/// parse a &str to a Kicad Module
pub fn parse(s: &str) -> Result<Module> {
    match symbolic_expressions::parser::parse_str(s) {
        Ok(s) => Result::from_sexp(&s),
        Err(x) => str_error(format!("ParseError: {:?}", x))
    }
}

mod data;
mod ser;
mod de;
