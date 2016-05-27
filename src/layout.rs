// (c) 2016 Productize SPRL <joost@productize.be>

use std::fmt;
use std::result;

// from parent
use Result;
use str_error as err;
use footprint;
use footprint::FromSexp;
use footprint::wrap;
use Sexp;
use symbolic_expressions;
use str_error;

// TODO: get rid of it
pub fn display_string(s:&str) -> String {
    if s.contains("(") || s.contains(" ") || s.len() == 0 {
        format!("\"{}\"", s)
    } else {
        s.clone()
    }
}

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
    pub tstamp:String,
}

// only used internally
enum GrElement {
    Start(footprint::Xy),
    End(footprint::Xy),
    Angle(i64),
    Layer(footprint::Layer),
    Width(f64),
    TStamp(String),
    At(footprint::At),
    Effects(footprint::Effects),
}

#[derive(Clone)]
pub struct GrLine {
    pub start:footprint::Xy,
    pub end:footprint::Xy,
    pub angle:i64,
    pub layer:footprint::Layer,
    pub width:f64,
    pub tstamp:String,
}

#[derive(Clone)]
pub struct GrArc {
    pub start:footprint::Xy,
    pub end:footprint::Xy,
    pub angle:i64,
    pub layer:footprint::Layer,
    pub width:f64,
    pub tstamp:String,
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
            if let Element::Net(ref net) = *element {
                v.push(net)
            }
        }
        v
    }

    pub fn netclasses(&self) -> Vec<&NetClass> {
        let mut v = vec![];
        for element in &self.elements {
            if let Element::NetClass(ref net_class) = *element {
                v.push(net_class)
            }
        }
        v
    }

    pub fn get_module(&self, reference:&String) -> Option<&footprint::Module> {
        for ref x in &self.elements[..] {
            match **x {
                Element::Module(ref m) => {
                    if m.is_reference(reference) {
                        return Some(&m)
                    }
                },
                Element::Net(_)      => (),
                Element::NetClass(_) => (),
                Element::Other(_)    => (),
                Element::Graphics(_) => (),
            }
        }
        None
    }
    
    pub fn modify_module<F>(&mut self, reference:&String, fun:F) -> Result<()> 
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
        str_error(format!("did not find module with reference {}", reference))
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
        s1 == s2
    }
    pub fn has_net(&self, name:&'static str) -> bool {
        for net in &self.nets {
            if &net[..] == name {
                return true
            }
        }
        false
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
        None
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
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        try!(writeln!(f, "(kicad_pcb (version {}) (host {} \"{}\")", self.version, self.host.tool, self.host.build));
        //let mut i = 0;
        try!(writeln!(f, "  {}", self.general));
        try!(writeln!(f, "  (page {})", self.page));
        try!(writeln!(f, "  {}", self.setup));
        try!(writeln!(f, "(layers "));
        for layer in &self.layers[..] {
            try!(writeln!(f, "  {}", layer));
        }
        try!(writeln!(f, ")"));
        for element in &self.elements[..] {
            try!(writeln!(f, "  {}", element));
            try!(writeln!(f, ""));
            // kludge to put setup at the right order in the file
            //if i == 3 {
            //    try!(writeln!(f, "  {}", self.setup));
            //}
            //i+=1;
        }
        writeln!(f, ")")
    }
}

impl fmt::Display for General {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        try!(writeln!(f, "(general"));
        try!(writeln!(f, "  (links {})", self.links));
        try!(writeln!(f, "  {}", self.area));
        try!(writeln!(f, "  (thickness {})", self.thickness));
        try!(writeln!(f, "  (drawings {})", self.drawings));
        try!(writeln!(f, "  (tracks {})", self.tracks));
        try!(writeln!(f, "  (zones {})", self.zones));
        try!(writeln!(f, "  (modules {})", self.modules));
        try!(writeln!(f, "  (nets {})", self.nets));
        writeln!(f, ")")
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        writeln!(f, "(area {} {} {} {})", self.x1, self.y1, self.x2, self.y2)
    }
}


impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
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
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        if self.name.contains("(") || self.name.contains(")") || self.name.len()==0 {
            write!(f, "(net {} \"{}\")", self.num, self.name)
        } else {
            write!(f, "(net {} {})", self.num, self.name)
        }
    }
}
impl fmt::Display for NetClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        try!(write!(f, "(net_class {} {} ", self.name, display_string(&self.desc)));
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

