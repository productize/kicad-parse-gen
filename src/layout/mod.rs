// (c) 2016-2017 Productize SPRL <joost@productize.be>

// extension: .kicad_pcb
// format: new-style

// from parent
use Result;
use symbolic_expressions;
use symbolic_expressions::IntoSexp;
use str_error;
use formatter::KicadFormatter;
use from_sexp;

pub use layout::data::*;

/// calculate the bounding box of a layout item
pub trait BoundingBox {
    /// calculate the bounding box of a layout item
    fn bounding_box(&self) -> (f64, f64, f64, f64);
}

/// item location can be adjusted
pub trait Adjust {
    /// adjust the location of the item
    fn adjust(&mut self, x: f64, y: f64);
}

/// convert a Kicad layout to a String
pub fn layout_to_string(layout: &Layout, indent_level: i64) -> Result<String> {
    let formatter = KicadFormatter::new(indent_level);
    let mut s = symbolic_expressions::ser::to_string_with_formatter(&layout.into_sexp(), formatter)
        ?;
    s.push('\n');
    Ok(s)
}

/// parse a &str to a Kicad layout
pub fn parse(s: &str) -> Result<Layout> {
    match symbolic_expressions::parser::parse_str(s) {
        Ok(s) => from_sexp(&s),
        Err(x) => str_error(format!("ParseError: {:?}", x)),
    }
}

mod data;
mod de;
mod ser;
