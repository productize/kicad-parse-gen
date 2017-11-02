// (c) 2016-2017 Productize SPRL <joost@productize.be>

//! Kicad file format parser and generator library
#![warn(missing_docs)]

#[macro_use]
extern crate log;
extern crate shellexpand;
extern crate symbolic_expressions;
// extern crate strum;
// #[macro_use]
// extern crate strum_macros;

use std::fmt;
use std::path::{Path, PathBuf};
use std::result;

pub use symbolic_expressions::{Sexp,SexpError};
pub use error::*;

/// read file utility wrapper function
pub use util::read_file;
/// write file utility wrapper function
pub use util::write_file;

fn parse_split_quote_aware_int(n_opt: Option<usize>, s: &str) -> Result<Vec<String>, KicadError> {
    let mut i = 0;
    let mut v: Vec<String> = vec![];
    // if we are inside our outside a quoted string
    let mut inside_quotation = false;
    let mut quotation_seen = false;
    let mut s2: String = "".into();
    for c in s.chars() {
        if let Some(n) = n_opt {
            if i == n {
                // if we're in the nth. just keep collecting in current string
                s2.push(c);
                continue;
            }
        }
        // detection of starting "
        if !inside_quotation && c == '\"' {
            inside_quotation = true;
            continue;
        }
        // detection of final "
        if inside_quotation && c == '\"' {
            inside_quotation = false;
            quotation_seen = true;
            continue;
        }
        // detection of space before or in between parts
        if !inside_quotation && c == ' ' {
            if !s2.is_empty() || quotation_seen {
                i += 1;
                v.push(s2.clone());
                s2.clear();
            }
            quotation_seen = false;
            continue;
        }
        s2.push(c);
    }
    if !s2.is_empty() || quotation_seen {
        v.push(s2.clone())
    }
    if let Some(n) = n_opt {
        if v.len() < n {
            return str_error(format!("expecting {} elements in {}", n, s));
        }
    }
    Ok(v)
}

fn parse_split_quote_aware_n(n: usize, s: &str) -> Result<Vec<String>, KicadError> {
    parse_split_quote_aware_int(Some(n), s)
}
fn parse_split_quote_aware(s: &str) -> Result<Vec<String>, KicadError> {
    parse_split_quote_aware_int(None, s)
}

/// types of Kicad files that can be found
#[derive(Debug)]
pub enum KicadFile {
    /// unknown file, probably no kicad file
    Unknown(PathBuf),
    /// a Kicad module, also know as a footprint
    Module(footprint::Module),
    /// a Kicad schematic file
    Schematic(schematic::Schematic),
    /// a Kicad layout file
    Layout(layout::Layout),
    /// a Kicad symbol library file
    SymbolLib(symbol_lib::SymbolLib),
    /// a Kicad project file
    Project(project::Project),
    /// a Kicad fp-lib-table file
    FpLibTable(fp_lib_table::FpLibTable),
}

/// types of Kicad files that we expect to read
#[derive(PartialEq)]
pub enum Expected {
    /// a Kicad module, also know as a footprint
    Module,
    /// a Kicad schematic file
    Schematic,
    /// a Kicad layout file
    Layout,
    /// a Kicad symbol library file
    SymbolLib,
    /// a Kicad project file
    Project,
    /// an fp-lib-table file
    FpLibTable,
    /// any Kicad file
    Any,
}


impl fmt::Display for KicadFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        match *self {
            KicadFile::Unknown(ref x) => write!(f, "unknown: {}", x.to_str().unwrap()),
            KicadFile::Module(_) => write!(f, "module"),
            KicadFile::Schematic(_) => write!(f, "schematic"),
            KicadFile::Layout(_) => write!(f, "layout"),
            KicadFile::SymbolLib(_) => write!(f, "symbollib"),
            KicadFile::Project(_) => write!(f, "project"),
            KicadFile::FpLibTable(_) => write!(f, "fp-lib-table"),
        }
    }
}

/// try to read a file, trying to parse it as the different formats
/// and matching it against the Expected type
#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
pub fn read_kicad_file(name: &Path, expected: Expected) -> Result<KicadFile, KicadError> {
    let data = read_file(name)?;
    match footprint::parse(&data) {
        Ok(module) => return Ok(KicadFile::Module(module)),
        Err(x) => if expected == Expected::Module {
            return Err(x);
        },
    }
    match schematic::parse(Some(PathBuf::from(name)), &data) {
        Ok(sch) => return Ok(KicadFile::Schematic(sch)),
        Err(x) => if expected == Expected::Schematic {
            return Err(x);
        },
    }
    match layout::parse(&data) {
        Ok(layout) => return Ok(KicadFile::Layout(layout)),
        Err(x) => if expected == Expected::Layout {
            return Err(x);
        },
    }
    match symbol_lib::parse_str(&data) {
        Ok(sl) => return Ok(KicadFile::SymbolLib(sl)),
        Err(x) => if expected == Expected::SymbolLib {
            return Err(x);
        },
    }
    match project::parse_str(&data) {
        Ok(p) => return Ok(KicadFile::Project(p)),
        Err(x) => if expected == Expected::Project {
            return Err(x);
        },
    }
    match fp_lib_table::parse(&data) {
        Ok(p) => return Ok(KicadFile::FpLibTable(p)),
        Err(x) => if expected == Expected::FpLibTable {
            return Err(x.into());
        },
    }
    Ok(KicadFile::Unknown(PathBuf::from(name)))
}

