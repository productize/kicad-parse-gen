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
    pub layers:Vec<Layer>,
    pub elements:Vec<Element>,
}

#[derive(Clone)]
pub enum Element {
    Module(footprint::Module),
    Net(Net),
    NetClass(NetClass),
    Graphics(Graphics),
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
pub struct Layer {
    pub num:i64,
    pub layer:footprint::Layer,
    pub layer_type:LayerType,
    pub hide:bool,
}

#[derive(Clone)]
pub enum LayerType {
    Signal,
    User,
}

#[derive(Clone)]
pub struct Setup {
    pub elements:Vec<SetupElement>,
    pub pcbplotparams:Vec<SetupElement>,
}

#[derive(Clone)]
pub struct SetupElement {
    pub name:String,
    pub value1:String,
    pub value2:Option<String>,
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

// TODO: support tstamp in graphics elements
#[derive(Clone)]
pub enum Graphics {
    GrText(GrText),
    GrLine(GrLine),
    GrArc(GrArc),
    Dimension(Dimension),
    Segment,
    Via,
    FilledPolygon,
}

#[derive(Clone)]
pub struct GrText {
    pub value:String,
    pub at:footprint::At,
    pub layer:footprint::Layer,
    pub effects:footprint::Effects,
}

#[derive(Clone)]
pub struct GrLine {
    pub start:footprint::Xy,
    pub end:footprint::Xy,
    pub angle:i64,
    pub layer:footprint::Layer,
    pub width:f64,
}

#[derive(Clone)]
pub struct GrArc {
    pub start:footprint::Xy,
    pub end:footprint::Xy,
    pub angle:i64,
    pub layer:footprint::Layer,
    pub width:f64,
}

#[derive(Clone)]
pub struct Dimension {
    pub name:String,
    pub width:f64,
    pub layer:footprint::Layer,
    pub text:GrText,
    pub feature1:footprint::Pts,
    pub feature2:footprint::Pts,
    pub crossbar:footprint::Pts,
    pub arrow1a:footprint::Pts,
    pub arrow1b:footprint::Pts,
    pub arrow2a:footprint::Pts,
    pub arrow2b:footprint::Pts,
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
            layers:vec![],
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
                Element::Net(_)      => (),
                Element::NetClass(_) => (),
                Element::Other(_)    => (),
                Element::Graphics(_) => (),
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
        Setup { elements:vec![], pcbplotparams:vec![] }
    }
    pub fn get(&self, s:&String) -> Option<&String> {
        for element in &self.elements {
            if &element.name[..] == &s[..] {
                return Some(&element.value1)
            }
        }
        return None
    }

    pub fn update_element(&mut self, name:&'static str, value:String) {
        for element in &mut self.elements {
            if &element.name[..] == name {
                element.value1 = value;
                return;
                    
            }
        }
        self.elements.push(SetupElement { name:String::from(name), value1:value, value2:None })
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
            Element::Graphics(ref s) => write!(f, "{}", s),
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
impl fmt::Display for SetupElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.value2 {
            Some(ref x) => writeln!(f, "   ({} {} {})", self.name, self.value1, x),
            None => writeln!(f, "   ({} {})", self.name, self.value1),
        }
    }
}

impl fmt::Display for Setup {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(writeln!(f, "(setup"));
        for ref k in &self.elements {
            try!(writeln!(f, "   ({} {})", k.name, k.value1));
        }
        try!(writeln!(f, " (pcbplotparams"));
        for ref k in &self.pcbplotparams {
            try!(writeln!(f, "     ({} {})", k.name, k.value1));
        }
        writeln!(f, "))")
    }
}

impl fmt::Display for Graphics {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Graphics::GrText(ref x) => write!(f, "{}", x),
            Graphics::GrLine(ref x) => write!(f, "{}", x),
            Graphics::GrArc(ref x) => write!(f, "{}", x),
            Graphics::Dimension(ref x) => write!(f, "{}", x),
            _ => write!(f, "(TODO)"),
        }
    }
}


impl fmt::Display for GrText {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(writeln!(f,"(gr_text {} {} (layer {})", rustysexp::display_string(&self.value), self.at, self.layer));
        try!(writeln!(f,"    {}", self.effects));
        write!(f,")")
    }
}

impl fmt::Display for GrLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "(gr_line {} {} (angle {}) (layer {}) (width {}))", self.start, self.end, self.angle, self.layer, self.width)
    }
}

