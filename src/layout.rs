// (c) 2016 Joost Yervante Damad <joost@productize.be>

use std::fmt;
use std::path::PathBuf;
use std::collections::HashMap;

// from parent
use ERes;
use err;
use read_file;
use footprint;

extern crate rustysexp;
use self::rustysexp::Sexp;

pub struct Layout {
    pub version:i64,
    pub setup:Setup,
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
pub struct Setup {
    pub elements:HashMap<String, Sexp>,
}


#[derive(Clone,PartialEq)]
pub struct Net {
    pub num:i64,
    pub name:String,
}

#[derive(Clone,PartialEq)]
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
    pub fn new() -> Layout {
        Layout {
            version:0,
            elements:vec![],
            setup:Setup{elements:HashMap::new()},
        }
    }

    pub fn nets(&self) -> Vec<&Net> {
        let mut v = vec![];
        for element in &self.elements {
            match *element {
                Element::Net(ref net) => v.push(net),
                _ => (),
            }
        }
        v
    }

    pub fn netclasses(&self) -> Vec<&NetClass> {
        let mut v = vec![];
        for element in &self.elements {
            match *element {
                Element::NetClass(ref net_class) => v.push(net_class),
                _ => (),
            }
        }
        v
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

    pub fn add_net(&mut self, num:i64, name:&'static str) {
        self.elements.push(Element::Net(Net { num:num, name:String::from(name) }));
    }
    pub fn add_netclass(&mut self, name:&'static str, desc:&'static str, clearance:f64, trace_width:f64, via_dia:f64, via_drill:f64, uvia_dia:f64, uvia_drill:f64, nets:Vec<String>) {
        self.elements.push(Element::NetClass(NetClass {
            name:String::from(name),
            desc:String::from(desc),
            clearance:clearance,
            trace_width:trace_width,
            via_dia:via_dia,
            via_drill:via_drill,
            uvia_dia:uvia_dia,
            uvia_drill:uvia_drill,
            nets:nets
            }));
    }
}

impl NetClass {
    pub fn equal_no_net(&self, other:&NetClass) -> bool {
        let mut s1 = self.clone();
        s1.nets = vec![];
        let mut s2 = other.clone();
        s2.nets = vec![];
        return s1 == s2;
    }
    pub fn has_net(&self, name:&'static str) -> bool {
        for net in &self.nets {
            if &net[..] == name {
                return true
            }
        }
        return false
    }
}


impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(writeln!(f, "(kicad_pcb (version {})", self.version));
        try!(writeln!(f, "  {}", self.setup));
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
        if self.name.contains("(") || self.name.contains(")") || self.name.len()==0 {
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

impl fmt::Display for Setup {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(f, "(setup"));
        for (_,e) in &self.elements {
            try!(write!(f, " {}", e));
        }
        write!(f, ")")
    }
}

fn parse_version(e:&Sexp) -> ERes<i64> {
    let l = try!(e.slice_atom("version"));
    l[0].i()
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

fn parse_net(e:&Sexp) -> ERes<Element> {
    let l = try!(e.slice_atom("net"));
    let num = try!(l[0].i());
    let name = try!(l[1].string()).clone();
    Ok(Element::Net(Net { name:name, num:num }))
}

fn parse_net_class(e:&Sexp) -> ERes<Element> {
    fn parse(e:&Sexp, name:&str) -> ERes<f64> {
        let l = try!(e.slice_atom(name));
        l[0].f()
    }
    let l = try!(e.slice_atom("net_class"));
    let name = try!(l[0].string()).clone();
    let desc = try!(l[1].string()).clone();
    let mut clearance = 0.1524;
    let mut trace_width = 0.2032;
    let mut via_dia = 0.675;
    let mut via_drill = 0.25;
    let mut uvia_dia = 0.508;
    let mut uvia_drill = 0.127;
    let mut nets = vec![];
    for x in &l[2..] {
        let list_name = try!(x.list_name());
        let xn = &list_name[..];
        match xn {
            "add_net" => {
                let l1 = try!(x.slice_atom("add_net"));
                nets.push(try!(l1[0].string()).clone())
            },
            "clearance" => clearance = try!(parse(x, xn)),
            "trace_width" => trace_width = try!(parse(x, xn)),
            "via_dia" => via_dia = try!(parse(x, xn)),
            "via_drill" => via_drill = try!(parse(x, xn)),
            "uvia_dia" => uvia_dia = try!(parse(x, xn)),
            "uvia_drill" => uvia_drill = try!(parse(x, xn)),
            _ => return Err(format!("unknown net_class field {}", list_name))
        }
    }
    let net_class = NetClass {
        name:name, desc:desc, clearance:clearance, via_dia:via_dia,
        via_drill:via_drill, uvia_dia:uvia_dia, uvia_drill:uvia_drill,
        nets:nets, trace_width:trace_width,
    };
    Ok(Element::NetClass(net_class))
}

fn parse_setup(e:&Sexp) -> ERes<Setup> {
    let mut h = HashMap::new();
    for v in try!(e.slice_atom("setup")) {
        h.insert(try!(v.list_name()).clone(), v.clone());
    }
    let s = Setup { elements:h };
    Ok(s)
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
            "net" => layout.elements.push(try!(parse_net(e))),
            "net_class" => layout.elements.push(try!(parse_net_class(e))),
            "setup" => layout.setup = try!(parse_setup(e)),
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
