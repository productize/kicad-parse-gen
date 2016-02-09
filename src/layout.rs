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
    pub version:i64,
    pub elements:Vec<Element>,
}

#[derive(Clone)]
pub enum Element {
    Module(footprint::Module),
    Net(Net),
    NetClass(NetClass),
    Other(Sexp),
}

#[derive(Clone)]
pub struct Net {
    pub num:i64,
    pub name:String,
}

#[derive(Clone)]
pub struct NetClass {
    pub name:String,
    pub desc:String,
    pub clearance:f64,
    pub trace_width:f64,
    pub via_dia:f64,
    pub via_drill:f64,
    pub uvia_dia:f64,
    pub uvia_drill:f64,
    pub nets:Vec<String>,
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
                Element::Net(_) => (),
                Element::NetClass(_) => (),
                Element::Other(_) => (),
            }
        }
        Err(format!("did not find module with reference {}", reference))
    }
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
            Element::Net(ref s) => write!(f, "{}", s),
            Element::NetClass(ref s) => write!(f, "{}", s),
        }
    }
}

impl fmt::Display for Net {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if self.name.contains("(") || self.name.contains(")") {
            write!(f, "(net {} \"{}\")", self.num, self.name)
        } else {
            write!(f, "(net {} {})", self.num, self.name)
        }
    }
}
impl fmt::Display for NetClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(f, "(net_class {} \"{}\" ", self.name, self.desc));
        try!(write!(f, "(clearance {}) ", self.clearance));
        try!(write!(f, "(trace_width {}) ", self.trace_width));
        try!(write!(f, "(via_dia {}) ", self.via_dia));
        try!(write!(f, "(via_drill {}) ", self.via_drill));
        try!(write!(f, "(uvia_dia {}) ", self.uvia_dia));
        try!(write!(f, "(uvia_drill {}) ", self.uvia_drill));
        for net in &self.nets {
            if net.contains("(") || net.contains(")") {
                try!(write!(f, "(add_net \"{}\")", net))
            } else {
                try!(write!(f, "(add_net {})", net))
            }
        }
        write!(f, ")")
        
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
