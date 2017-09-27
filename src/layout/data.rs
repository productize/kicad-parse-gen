// (c) 2016-2017 Productize SPRL <joost@productize.be>

// from parent
use Result;
use str_error;
use footprint;
use Sexp;
use layout::{Adjust, Bound, BoundingBox};
use std::{fmt, result};

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
#[derive(Clone, Debug)]
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
    /// circle
    GrCircle(GrCircle),
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
#[derive(Clone, Debug)]
pub struct Zone {
    /// net number of the zone
    pub net: i64,
    /// net name of the zone
    pub net_name: NetName,
    /// layer
    pub layer: footprint::Layer,
    /// tstamp
    pub tstamp: String,
    /// hatch
    pub hatch: Hatch,
    /// priority
    pub priority: u64,
    /// connect pads
    pub connect_pads: ConnectPads,
    /// minimum thickness
    pub min_thickness: f64,
    /// keepout
    pub keepout: Option<Keepout>,
    /// fill
    pub fill: Fill,
    /// polygons
    pub polygons: Vec<footprint::Pts>,
    /// filled polygons
    pub filled_polygons: Vec<footprint::Pts>,
    /// filled segments
    pub fill_segments: Option<footprint::Pts>,
    /// other (uninterpreted symbolic-expressions)
    pub other: Vec<Sexp>,
}

impl Adjust for Zone {
    fn adjust(&mut self, x: f64, y: f64) {
        for p in &mut self.polygons {
            p.adjust(x, y)
        }
        for p in &mut self.filled_polygons {
            p.adjust(x, y)
        }
        for p in &mut self.fill_segments {
            p.adjust(x, y)
        }
    }
}

impl BoundingBox for Zone {
    fn bounding_box(&self) -> Bound {
        let mut b = Bound::default();
        for p in &self.polygons {
            b.update(&p.bounding_box());
        }
        for p in &self.filled_polygons {
            b.update(&p.bounding_box());
        }
        for p in &self.fill_segments {
            b.update(&p.bounding_box());
        }
        b.swap_if_needed();
        b
    }
}

/// a zone hatch
#[derive(Clone, Debug, Default)]
pub struct Hatch {
    /// hatching style
    pub style: String,
    /// hatching pitch
    pub pitch: f64,
}

/// a zone connect pads
#[derive(Clone, Debug, Default)]
pub struct ConnectPads {
    /// connection type
    pub connection: Option<String>,
    /// clearance
    pub clearance: f64,
}

/// keepout of a zone
//  (keepout (tracks not_allowed) (vias not_allowed) (copperpour allowed))
#[derive(Clone, Debug, Default)]
pub struct Keepout {
    /// tracks
    pub tracks: bool,
    /// vias
    pub vias: bool,
    /// copperpour
    pub copperpour: bool,
}

/// fill of a zone
//  (fill yes (arc_segments 16) (thermal_gap 0.508) (thermal_bridge_width 0.508))
#[derive(Clone, Debug, Default)]
pub struct Fill {
    /// if it is filled (default no)
    pub filled: bool,
    /// if it is segment mode
    pub segment: bool,
    /// number of arc segments
    pub arc_segments: i64,
    /// thermal relief gap
    pub thermal_gap: f64,
    /// thermal relief copper bridge
    pub thermal_bridge_width: f64,
    /// smoothing
    pub smoothing: Option<String>,
    /// corner radius
    pub corner_radius: f64,
}

/// build host info
#[derive(Clone, Debug)]
pub struct Host {
    /// tool name
    pub tool: String,
    /// tool version
    pub build: String,
}

/// general information
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug, Default)]
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
#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub enum LayerType {
    /// signal layer
    Signal,
    /// power layer
    Power,
    /// mixed layer
    Mixed,
    /// jumper layer
    Jumper,
    /// user layer
    User,
}

/// setup elements of the layout
#[derive(Clone, Debug, Default)]
pub struct Setup {
    /// the setup elements
    pub elements: Vec<SetupElement>,
    /// the pcb plot elements
    pub pcbplotparams: Vec<SetupElement>,
}