impl fmt::Display for GrArc {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "(gr_arc {} {} (angle {}) (layer {}) (width {}))", self.start, self.end, self.angle, self.layer, self.width)
    }
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(writeln!(f, "(dimension {} (width {}) (layer {})", rustysexp::display_string(&self.name), self.width, self.layer));
        try!(writeln!(f, "{}", self.text));
        try!(writeln!(f, "(feature1 {})", self.feature1));
        try!(writeln!(f, "(feature2 {})", self.feature2));
        try!(writeln!(f, "(crossbar {})", self.crossbar));
        try!(writeln!(f, "(arrow1a {})", self.arrow1a));
        try!(writeln!(f, "(arrow1b {})", self.arrow1b));
        try!(writeln!(f, "(arrow2a {})", self.arrow2a));
        try!(writeln!(f, "(arrow2b {})", self.arrow2b));
        writeln!(f, ")")
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
        let links = try!(l[0].named_value_i("links"));
        let no_connects = try!(l[1].named_value_i("no_connects"));
        let area = try!(ERes::from_sexp(&l[2]));
        let thickness = try!(l[3].named_value_f("thickness"));
        let drawings = try!(l[4].named_value_i("drawings"));
        let tracks = try!(l[5].named_value_i("tracks"));
        let zones = try!(l[6].named_value_i("zones"));
        let modules = try!(l[7].named_value_i("modules"));
        let nets = try!(l[8].named_value_i("nets"));
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

impl FromSexp for ERes<Vec<Layer> > {
    fn from_sexp(s:&Sexp) -> ERes<Vec<Layer> > {
        let mut v = vec![];
        let l = try!(s.slice_atom("layers"));
        for x in l {
            let layer = try!(ERes::from_sexp(&x));
            v.push(layer)
        }
        Ok(v)
    }
}

impl FromSexp for ERes<Layer> {
    fn from_sexp(s:&Sexp) -> ERes<Layer> {
        let l = try!(s.list());
        //println!("making layer from {}", s);
        if l.len() != 3 && l.len() != 4 {
            return Err(format!("expecting 3 elements in layer: {}", s))
        }
        let num = try!(l[0].i());
        let layer = try!(footprint::Layer::from_string(try!(l[1].string()).clone()));
        let layer_type = try!(ERes::from_sexp(&l[2]));
        let hide = if l.len() == 3 {
            false
        } else {
            let h = try!(l[3].string());
            match &h[..] {
                "hide" => true,
                _ => false,
            }
        };
        Ok(Layer { num:num, layer:layer, layer_type:layer_type, hide:hide })
    }
}

impl FromSexp for ERes<LayerType> {
    fn from_sexp(s:&Sexp) -> ERes<LayerType> {
        let x = try!(s.string());
        match &x[..] {
            "signal" => Ok(LayerType::Signal),
            "user" => Ok(LayerType::User),
            _ => Err(format!("unknown layertype {} in {}", x, s)),
        }
    }
}

impl FromSexp for ERes<SetupElement> {
    fn from_sexp(s:&Sexp) -> ERes<SetupElement> {
        let l = try!(s.list());
        if l.len() != 2 && l.len() != 3 {
            return Err(format!("expecting 2 or 3 elements in setup element: {}", s))
        }
        let name = try!(l[0].string()).clone();
        let value1 = try!(l[1].string()).clone();
        let value2 = match l.len() {
            3 => Some(try!(l[2].string()).clone()),
            _ => None,
        };
        Ok(SetupElement { name:name, value1:value1, value2:value2, })
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
        let mut elements = vec![];
        let mut pcbplotparams = vec![];
        for v in try!(s.slice_atom("setup")) {
            let n = v.list_name().unwrap().clone();
            match &n[..] {
                "pcbplotparams" => {
                    for y in try!(v.slice_atom("pcbplotparams")) {
                        let p_e = try!(ERes::from_sexp(&y));
                        pcbplotparams.push(p_e)
                    }
                },
                _ => {
                    let setup_element = try!(ERes::from_sexp(&v));
                    elements.push(setup_element)
                }
            }
        }
        let s = Setup { elements:elements , pcbplotparams:pcbplotparams };
        Ok(s)
    }
}

// for some reason this needs to be in a subfunction or it doesn't work
fn parse_other(e:&Sexp) -> Element {
    let e2 = e.clone();
    Element::Other(e2)
}

