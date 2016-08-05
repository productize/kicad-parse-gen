// (c) 2016 Productize SPRL <joost@productize.be>

// extension: .kicad_pcb
// format: new-style

// from parent
use Result;
use symbolic_expressions;
use symbolic_expressions::IntoSexp;
use str_error;
use formatter::KicadFormatter;
use FromSexp;

pub use layout::data::*;

/// convert a Kicad layout to a String
pub fn layout_to_string(layout: &Layout, indent_level: i64) -> Result<String> {
    let formatter = KicadFormatter::new(indent_level);
    symbolic_expressions::ser::to_string_with_formatter(&layout.into_sexp(), formatter)
        .map_err(From::from)
}

/// parse a &str to a Kicad layout
pub fn parse(s: &str) -> Result<Layout> {
    match symbolic_expressions::parser::parse_str(s) {
        Ok(s) => Result::from_sexp(&s),
        Err(x) => str_error(format!("ParseError: {:?}", x)),
    }
}

mod data;
mod de;
mod ser;