/// a generic setup element
#[derive(Clone, Debug)]
pub struct SetupElement {
    /// a name
    pub name: String,
    /// a first value
    pub value1: String,
    /// an optional second value
    pub value2: Option<String>,
}

/// a netname
#[derive(Clone, Debug, PartialEq, Default)]
pub struct NetName(pub String);

impl<'a> From<&'a str> for NetName {
    fn from(s: &'a str) -> NetName {
        NetName(s.into())
    }
}

impl From<String> for NetName {
    fn from(s: String) -> NetName {
        NetName(s)
    }
}

impl NetName {
    /// replace the block_name in a net name
    pub fn replace_block(&mut self, from: &str, to: &str) {
        let from_pat = format!("/{}/", from);
        let to_pat = format!("/{}/", to);
        let n = self.0.replace(&from_pat, &to_pat);
        self.0 = n;
    }

    /// rename a net
    pub fn rename(&mut self, from: &str, to: &str) {
        if self.0 == from {
            self.0 = to.into();
        }
    }
}

/// a net
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Net {
    /// net number
    pub num: i64,
    /// net name
    pub name: NetName,
}

/// a net class
#[derive(Clone, PartialEq, Debug)]
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
    pub nets: Vec<NetName>,
}

/// text
#[derive(Clone, Debug, Default)]
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
    pub tstamp: Option<String>,
}

impl Adjust for GrText {
    fn adjust(&mut self, x: f64, y: f64) {
        self.at.adjust(x, y)
    }
}

impl BoundingBox for GrText {
    fn bounding_box(&self) -> Bound {
        debug!("poor bounding box for GrText");
        Bound::new(self.at.x, self.at.y, self.at.x, self.at.y)
    }
}

/// line
#[derive(Clone, Debug, Default)]
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
    pub tstamp: Option<String>,
}

impl Adjust for GrLine {
    fn adjust(&mut self, x: f64, y: f64) {
        self.start.adjust(x, y);
        self.end.adjust(x, y)
    }
}

impl BoundingBox for GrLine {
    fn bounding_box(&self) -> Bound {
        let x1 = self.start.x.min(self.end.x);
        let y1 = self.start.y.min(self.end.y);
        let x2 = self.start.x.max(self.end.x);
        let y2 = self.start.y.max(self.end.y);
        Bound::new(x1, y1, x2, y2)
    }
}

/// arc
#[derive(Clone, Debug, Default)]
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
    pub tstamp: Option<String>,
}

impl Adjust for GrArc {
    fn adjust(&mut self, x: f64, y: f64) {
        self.start.adjust(x, y);
        self.end.adjust(x, y);
    }
}

impl BoundingBox for GrArc {
    fn bounding_box(&self) -> Bound {
        let x1 = self.start.x.min(self.end.x);
        let y1 = self.start.y.min(self.end.y);
        let x2 = self.start.x.max(self.end.x);
        let y2 = self.start.y.max(self.end.y);
        Bound::new(x1, y1, x2, y2)
    }
}

/// `gr_circle`
// (gr_circle (center 178.6 68.8) (end 176.1 68.7) (layer Eco2.User) (width 0.2))
#[derive(Clone, Debug, Default)]
pub struct GrCircle {
    /// center point
    pub center: footprint::Xy,
    /// end point
    pub end: footprint::Xy,
    /// layer
    pub layer: footprint::Layer,
    /// width
    pub width: f64,
    /// timestamp
    pub tstamp: Option<String>,
}

impl Adjust for GrCircle {
    fn adjust(&mut self, x: f64, y: f64) {
        self.center.x += x;
        self.center.y += y;
        self.end.x += x;
        self.end.y += y;
    }
}

impl BoundingBox for GrCircle {
    fn bounding_box(&self) -> Bound {
        let r = (self.center.x - self.end.x) * (self.center.x - self.end.x) +
            (self.center.y - self.end.y) * (self.center.y - self.end.y);
        let r = r.sqrt();
        Bound::new(
            self.center.x - r,
            self.center.y - r,
            self.center.x + r,
            self.center.y + r,
        )
    }
}

/// dimension
#[derive(Clone, Debug, Default)]
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

