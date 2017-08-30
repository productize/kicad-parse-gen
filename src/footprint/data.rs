// (c) 2016-2017 Productize SPRL <joost@productize.be>

use {Adjust, Bound, BoundingBox};
use symbolic_expressions::iteratom::SResult;

pub use layout::NetName;

use checkfix::{CheckFix, Config, CheckFixData};

/// implement to allow a Module and it's sub Element be flippable
pub trait Flip {
    /// flip me
    fn flip(&mut self);
}

/// rotate a module to be able to compare rotated modules
pub trait Rotate {
    /// rotate
    fn rotate(&mut self, rot: f64);
}

/// a Kicad module, with a name and a list of elements
#[derive(Debug, Clone)]
pub struct Module {
    /// name of the Kicad Module
    pub name: String,
    /// elements contained within the Kicad Module
    pub elements: Vec<Element>,
}

trait Named {
    fn name(&self) -> &'static str;
}

impl Module {
    /// create a Module
    pub fn new(name: String) -> Module {
        Module {
            name: name,
            elements: vec![],
        }
    }
    /// append an Element to a Module
    pub fn append(&mut self, e: Element) {
        self.elements.push(e)
    }
    /// check if a Module has a reference Element with the specified name
    pub fn is_reference_with_name(&self, reference: &str) -> bool {
        for element in &self.elements {
            if let Element::FpText(ref fp_text) = *element {
                if fp_text.name == "reference" && fp_text.value == *reference {
                    return true;
                }
            }
        }
        false
    }

    /// get the Reference element from the Module if it exists
    pub fn get_reference(&self) -> Option<&String> {
        self.get_reference_text().map(|a| &a.value)
    }

    /// get a `FpText` field by name
    pub fn get_text(&self, which:&'static str) -> Option<&FpText> {
        for element in &self.elements {
            if let Element::FpText(ref fp_text) = *element {
                if fp_text.name == which {
                    return Some(fp_text);
                }
            }
        }
        None
    }

    /// mutably get a `FpText` field by name
    pub fn get_text_mut(&mut self, which:&'static str) -> Option<&mut FpText> {
        for element in &mut self.elements {
            if let Element::FpText(ref mut fp_text) = *element {
                if fp_text.name == which {
                    return Some(fp_text);
                }
            }
        }
        None
    }

    /// get the reference `FpText`
    pub fn get_reference_text(&self) -> Option<&FpText> {
        self.get_text("reference")
    }

    /// mutably get the reference `FpText`
    pub fn get_reference_text_mut(&mut self) -> Option<&mut FpText> {
        self.get_text_mut("reference")
    }

    /// get the reference2 `FpText`
    pub fn get_reference2_text(&self) -> Option<&FpText> {
        self.get_text("user")
    }

    /// mutably get the reference2 `FpText`
    pub fn get_reference2_text_mut(&mut self) -> Option<&mut FpText> {
        self.get_text_mut("user")
    }

    /// mutably get the value `FpText`
    pub fn get_value_text_mut(&mut self) -> Option<&mut FpText> {
        self.get_text_mut("value")
    }


    /// check if the module has the "smd" attribute
    pub fn has_smd_attr(&self) -> bool {
        for element in &self.elements {
            if let Element::Attr(ref attr) = *element {
                if attr.as_str() == "smd" {
                    return true;
                }
            }
        }
        false
    }

    /// get the value `FpText`
    pub fn get_value_text(&self) -> Option<&FpText> {
        self.get_text("value")
    }

    /// check if a Module has a tstamp Element and return it
    pub fn get_tstamp(&self) -> Option<i64> {
        for element in &self.elements {
            if let Element::TStamp(stamp) = *element {
                return Some(stamp);
            }
        }
        None
    }

    /// check if a Module has a tedit Element and return it
    pub fn get_tedit(&self) -> Option<i64> {
        for element in &self.elements {
            if let Element::TEdit(stamp) = *element {
                return Some(stamp);
            }
        }
        None
    }

    /// update the name of the reference element specified by name, if found
    pub fn set_reference(&mut self, reference: &str, reference2: &str) {
        // println!("debug: searching '{}'", reference);
        for element in &mut self.elements {
            if let Element::FpText(ref mut fp_text) = *element {
                if fp_text.name == "reference" && fp_text.value == *reference {
                    fp_text.value.clear();
                    fp_text.value.push_str(reference2);
                }
            }
        }
    }
    /// check if there is an At element and return the coordinates found
    /// returns the default of (0.0,0.0) if not found
    pub fn at(&self) -> (f64, f64) {
        for element in &self.elements {
            if let Element::At(ref at) = *element {
                return (at.x, at.y);
            }
        }
        (0.0, 0.0)
    }

    /// check if there is an At element and return the rotation found
    /// returns the default of 0.0 if not found
    pub fn get_rotation(&self) -> f64 {
        for element in &self.elements {
            if let Element::At(ref at) = *element {
                return at.rot;
            }
        }
        0.0
    }

    /// adjust the At element contained in the module
    pub fn adjust_at(&mut self, x: f64, y: f64) {
        for element in &mut self.elements {
            if let Element::At(ref mut at) = *element {
                at.x += x;
                at.y += y;
            }
        }
    }