impl FromSexp for ERes<GrText> {
    fn from_sexp(s:&Sexp) -> ERes<GrText> {
        let l = try!(s.slice_atom_num("gr_text", 4));
        let value = try!(l[0].string()).clone();
        let at = try!(ERes::from_sexp(&l[1]));
        let layer = try!(ERes::from_sexp(&l[2]));
        let effects = try!(ERes::from_sexp(&l[3]));
        Ok(GrText { value:value, at:at, layer:layer, effects:effects })
    }
}

impl FromSexp for ERes<GrLine> {
    fn from_sexp(s:&Sexp) -> ERes<GrLine> {
        println!("GrLine: {}", s);
        let l = try!(s.slice_atom("gr_line"));
        if l.len() !=4 && l.len() != 5 {
            return Err(format!("expected 4 or 5 elements in {}", s))
        }
        let start = try!(ERes::from_sexp(&l[0]));
        let end = try!(ERes::from_sexp(&l[1]));
        let (angle,i) = if l.len() == 4 {
            (0,2)
        } else {
            let l2 = try!(l[2].slice_atom("angle"));
            (try!(l2[0].i()), 3)
        };
        let layer = try!(ERes::from_sexp(&l[i]));
        let width = {
            let l2 = try!(l[i+1].slice_atom("width"));
            try!(l2[0].f())
        };
        Ok(GrLine { start:start, end:end, angle:angle, layer:layer, width:width })
    }
}

impl FromSexp for ERes<GrArc> {
    fn from_sexp(s:&Sexp) -> ERes<GrArc> {
        let l = try!(s.slice_atom_num("gr_arc", 5));
        let start = try!(ERes::from_sexp(&l[0]));
        let end = try!(ERes::from_sexp(&l[1]));
        let angle = {
            let l2 = try!(l[2].slice_atom("angle"));
            try!(l2[0].i())
        };
        let layer = try!(ERes::from_sexp(&l[3]));
        let width = {
            let l2 = try!(l[4].slice_atom("width"));
            try!(l2[0].f())
        };
        Ok(GrArc { start:start, end:end, angle:angle, layer:layer, width:width })
    }
}

impl FromSexp for ERes<Dimension> {
    fn from_sexp(s:&Sexp) -> ERes<Dimension> {
        let l = try!(s.slice_atom_num("dimension", 11));
        let name = try!(l[0].string()).clone();
        let width = {
            let l2 = try!(l[1].slice_atom("width"));
            try!(l2[0].f())
        };
        let layer    = try!(ERes::from_sexp(&l[2]));
        let text     = try!(ERes::from_sexp(&l[3]));
        let feature1 = try!(ERes::from_sexp(try!(l[4].named_value("feature1"))));
        let feature2 = try!(ERes::from_sexp(try!(l[5].named_value("feature2"))));
        let crossbar = try!(ERes::from_sexp(try!(l[6].named_value("crossbar"))));
        let arrow1a = try!(ERes::from_sexp(try!(l[7].named_value("arrow1a"))));
        let arrow1b = try!(ERes::from_sexp(try!(l[8].named_value("arrow1b"))));
        let arrow2a = try!(ERes::from_sexp(try!(l[9].named_value("arrow2a"))));
        let arrow2b = try!(ERes::from_sexp(try!(l[10].named_value("arrow2b"))));
        Ok(Dimension { name:name, width:width, layer:layer, text:text, feature1:feature1, feature2:feature2, crossbar:crossbar, arrow1a:arrow1a, arrow1b:arrow1b, arrow2a:arrow2a, arrow2b:arrow2b })
    }
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
                "layers" => {
                    layout.layers = try!(ERes::from_sexp(&e))
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
                "gr_text" => {
                    let g = try!(wrap(e, ERes::from_sexp, Graphics::GrText));
                    layout.elements.push(Element::Graphics(g))
                },
                "gr_line" => {
                    let g = try!(wrap(e, ERes::from_sexp, Graphics::GrLine));
                    layout.elements.push(Element::Graphics(g))
                },
                "gr_arc" => {
                    let g = try!(wrap(e, ERes::from_sexp, Graphics::GrArc));
                    layout.elements.push(Element::Graphics(g))
                },
                "dimension" => {
                    let g = try!(wrap(e, ERes::from_sexp, Graphics::Dimension));
                    layout.elements.push(Element::Graphics(g))
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