impl Adjust for Dimension {
    fn adjust(&mut self, x: f64, y: f64) {
        self.text.adjust(x, y);
        self.feature1.adjust(x, y);
        self.feature2.adjust(x, y);
        self.crossbar.adjust(x, y);
        self.arrow1a.adjust(x, y);
        self.arrow1b.adjust(x, y);
        self.arrow2a.adjust(x, y);
        self.arrow2b.adjust(x, y)
    }
}

impl BoundingBox for Dimension {
    fn bounding_box(&self) -> Bound {
        let mut b = Bound::default();
        b.update(&self.text.bounding_box());
        b.update(&self.feature1.bounding_box());
        b.update(&self.feature2.bounding_box());
        b.update(&self.crossbar.bounding_box());
        b.update(&self.arrow1a.bounding_box());
        b.update(&self.arrow1b.bounding_box());
        b.update(&self.arrow2a.bounding_box());
        b.update(&self.arrow2b.bounding_box());
        b
    }
}

/// segment
// (segment (start 117.5548 123.4602) (end 118.3848 122.6302) (width 0.2032) (layer B.Cu) (net 0) (tstamp 55E99398) (status foo))
#[derive(Clone, Debug, Default)]
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

impl Adjust for Segment {
    fn adjust(&mut self, x: f64, y: f64) {
        self.start.adjust(x, y);
        self.end.adjust(x, y)
    }
}

impl BoundingBox for Segment {
    fn bounding_box(&self) -> Bound {
        let x1 = self.start.x.min(self.end.x);
        let y1 = self.start.y.min(self.end.y);
        let x2 = self.start.x.max(self.end.x);
        let y2 = self.start.y.max(self.end.y);
        Bound::new(x1, y1, x2, y2)
    }
}

// TODO: support blind and micro via
/// via
// (via [blind] [micro] (at 132.1948 121.2202) (size 0.675) (drill 0.25) (layers F.Cu B.Cu) (net 19))
#[derive(Clone, Debug, Default)]
pub struct Via {
    /// blind/buried via
    pub blind: bool,
    /// micro via
    pub micro: bool,
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

impl Adjust for Via {
    fn adjust(&mut self, x: f64, y: f64) {
        self.at.adjust(x, y);
    }
}

impl BoundingBox for Via {
    fn bounding_box(&self) -> Bound {
        let x1 = self.at.x - self.size;
        let y1 = self.at.y - self.size;
        let x2 = self.at.x + self.size;
        let y2 = self.at.y + self.size;
        Bound::new(x1, y1, x2, y2)
    }
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

impl Adjust for Layout {
    fn adjust(&mut self, x: f64, y: f64) {
        for e in &mut self.elements {
            e.adjust(x, y)
        }
    }
}

impl BoundingBox for Layout {
    fn bounding_box(&self) -> Bound {
        let mut bound = Bound::default();
        for e in &self.elements {
            bound.update(&e.bounding_box());
        }
        bound.swap_if_needed();
        bound
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
        let update = |name: &mut NetName| {
            name.rename(old_name, new_name);
            Ok(())
        };
        self.update_net_names(update).unwrap();
    }

    /// update net names in a layout
    pub fn update_net_names<F>(&mut self, update: F) -> Result<()>
    where
        F: Fn(&mut NetName) -> Result<()>,
    {
        for element in &mut self.elements {
            match *element {
                Element::Net(ref mut net) => {
                    update(&mut net.name)?;
                }
                Element::NetClass(ref mut nc) => for name in &mut nc.nets {
                    update(name)?;
                },
                Element::Module(ref mut module) => for m_e in &mut module.elements {
                    if let footprint::Element::Pad(ref mut pad) = *m_e {
                        if let Some(ref mut net) = pad.net {
                            update(&mut net.name)?;
                        }
                    }
                },
                Element::Zone(ref mut zone) => {
                    update(&mut zone.net_name)?;
                }
                _ => (),
            }
        }
        Ok(())
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
        for x in &self.elements[..] {
            match *x {
                Element::Module(ref m) => if m.is_reference_with_name(reference) {
                    return Some(m);
                },
                Element::Via(_) |
                Element::Segment(_) |
                Element::Net(_) |
                Element::NetClass(_) |
                Element::GrText(_) |
                Element::GrLine(_) |
                Element::GrArc(_) |
                Element::GrCircle(_) |
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
            if let Element::Module(ref m) = *e {
                v.push(m);
            }
        }
        v
    }

    /// modify a module
    pub fn modify_module<F>(&mut self, reference: &str, fun: F) -> Result<()>
    where
        F: Fn(&mut footprint::Module) -> (),
    {
        for x in &mut self.elements {
            match *x {
                Element::Module(ref mut m) => if m.is_reference_with_name(reference) {
                    return Ok(fun(m));
                },
                Element::Via(_) |
                Element::Segment(_) |
                Element::Net(_) |
                Element::NetClass(_) |
                Element::GrText(_) |
                Element::GrLine(_) |
                Element::GrArc(_) |
                Element::GrCircle(_) |
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
            name: name.into(),
        }));
    }

