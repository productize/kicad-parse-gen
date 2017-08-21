// (c) 2016-2017 Productize SPRL <joost@productize.be>

// extension: .kicad_pcb
// format: new-style

// from parent
use Result;
use symbolic_expressions;
use symbolic_expressions::IntoSexp;
use formatter::KicadFormatter;

pub use layout::data::*;

use {Adjust, Bound, BoundingBox};

/// convert a Kicad layout to a String
pub fn layout_to_string(layout: &Layout, indent_level: i64) -> Result<String> {
    let formatter = KicadFormatter::new(indent_level);
    let mut s =
        symbolic_expressions::ser::to_string_with_formatter(&layout.into_sexp(), formatter)?;
    s.push('\n');
    Ok(s)
}

/// parse a &str to a Kicad layout
pub fn parse(s: &str) -> Result<Layout> {
    let x = match symbolic_expressions::parser::parse_str(s) {
        Ok(s) => symbolic_expressions::from_sexp(&s),
        Err(x) => Err(format!("ParseError: {:?}", x).into()),
    };
    Ok(x?)
}

mod data;
mod de;
mod ser;