    /// check if the Module is on the front layer
    pub fn is_front(&self) -> bool {
        for element in &self.elements {
            if let Element::Layer(ref layer) = *element {
                return &layer[..] == "F.Cu";
            }
        }
        true
    }

    /// rename a net
    pub fn rename_net(&mut self, old_name: &str, new_name: &str) {
        for element in &mut self.elements {
            if let Element::Pad(ref mut pad) = *element {
                pad.rename_net(old_name, new_name)
            }
        }
    }

    /// return a list of `Pad`s contained in the module
    pub fn pads(&self) -> Vec<&Pad> {
        let mut v = vec![];
        for element in &self.elements {
            if let Element::Pad(ref pad) = *element {
                v.push(pad)
            }
        }
        v
    }

    /// return a list of `FpLine`s contained in the module
    pub fn lines(&self) -> Vec<&FpLine> {
        let mut v = vec![];
        for element in &self.elements {
            if let Element::FpLine(ref line) = *element {
                v.push(line)
            }
        }
        v
    }
}

impl BoundingBox for Module {
    fn bounding_box(&self) -> Bound {
        let (x, y) = self.at();
        let mut b = Bound::new(x, y, x, y);
        for element in &self.elements {
            let mut b2 = element.bounding_box();
            b2.x1 += x;
            b2.y1 += y;
            b2.x2 += x;
            b2.y2 += y;
            // trace!("{}: {:?}", element.name(), b2);
            b.update(&b2);
        }
        b.swap_if_needed();
        // trace!("Module {} bb: {:?}", self.name, b);
        b
    }
}

impl Adjust for Module {
    fn adjust(&mut self, x: f64, y: f64) {
        self.adjust_at(x, y)
    }
}

impl Flip for Module {
    fn flip(&mut self) {
        for e in &mut self.elements {
            e.flip()
        }
    }
}

impl Rotate for Module {
    fn rotate(&mut self, rot: f64) {
        for e in &mut self.elements {
            e.rotate(rot)
        }
    }
}

/// elements that can be found in a Module
#[derive(Debug, Clone, PartialEq)]
pub enum Element {
    /// solder mask margin
    SolderMaskMargin(f64),
    /// layer name
    Layer(String), // TODO: use Layer type
    /// description
    Descr(String),
    /// Tags element
    Tags(String),
    /// Attr element
    Attr(String),
    /// text
    FpText(FpText),
    /// pad
    Pad(Pad),
    /// polygon
    FpPoly(FpPoly),
    /// line
    FpLine(FpLine),
    /// circle
    FpCircle(FpCircle),
    /// arc
    FpArc(FpArc),
    /// edited time stamp
    TEdit(i64),
    /// time stamp
    TStamp(i64),
    /// Path element
    Path(String),
    /// location of module in layout
    At(At),
    /// 3D model information
    Model(Model),
    /// Clearance override for module
    Clearance(f64),
    /// is the module locked
    Locked,
}

impl BoundingBox for Element {
    fn bounding_box(&self) -> Bound {
        match *self {
            Element::Pad(ref x) => x.bounding_box(),
            Element::FpPoly(ref x) => x.bounding_box(),
            Element::FpLine(ref x) => x.bounding_box(),
            Element::FpCircle(ref x) => x.bounding_box(),
            Element::FpText(ref x) => x.bounding_box(),
            Element::FpArc(ref x) => x.bounding_box(),
            Element::At(_) |
            Element::Layer(_) |
            Element::TEdit(_) |
            Element::Descr(_) |
            Element::Path(_) |
            Element::Model(_) |
            Element::Attr(_) |
            Element::SolderMaskMargin(_) |
            Element::Clearance(_) |
            Element::Tags(_) |
            Element::Locked |
            Element::TStamp(_) => Bound::default(),
        }
    }
}

impl Named for Element {
    fn name(&self) -> &'static str {
        match *self {
            Element::Pad(_) => "Pad",
            Element::FpPoly(_) => "FpPoly",
            Element::FpLine(_) => "FpLine",
            Element::FpCircle(_) => "FpCircle",
            Element::FpArc(_) => "FpArc",
            Element::FpText(_) => "FpText",
            Element::At(_) => "At",
            Element::Layer(_) => "Layer",
            Element::TEdit(_) => "TEdit",
            Element::Descr(_) => "Descr",
            Element::Path(_) => "Path",
            Element::Model(_) => "Model",
            Element::TStamp(_) => "Tstamp",
            Element::SolderMaskMargin(_) => "SolderMaskMargin",
            Element::Clearance(_) | Element::Tags(_) => "Tags",
            Element::Attr(_) => "Attr",
            Element::Locked => "Locked",
        }
    }
}

impl Flip for Element {
    fn flip(&mut self) {
        match *self {
            Element::Pad(ref mut p) => p.flip(),
            Element::FpPoly(ref mut p) => p.flip(),
            Element::FpLine(ref mut p) => p.flip(),
            Element::FpCircle(ref mut p) => p.flip(), 
            Element::FpArc(ref mut p) => p.flip(),
            Element::FpText(ref mut p) => p.flip(),
            Element::At(ref mut p) => p.flip(),
            Element::Layer(ref mut p) => match p.as_str() {
                "F.Cu" => {
                    p.clear();
                    p.push_str("B.Cu");
                }
                "B.Cu" => {
                    p.clear();
                    p.push_str("F.Cu");
                }
                _ => (),
            },
            Element::TEdit(_) |
            Element::Descr(_) |
            Element::Path(_) |
            Element::Model(_) |
            Element::TStamp(_) |
            Element::SolderMaskMargin(_) |
            Element::Clearance(_) |
            Element::Tags(_) |
            Element::Attr(_) |
            Element::Locked => (),
        }
    }
}