// (0 F.Cu signal)
impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        if self.hide == false {
            write!(f, "({} {} {})", self.num, self.layer, self.layer_type)
        } else {
            write!(f, "({} {} {} hide)", self.num, self.layer, self.layer_type)
        }
            
    }
}

impl fmt::Display for LayerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let s = match *self {
            LayerType::Signal => "signal",
            LayerType::User => "user",
        };
        write!(f, "{}", s)
    }
}


impl fmt::Display for SetupElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self.value2 {
            Some(ref x) => writeln!(f, "   ({} {} {})", self.name, self.value1, x),
            None => writeln!(f, "   ({} {})", self.name, self.value1),
        }
    }
}

impl fmt::Display for Setup {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        try!(writeln!(f, "(setup"));
        for ref k in &self.elements {
            try!(writeln!(f, "   {}", k));
        }
        try!(writeln!(f, " (pcbplotparams"));
        for ref k in &self.pcbplotparams {
            try!(writeln!(f, "     {}", k));
        }
        writeln!(f, "))")
    }
}

impl fmt::Display for Graphics {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
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
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        try!(writeln!(f,"(gr_text {} {} (layer {})", display_string(&self.value), self.at, self.layer));
        try!(writeln!(f,"    {}", self.effects));
        write!(f,")")
    }
}

impl fmt::Display for GrLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "(gr_line {} {} (angle {}) (layer {}) (width {}))", self.start, self.end, self.angle, self.layer, self.width)
    }
}

impl fmt::Display for GrArc {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "(gr_arc {} {} (angle {}) (layer {}) (width {}))", self.start, self.end, self.angle, self.layer, self.width)
    }
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        try!(writeln!(f, "(dimension {} (width {}) (layer {})", display_string(&self.name), self.width, self.layer));
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

fn parse_version(e:&Sexp) -> Result<i64> {
    let l = try!(e.slice_atom("version"));
    l[0].i().map_err(From::from)
}

fn parse_page(e:&Sexp) -> Result<String> {
    let l = try!(e.slice_atom("page"));
    Ok(try!(l[0].string()).clone())
}

impl FromSexp for Result<Net> {
    fn from_sexp(s:&Sexp) -> Result<Net> {
        let l = try!(s.slice_atom_num("net", 2));
        let num = try!(l[0].i());
        let name = try!(l[1].string()).clone();
        Ok(Net { name:name, num:num })
    }
}

impl FromSexp for Result<Host> {
    fn from_sexp(s:&Sexp) -> Result<Host> {
        let l = try!(s.slice_atom_num("host", 2));
        let tool = try!(l[0].string()).clone();
        let build = try!(l[1].string()).clone();
        Ok(Host { tool:tool, build:build })
    }
}

