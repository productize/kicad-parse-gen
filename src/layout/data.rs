// (c) 2016 Productize SPRL <joost@productize.be>

// from parent
use Result;
use str_error;
use footprint;
use Sexp;

#[derive(Debug)]
pub struct Layout {
    pub version:i64,
    pub host:Host,
    pub general:General,
    pub page:String,
    pub setup:Setup,
    pub layers:Vec<Layer>,
    pub elements:Vec<Element>,
}

#[derive(Clone,Debug)]
pub enum Element {
    Module(footprint::Module),
    Net(Net),
    NetClass(NetClass),
    
    GrText(GrText),
    GrLine(GrLine),
    GrArc(GrArc),
    Dimension(Dimension),
    //Segment,
    //Via,
    //FilledPolygon,
    Zone(Zone),
    Other(Sexp),
}

#[derive(Clone,Debug)]
pub struct Zone {
    pub net:i64,
    pub net_name:String,
    pub other:Vec<Sexp>,
}

#[derive(Clone,Debug)]
pub struct Host {
    pub tool:String,
    pub build:String,
}

#[derive(Clone,Debug)]
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

#[derive(Clone,Debug,Default)]
pub struct Area {
    pub x1:f64, pub y1:f64,
    pub x2:f64, pub y2:f64,
}

#[derive(Clone,Debug)]
pub struct Layer {
    pub num:i64,
    pub layer:footprint::Layer,
    pub layer_type:LayerType,
    pub hide:bool,
}

#[derive(Clone,Debug)]
pub enum LayerType {
    Signal,
    User,
}

#[derive(Clone,Debug,Default)]
pub struct Setup {
    pub elements:Vec<SetupElement>,
    pub pcbplotparams:Vec<SetupElement>,
}

#[derive(Clone,Debug)]
pub struct SetupElement {
    pub name:String,
    pub value1:String,
    pub value2:Option<String>,
}


#[derive(Clone,PartialEq,Debug)]
pub struct Net {
    pub num:i64,
    pub name:String,
}

#[derive(Clone,PartialEq,Debug)]
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

#[derive(Clone,Debug)]
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

#[derive(Clone,Debug)]
pub struct GrLine {
    pub start:footprint::Xy,
    pub end:footprint::Xy,
    pub angle:i64,
    pub layer:footprint::Layer,
    pub width:f64,
    pub tstamp:String,
}

#[derive(Clone,Debug)]
pub struct GrArc {
    pub start:footprint::Xy,
    pub end:footprint::Xy,
    pub angle:i64,
    pub layer:footprint::Layer,
    pub width:f64,
    pub tstamp:String,
}

#[derive(Clone,Debug)]
pub struct Dimension {
    pub name:String,
    pub width:f64,
    pub layer:footprint::Layer,
    pub tstamp:Option<String>,
    pub text:GrText,
    pub feature1:footprint::Pts,
    pub feature2:footprint::Pts,
    pub crossbar:footprint::Pts,
    pub arrow1a:footprint::Pts,
    pub arrow1b:footprint::Pts,
    pub arrow2a:footprint::Pts,
    pub arrow2b:footprint::Pts,
}

impl Default for Layout {
    fn default() -> Layout {
        Layout {
            version:4,
            host:Host::default(),
            elements:vec![],
            setup:Setup::default(),
            general:General::default(),
            page:String::from("A4"),
            layers:vec![],
        }
    }
}

impl Layout {
    pub fn nets(&self) -> Vec<&Net> {
        let mut v = vec![];
        for element in &self.elements {
            if let Element::Net(ref net) = *element {
                v.push(net)
            }
        }
        v
    }

    pub fn change_net_name(&mut self, old_name:&str, new_name:&str) {
        // 1. change name in list of nets
        let mut found = false;
        for element in &mut self.elements {
            if let Element::Net(ref mut net) = *element {
                if &net.name == old_name {
                    found = true;
                    net.name.clear();
                    net.name.push_str(new_name);
                }
            }
        }
        if !found { return }
        for element in &mut self.elements {
            // 2. change net name in net_class (add_net)
            if let Element::NetClass(ref mut net_class) = *element {
                let mut not_old:Vec<String> = net_class.nets.iter().filter(|&x| &x[..] != old_name).cloned().collect();
                if not_old.len() < net_class.nets.len() {
                    // net was in the net list of the net_class
                    not_old.push(new_name.to_string());
                    net_class.nets = not_old;
                }
            }
            // 3. change net name in pads in modules
            else if let Element::Module(ref mut module) = *element {
                module.rename_net(old_name, new_name)
            }
            // 4. change net name in zones (net_name)
            else if let Element::Zone(ref mut zone) = *element {
                if &zone.net_name == old_name {
                    zone.net_name = new_name.to_string()
                }
            }
        }
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
                    if m.is_reference_with_name(reference) {
                        return Some(m)
                    }
                },
                Element::Net(_) |
                Element::NetClass(_) |
                Element::GrText(_) |
                Element::GrLine(_) |
                Element::GrArc(_) |
                Element::Dimension(_) |
                Element::Zone(_) |
                Element::Other(_) => (),
            }
        }
        None
    }
    
    pub fn modify_module<F>(&mut self, reference:&str, fun:F) -> Result<()> 
        where F:Fn(&mut footprint::Module) -> ()
    {
        for ref mut x in &mut self.elements {
            match **x {
                Element::Module(ref mut m) => {
                    if m.is_reference_with_name(reference) {
                        return Ok(fun(m))
                    }
                },
                Element::Net(_) |
                Element::NetClass(_) |
                Element::GrText(_) |
                Element::GrLine(_) |
                Element::GrArc(_) |
                Element::Dimension(_) |
                Element::Zone(_) |
                Element::Other(_) => (),
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

impl Default for Host {
    fn default() -> Host {
        Host { tool:String::from("pcbnew"), build:String::from("custom") }
    }
}

impl Default for General {
    fn default() -> General {
        General {
            links:0,
            no_connects:0,
            area:Area::default(),
            thickness:1.6,
            drawings:0,
            tracks:0,
            zones:0,
            modules:0,
            nets:0,
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
