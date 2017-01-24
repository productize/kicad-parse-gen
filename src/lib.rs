// (c) 2016 Productize SPRL <joost@productize.be>

//! Kicad file format parser and generator library

#![warn(missing_docs)]

extern crate symbolic_expressions;
#[macro_use]
extern crate error_chain;

use std::fmt;
use std::slice::Iter;

pub use symbolic_expressions::Sexp;
pub use error::*;

/// convert from a symbolic-expression to something
pub trait FromSexp
    where Self: Sized
{
    /// convert from a symbolic-expression to something
    fn from_sexp(&Sexp) -> Result<Self>;
}

/// convert from a symbolic-expression to something (dispatcher)
pub fn from_sexp<T: FromSexp>(s: &Sexp) -> Result<T> {
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
    match footprint::parse(&data) {
        Ok(module) => return Ok(KicadFile::Module(module)),
        Err(x) => {
            if expected == Expected::Module {
                return Err(x);
            }
        }
    }
    match schematic::parse(Some(pb), &data) {
        Ok(sch) => return Ok(KicadFile::Schematic(sch)),
        Err(x) => {
            if expected == Expected::Schematic {
                return Err(x);
            }
        }
    }
    match layout::parse(&data) {
        Ok(layout) => return Ok(KicadFile::Layout(layout)),
        Err(x) => {
            if expected == Expected::Layout {
                return Err(x);
            }
        }
    }
    match symbol_lib::parse_str(&data) {
        Ok(sl) => return Ok(KicadFile::SymbolLib(sl)),
        Err(x) => {
            if expected == Expected::SymbolLib {
                return Err(x);
            }
        }
    }
    match project::parse_str(&data) {
        Ok(p) => return Ok(KicadFile::Project(p)),
        Err(x) => {
            if expected == Expected::Project {
                return Err(x);
            }
        }
    }
    Ok(KicadFile::Unknown(name.into()))
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

/// Atom iterator wrapper
pub struct IterAtom<'a> {
    iter:Iter<'a,Sexp>,
}

/*
impl<'a> From<Iter<'a, Sexp>> for IterAtom<'a> {
    fn from(i:Iter<'a, Sexp>) -> IterAtom<'a> {
        IterAtom { iter: i }
    }
}
*/

impl<'a> IterAtom<'a> {

    fn new(s:&'a Sexp, name:&str) -> Result<IterAtom<'a>> {
        let i = s.iter_atom(name)?;
        Ok(IterAtom { iter: i })
    }
    
    fn expect<T,F>(&mut self, sname:&str, name:&str, get:F) -> Result<T>
        where F:Fn(&Sexp) -> Result<T>
    {
        match self.iter.next() {
            Some(x) => get(x),
            None => return Err(format!("missing {} field in {}",name, sname).into()),
        }
    }

    fn optional<T,F>(&mut self, or:T, get:F) -> Result<T>
        where F:Fn(&Sexp) -> Result<T>
    {
        let x = match self.iter.next() {
            Some(x) => get(x)?,
            None => or,
        };
        Ok(x)
    }
    
    /// expect an integer while iterating a `Sexp` list
    pub fn i(&mut self, sname:&str, name:&str) -> Result<i64>
    {
        self.expect(sname, name, |x| x.i().map_err(From::from))
    }

    /// expect a float while iterating a `Sexp` list
    pub fn f(&mut self, sname:&str, name:&str) -> Result<f64> {
        self.expect(sname, name, |x| x.f().map_err(From::from))
    }

    /// expect a String while iterating a `Sexp` list
    pub fn s(&mut self, sname:&str, name:&str) -> Result<String> {
        self.expect(sname, name, |x| x.string().map(|y| y.clone()).map_err(From::from))
    }
    
    /// expect a `Sexp` while iterating a `Sexp` list
    pub fn t<T:FromSexp>(&mut self, sname:&str, name:&str) -> Result<T> {
        self.expect(sname, name, |x| T::from_sexp(x))
    }

    /// optional integer while iterating a `Sexp` list
    pub fn opt_i(&mut self, or:i64) -> Result<i64> {
        self.optional(or, |x| x.i().map_err(From::from))
    }

    /// optional float while iterating a `Sexp` list
    pub fn opt_f(&mut self, or:f64) -> Result<f64> {
        self.optional(or, |x| x.f().map_err(From::from))
    }
    
    /// optional String while iterating a `Sexp` list
    pub fn opt_s(&mut self, or:String) -> Result<String> {
        self.optional(or, |x| x.string().map(|y| y.clone()).map_err(From::from))
    }
    
    /// optional `Sexp` while iterating a `Sexp` list
    pub fn opt_t<T:FromSexp>(&mut self) -> Result<Option<T>> {
        let x = match self.iter.next() {
            Some(x) => {
                let t:T = T::from_sexp(x)?;
                Some(t)
            },
            None => None,
        };
        Ok(x)
    }

    /// expect remainder if iterator to be a `Vec<T>`
    pub fn vec<T:FromSexp>(&mut self) -> Result<Vec<T>> {
        let mut res = Vec::new();
        loop {
            match self.iter.next() {
                None => break,
                Some(e) => {
                    let p = from_sexp(e)?;
                    res.push(p);
                }
            }
        }
        Ok(res)
    }
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