impl FromSexp for Result<General> {
    fn from_sexp(s:&Sexp) -> Result<General> {
        let l = try!(s.slice_atom_num("general", 9));
        let links = try!(l[0].named_value_i("links"));
        let no_connects = try!(l[1].named_value_i("no_connects"));
        let area = try!(Result::from_sexp(&l[2]));
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

impl FromSexp for Result<Area> {
    fn from_sexp(s:&Sexp) -> Result<Area> {
        let l = try!(s.slice_atom_num("area", 4));
        let x1 = try!(l[0].f());
        let y1 = try!(l[1].f());
        let x2 = try!(l[2].f());
        let y2 = try!(l[3].f());
        Ok(Area { x1:x1, y1:y1, x2:x2, y2:y2 })
    }
}

impl FromSexp for Result<Vec<Layer> > {
    fn from_sexp(s:&Sexp) -> Result<Vec<Layer> > {
        let mut v = vec![];
        let l = try!(s.slice_atom("layers"));
        for x in l {
            let layer = try!(Result::from_sexp(&x));
            v.push(layer)
        }
        Ok(v)
    }
}

impl FromSexp for Result<Layer> {
    fn from_sexp(s:&Sexp) -> Result<Layer> {
        let l = try!(s.list());
        //println!("making layer from {}", s);
        if l.len() != 3 && l.len() != 4 {
            return str_error(format!("expecting 3 or 4 elements in layer: {}", s))
        }
        let num = try!(l[0].i());
        let layer = try!(footprint::Layer::from_string(try!(l[1].string()).clone()));
        let layer_type = try!(Result::from_sexp(&l[2]));
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

impl FromSexp for Result<LayerType> {
    fn from_sexp(s:&Sexp) -> Result<LayerType> {
        let x = try!(s.string());
        match &x[..] {
            "signal" => Ok(LayerType::Signal),
            "user" => Ok(LayerType::User),
            _ => str_error(format!("unknown layertype {} in {}", x, s)),
        }
    }
}

impl FromSexp for Result<SetupElement> {
    fn from_sexp(s:&Sexp) -> Result<SetupElement> {
        let l = try!(s.list());
        if l.len() != 2 && l.len() != 3 {
            return str_error(format!("expecting 2 or 3 elements in setup element: {}", s))
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

impl FromSexp for Result<NetClass> {
    fn from_sexp(s:&Sexp) -> Result<NetClass> {
        fn parse(e:&Sexp, name:&str) -> Result<f64> {
            let l = try!(e.slice_atom(name));
            l[0].f().map_err(From::from)
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
                _ => return str_error(format!("unknown net_class field {}", list_name))
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

impl FromSexp for Result<Setup> {
    fn from_sexp(s:&Sexp) -> Result<Setup> {
        let mut elements = vec![];
        let mut pcbplotparams = vec![];
        for v in try!(s.slice_atom("setup")) {
            let n = v.list_name().unwrap().clone();
            match &n[..] {
                "pcbplotparams" => {
                    for y in try!(v.slice_atom("pcbplotparams")) {
                        let p_e = try!(Result::from_sexp(&y));
                        pcbplotparams.push(p_e)
                    }
                },
                _ => {
                    let setup_element = try!(Result::from_sexp(&v));
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

impl FromSexp for Result<GrText> {
    fn from_sexp(s:&Sexp) -> Result<GrText> {
        let l = try!(s.slice_atom("gr_text"));
        let value = try!(l[0].string()).clone();
        let mut layer = footprint::Layer::new();
        let mut tstamp = String::from("");
        let mut at = footprint::At::new_empty();
        let mut effects = footprint::Effects::new();
        for x in &l[1..] {
            let elem = try!(Result::from_sexp(x));
            match elem {
                GrElement::At(x) => at = x,
                GrElement::Layer(x) => layer = x,
                GrElement::TStamp(x) => tstamp = x,
                GrElement::Effects(x) => effects = x,
                _ => (), // TODO
            }
        }
        Ok(GrText { value:value, at:at, layer:layer, effects:effects, tstamp:tstamp, })
    }
}

impl FromSexp for Result<GrElement> {
    fn from_sexp(s:&Sexp) -> Result<GrElement> {
        match &try!(s.list_name())[..] {
            "start" => wrap(s, Result::from_sexp, GrElement::Start),
            "end" => wrap(s, Result::from_sexp, GrElement::End),
            "angle" => {
                let l2 = try!(s.slice_atom("angle"));
                Ok(GrElement::Angle(try!(l2[0].i())))
            },
            "layer" => wrap(s, Result::from_sexp, GrElement::Layer),
            "width" => {
                let l2 = try!(s.slice_atom("width"));
                Ok(GrElement::Width(try!(l2[0].f())))
            },
            "tstamp" => {
                let l2 = try!(s.slice_atom("tstamp"));
                let sx = try!(l2[0].string()).clone();
                Ok(GrElement::TStamp(sx))
            },
            "at" => wrap(s, Result::from_sexp, GrElement::At),
            "effects" => wrap(s, Result::from_sexp, GrElement::Effects),
            x => {
                str_error(format!("unknown element {} in {}", x, s))
            }
        }
    }
}


impl FromSexp for Result<GrLine> {
    fn from_sexp(s:&Sexp) -> Result<GrLine> {
        //println!("GrLine: {}", s);
        let l = try!(s.slice_atom("gr_line"));
        let mut start = footprint::Xy::new_empty(footprint::XyType::Start);
        let mut end = footprint::Xy::new_empty(footprint::XyType::End);
        let mut angle = 0;
        let mut layer = footprint::Layer::new();
        let mut width = 0.0_f64;
        let mut tstamp = String::from("");
        for x in l {
            let elem = try!(Result::from_sexp(x));
            match elem {
                GrElement::Start(x) => start = x,
                GrElement::End(x) => end = x,
                GrElement::Angle(x) => angle = x,
                GrElement::Layer(x) => layer = x,
                GrElement::TStamp(x) => tstamp = x,
                GrElement::Width(x) => width = x,
                _ => (), // TODO
            }
        }
        Ok(GrLine { start:start, end:end, angle:angle, layer:layer, width:width, tstamp:tstamp })
    }
}

impl FromSexp for Result<GrArc> {
    fn from_sexp(s:&Sexp) -> Result<GrArc> {
        let l = try!(s.slice_atom("gr_arc"));
        let mut start = footprint::Xy::new_empty(footprint::XyType::Start);
        let mut end = footprint::Xy::new_empty(footprint::XyType::End);
        let mut angle = 0;
        let mut layer = footprint::Layer::new();
        let mut width = 0.0_f64;
        let mut tstamp = String::from("");
        for x in l {
            let elem = try!(Result::from_sexp(x));
            match elem {
                GrElement::Start(x) => start = x,
                GrElement::End(x) => end = x,
                GrElement::Angle(x) => angle = x,
                GrElement::Layer(x) => layer = x,
                GrElement::TStamp(x) => tstamp = x,
                GrElement::Width(x) => width = x,
                _ => (), // TODO
            }
        }
        Ok(GrArc { start:start, end:end, angle:angle, layer:layer, width:width, tstamp:tstamp })
    }
}

impl FromSexp for Result<Dimension> {
    fn from_sexp(s:&Sexp) -> Result<Dimension> {
        let l = try!(s.slice_atom_num("dimension", 11));
        let name = try!(l[0].string()).clone();
        let width = {
            let l2 = try!(l[1].slice_atom("width"));
            try!(l2[0].f())
        };
        let layer    = try!(Result::from_sexp(&l[2]));
        let text     = try!(Result::from_sexp(&l[3]));
        let feature1 = try!(Result::from_sexp(try!(l[4].named_value("feature1"))));
        let feature2 = try!(Result::from_sexp(try!(l[5].named_value("feature2"))));
        let crossbar = try!(Result::from_sexp(try!(l[6].named_value("crossbar"))));
        let arrow1a = try!(Result::from_sexp(try!(l[7].named_value("arrow1a"))));
        let arrow1b = try!(Result::from_sexp(try!(l[8].named_value("arrow1b"))));
        let arrow2a = try!(Result::from_sexp(try!(l[9].named_value("arrow2a"))));
        let arrow2b = try!(Result::from_sexp(try!(l[10].named_value("arrow2b"))));
        Ok(Dimension { name:name, width:width, layer:layer, text:text, feature1:feature1, feature2:feature2, crossbar:crossbar, arrow1a:arrow1a, arrow1b:arrow1b, arrow2a:arrow2a, arrow2b:arrow2b })
    }
}

impl FromSexp for Result<Layout> {
    fn from_sexp(s:&Sexp) -> Result<Layout> {
        let l1 = try!(s.slice_atom("kicad_pcb"));
        let mut layout = Layout::new();
        for ref e in l1 {
            match &try!(e.list_name())[..] {
                "version" => {
                    layout.version = try!(parse_version(e))
                },
                "host" => {
                    layout.host = try!(Result::from_sexp(&e))
                },
                "general" => {
                    layout.general = try!(Result::from_sexp(&e))
                },
                "page" => {
                    layout.page = try!(parse_page(&e))
                },
                "layers" => {
                    layout.layers = try!(Result::from_sexp(&e))
                },
                "module" => {
                    let module = try!(wrap(e, Result::from_sexp, Element::Module));
                    layout.elements.push(module)
                },
                "net" => {
                    let net = try!(wrap(e, Result::from_sexp, Element::Net));
                    layout.elements.push(net)
                },
                "net_class" => {
                    let nc = try!(wrap(e, Result::from_sexp, Element::NetClass));
                    layout.elements.push(nc)
                },
                "gr_text" => {
                    let g = try!(wrap(e, Result::from_sexp, Graphics::GrText));
                    layout.elements.push(Element::Graphics(g))
                },
                "gr_line" => {
                    let g = try!(wrap(e, Result::from_sexp, Graphics::GrLine));
                    layout.elements.push(Element::Graphics(g))
                },
                "gr_arc" => {
                    let g = try!(wrap(e, Result::from_sexp, Graphics::GrArc));
                    layout.elements.push(Element::Graphics(g))
                },
                "dimension" => {
                    let g = try!(wrap(e, Result::from_sexp, Graphics::Dimension));
                    layout.elements.push(Element::Graphics(g))
                },
                "setup" => {
                    layout.setup = try!(Result::from_sexp(&e))
                },
                _ => {
                    //println!("unimplemented: {}", e);
                    layout.elements.push(parse_other(e))
                },
            }
        }
        Ok(layout)
    }
}


pub fn parse(s: &str) -> Result<Layout> {
    match symbolic_expressions::parser::parse_str(s) {
        Ok(s) => Result::from_sexp(&s),
        Err(x) => str_error(format!("ParseError: {:?}", x)),
    }
}
