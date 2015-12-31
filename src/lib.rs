// (c) 2015-2016 Joost Yervante Damad <joost@damad.be>

use std::fmt;
use std::fs::File;
use std::io::Read;

extern crate rustysexp;
use rustysexp::Sexp;
use rustysexp::Atom;

type ERes<T> = Result<T, String>;

fn err<T>(msg: &str) -> ERes<T> {
    Err(String::from(msg))
}

pub enum Element {
    Layer(String),
    Descr(String),
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
    fn append(&mut self, e: Element) {
        self.elements.push(e)
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(f, "(module {}\n", self.name));
        for e in &self.elements {
            try!(write!(f, "{}\n", e))
        };
        write!(f, ")")
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Element::Layer(ref s) => write!(f, "(layer {})", s),
            Element::Descr(ref s) => write!(f, "(descr \"{}\")", s),
            Element::FpText => write!(f, "(fp_text)"),
            Element::Pad => write!(f, "(pad)"),
            Element::FpPoly => write!(f, "(fp_poly)"),
            Element::FpLine => write!(f, "(fp_line)"),
            Element::FpCircle => write!(f, "(fp_circle)"),
        }
    }
}

fn parse_layer(v: &Vec<Sexp>) -> ERes<Element> {
    match v[1] {
        Sexp::Atom(Atom::S(ref s)) => {
            Ok(Element::Layer(s.clone()))
        }
        ref x => Err(format!("unexpected element in layer: {}", x))
    }
}

fn parse_descr(v: &Vec<Sexp>) -> ERes<Element> {
    match v[1] {
        Sexp::Atom(Atom::Q(ref s)) => {
            Ok(Element::Descr(s.clone()))
        }
        ref x => Err(format!("unexpected element in descr: {}", x))
    }
}
fn parse_fp_text(v: &Vec<Sexp>) -> ERes<Element> {
    Ok(Element::FpText)
}
fn parse_pad(v: &Vec<Sexp>) -> ERes<Element> {
    Ok(Element::Pad)
}
fn parse_fp_poly(v: &Vec<Sexp>) -> ERes<Element> {
    Ok(Element::FpPoly)
}
fn parse_fp_line(v: &Vec<Sexp>) -> ERes<Element> {
    Ok(Element::FpLine)
}
fn parse_fp_circle(v: &Vec<Sexp>) -> ERes<Element> {
    Ok(Element::FpCircle)
}

fn parse_element_list(v: &Vec<Sexp>) -> ERes<Element> {
    match v[0] {
        Sexp::Atom(Atom::S(ref s)) => {
            match &s[..] {
                "layer" => parse_layer(v),
                "descr" => parse_descr(v),
                "fp_text" => parse_fp_text(v),
                "pad" => parse_pad(v),
                "fp_poly" => parse_fp_poly(v),
                "fp_line" => parse_fp_line(v),
                "fp_circle" => parse_fp_circle(v),
                x => Err(format!("unknown element in module: {}", x))
            }
        }
        _ => err("expecting atom")
    }
}

fn parse_element(s: &Sexp) -> ERes<Element> {
    match *s {
        Sexp::List(ref v) => parse_element_list(&v),
        _ => err("expecting element list in module")
    }
}

fn parse_module_list(v: &Vec<Sexp>) -> ERes<Module> {
    let mut module = (match v[0] {
        Sexp::Atom(Atom::S(ref s)) if s == "module" => {
            match v[1] {
                Sexp::Atom(Atom::S(ref s)) => {
                    Ok(Module::new(s))
                }
                ref x => return Err(format!("expecting module name got {}", x))
            }
        }
        _ => err("expecting module")
    }).unwrap();
    for e in &v[2..] {
        let el = try!(parse_element(e));
        module.append(el)
    }
    Ok(module)
}

fn parse_module(s: Sexp) -> ERes<Module> {
    match s {
        Sexp::List(v) => parse_module_list(&v),
        _ => err("expecting top level list")
    }
}

fn parse(s: &str) -> ERes<Module> {
    match rustysexp::parse_str(s) {
        Ok(s) => parse_module(s),
        Err(x) => Err(format!("IOError: {}", x))
    }
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

