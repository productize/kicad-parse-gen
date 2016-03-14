// (c) 2016 Productize SPRL <joost@productize.be>

use std::fmt;
use std::fs::File;
use std::io::Read;
use std::io::Write;

extern crate rustc_serialize;

#[macro_use]
extern crate nom;

pub type ERes<T> = Result<T, String>;

pub fn err<T>(msg: &str) -> ERes<T> {
    Err(String::from(msg))
}

pub fn read_file(name: &str) -> ERes<String> {
    let mut f = try!(match File::open(name) {
        Ok(f) => Ok(f),
        Err(err) => Err(format!("open error in file '{}': {}", name, err))
    });
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(err) => Err(format!("read error in file '{}': {}", name, err))
    }
}

pub fn write_file(name:&str, data:&String) -> ERes<()> {
    let mut f = try!(match File::create(name) {
        Ok(f) => Ok(f),
        Err(err) => Err(format!("create error in file '{}': {}", name, err))
    });
    try!(match write!(&mut f, "{}", data) {
        Ok(f) => Ok(f),
        Err(err) => Err(format!("write error in file '{}': {}", name, err))
    });
         
    Ok(())
    
}

pub fn parse_split_quote_aware(s:&String) -> Vec<String> {
    let mut v:Vec<String> = vec![];
    let mut in_q:bool = false;
    let mut q_seen:bool = false;
    let mut s2:String = String::from("");
    for c in s.chars() {
        if !in_q && c == '\"' {
            in_q = true;
            //s2.push(c);
            continue
        }
        if in_q && c == '\"' {
            in_q = false;
            //s2.push(c);
            q_seen = true;
            continue
        }
        if !in_q && c == ' ' {
            if s2.len() > 0 || q_seen {
                v.push(s2.clone());
                s2.clear();
            }
            q_seen = false;
            continue;
        }
        s2.push(c);
    }
    if s2.len() > 0 || q_seen {
        v.push(s2.clone())
    }
    return v
}

pub fn parse_split_quote_aware_n(n:usize, s:&String) -> ERes<Vec<String>> {
    let mut i = 0;
    let mut v:Vec<String> = vec![];
    let mut in_q:bool = false;
    let mut q_seen:bool = false;
    let mut s2:String = String::from("");
    for c in s.chars() {
        if i == n { // if we're in the nth. just keep collecting in current string
            s2.push(c);
            continue;
        }
        if !in_q && c == '\"' {
            in_q = true;
            //s2.push(c);
            continue
        }
        if in_q && c == '\"' {
            in_q = false;
            //s2.push(c);
            q_seen = true;
            continue
        }
        if !in_q && c == ' ' {
            if s2.len() > 0 || q_seen {
                i += 1;
                v.push(s2.clone());
                s2.clear();
            }
            q_seen = false;
            continue;
        }
        s2.push(c);
    }
    if s2.len() > 0 || q_seen {
        v.push(s2.clone())
    }
    if v.len() < n {
        return Err(format!("expecting {} elements in {}", n, s))
    }
    Ok(v)
}

pub mod footprint;
pub mod schematic;
pub mod layout;
pub mod symbol_lib;
pub mod project;

//pub mod schematic2;

pub enum KicadFile {
    Unknown(String),
    Module(footprint::Module),
    Schematic(schematic::Schematic),
    Layout(layout::Layout),
    SymbolLib(symbol_lib::SymbolLib),
    Project(project::Project),
}

#[derive(PartialEq)]
pub enum Expected {
    Module,
    Schematic,
    Layout,
    SymbolLib,
    Project,
    Any,
}


impl fmt::Display for KicadFile {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            KicadFile::Unknown(ref x)   => write!(f, "unknown: {}", x),
            KicadFile::Module(_)    => write!(f, "module"),
            KicadFile::Schematic(_) => write!(f, "schematic"),
            KicadFile::Layout(_)    => write!(f, "layout"),
            KicadFile::SymbolLib(_) => write!(f, "symbollib"),
            KicadFile::Project(_)   => write!(f, "project"),
        }
    }
}


pub fn read_kicad_file(name: &str, expected:Expected) -> ERes<KicadFile> {
    let pb = std::path::PathBuf::from(name);
    let data = try!(read_file(name));
    let mut msg = String::new();
    match footprint::parse(&data) {
        Ok(module) => return Ok(KicadFile::Module(module)),
        Err(x) => { if expected == Expected::Module { msg = format!("{}", x) } },
    }
    match schematic::parse(Some(pb), &data) {
        Ok(sch) => return Ok(KicadFile::Schematic(sch)),
        Err(x) => { if expected == Expected::Schematic { msg = format!("{}", x) } },
    }
    match layout::parse(&data) {
        Ok(layout) => return Ok(KicadFile::Layout(layout)),
        Err(x) => { if expected == Expected::Layout { msg = format!("{}", x) } },
    }
    match symbol_lib::parse_str(&data) {
        Ok(sl) => return Ok(KicadFile::SymbolLib(sl)),
        Err(x) => { if expected == Expected::SymbolLib { msg = format!("{}", x) } },
    }
    match project::parse_str(&data) {
        Ok(p) => return Ok(KicadFile::Project(p)),
        Err(x) => { if expected == Expected::Project { msg = format!("{}", x) } },
    }
    return Ok(KicadFile::Unknown(format!("{}: {}", name, msg)))
}

pub fn read_module(name: &str) -> ERes<footprint::Module> {
    match try!(read_kicad_file(name, Expected::Module)) {
        KicadFile::Module(mo) => Ok(mo),
        x => Err(format!("unexpected {} in {}", x, name)),
    }
}

pub fn read_schematic(name: &str) -> ERes<schematic::Schematic> {
    match try!(read_kicad_file(name, Expected::Schematic)) {
        KicadFile::Schematic(mo) => Ok(mo),
        x => Err(format!("unexpected {} in {}", x, name)),
    }
}

pub fn read_layout(name: &str) -> ERes<layout::Layout> {
    match try!(read_kicad_file(name, Expected::Layout)) {
        KicadFile::Layout(mo) => Ok(mo),
        x => Err(format!("unexpected {} in {}", x, name)),
    }
}

pub fn read_symbol_lib(name: &str) -> ERes<symbol_lib::SymbolLib> {
    match try!(read_kicad_file(name, Expected::SymbolLib)) {
        KicadFile::SymbolLib(mo) => Ok(mo),
        x => Err(format!("unexpected {} in {}", x, name)),
    }
}

pub fn read_project(name: &str) -> ERes<project::Project> {
    match try!(read_kicad_file(name, Expected::Project)) {
        KicadFile::Project(mo) => Ok(mo),
        x => Err(format!("unexpected {} in {}", x, name)),
    }
}