impl Rotate for Element {
    fn rotate(&mut self, rot: f64) {
        match *self {
            Element::Pad(ref mut p) => p.rotate(rot),
            Element::FpText(ref mut p) => p.rotate(rot),
            Element::At(ref mut p) => p.rotate(rot),
            Element::FpPoly(_) |
            Element::FpLine(_) |
            Element::FpCircle(_) |
            Element::FpArc(_) |
            Element::Layer(_) |
            Element::TEdit(_) |
            Element::Descr(_) |
            Element::Path(_) |
            Element::Model(_) |
            Element::TStamp(_) |
            Element::SolderMaskMargin(_) |
            Element::Clearance(_) |
            Element::Tags(_) |
            Element::Attr(_) |
            Element::Locked => (),
        }
    }
}


/// text element
#[derive(Debug, Clone)]
pub struct FpText {
    /// name
    pub name: String,
    /// text
    pub value: String,
    /// location
    pub at: At,
    /// layer
    pub layer: Layer,
    /// text effects
    pub effects: Effects,
    /// is it a hidden text
    pub hide: bool,
}

impl Flip for FpText {
    fn flip(&mut self) {
        self.at.flip();
        self.layer.flip();
        self.effects.flip();
    }
}

impl Rotate for FpText {
    fn rotate(&mut self, rot: f64) {
        self.at.rotate(rot)
    }
}

impl PartialEq for FpText {
    fn eq(&self, other: &FpText) -> bool {
        if self.name == "reference" && other.name == "reference" {
            return true;
        }
        if self.name == "value" && other.name == "value" {
            return true;
        }
        if self.at != other.at {
            return false;
        }
        if self.name != other.name {
            return false;
        }
        if self.value != other.value {
            return false;
        }
        if self.at != other.at {
            return false;
        }
        if self.layer != other.layer {
            return false;
        }
        if self.effects != other.effects {
            return false;
        }
        if self.hide != other.hide {
            return false;
        }
        true
    }
}

impl BoundingBox for FpText {
    fn bounding_box(&self) -> Bound {
        let (x, y) = (self.at.x, self.at.y);
        debug!("bound for FpText is poor");
        Bound::new(x, y, x, y)
    }
}

impl FpText {
    /// create a text with given name and value
    pub fn new(name: String, value: String) -> FpText {
        FpText {
            name: name,
            value: value,
            at: At::default(),
            layer: Layer::default(),
            effects: Effects::default(),
            hide: false,
        }
    }
    /// set the text effects of the text
    pub fn set_effects(&mut self, effects: &Effects) {
        self.effects.clone_from(effects)
    }
    /// set the layer of the text
    pub fn set_layer(&mut self, layer: &Layer) {
        self.layer.clone_from(layer)
    }
}

/// a location and rotation in a layout
#[derive(Debug, Clone, Default, PartialEq)]
pub struct At {
    /// x coordinate
    pub x: f64,
    /// y coordinate
    pub y: f64,
    /// rotation
    pub rot: f64,
}

impl Flip for At {
    fn flip(&mut self) {
        self.y = -self.y;
        self.rot = 360.0 - self.rot;
    }
}

impl Adjust for At {
    fn adjust(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y
    }
}

impl At {
    /// create a location
    pub fn new(x: f64, y: f64, rot: f64) -> At {
        At {
            x: x,
            y: y,
            rot: rot,
        }
    }
}

impl Rotate for At {
    fn rotate(&mut self, rot: f64) {
        self.rot += rot;
        if self.rot >= 360.0 {
            self.rot -= 360.0
        }
        if self.rot < 0.0 {
            self.rot += 360.0
        }
    }
}

/// font attributes for text
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Font {
    /// size of the font
    pub size: Xy,
    /// thickness of the font
    pub thickness: f64,
    /// if it is italic
    pub italic: bool,
}

/// text effects
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Effects {
    /// the font used
    pub font: Font,
    /// the text justification
    pub justify: Option<Justify>,
}

impl Flip for Effects {
    fn flip(&mut self) {
        self.justify = match self.justify {
            None => Some(Justify::Mirror),
            Some(_) => None,
        }
    }
}

impl Effects {
    /// create a text effects element from font and justification
    pub fn from_font(font: Font, justify: Option<Justify>) -> Effects {
        Effects {
            font: font,
            justify: justify,
        }
    }
}

/// text justification
#[derive(Debug, Clone, PartialEq)]
pub enum Justify {
    /// the text is mirrored
    Mirror,
    /// the text is left-justified
    Left,
    /// the text is right-justified
    Right,
}

/// the type of X-Y element
#[derive(Debug, Clone, PartialEq)]
pub enum XyType {
    /// regular
    Xy,
    /// starting point
    Start,
    /// ending point
    End,
    /// size
    Size,
    /// center point
    Center,
    /// rectangular delta
    RectDelta,
}

