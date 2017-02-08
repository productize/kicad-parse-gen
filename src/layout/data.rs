// (c) 2016-2017 Productize SPRL <joost@productize.be>

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
    /// segment
    Segment(Segment),
    /// via
    Via(Via),
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
    /// X1 coordinate
    pub x1: f64,
    /// Y1 coordinate
    pub y1: f64,
    /// X2 coordinate
    pub x2: f64,
    /// Y2 coordinate
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

/// setup elements of the layout
#[derive(Clone,Debug,Default)]
pub struct Setup {
    /// the setup elements
    pub elements: Vec<SetupElement>,
    /// the pcb plot elements
    pub pcbplotparams: Vec<SetupElement>,
}

/// a generic setup element
#[derive(Clone,Debug)]
pub struct SetupElement {
    /// a name
    pub name: String,
    /// a first value
    pub value1: String,
    /// an optional second value
    pub value2: Option<String>,
}


/// a net
#[derive(Clone,PartialEq,Debug)]
pub struct Net {
    /// net number
    pub num: i64,
    /// net name
    pub name: String,
}

/// a net class
#[derive(Clone,PartialEq,Debug)]
pub struct NetClass {
    /// name
    pub name: String,
    /// description
    pub desc: String,
    /// clearance
    pub clearance: f64,
    /// trace width
    pub trace_width: f64,
    /// via diameter
    pub via_dia: f64,
    /// via drill
    pub via_drill: f64,
    /// micro via diameter
    pub uvia_dia: f64,
    /// micro via drill
    pub uvia_drill: f64,
    /// differential pair gap
    pub diff_pair_gap: Option<f64>,
    /// differential pair width
    pub diff_pair_width: Option<f64>,
    /// associated nets
    pub nets: Vec<String>,
}

/// text
#[derive(Clone,Debug)]
pub struct GrText {
    /// text
    pub value: String,
    /// location
    pub at: footprint::At,
    /// layer
    pub layer: footprint::Layer,
    /// text effects
    pub effects: footprint::Effects,
    /// timestamp
    pub tstamp: String,
}

/// line
#[derive(Clone,Debug)]
pub struct GrLine {
    /// start point
    pub start: footprint::Xy,
    /// end point
    pub end: footprint::Xy,
    /// angle
    pub angle: f64,
    /// layer
    pub layer: footprint::Layer,
    /// width
    pub width: f64,
    /// time stamp
    pub tstamp: String,
}

/// arc
#[derive(Clone,Debug)]
pub struct GrArc {
    /// start point
    pub start: footprint::Xy,
    /// end point
    pub end: footprint::Xy,
    /// angle
    pub angle: f64,
    /// layer
    pub layer: footprint::Layer,
    /// width
    pub width: f64,
    /// timestamp
    pub tstamp: String,
}

/// dimension
#[derive(Clone,Debug)]
pub struct Dimension {
    /// name
    pub name: String,
    /// width
    pub width: f64,
    /// layer
    pub layer: footprint::Layer,
    /// time stamp
    pub tstamp: Option<String>,
    /// text
    pub text: GrText,
    /// feature1
    pub feature1: footprint::Pts,
    /// feature2
    pub feature2: footprint::Pts,
    /// crossbar
    pub crossbar: footprint::Pts,
    /// arrow1a
    pub arrow1a: footprint::Pts,
    /// arrow1b
    pub arrow1b: footprint::Pts,
    /// arrow2a
    pub arrow2a: footprint::Pts,
    /// arrow2b
    pub arrow2b: footprint::Pts,
}

/// segment
// (segment (start 117.5548 123.4602) (end 118.3848 122.6302) (width 0.2032) (layer B.Cu) (net 0) (tstamp 55E99398))
#[derive(Clone,Debug)]
pub struct Segment {
    /// start point
    pub start: footprint::Xy,
    /// end point
    pub end: footprint::Xy,
    /// width
    pub width: f64,
    /// layer
    pub layer: footprint::Layer,
    /// net
    pub net: i64,
    /// tstamp
    pub tstamp: Option<String>,
    /// status
    pub status: Option<String>,
}

/// via
// (via (at 132.1948 121.2202) (size 0.675) (drill 0.25) (layers F.Cu B.Cu) (net 19))
#[derive(Clone,Debug)]
pub struct Via {
    /// at
    pub at: footprint::At,
    /// size
    pub size: f64,
    /// drill
    pub drill: f64,
    /// layers
    pub layers: footprint::Layers,
    /// net
    pub net: i64,
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
    /// get lists of nets
    pub fn nets(&self) -> Vec<&Net> {
        let mut v = vec![];
        for element in &self.elements {
            if let Element::Net(ref net) = *element {
                v.push(net)
            }
        }
        v
    }

    /// change net name
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

    /// get list of netclasses
    pub fn netclasses(&self) -> Vec<&NetClass> {
        let mut v = vec![];
        for element in &self.elements {
            if let Element::NetClass(ref net_class) = *element {
                v.push(net_class)
            }
        }
        v
    }

    /// get module
    pub fn get_module(&self, reference: &str) -> Option<&footprint::Module> {
        for ref x in &self.elements[..] {
            match **x {
                Element::Module(ref m) => {
                    if m.is_reference_with_name(reference) {
                        return Some(m);
                    }
                }
                Element::Via(_) |
                Element::Segment(_) |
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

    /// get list of contained modules
    pub fn get_modules(&self) -> Vec<&footprint::Module> {
        let mut v = vec![];
        for e in &self.elements {
            match *e {
                Element::Module(ref m) => {
                    v.push(m)
                }
                _ => ()
            }
        }
        v
    }
    
    /// modify a module
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
                Element::Via(_) |
                Element::Segment(_) |
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

    /// add a net
    pub fn add_net(&mut self, num: i64, name: &'static str) {
        self.elements.push(Element::Net(Net {
            num: num,
            name: String::from(name),
        }));
    }

    /// add a net class
    pub fn add_netclass(&mut self,
                        name: &'static str,
                        desc: &'static str,
                        clearance: f64,
                        trace_width: f64,
                        via_dia: f64,
                        via_drill: f64,
                        uvia_dia: f64,
                        uvia_drill: f64,
                        diff_pair_gap: Option<f64>,
                        diff_pair_width: Option<f64>,
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
            diff_pair_gap: diff_pair_gap,
            diff_pair_width: diff_pair_width,
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
    /// check if two netclasses are equal not looking at the nets
    pub fn equal_no_net(&self, other: &NetClass) -> bool {
        let mut s1 = self.clone();
        s1.nets = vec![];
        let mut s2 = other.clone();
        s2.nets = vec![];
        s1 == s2
    }

    /// check if a netclass has a net
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
    /// get a setup element
    pub fn get(&self, s: &str) -> Option<&String> {
        for element in &self.elements {
            if &element.name[..] == &s[..] {
                return Some(&element.value1);
            }
        }
        None
    }

    /// update a setup element
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
