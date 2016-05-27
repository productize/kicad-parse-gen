// (c) 2016 Productize SPRL <joost@productize.be>

// extension: .kicad_pcb
// format: new-style

use std::fmt;
use std::result;

// from parent
use Result;
use str_error as err;
use footprint;
use footprint::FromSexp;
use footprint::wrap;
use Sexp;
use symbolic_expressions;
use str_error;

pub use layout::data::Layout;

pub fn parse(s: &str) -> Result<Layout> {
    match symbolic_expressions::parser::parse_str(s) {
        Ok(s) => Result::from_sexp(&s),
        Err(x) => str_error(format!("ParseError: {:?}", x)),
    }
}

mod data;
mod de;