impl Default for XyType {
    fn default() -> XyType {
        XyType::Xy
    }
}

/// X-Y element
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Xy {
    /// x coordinate
    pub x: f64,
    /// y coorginate
    pub y: f64,
    /// the type of X-Y
    pub t: XyType,
}

impl Flip for Xy {
    fn flip(&mut self) {
        self.y = -self.y;
    }
}

impl Adjust for Xy {
    fn adjust(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y
    }
}

impl Xy {
    /// create a new X-Y coordinate
    pub fn new(x: f64, y: f64, t: XyType) -> Xy {
        Xy { x: x, y: y, t: t }
    }
    /// create a new default X-Y coordinate of a certain type
    pub fn new_empty(t: XyType) -> Xy {
        Xy {
            x: 0.0,
            y: 0.0,
            t: t,
        }
    }
}

/// a list of X-Y coordinates
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Pts {
    /// the list of X-Y coordinates
    pub elements: Vec<Xy>,
}

impl Flip for Pts {
    fn flip(&mut self) {
        for e in &mut self.elements {
            e.flip()
        }
    }
}

impl Adjust for Pts {
    fn adjust(&mut self, x: f64, y: f64) {
        for e in &mut self.elements {
            e.adjust(x, y)
        }
    }
}

impl BoundingBox for Pts {
    fn bounding_box(&self) -> Bound {
        let mut b = Bound::default();
        for e in &self.elements {
            let b2 = Bound::new(e.x, e.y, e.x, e.y);
            b.update(&b2);
        }
        b.swap_if_needed();
        b
    }
}

/// a drill
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Drill {
    /// shape of the drill
    pub shape: Option<String>,
    /// width of the drill
    pub width: f64,
    /// height of the drill
    pub height: f64,
    /// x-offset of the drill
    pub offset_x: f64,
    /// y-offset of the drill
    pub offset_y: f64,
}

/// type of a Pad
#[derive(Debug, Clone, PartialEq)]
pub enum PadType {
    /// surface mount
    Smd,
    /// through-hole
    Pth,
    /// non-plated through-hole
    NpPth,
}

impl PadType {
    /// convert a &str to a pad type
    pub fn from_string(s: &str) -> SResult<PadType> {
        match s {
            "smd" => Ok(PadType::Smd),
            "thru_hole" => Ok(PadType::Pth),
            "np_thru_hole" => Ok(PadType::NpPth),
            x => Err(format!("unknown PadType {}", x).into()),
        }
    }
}

/// shape of a pad
#[derive(Debug, Clone, PartialEq)]
pub enum PadShape {
    /// rectangular
    Rect,
    /// circular
    Circle,
    /// oval
    Oval,
    /// trapezoid
    Trapezoid, // TODO
}

impl PadShape {
    /// convert a &str to a pad shape
    pub fn from_string(s: &str) -> SResult<PadShape> {
        match s {
            "rect" => Ok(PadShape::Rect),
            "circle" => Ok(PadShape::Circle),
            "oval" => Ok(PadShape::Oval),
            "trapezoid" => Ok(PadShape::Trapezoid),
            x => Err(format!("unknown PadShape: {}", x).into()),
        }
    }
}

/// side of a layer
#[derive(Debug, Clone, PartialEq)]
pub enum LayerSide {
    /// front side
    Front,
    /// back side
    Back,
    /// Dwgs side
    Dwgs,
    /// Cmts side
    Cmts,
    /// Eco1 side
    Eco1,
    /// Eco2 side
    Eco2,
    /// edge of the board
    Edge,
    /// both sides
    Both,
    /// Inner layer 1
    In1,
    /// Inner layer 2
    In2,
    /// no side
    None,
}

impl Flip for LayerSide {
    fn flip(&mut self) {
        let n = match *self {
            LayerSide::Front => LayerSide::Back,
            LayerSide::Back => LayerSide::Front,
            ref x => x.clone(),
        };
        *self = n;
    }
}

impl Default for LayerSide {
    fn default() -> LayerSide {
        LayerSide::Front
    }
}

/// type of a layer
#[derive(Debug, Clone, PartialEq)]
pub enum LayerType {
    /// copper layer
    Cu,
    /// paste layer
    Paste,
    /// solder mask layer
    Mask,
    /// silk screen layer
    SilkS,
    /// user layer
    User,
    /// adhesive layer
    Adhes,
    /// cuts layer
    Cuts,
    /// CrtYd layer
    CrtYd,
    /// fabrication layer
    Fab,
    /// margin layer
    Margin,
    /// an other custom named layer
    Other(String),
}

impl Default for LayerType {
    fn default() -> LayerType {
        LayerType::Cu
    }
}

/// a pcb layer, with a side and a type
#[derive(Debug, Clone, Default)]
pub struct Layer {
    /// side of the layer
    pub side: LayerSide,
    /// type of the layer
    pub t: LayerType,
}

impl Flip for Layer {
    fn flip(&mut self) {
        self.side.flip()
    }
}

impl PartialEq for Layer {
    fn eq(&self, other: &Layer) -> bool {
        self.t == other.t
    }
}

