// (c) 2015 Joost Yervante Damad <joost@damad.be>

use std::fmt;
use std::fs::File;
use std::io::Read;

extern crate rustysexp;
use rustysexp::Sexp;
use rustysexp::Atom;

type ERes<T> = Result<T, &'static str>;

fn err<T>(msg: &'static str) -> ERes<T> { Err(msg) }

pub enum Element {
    Layer,
    Descr,
    FpText,
    Pad,
    FpPoly,
    FpLine,
    FpCircle
}

pub struct Module {
    name: String,
    elements: Vec<Element>
}

impl Module {
    fn new(name: &String) -> Module {
        Module { name: name.clone(), elements: vec![] }
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "Module({})", self.name)
    }
}

fn parse_module_list(v: Vec<Sexp>) -> ERes<Module> {
    match v[0] {
        Sexp::Atom(Atom::S(ref s)) if s == "module" => {
            match v[1] {
                Sexp::Atom(Atom::S(ref s)) => {
                    Ok(Module::new(s))
                }
                _ => err("expecting module name")
            }
        }
        _ => err("expecting module")
    }
}

fn parse_module(s: Sexp) -> ERes<Module> {
    match s {
        Sexp::List(v) => parse_module_list(v),
        _ => err("expecting top level list")
    }
}

fn parse(s: &str) -> ERes<Module> {
    let s = rustysexp::parse_str(s);
    parse_module(s)
}


fn read_file(name: &str) -> Result<String,std::io::Error> {
    let mut f = try!(File::open(name));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}

pub fn parse_str(s: &str) -> Module {
    parse(s).unwrap()
}

pub fn parse_file(name: &str) -> Module {
    let s = read_file(name).unwrap();
    parse(&s[..]).unwrap()
}

