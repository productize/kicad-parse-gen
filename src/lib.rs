// (c) 2016 Joost Yervante Damad <joost@productize.be>

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

macro_rules! fail {
    ($expr:expr) => (
        return Err(::std::error::FromError::from_error($expr));
    )
}

macro_rules! println_err(
    ($($arg:tt)*) => (
        match writeln!(&mut ::std::io::stderr(), $($arg)* ) {
            Ok(_) => {},
            Err(x) => panic!("Unable to write to stderr: {}", x),
        }
    )
);

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

//pub mod schematic2;

pub enum KicadFile {
    Unknown(String),
    Module(footprint::Module),
    Schematic(schematic::Schematic),
    Layout(layout::Layout),
    SymbolLib(symbol_lib::SymbolLib),
}

pub fn read_kicad_file(name: &str) -> ERes<KicadFile> {
    let data = try!(read_file(name));
    match footprint::parse_str(&data) {
        Ok(module) => return Ok(KicadFile::Module(module)),
        _ => (),
    }
    match schematic::parse_str(&data) {
        Ok(sch) => return Ok(KicadFile::Schematic(sch)),
        _ => (),
    }
    match layout::parse_str(&data) {
        Ok(layout) => return Ok(KicadFile::Layout(layout)),
        _ => (),
    }
    match symbol_lib::parse_str(&data) {
        Ok(sl) => return Ok(KicadFile::SymbolLib(sl)),
        _ => (),
    }
    return Ok(KicadFile::Unknown(String::from("unknown")))
}

