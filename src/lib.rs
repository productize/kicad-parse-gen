// (c) 2016 Productize SPRL <joost@productize.be>

//! Kicad file format parser and generator library

#![warn(missing_docs)]

extern crate symbolic_expressions;

use std::fmt;

pub use symbolic_expressions::Sexp;
pub use error::*;

/// convert from a symbolic-expression to something
pub trait FromSexp where Self:Sized {
    /// convert from a symbolic-expression to something
    fn from_sexp(&Sexp) -> Result<Self>;
}

/// convert from a symbolic-expression to something (dispatcher)
pub fn from_sexp<T:FromSexp>(s:&Sexp) -> Result<T> {
    T::from_sexp(s)
}

/// read file utility wrapper function
pub use util::read_file;
/// write file utility wrapper function
pub use util::write_file;

fn parse_split_quote_aware(s: &str) -> Vec<String> {
    let mut v: Vec<String> = vec![];
    let mut in_q: bool = false;
    let mut q_seen: bool = false;
    let mut s2: String = String::from("");
    for c in s.chars() {
        if !in_q && c == '\"' {
            in_q = true;
            // s2.push(c);
            continue;
        }
        if in_q && c == '\"' {
            in_q = false;
            // s2.push(c);
            q_seen = true;
            continue;
        }
        if !in_q && c == ' ' {
            if !s2.is_empty() || q_seen {
                v.push(s2.clone());
                s2.clear();
            }
            q_seen = false;
            continue;
        }
        s2.push(c);
    }
    if !s2.is_empty() || q_seen {
        v.push(s2.clone())
    }
    v
}

fn parse_split_quote_aware_n(n: usize, s: &str) -> Result<Vec<String>> {
    let mut i = 0;
    let mut v: Vec<String> = vec![];
    let mut in_q: bool = false;
    let mut q_seen: bool = false;
    let mut s2: String = String::from("");
    for c in s.chars() {
        if i == n {
            // if we're in the nth. just keep collecting in current string
            s2.push(c);
            continue;
        }
        if !in_q && c == '\"' {
            in_q = true;
            // s2.push(c);
            continue;
        }
        if in_q && c == '\"' {
            in_q = false;
            // s2.push(c);
            q_seen = true;
            continue;
        }
        if !in_q && c == ' ' {
            if !s2.is_empty() || q_seen {
                i += 1;
                v.push(s2.clone());
                s2.clear();
            }
            q_seen = false;
            continue;
        }
        s2.push(c);
    }
    if !s2.is_empty() || q_seen {
        v.push(s2.clone())
    }
    if v.len() < n {
        return str_error(format!("expecting {} elements in {}", n, s));
    }
    Ok(v)
}

/// types of Kicad files that can be found
#[derive(Debug)]
pub enum KicadFile {
    /// unknown file, probably no kicad file
    Unknown(String),
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
    /// any Kicad file
    Any,
}


impl fmt::Display for KicadFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        match *self {
            KicadFile::Unknown(ref x) => write!(f, "unknown: {}", x),
            KicadFile::Module(_) => write!(f, "module"),
            KicadFile::Schematic(_) => write!(f, "schematic"),
            KicadFile::Layout(_) => write!(f, "layout"),
            KicadFile::SymbolLib(_) => write!(f, "symbollib"),
            KicadFile::Project(_) => write!(f, "project"),
        }
    }
}

