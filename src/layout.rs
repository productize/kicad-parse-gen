// (c) 2016 Joost Yervante Damad <joost@productize.be>

use std::fmt;
use std::path::PathBuf;

// from parent
use ERes;
use err;
use read_file;
use footprint;

extern crate rustysexp;
use self::rustysexp::Sexp;

pub struct Layout {
    version:i64,
    elements:Vec<Element>,
}

impl Layout {
    fn new() -> Layout {
        Layout {
            version:0,
            elements:vec![],
        }
    }

    pub fn modify_module<F>(&mut self, reference:&String, fun:F) -> ERes<()> 
        where F:Fn(&mut footprint::Module) -> ()
    {
        for ref mut x in &mut self.elements[..] {
            match **x {
                Element::Module(ref mut m) => {
                    if m.is_reference(reference) {
                        return Ok(fun(m))
                    }
                },
                Element::Other(_) => (),
            }
        }
        Err(format!("did not find module with reference {}", reference))
    }
}

enum Element {
    Other(Sexp),
    Module(footprint::Module),
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(writeln!(f, "(kicad_pcb (version {})", self.version));
        for element in &self.elements[..] {
            try!(writeln!(f, "  {}", element));
            try!(writeln!(f, ""));
        }
        writeln!(f, ")")
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Element::Other(ref s) => write!(f, "{}", s),
            Element::Module(ref s) => write!(f, "{}", s),
        }
    }
}


fn parse_version(e:&Sexp) -> ERes<i64> {
    let l = try!(e.slice_atom("version"));
    try!(l[0].atom()).i()
}

fn parse_other(e:&Sexp) -> Element {
    let e2 = e.clone();
    Element::Other(e2)
}

fn parse_module(e:&Sexp) -> ERes<Element> {
    let e2 = e.clone();
    let m = try!(footprint::parse_module(e2));
    Ok(Element::Module(m))
}

fn parse(s: &str) -> ERes<Layout> {
    let exp = match rustysexp::parse_str(s) {
        Ok(s) => s,
        Err(x) => return Err(format!("ParseError: {}", x)),
    };
    let l1 = try!(exp.slice_atom("kicad_pcb"));
    let mut layout = Layout::new();
    
    for ref e in l1 {
        match &try!(e.list_name())[..] {
            "version" => layout.version = try!(parse_version(e)),
            "module" => layout.elements.push(try!(parse_module(e))),
            _ => layout.elements.push(parse_other(e)),
        }
    }
    Ok(layout)
}

pub fn parse_str(s: &str) -> ERes<Layout> {
    parse(s)
}

pub fn parse_file(filename: &PathBuf) -> ERes<Layout> {
    let name = filename.to_str().unwrap();
    let s = try!(match read_file(name) {
        Ok(s) => Ok(s),
        Err(x) => Err(format!("io error: {}", x))
    });
    parse(&s[..])
}
