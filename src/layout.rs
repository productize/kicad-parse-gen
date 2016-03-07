// (c) 2016 Productize SPRL <joost@productize.be>

use std::fmt;

// from parent
use ERes;
use err;
use footprint;
use footprint::FromSexp;
use footprint::wrap;

extern crate rustysexp;
use self::rustysexp::Sexp;

pub struct Layout {
    pub version:i64,
    pub host:Host,
    pub general:General,
    pub page:String,
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
pub struct Host {
    pub tool:String,
    pub build:String,
}

#[derive(Clone)]
pub struct General {
    pub links:i64,
    pub no_connects:i64,
    pub area:Area,
    pub thickness:f64,
    pub drawings:i64,
    pub tracks:i64,
    pub zones:i64,
    pub modules:i64,
    pub nets:i64,
}

#[derive(Clone)]
pub struct Area {
    x1:f64, y1:f64,
    x2:f64, y2:f64,
}

#[derive(Clone)]
pub struct Setup {
    pub elements:Vec<(String, String)>,
    pub pcbplotparams:Option<Sexp>,
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
            version:4,
            host:Host::new(),
            elements:vec![],
            setup:Setup::new(),
            general:General::new(),
            page:String::from("A4"),
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

impl Host {
    pub fn new() -> Host {
        Host { tool:String::from("pcbnew"), build:String::from("custom") }
    }
}

impl General {
    fn new() -> General {
        General {
            links:0,
            no_connects:0,
            area:Area::new(),
            thickness:1.6,
            drawings:0,
            tracks:0,
            zones:0,
            modules:0,
            nets:0,
        }
    }
}

impl Area {
    fn new() -> Area {
        Area {
            x1:0.0, y1:0.0,
            x2:0.0, y2:0.0,
        }
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

impl Setup {
    pub fn new() -> Setup {
        let mut s2 = rustysexp::Sexp::new_empty();
        let e = rustysexp::Element::String(String::from("pcbplotparams"));
        s2.element = e;
        let v = vec![s2];
        let mut s1 = rustysexp::Sexp::new_empty();
        s1.element = rustysexp::Element::List(v);
        Setup { elements:vec![], pcbplotparams:Some(s1) }
    }
    pub fn get(&self, s:&String) -> Option<&String> {
        for element in &self.elements {
            if &element.0[..] == &s[..] {
                return Some(&element.1)
            }
        }
        return None
    }

    pub fn update_element(&mut self, name:&'static str, value:String) {
        for element in &mut self.elements {
            if &element.0[..] == name {
                element.1 = value;
                return;
                    
            }
        }
        self.elements.push((String::from(name), value));
    }
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(writeln!(f, "(kicad_pcb (version {}) (host {} \"{}\")", self.version, self.host.tool, self.host.build));
        let mut i = 0;
        for element in &self.elements[..] {
            try!(writeln!(f, "  {}", element));
            try!(writeln!(f, ""));
            // kludge to put setup at the right order in the file
            if i == 3 {
                try!(writeln!(f, "  {}", self.setup));
            }
            i+=1;
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
        try!(write!(f, "(net_class {} {} ", self.name, rustysexp::display_string(&self.desc)));
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
        try!(writeln!(f, "(setup"));
        for ref k in &self.elements {
            try!(writeln!(f, "   ({} {})", k.0, k.1));
        }
        match self.pcbplotparams {
            None => writeln!(f, ")"),
            Some(ref s) => writeln!(f, " {})", s),
        }
    }
}

fn parse_version(e:&Sexp) -> ERes<i64> {
    let l = try!(e.slice_atom("version"));
    l[0].i()
}

fn parse_page(e:&Sexp) -> ERes<String> {
    let l = try!(e.slice_atom("page"));
    Ok(try!(l[0].string()).clone())
}

impl FromSexp for ERes<Net> {
    fn from_sexp(s:&Sexp) -> ERes<Net> {
        let l = try!(s.slice_atom_num("net", 2));
        let num = try!(l[0].i());
        let name = try!(l[1].string()).clone();
        Ok(Net { name:name, num:num })
    }
}

impl FromSexp for ERes<Host> {
    fn from_sexp(s:&Sexp) -> ERes<Host> {
        let l = try!(s.slice_atom_num("host", 2));
        let tool = try!(l[0].string()).clone();
        let build = try!(l[1].string()).clone();
        Ok(Host { tool:tool, build:build })
    }
}

impl FromSexp for ERes<General> {
    fn from_sexp(s:&Sexp) -> ERes<General> {
        let l = try!(s.slice_atom_num("general", 9));
        let links = try!(l[0].i());
        let no_connects = try!(l[1].i());
        let area = try!(ERes::from_sexp(&l[2]));
        let thickness = try!(l[3].f());
        let drawings = try!(l[4].i());
        let tracks = try!(l[5].i());
        let zones = try!(l[6].i());
        let modules = try!(l[7].i());
        let nets = try!(l[8].i());
        Ok(General {
            links:links,
            no_connects:no_connects,
            area:area,
            thickness:thickness,
            drawings:drawings,
            tracks:tracks,
            zones:zones,
            modules:modules,
            nets:nets,
        })
    }
}

impl FromSexp for ERes<Area> {
    fn from_sexp(s:&Sexp) -> ERes<Area> {
        let l = try!(s.slice_atom_num("area", 4));
        let x1 = try!(l[0].f());
        let y1 = try!(l[1].f());
        let x2 = try!(l[2].f());
        let y2 = try!(l[3].f());
        Ok(Area { x1:x1, y1:y1, x2:x2, y2:y2 })
    }
}

impl FromSexp for ERes<NetClass> {
    fn from_sexp(s:&Sexp) -> ERes<NetClass> {
        fn parse(e:&Sexp, name:&str) -> ERes<f64> {
            let l = try!(e.slice_atom(name));
            l[0].f()
        }
        let l = try!(s.slice_atom("net_class"));
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
        Ok(net_class)
    }
}

impl FromSexp for ERes<Setup> {
    fn from_sexp(s:&Sexp) -> ERes<Setup> {
        let mut h = vec![];
        let mut pcbplotparams = None;
        for v in try!(s.slice_atom("setup")) {
            let n = v.list_name().unwrap().clone();
            match &n[..] {
                "pcbplotparams" => { pcbplotparams = Some(v.clone()); },
                x => {
                    // TODO: this is a kludge, implement proper data type for Setup
                    let v2 = v.slice_atom(x).unwrap();
                    let mut s = String::new();
                    let mut first = true;
                    for z in v2 {
                        if !first { s.push_str(" ") };
                        first = false;
                        s.push_str(z.string().unwrap());
                    }
                    h.push((n.clone(), s));
                },
            }
        }
        let s = Setup { elements:h , pcbplotparams:pcbplotparams };
        Ok(s)
    }
}

// for some reason this needs to be in a subfunction or it doesn't work
fn parse_other(e:&Sexp) -> Element {
    let e2 = e.clone();
    Element::Other(e2)
}


impl FromSexp for ERes<Layout> {
    fn from_sexp(s:&Sexp) -> ERes<Layout> {
        let l1 = try!(s.slice_atom("kicad_pcb"));
        let mut layout = Layout::new();
        for ref e in l1 {
            match &try!(e.list_name())[..] {
                "version" => {
                    layout.version = try!(parse_version(e))
                },
                "host" => {
                    layout.host = try!(ERes::from_sexp(&e))
                },
                "general" => {
                    layout.general = try!(ERes::from_sexp(&e))
                },
                "page" => {
                    layout.page = try!(parse_page(&e))
                },
                "module" => {
                    let module = try!(wrap(e, ERes::from_sexp, Element::Module));
                    layout.elements.push(module)
                },
                "net" => {
                    let net = try!(wrap(e, ERes::from_sexp, Element::Net));
                    layout.elements.push(net)
                },
                "net_class" => {
                    let nc = try!(wrap(e, ERes::from_sexp, Element::NetClass));
                    layout.elements.push(nc)
                },
                "setup" => {
                    layout.setup = try!(ERes::from_sexp(&e))
                },
                _ => {
                    println!("unimplemented: {}", e);
                    layout.elements.push(parse_other(e))
                },
            }
        }
        Ok(layout)
    }
}


pub fn parse(s: &str) -> ERes<Layout> {
    match rustysexp::parse_str(s) {
        Ok(s) => ERes::from_sexp(&s),
        Err(x) => return Err(format!("ParseError: {}", x)),
    }
}