    /// add a net class
    pub fn add_netclass(
        &mut self,
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
        nets: Vec<String>,
    ) {
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
            nets: nets.into_iter().map(|x| x.into()).collect(),
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
            if &net.0[..] == name {
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
            if element.name[..] == s[..] {
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

impl Adjust for Element {
    fn adjust(&mut self, x: f64, y: f64) {
        match *self {
            Element::Module(ref mut e) => e.adjust(x, y),
            Element::GrText(ref mut e) => e.adjust(x, y),
            Element::GrLine(ref mut e) => e.adjust(x, y),
            Element::GrArc(ref mut e) => e.adjust(x, y),
            Element::GrCircle(ref mut e) => e.adjust(x, y),
            Element::Dimension(ref mut e) => e.adjust(x, y),
            Element::Segment(ref mut e) => e.adjust(x, y),
            Element::Via(ref mut e) => e.adjust(x, y),
            Element::Zone(ref mut e) => e.adjust(x, y),
            Element::Net(_) | Element::NetClass(_) | Element::Other(_) => (),
        }
    }
}

impl BoundingBox for Element {
    fn bounding_box(&self) -> Bound {
        match *self {
            Element::Module(ref e) => e.bounding_box(),
            Element::GrText(ref e) => e.bounding_box(),
            Element::GrLine(ref e) => e.bounding_box(),
            Element::GrArc(ref e) => e.bounding_box(),
            Element::GrCircle(ref e) => e.bounding_box(),
            Element::Dimension(ref e) => e.bounding_box(),
            Element::Segment(ref e) => e.bounding_box(),
            Element::Via(ref e) => e.bounding_box(),
            Element::Zone(ref e) => e.bounding_box(),
            Element::Net(_) | Element::NetClass(_) | Element::Other(_) => Bound::default(),
        }
    }
}

impl NetName {
    /// check if a net is an unnamed net
    pub fn is_unnamed_net(&self) -> Option<(String, String)> {
        if self.0.starts_with("Net-(") {
            let v: Vec<&str> = self.0.split(|c| c == '-' || c == '(' || c == ')').collect();
            if v.len() != 5 {
                None
            } else {
                Some((v[2].into(), v[3].into()))
            }
        } else {
            None
        }
    }

    /// update the component name in an unnamed net
    pub fn set_unnamed_net(&mut self, new_name: &str) -> Result<()> {
        if let Some((_, pad)) = self.is_unnamed_net() {
            self.0 = format!("Net-({}-{})", new_name, pad);
            Ok(())
        } else {
            Err(format!("net is not an unnamed net: {}", self.0).into())
        }
    }
}

impl fmt::Display for NetName {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

#[test]
fn test_is_unnamed_net() {
    let n = Net {
        num: 1,
        name: "HELLO".into(),
    };
    assert_eq!(n.name.is_unnamed_net(), None);
    let n = Net {
        num: 1,
        name: "Net-(L1-Pad1)".into(),
    };
    assert_eq!(n.name.is_unnamed_net(), Some(("L1".into(), "Pad1".into())));
}

#[test]
fn test_unnamed_rename() {
    let mut n = Net {
        num: 1,
        name: "Net-(L1-Pad1)".into(),
    };
    n.name.set_unnamed_net("L101").unwrap();
    assert_eq!(
        n.name.is_unnamed_net(),
        Some(("L101".into(), "Pad1".into()))
    );
}