impl Layer {
    /// create a layer from a String
    pub fn from_string(s: &str) -> SResult<Layer> {
        let sp: Vec<&str> = s.split('.').collect();
        let mut side = LayerSide::None;
        let s_t = if sp.len() == 2 {
            side = match sp[0] {
                "F" => LayerSide::Front,
                "B" => LayerSide::Back,
                "Dwgs" => LayerSide::Dwgs,
                "Cmts" => LayerSide::Cmts,
                "Eco1" => LayerSide::Eco1,
                "Eco2" => LayerSide::Eco2,
                "Edge" => LayerSide::Edge,
                "In1" => LayerSide::In1,
                "In2" => LayerSide::In2,
                "*" => LayerSide::Both,
                x => return Err(format!("unknown layer side {}", x).into()),
            };
            sp[1]
        } else {
            sp[0]
        };

        let t = match s_t {
            "Cu" => LayerType::Cu,
            "Paste" => LayerType::Paste,
            "Mask" => LayerType::Mask,
            "SilkS" => LayerType::SilkS,
            "User" => LayerType::User,
            "Adhes" => LayerType::Adhes,
            "Cuts" => LayerType::Cuts,
            "CrtYd" => LayerType::CrtYd,
            "Fab" => LayerType::Fab,
            "Margin" => LayerType::Margin,
            x => LayerType::Other(String::from(x)),
        };
        Ok(Layer { side: side, t: t })
    }
}

/// a list of layers
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Layers {
    /// a list of layers
    pub layers: Vec<Layer>,
}

impl Flip for Layers {
    fn flip(&mut self) {
        for layer in &mut self.layers {
            layer.flip()
        }
    }
}

impl Layers {
    /// append a layer to a list of layers
    pub fn append(&mut self, layer: Layer) {
        self.layers.push(layer)
    }
}

/// a pad
#[derive(Debug, Clone)]
pub struct Pad {
    /// name
    pub name: String,
    /// type
    pub t: PadType,
    /// shape
    pub shape: PadShape,
    /// size
    pub size: Xy,
    /// offset
    pub rect_delta: Option<Xy>,
    /// location
    pub at: At,
    /// layers
    pub layers: Layers,
    /// associated net
    pub net: Option<Net>,
    /// zone connect
    pub zone_connect: Option<i64>,
    /// drill
    pub drill: Option<Drill>,
    /// solder paste margin
    pub solder_paste_margin: Option<f64>,
    /// solder mask margin
    pub solder_mask_margin: Option<f64>,
    /// clearance
    pub clearance: Option<f64>,
    /// thermal gap
    pub thermal_gap: Option<f64>,
}

impl Flip for Pad {
    fn flip(&mut self) {
        self.at.flip();
        self.layers.flip();
    }
}

impl Rotate for Pad {
    fn rotate(&mut self, rot: f64) {
        self.at.rotate(rot)
    }
}

impl PartialEq for Pad {
    fn eq(&self, other: &Pad) -> bool {
        if self.at != other.at {
            return false;
        }
        if self.name != other.name {
            return false;
        }
        if self.t != other.t {
            return false;
        }
        if self.shape != other.shape {
            return false;
        }
        if self.size != other.size {
            return false;
        }
        if self.rect_delta != other.rect_delta {
            return false;
        }
        if self.layers != other.layers {
            return false;
        }
        if self.zone_connect != other.zone_connect {
            return false;
        }
        if self.drill != other.drill {
            return false;
        }
        if self.solder_paste_margin != other.solder_paste_margin {
            return false;
        }
        if self.solder_mask_margin != other.solder_mask_margin {
            return false;
        }
        if self.clearance != other.clearance {
            return false;
        }
        if self.thermal_gap != other.thermal_gap {
            return false;
        }
        true
    }
}

impl Pad {
    /// create a pad with a name, type and shape
    pub fn new(name: String, t: PadType, shape: PadShape) -> Pad {
        Pad {
            name: name,
            t: t,
            shape: shape,
            size: Xy::new_empty(XyType::Size),
            rect_delta: None,
            at: At::default(),
            layers: Layers::default(),
            net: None,
            zone_connect: None,
            drill: None,
            solder_paste_margin: None,
            solder_mask_margin: None,
            clearance: None,
            thermal_gap: None,
        }
    }

    /// rename the net of a pad
    pub fn rename_net(&mut self, old_name: &str, new_name: &str) {
        let new_net = if let Some(ref net) = self.net {
            if net.name.0 == old_name {
                Some(Net {
                    name: new_name.into(),
                    ..*net
                })
            } else {
                Some(net.clone())
            }
        } else {
            None
        };
        self.net = new_net
    }

    /// set the net of a pad
    pub fn set_net(&mut self, net: Net) {
        self.net = Some(net)
    }

    /// set the drill of a pad
    pub fn set_drill(&mut self, drill: Drill) {
        self.drill = Some(drill)
    }
}

impl BoundingBox for Pad {
    fn bounding_box(&self) -> Bound {
        let x = self.at.x;
        let y = self.at.y;
        let (dx, dy) = if self.at.rot < 0.1 {
            (self.size.x, self.size.y)
        } else {
            (self.size.y, self.size.x)
        };
        Bound::new(x - dx / 2.0, y - dy / 2.0, x + dx / 2.0, y + dy / 2.0)
    }
}

