// (c) 2016 Productize SPRL <joost@productize.be>

// from parent
use Result;
use str_error;
use footprint;
use Sexp;

/// a Kicad layout
#[derive(Debug)]
pub struct Layout {
    /// version of file
    pub version: i64,
    /// build-host
    pub host: Host,
    /// general information
    pub general: General,
    /// page size
    pub page: String,
    /// setup information
    pub setup: Setup,
    /// layers
    pub layers: Vec<Layer>,
    /// layout elements
    pub elements: Vec<Element>,
}

/// layout elements
#[derive(Clone,Debug)]
pub enum Element {
    /// module
    Module(footprint::Module),
    /// net
    Net(Net),
    /// netclass
    NetClass(NetClass),
    ///  text
    GrText(GrText),
    /// line
    GrLine(GrLine),
    /// arc
    GrArc(GrArc),
    /// dimension
    Dimension(Dimension),
    // Segment,
    // Via,
    // FilledPolygon,
    /// zone
    Zone(Zone),
    /// other (uninterpreted symbolic-expression)
    Other(Sexp),
}

/// a zone region
#[derive(Clone,Debug)]
pub struct Zone {
    /// net number of the zone
    pub net: i64,
    /// net name of the zone
    pub net_name: String,
    /// other (uninterpreted symbolic-expressions)
    pub other: Vec<Sexp>,
}

/// build host info
#[derive(Clone,Debug)]
pub struct Host {
    /// tool name
    pub tool: String,
    /// tool version
    pub build: String,
}

/// general information
#[derive(Clone,Debug)]
pub struct General {
    /// number of links
    pub links: i64,
    /// number of no-connect
    pub no_connects: i64,
    /// area of layout
    pub area: Area,
    /// thickness
    pub thickness: f64,
    /// number of drawings
    pub drawings: i64,
    /// number of tracks
    pub tracks: i64,
    /// number of zones
    pub zones: i64,
    /// number of moduls
    pub modules: i64,
    /// number of nets
    pub nets: i64,
}

/// area
#[derive(Clone,Debug,Default)]
pub struct Area {
    // X1 coordinate
    pub x1: f64,
    // Y1 coordinate
    pub y1: f64,
    // X2 coordinate
    pub x2: f64,
    // Y2 coordinate
    pub y2: f64,
}

/// layer
#[derive(Clone,Debug)]
pub struct Layer {
    /// layer number
    pub num: i64,
    /// layer
    pub layer: footprint::Layer,
    /// layer type
    pub layer_type: LayerType,
    /// if the layer is shown
    pub hide: bool,
}

/// layer type
#[derive(Clone,Debug)]
pub enum LayerType {
    /// signal layer
    Signal,
    /// user layer
    User,
}

#[derive(Clone,Debug,Default)]
pub struct Setup {
    pub elements: Vec<SetupElement>,
    pub pcbplotparams: Vec<SetupElement>,
}

#[derive(Clone,Debug)]
pub struct SetupElement {
    pub name: String,
    pub value1: String,
    pub value2: Option<String>,
}


#[derive(Clone,PartialEq,Debug)]
pub struct Net {
    pub num: i64,
    pub name: String,
}

#[derive(Clone,PartialEq,Debug)]
pub struct NetClass {
    pub name: String,
    pub desc: String,
    pub clearance: f64,
    pub trace_width: f64,
    pub via_dia: f64,
    pub via_drill: f64,
    pub uvia_dia: f64,
    pub uvia_drill: f64,
    pub nets: Vec<String>,
}

#[derive(Clone,Debug)]
pub struct GrText {
    pub value: String,
    pub at: footprint::At,
    pub layer: footprint::Layer,
    pub effects: footprint::Effects,
    pub tstamp: String,
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
    pub start: footprint::Xy,
    pub end: footprint::Xy,
    pub angle: i64,
    pub layer: footprint::Layer,
    pub width: f64,
    pub tstamp: String,
}

#[derive(Clone,Debug)]
pub struct GrArc {
    pub start: footprint::Xy,
    pub end: footprint::Xy,
    pub angle: i64,
    pub layer: footprint::Layer,
    pub width: f64,
    pub tstamp: String,
}