/// try to read a file, trying to parse it as the different formats
/// and matching it against the Expected type
pub fn read_kicad_file(name: &str, expected: Expected) -> Result<KicadFile> {
    let pb = std::path::PathBuf::from(name);
    let data = read_file(name)?;
    let mut msg = String::new();
    match footprint::parse(&data) {
        Ok(module) => return Ok(KicadFile::Module(module)),
        Err(x) => {
            if expected == Expected::Module {
                msg = format!("{}", x)
            }
        }
    }
    match schematic::parse(Some(pb), &data) {
        Ok(sch) => return Ok(KicadFile::Schematic(sch)),
        Err(x) => {
            if expected == Expected::Schematic {
                msg = format!("{}", x)
            }
        }
    }
    match layout::parse(&data) {
        Ok(layout) => return Ok(KicadFile::Layout(layout)),
        Err(x) => {
            if expected == Expected::Layout {
                msg = format!("{}", x)
            }
        }
    }
    match symbol_lib::parse_str(&data) {
        Ok(sl) => return Ok(KicadFile::SymbolLib(sl)),
        Err(x) => {
            if expected == Expected::SymbolLib {
                msg = format!("{}", x)
            }
        }
    }
    match project::parse_str(&data) {
        Ok(p) => return Ok(KicadFile::Project(p)),
        Err(x) => {
            if expected == Expected::Project {
                msg = format!("{}", x)
            }
        }
    }
    Ok(KicadFile::Unknown(format!("{}: {}", name, msg)))
}

/// read a file, expecting it to be a Kicad module file
pub fn read_module(name: &str) -> Result<footprint::Module> {
    match read_kicad_file(name, Expected::Module)? {
        KicadFile::Module(mo) => Ok(mo),
        x => str_error(format!("unexpected {} in {}", x, name)),
    }
}

/// read a file, expecting it to be a Kicad schematic
pub fn read_schematic(name: &str) -> Result<schematic::Schematic> {
    match read_kicad_file(name, Expected::Schematic)? {
        KicadFile::Schematic(mo) => Ok(mo),
        x => str_error(format!("unexpected {} in {}", x, name)),
    }
}

/// read a file, expecting it to be a Kicad layout file
pub fn read_layout(name: &str) -> Result<layout::Layout> {
    match read_kicad_file(name, Expected::Layout)? {
        KicadFile::Layout(mo) => Ok(mo),
        x => str_error(format!("unexpected {} in {}", x, name)),
    }
}

/// write out a kicad Layout to a file
pub fn write_layout(layout: &layout::Layout, name: &str) -> Result<()> {
    let s = layout::layout_to_string(layout, 0)?;
    write_file(name, &s)
}

/// read a file, expecting it to be a Kicad symbol library file
pub fn read_symbol_lib(name: &str) -> Result<symbol_lib::SymbolLib> {
    match read_kicad_file(name, Expected::SymbolLib)? {
        KicadFile::SymbolLib(mo) => Ok(mo),
        x => str_error(format!("unexpected {} in {}", x, name)),
    }
}

/// read a file, expecting it to be a Kicad project file
pub fn read_project(name: &str) -> Result<project::Project> {
    match read_kicad_file(name, Expected::Project)? {
        KicadFile::Project(mo) => Ok(mo),
        x => str_error(format!("unexpected {} in {}", x, name)),
    }
}

// put here so it is accessible to all subfiles privately
#[derive(Debug)]
enum Part {
    At(footprint::At),
    Layer(footprint::Layer),
    Hide,
    Effects(footprint::Effects),
    Layers(footprint::Layers),
    Width(f64),
    Angle(f64),
    Xy(footprint::Xy),
    Pts(footprint::Pts),
    Thickness(f64),
    Net(footprint::Net),
    Drill(footprint::Drill),
    SolderPasteMargin(f64),
    SolderMaskMargin(f64),
    Clearance(f64),
}

// put here so it is accessible to all subfiles privately
enum GrElement {
    Start(footprint::Xy),
    End(footprint::Xy),
    Angle(f64),
    Layer(footprint::Layer),
    Width(f64),
    TStamp(String),
    At(footprint::At),
    Effects(footprint::Effects),
}

/// calculate the bounding box of an item
pub trait BoundingBox {
    /// calculate the bounding box of an item
    fn bounding_box(&self) -> (f64, f64, f64, f64);
}

fn wrap<X, Y, F, G>(s: &Sexp, make: F, wrapper: G) -> Result<Y>
    where F: Fn(&Sexp) -> Result<X>,
          G: Fn(X) -> Y
{
    Ok(wrapper(make(s)?))
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

mod util;
mod formatter;
