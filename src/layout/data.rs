// (c) 2016 Productize SPRL <joost@productize.be>

use std::result;

// from parent
use Result;
use str_error;
use footprint;
use footprint::FromSexp;
use footprint::wrap;
use Sexp;
use symbolic_expressions;

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
    pub x1:f64, pub y1:f64,
    pub x2:f64, pub y2:f64,
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
pub enum GrElement {
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

    pub fn get_module(&self, reference:&str) -> Option<&footprint::Module> {
        for ref x in &self.elements[..] {
            match **x {
                Element::Module(ref m) => {
                    if m.is_reference(reference) {
                        return Some(m)
                    }
                },
                Element::Net(_) |
                Element::NetClass(_) |
                Element::Other(_) |
                Element::Graphics(_) => (),
            }
        }
        None
    }
    
    pub fn modify_module<F>(&mut self, reference:&str, fun:F) -> Result<()> 
        where F:Fn(&mut footprint::Module) -> ()
    {
        for ref mut x in &mut self.elements[..] {
            match **x {
                Element::Module(ref mut m) => {
                    if m.is_reference(reference) {
                        return Ok(fun(m))
                    }
                },
                Element::Net(_) |
                Element::NetClass(_) |
                Element::Other(_) |
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
    pub fn new() -> General {
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
    pub fn new() -> Area {
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
    pub fn get(&self, s:&str) -> Option<&String> {
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