#[derive(Clone,Debug)]
pub struct Dimension {
    pub name: String,
    pub width: f64,
    pub layer: footprint::Layer,
    pub tstamp: Option<String>,
    pub text: GrText,
    pub feature1: footprint::Pts,
    pub feature2: footprint::Pts,
    pub crossbar: footprint::Pts,
    pub arrow1a: footprint::Pts,
    pub arrow1b: footprint::Pts,
    pub arrow2a: footprint::Pts,
    pub arrow2b: footprint::Pts,
}

impl Default for Layout {
    fn default() -> Layout {
        Layout {
            version: 4,
            host: Host::default(),
            elements: vec![],
            setup: Setup::default(),
            general: General::default(),
            page: String::from("A4"),
            layers: vec![],
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

    pub fn change_net_name(&mut self, old_name: &str, new_name: &str) {
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
        if !found {
            return;
        }
        for element in &mut self.elements {
            // 2. change net name in net_class (add_net)
            if let Element::NetClass(ref mut net_class) = *element {
                let mut not_old: Vec<String> =
                    net_class.nets.iter().filter(|&x| &x[..] != old_name).cloned().collect();
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

    pub fn get_module(&self, reference: &str) -> Option<&footprint::Module> {
        for ref x in &self.elements[..] {
            match **x {
                Element::Module(ref m) => {
                    if m.is_reference_with_name(reference) {
                        return Some(m);
                    }
                }
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

    pub fn modify_module<F>(&mut self, reference: &str, fun: F) -> Result<()>
        where F: Fn(&mut footprint::Module) -> ()
    {
        for ref mut x in &mut self.elements {
            match **x {
                Element::Module(ref mut m) => {
                    if m.is_reference_with_name(reference) {
                        return Ok(fun(m));
                    }
                }
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

    pub fn add_net(&mut self, num: i64, name: &'static str) {
        self.elements.push(Element::Net(Net {
            num: num,
            name: String::from(name),
        }));
    }
    pub fn add_netclass(&mut self,
                        name: &'static str,
                        desc: &'static str,
                        clearance: f64,
                        trace_width: f64,
                        via_dia: f64,
                        via_drill: f64,
                        uvia_dia: f64,
                        uvia_drill: f64,
                        nets: Vec<String>) {
        self.elements.push(Element::NetClass(NetClass {
            name: String::from(name),
            desc: String::from(desc),
            clearance: clearance,
            trace_width: trace_width,
            via_dia: via_dia,
            via_drill: via_drill,
            uvia_dia: uvia_dia,
            uvia_drill: uvia_drill,
            nets: nets,
        }));
    }
}

impl Default for Host {
    fn default() -> Host {
        Host {
            tool: String::from("pcbnew"),
            build: String::from("custom"),
        }
    }
}

impl Default for General {
    fn default() -> General {
        General {
            links: 0,
            no_connects: 0,
            area: Area::default(),
            thickness: 1.6,
            drawings: 0,
            tracks: 0,
            zones: 0,
            modules: 0,
            nets: 0,
        }
    }
}

impl NetClass {
    pub fn equal_no_net(&self, other: &NetClass) -> bool {
        let mut s1 = self.clone();
        s1.nets = vec![];
        let mut s2 = other.clone();
        s2.nets = vec![];
        s1 == s2
    }
    pub fn has_net(&self, name: &'static str) -> bool {
        for net in &self.nets {
            if &net[..] == name {
                return true;
            }
        }
        false
    }
}

impl Setup {
    pub fn get(&self, s: &str) -> Option<&String> {
        for element in &self.elements {
            if &element.name[..] == &s[..] {
                return Some(&element.value1);
            }
        }
        None
    }

    pub fn update_element(&mut self, name: &'static str, value: String) {
        for element in &mut self.elements {
            if &element.name[..] == name {
                element.value1 = value;
                return;

            }
        }
        self.elements.push(SetupElement {
            name: String::from(name),
            value1: value,
            value2: None,
        })
    }
}