/// a polygon
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FpPoly {
    /// points
    pub pts: Pts,
    /// width
    pub width: f64,
    /// layer
    pub layer: Layer,
}

impl Flip for FpPoly {
    fn flip(&mut self) {
        self.pts.flip();
        self.layer.flip()
    }
}

impl BoundingBox for FpPoly {
    fn bounding_box(&self) -> Bound {
        let mut b = Bound::default();
        for p in &self.pts.elements {
            let b2 = Bound::new(p.x, p.y, p.x, p.y);
            b.update(&b2);
        }
        b.swap_if_needed();
        b
    }
}

/// a line
#[derive(Debug, Clone, PartialEq)]
pub struct FpLine {
    /// start point
    pub start: Xy,
    /// end point
    pub end: Xy,
    /// layer
    pub layer: Layer,
    /// width
    pub width: f64,
}

impl Flip for FpLine {
    fn flip(&mut self) {
        self.start.flip();
        self.end.flip();
        self.layer.flip()
    }
}

impl Default for FpLine {
    fn default() -> FpLine {
        FpLine {
            start: Xy::new_empty(XyType::Start),
            end: Xy::new_empty(XyType::End),
            layer: Layer::default(),
            width: 0.0,
        }
    }
}

impl BoundingBox for FpLine {
    fn bounding_box(&self) -> Bound {
        Bound::new(self.start.x, self.start.y, self.end.x, self.end.y)
    }
}

impl FpLine {
    fn make(x1:f64, y1:f64, x2:f64, y2:f64, t:LayerType) -> FpLine {
        let mut line1 = FpLine::default();
        line1.start.x = x1;
        line1.start.y = y1;
        line1.end.x = x2;
        line1.end.y = x1;
        line1.layer.t = t;
        line1
    }
}

/// a circle
#[derive(Debug, Clone, PartialEq)]
pub struct FpCircle {
    /// center point
    pub center: Xy,
    /// end point
    pub end: Xy,
    /// layer
    pub layer: Layer,
    /// width
    pub width: f64,
}

impl Default for FpCircle {
    fn default() -> FpCircle {
        FpCircle {
            center: Xy::new_empty(XyType::Center),
            end: Xy::new_empty(XyType::End),
            layer: Layer::default(),
            width: 0.0,
        }
    }
}

impl Flip for FpCircle {
    fn flip(&mut self) {
        self.center.flip();
        self.end.flip();
        self.layer.flip()
    }
}

impl BoundingBox for FpCircle {
    fn bounding_box(&self) -> Bound {
        let dx = self.center.x - self.end.x;
        let dy = self.center.y - self.end.y;
        let d2 = dx * dx + dy * dy;
        let d = d2.sqrt();
        Bound::new(
            self.center.x - d,
            self.center.y - d,
            self.center.x + d,
            self.center.y + d,
        )
    }
}

/// an arc
#[derive(Debug, Clone, PartialEq)]
pub struct FpArc {
    /// start point
    pub start: Xy,
    /// end point
    pub end: Xy,
    /// angle
    pub angle: f64,
    /// layer
    pub layer: Layer,
    /// width
    pub width: f64,
}

impl Flip for FpArc {
    fn flip(&mut self) {
        self.start.flip();
        self.end.flip();
        self.layer.flip()
    }
}

impl BoundingBox for FpArc {
    fn bounding_box(&self) -> Bound {
        // perhaps not correct
        Bound::new(self.start.x, self.start.y, self.end.x, self.end.y)
    }
}

impl Default for FpArc {
    fn default() -> FpArc {
        FpArc {
            start: Xy::new_empty(XyType::Start),
            end: Xy::new_empty(XyType::End),
            angle: 0.0,
            layer: Layer::default(),
            width: 0.0,
        }
    }
}

/// a net
#[derive(Debug, Clone, PartialEq)]
pub struct Net {
    /// net number
    pub num: i64,
    /// net name
    pub name: NetName,
}

/// a 3D model
#[derive(Debug, Clone, PartialEq)]
pub struct Model {
    /// name
    pub name: String,
    /// location
    pub at: Xyz,
    /// scale
    pub scale: Xyz,
    /// rotation
    pub rotate: Xyz,
}

/// a 3D X-Y-Z coordinate
#[derive(Debug, Clone, PartialEq)]
pub struct Xyz {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// Z coordinate
    pub z: f64,
}

impl Xyz {
    /// create a X-Y-Z coordinate
    pub fn new(x: f64, y: f64, z: f64) -> Xyz {
        Xyz { x: x, y: y, z: z }
    }
}

