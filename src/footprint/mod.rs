// (c) 2015-2017 Productize SPRL <joost@productize.be>

// extension: .kicad_mod
// format: new-style

use std::fmt;
use std::result;

// get from parent
use formatter::KicadFormatter;
use symbolic_expressions;
use symbolic_expressions::IntoSexp;
use KicadError;

// pub use footprint;
pub use footprint::data::*;

/// convert a Kicad Module (footprint) to a String
pub fn module_to_string(module: &Module, indent_level: i64) -> Result<String, KicadError> {
    let formatter = KicadFormatter::new(indent_level);
    symbolic_expressions::ser::to_string_with_formatter(&module.into_sexp(), formatter)
        .map_err(From::from)
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self.side {
            LayerSide::Front => write!(f, "F."),
            LayerSide::Back => write!(f, "B."),
            LayerSide::Dwgs => write!(f, "Dwgs."),
            LayerSide::Cmts => write!(f, "Cmts."),
            LayerSide::Eco1 => write!(f, "Eco1."),
            LayerSide::Eco2 => write!(f, "Eco2."),
            LayerSide::Edge => write!(f, "Edge."),
            LayerSide::In1 => write!(f, "In1."),
            LayerSide::In2 => write!(f, "In2."),
            LayerSide::In3 => write!(f, "In3."),
            LayerSide::In4 => write!(f, "In4."),
            LayerSide::Both => write!(f, "*."),
            LayerSide::None => Ok(()),
        }?;
        match self.t {
            LayerType::Cu => write!(f, "Cu"),
            LayerType::Paste => write!(f, "Paste"),
            LayerType::Mask => write!(f, "Mask"),
            LayerType::SilkS => write!(f, "SilkS"),
            LayerType::User => write!(f, "User"),
            LayerType::Adhes => write!(f, "Adhes"),
            LayerType::Cuts => write!(f, "Cuts"),
            LayerType::CrtYd => write!(f, "CrtYd"),
            LayerType::Fab => write!(f, "Fab"),
            LayerType::Margin => write!(f, "Margin"),
            LayerType::Other(ref x) => write!(f, "{}", x),
        }
    }
}

/// parse a &str to a Kicad Module
pub fn parse(s: &str) -> Result<Module, KicadError> {
    let t = symbolic_expressions::parser::parse_str(s)?;
    let s = symbolic_expressions::from_sexp(&t)?;
    Ok(s)
}

mod data;
mod de;
mod ser;