/// read a file, expecting it to be a Kicad module file
pub fn read_module(name: &Path) -> Result<footprint::Module, KicadError> {
    match read_kicad_file(name, Expected::Module)? {
        KicadFile::Module(mo) => Ok(mo),
        x => str_error(format!("unexpected {} in {}", x, name.display())),
    }
}

/// read a file, expecting it to be a Kicad schematic
pub fn read_schematic(name: &Path) -> Result<schematic::Schematic, KicadError> {
    match read_kicad_file(name, Expected::Schematic)? {
        KicadFile::Schematic(mo) => Ok(mo),
        x => str_error(format!("unexpected {} in {}", x, name.display())),
    }
}

/// read a file, expecting it to be a Kicad layout file
pub fn read_layout(name: &Path) -> Result<layout::Layout, KicadError> {
    match read_kicad_file(name, Expected::Layout)? {
        KicadFile::Layout(mo) => Ok(mo),
        x => str_error(format!("unexpected {} in {}", x, name.display())),
    }
}

/// write out a kicad Layout to a file
pub fn write_layout(layout: &layout::Layout, name: &Path) -> Result<(), KicadError> {
    let s = layout::layout_to_string(layout, 0)?;
    write_file(name, &s)
}

/// write out a kicad `Module` to a file
pub fn write_module(module: &footprint::Module, name: &Path) -> Result<(), KicadError> {
    let s = footprint::module_to_string(module, 0)?;
    write_file(&name, &s)
}

/// read a file, expecting it to be a Kicad symbol library file
pub fn read_symbol_lib(name: &Path) -> Result<symbol_lib::SymbolLib, KicadError> {
    match read_kicad_file(name, Expected::SymbolLib)? {
        KicadFile::SymbolLib(mo) => Ok(mo),
        x => str_error(format!("unexpected {} in {}", x, name.display())),
    }
}

/// read a file, expecting it to be a Kicad project file
pub fn read_project(name: &Path) -> Result<project::Project, KicadError> {
    match read_kicad_file(name, Expected::Project)? {
        KicadFile::Project(mo) => Ok(mo),
        x => str_error(format!("unexpected {} in {}", x, name.display())),
    }
}

/// read a file, expecting it to be an fp-lib-table
pub fn read_fp_lib_table(name: &Path) -> Result<fp_lib_table::FpLibTable, KicadError> {
    match read_kicad_file(name, Expected::FpLibTable)? {
        KicadFile::FpLibTable(mo) => Ok(mo),
        x => str_error(format!("unexpected {} in {}", x, name.display())),
    }
}

fn wrap<X, Y, F, G>(s: &Sexp, make: F, wrapper: G) -> result::Result<Y,SexpError>
where
    F: Fn(&Sexp) -> result::Result<X,SexpError>,
    G: Fn(X) -> Y,
{
    Ok(wrapper(make(s)?))
}

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
    /// item is bounded
    pub is_bounded: bool,
}

impl Default for Bound {
    fn default() -> Bound {
        Bound {
            x1: 0.0,
            y1: 0.0,
            x2: 0.0,
            y2: 0.0,
            is_bounded: false,
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
            is_bounded: true,
        }
    }
    /// create a new bound
    pub fn new_from_i64(x1: i64, y1: i64, x2: i64, y2: i64) -> Bound {
        Bound {
            x1: x1 as f64,
            y1: y1 as f64,
            x2: x2 as f64,
            y2: y2 as f64,
            is_bounded: true,
        }
    }

    /// update the bound with another one
    pub fn update(&mut self, other: &Bound) {
        if other.is_bounded {
            if !self.is_bounded {
                self.is_bounded = true;
                self.x1 = other.x1;
                self.y1 = other.y1;
                self.x2 = other.x2;
                self.y2 = other.y2;
            } else {
                self.x1 = self.x1.min(other.x1);
                self.y1 = self.y1.min(other.y1);
                self.x2 = self.x2.max(other.x2);
                self.y2 = self.y2.max(other.y2);
            }
        }
    }

    /// call this when you constructed a default bound and potentionally had zero updates
    pub fn swap_if_needed(&mut self) {
        if self.x1 > self.x2 {
            std::mem::swap(&mut self.x1, &mut self.x2);
        }
        if self.y1 > self.y2 {
            std::mem::swap(&mut self.y1, &mut self.y2);
        }
    }

    /// calculate the width of the `Bound`
    pub fn width(&self) -> f64 {
        (self.x1 - self.x2).abs()
    }

    /// calculate the height of the `Bound`
    pub fn height(&self) -> f64 {
        (self.y1 - self.y2).abs()
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

/// ordering used for ordering by component reference
/// to e.g. avoid U1 U101 U2 and get U1 U2 U101
pub fn reference_ord(r: &str) -> (char, i64) {
    let c = r.chars().nth(0).unwrap();
    let mut s = String::new();
    for c in r.chars() {
        if c >= '0' && c <= '9' {
            s.push(c)
        }
        // if the &str contains a list of references,
        // only take 1st element
        if c == ',' {
            break;
        }
    }
    let num = match s.parse::<i64>() {
        Ok(n) => n,
        Err(_) => 0,
    };
    (c, num)
}


/// Kicad error handling code and types
pub mod error;
/// Kicad footprint format handling
pub mod footprint;
/// Kicad schematic format handling
pub mod schematic;
/// Kicad layout format handling
pub mod layout;
/// Kicad symbol library format handling
pub mod symbol_lib;
/// Kicad project format handling
pub mod project;
/// Kicad fp-lib-table format handling
pub mod fp_lib_table;
/// checking and fixing related to the Kicad Library Convention
pub mod checkfix;

mod util;
mod formatter;