impl CheckFix for Module {
    fn check(&self, config: &Config) -> Vec<CheckFixData> {
        let mut v = vec![];
        let name = &self.name;
        let font_size = config.m.font_size;
        let font_thickness = config.m.font_thickness;

        if let Some(reference) = self.get_reference_text() {
            // 7.3 reference is correctly placed
            if reference.value.as_str() != "REF**" {
                v.push(CheckFixData::new(
                    7,
                    3,
                    name.clone(),
                    "reference should be REF**",
                ));
            }
            // 7.3 reference is on F.SilkS or B.SilkS
            if reference.layer.t != LayerType::SilkS {
                v.push(CheckFixData::new(
                    7,
                    3,
                    name.clone(),
                    "reference should be on SilkS layertype",
                ));
            }
            // 7.3 reference should not be hidden
            if reference.hide {
                v.push(CheckFixData::new(
                    7,
                    3,
                    name.clone(),
                    "reference should not be hidden",
                ));
            }
            // 7.3 aspect ratio should be 1:1
            if reference.effects.font.size.x != reference.effects.font.size.y {
                v.push(CheckFixData::new(
                    7,
                    3,
                    name.clone(),
                    "reference label font aspect ratio should be 1:1",
                ));
            }
            // 7.3 font height should be 1.0
            // this is kind of big :(
            if reference.effects.font.size.y != font_size {
                v.push(CheckFixData::info(
                    7,
                    3,
                    name.clone(),
                    format!("reference label should have height {}", font_size),
                ));
            }
            // 7.3 font width should be 1.0
            // this is kind of big :(
            if reference.effects.font.size.x != font_size {
                v.push(CheckFixData::info(
                    7,
                    3,
                    name.clone(),
                    format!("reference label should have width {}", font_size),
                ));
            }
            // 7.4 font thickness should be 0.15
            if reference.effects.font.thickness != font_thickness {
                v.push(CheckFixData::info(
                    7,
                    3,
                    name.clone(),
                    format!("reference label thickness should be {}", font_thickness),
                ))
            }
        // 7.3 TODO: further silkscreen checks
        // 7.3 TODO: check for intersection with pads etc
        } else {
            // 7.3 missing reference
            v.push(CheckFixData::new(7, 3, name.clone(), "reference missing"));
        }

        if let Some(value) = self.get_value_text() {
            // 7.4 value text has to match value
            if value.value.as_str() != name.as_str() {
                v.push(CheckFixData::new(
                    7,
                    4,
                    name.clone(),
                    "value text has to match footprint name",
                ));
            }
            // 7.4 value has to be on F.Fab or B.Fab
            if value.layer.t != LayerType::Fab {
                v.push(CheckFixData::new(
                    7,
                    4,
                    name.clone(),
                    "value text has to be on Fab layer",
                ));
            }
            // 7.4 value should not be hidden
            if value.hide {
                v.push(CheckFixData::new(
                    7,
                    4,
                    name.clone(),
                    "value text should not be hidden",
                ));
            }
            // 7.4 font height should be 1.0
            // this is kind of big :(
            if value.effects.font.size.y != font_size {
                v.push(CheckFixData::info(
                    7,
                    3,
                    name.clone(),
                    format!("value label should have height {}", font_size),
                ));
            }
            // 7.4 font width should be 1.0
            // this is kind of big :(
            if value.effects.font.size.x != font_size {
                v.push(CheckFixData::info(
                    7,
                    3,
                    name.clone(),
                    format!("value label should have width {}", font_size),
                ));
            }
            // 7.4 font thickness should be 0.15
            if value.effects.font.thickness != font_thickness {
                v.push(CheckFixData::info(
                    7,
                    3,
                    name.clone(),
                    format!("value label thickness should be {}", font_thickness),
                ))
            }
        // TODO
        } else {
            // 7.4 missing value
            v.push(CheckFixData::new(7, 4, name.clone(), "value missing"));
        }

        if let Some(reference) = self.get_reference2_text() {
            // 7.4 reference 2 is correctly named
            if reference.value.as_str() != "%R" {
                v.push(CheckFixData::new(7, 4, name.clone(), "reference 2 should be %R"));
            }
            // 7.4 reference 2 is on F.Fab or B.Fab
            if reference.layer.t != LayerType::Fab {
                v.push(CheckFixData::new(
                    7,
                    4,
                    name.clone(),
                    "reference 2 should be on Fab layertype",
                ));
            }
            // 7.4 aspect ratio should be 1:1
            if reference.effects.font.size.x != reference.effects.font.size.y {
                v.push(CheckFixData::new(
                    7,
                    4,
                    name.clone(),
                    "reference 2 label font aspect ratio should be 1:1",
                ));
            }
            // 7.4 font height should be 1.0
            // this is kind of big :(
            if reference.effects.font.size.y != font_size {
                v.push(CheckFixData::info(
                    7,
                    3,
                    name.clone(),
                    format!("reference 2 label should have height {}", font_size),
                ));
            }
            // 7.4 font width should be 1.0
            // this is kind of big :(
            if reference.effects.font.size.x != font_size {
                v.push(CheckFixData::info(
                    7,
                    3,
                    name.clone(),
                    format!("reference 2 label should have width {}", font_size),
                ));
            }
            // 7.4 font thickness should be 0.15
            if reference.effects.font.thickness != font_thickness {
                v.push(CheckFixData::info(
                    7,
                    3,
                    name.clone(),
                    format!("reference 2 label thickness should be {}", font_thickness),
                ))
            }
        // 7.4 TODO: check for intersection with pads etc
        // 7.4 TODO: check Fab line widths
        } else {
            // 7.4 missing reference 2
            v.push(CheckFixData::new(7, 4, name.clone(), "reference 2 missing"));
        }
        // TODO 7.5 CrtYd checking
        // for now just check that there are 4 CrtYd lines
        let mut c = 0;
        for line in self.lines() {
            if line.layer.t == LayerType::CrtYd {
                c += 1;
            }
        }
        if c < 4 {
            v.push(CheckFixData::new(
                7,
                5,
                name.clone(),
                "missing courtyard on CrtYd layer",
            ));
        }
        // 8.1 For surface-mount devices, placement type must be set to "Surface Mount"
        let mut smd = 0;
        let mut pth = 0;
        for pad in self.pads() {
            if pad.t == PadType::Smd {
                smd += 1;
            } else if pad.t == PadType::Pth {
                pth += 1;
            }
        }
        if pth == 0 && smd > 0 {
            if !self.has_smd_attr() {
                v.push(CheckFixData::new(
                    8,
                    1,
                    name.clone(),
                    "SMD components need to have placement Smd (Normal+Insert in Properties)",
                ));
            }
        } else if pth > 0 && smd == 0 {
            // 9.1 For through-hole devices, placement type must be set to "Through Hole
            if self.has_smd_attr() {
                v.push(CheckFixData::new(
                    9,
                    1,
                    name.clone(),
                    "SMD components need to have placement Smd (Normal+Insert in Properties)",
                ));
            }
            // TODO 9.2 For through-hole components, footprint anchor is set on pad 1
            // TODO 9.4 Layer requirements
            // TODO 9.5 Minimum drilled hole diameter is the maximum lead diameter plus 0.20mm (IPC-2222 Class 2)
            // TODO 9.6 Minimum annular ring
        }
        // TODO 8.2 For surface-mount devices, footprint anchor is placed in the middle of the footprint (IPC-7351).
        // TODO 8.3 SMD pad layer requirements
        // TODO 10.1 Footprint name must match its filename. (.kicad_mod files)
        // TODO 10.2 description and keyword tags
        // TODO 10.3 all other fields are at default
        // TODO 10.4 3D model reference
        v
    }

