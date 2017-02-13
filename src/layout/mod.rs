// (c) 2016-2017 Productize SPRL <joost@productize.be>

// extension: .kicad_pcb
// format: new-style

// from parent
use Result;
use symbolic_expressions;
use symbolic_expressions::IntoSexp;
use formatter::KicadFormatter;

pub use layout::data::*;

#[derive(Debug)]
/// A bounding box
pub struct Bound {
    /// smaller x
    pub x1: f64,
    /// smaller y
    pub y1: f64,
    /// bigger x
    pub x2: f64,
    /// bigger y
    pub y2: f64,
}

impl Default for Bound {
    fn default() -> Bound {
        Bound {
            x1: 10000.0,
            y1: 10000.0,
            x2: 0.0,
            y2: 0.0,
        }
    }
}

impl Bound {
    /// create a new bound
    pub fn new(x1: f64, y1: f64, x2: f64, y2: f64) -> Bound {
        Bound {
            x1: x1,
            y1: y1,
            x2: x2,
            y2: y2,
        }
    }

    /// update the bound with another one
    pub fn update(&mut self, other: &Bound) {
        self.x1 = self.x1.min(other.x1);
        self.y1 = self.y1.min(other.y1);
        self.x2 = self.x2.max(other.x2);
        self.y2 = self.y2.max(other.y2);
    }

    /// call this when you constructed a default bound and potentionally had zero updates
    pub fn swap_if_needed(&mut self) {
        if self.x1 > self.x2 {
            let t = self.x1;
            self.x1 = self.x2;
            self.x2 = t;
        }
        if self.y1 > self.y2 {
            let t = self.y1;
            self.y1 = self.y2;
            self.y2 = t;
        }
    }
}

/// calculate the bounding box of a layout item
pub trait BoundingBox {
    /// calculate the bounding box of a layout item
    fn bounding_box(&self) -> Bound;
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
    let x = match symbolic_expressions::parser::parse_str(s) {
        Ok(s) => symbolic_expressions::from_sexp(&s),
        Err(x) => Err(format!("ParseError: {:?}", x).into()),
    };
    Ok(x?)
}

mod data;
mod de;
mod ser;