    fn fix(&mut self, config: &Config) {
        let name = self.name.clone();
        // fix reference
        if let Some(reference) = self.get_reference_text_mut() {
            reference.value.clear();
            reference.value.push_str("REF**");
            reference.layer.t = LayerType::SilkS;
            reference.hide = false;
            reference.effects.font.size.x = config.m.font_size;
            reference.effects.font.size.y = config.m.font_size;
            reference.effects.font.thickness = config.m.font_thickness;
        } else {
            // reference should always be there
        }
        // fix value
        if let Some(value) = self.get_value_text_mut() {
            value.value.clear();
            value.value.push_str(&name);
            value.layer.t = LayerType::Fab;
            value.hide = false;
            value.effects.font.size.x = config.m.font_size;
            value.effects.font.size.y = config.m.font_size;
            value.effects.font.thickness = config.m.font_thickness;
        } else {
            // value should always be there
        }
        // fix reference2
        {
            if self.get_reference2_text().is_none() {
                let size = Xy {
                    x: config.m.font_size,
                    y: config.m.font_size,
                    t: XyType::default(),
                };
                let font = Font {
                    size: size,
                    thickness: config.m.font_thickness,
                    italic: false,
                };
                let ref2 = FpText {
                    name: "user".into(),
                    value: "%R".into(),
                    at: At::new(0.0, 0.0, 0.0),
                    layer: Layer::from_string("F.Fab").unwrap(),
                    effects: Effects::from_font(font, None),
                    hide: false
                };
                self.elements.push(Element::FpText(ref2));
            };
            let ref2 = self.get_reference2_text_mut().unwrap();
            ref2.value.clear();
            ref2.value.push_str("%R");
            ref2.layer.t = LayerType::Fab;
            ref2.hide = false;
            ref2.effects.font.size.x = config.m.font_size;
            ref2.effects.font.size.y = config.m.font_size;
            ref2.effects.font.thickness = config.m.font_thickness;
        }
        // set Surface Mount placement for SMD components
        {
            let mut smd = 0;
            let mut pth = 0;
            for pad in self.pads() {
                if pad.t == PadType::Smd {
                    smd += 1;
                } else if pad.t == PadType::Pth {
                    pth += 1;
                }
            }
            if pth == 0 && smd > 0 {
                if !self.has_smd_attr() {
                    self.elements.push(Element::Attr("smd".into()))
                }
            }
        }
        // Generate CrtYd if none found
        let mut c = 0;
        for line in self.lines() {
            if line.layer.t == LayerType::CrtYd {
                c += 1;
            }
        }
        if c < 4 { // this is not perfect of course...
            let bound = self.bounding_box();
            let (x,y) = self.at();
            let offset = 0.15; // TODO
            // make relative to center of module and add offset
            let x1 = bound.x1 - x - offset;
            let y1 = bound.y1 - y - offset;
            let x2 = bound.x2 - x + offset;
            let y2 = bound.y2 - y + offset;
            let line1 = FpLine::make(x1, y1, x2, y1, LayerType::CrtYd);
            let line2 = FpLine::make(x2, y1, x2, y2, LayerType::CrtYd);
            let line3 = FpLine::make(x2, y2, x1, y2, LayerType::CrtYd);
            let line4 = FpLine::make(x1, y2, x1, y1, LayerType::CrtYd);
            self.elements.push(Element::FpLine(line1));
            self.elements.push(Element::FpLine(line2));
            self.elements.push(Element::FpLine(line3));
            self.elements.push(Element::FpLine(line4));
        }
    }
}
