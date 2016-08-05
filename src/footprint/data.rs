// (c) 2016 Productize SPRL <joost@productize.be>

use str_error;
use Result;

/// a Kicad module, with a name and a list of elements
#[derive(Debug,Clone)]
pub struct Module {
    /// name of the Kicad Module
    pub name: String,
    /// elements contained within the Kicad Module
    pub elements: Vec<Element>
}

impl Module {
    /// create a Module
    pub fn new(name: String) -> Module {
        Module { name: name, elements: vec![] }
    }
    /// append an Element to a Module
    pub fn append(&mut self, e: Element) {
        self.elements.push(e)
    }
    /// check if a Module has a reference Element with the specified name
    pub fn is_reference_with_name(&self, reference:&str) -> bool {
        for element in &self.elements[..] {
            if let Element::FpText(ref fp_text) = *element {
                if fp_text.name == "reference" && fp_text.value == *reference {
                    return true
                }
            }
        }
        false
    }
    /// update the name of the reference element specified by name, if found
    pub fn set_reference(&mut self, reference:&str, reference2:&str) {
        //println!("debug: searching '{}'", reference);
        for ref mut element in &mut self.elements[..] {
            if let Element::FpText(ref mut fp_text) = **element {
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
        for element in &self.elements[..] {
            if let Element::At(ref at) = *element {
                return (at.x, at.y)
            }
        }
        (0.0, 0.0)
    }

    /// check if the Module is on the front layer
    pub fn is_front(&self) -> bool {
        for element in &self.elements[..] {
            if let Element::Layer(ref layer) = *element {
                return &layer[..] == "F.Cu"
            }
        }
        true
    }

    /// calculate the bounding box of the module
    pub fn bounding_box(&self) -> (f64, f64, f64, f64) {
        let mut x1 = 10000.0_f64;
        let mut y1 = 10000.0_f64;
        let mut x2 = 0.0_f64;
        let mut y2 = 0.0_f64;
        let (x,y) = self.at();
        for element in &self.elements {
            match element.bounding_box() {
                None => (),
                Some((x1a, y1a, x2a, y2a)) => {
                    x1 = x1.min(x+x1a);
                    y1 = y1.min(y+y1a);
                    x2 = x2.max(x+x2a);
                    y2 = y2.max(y+y2a);
                }
            }
        }
        let (x1, x2) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        let (y1, y2) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        (x1, y1, x2, y2)
    }

    /// rename a net
    pub fn rename_net(&mut self, old_name:&str, new_name:&str) {
        for element in &mut self.elements[..] {
            if let Element::Pad(ref mut pad) = *element {
                pad.rename_net(old_name, new_name)
            }
        }
    }
}

#[derive(Debug,Clone)]
pub enum Element {
    SolderMaskMargin(f64),
    Layer(String), // TODO: use Layer type
    Descr(String),
    Tags(String),
    Attr(String),
    FpText(FpText),
    Pad(Pad),
    FpPoly(FpPoly),
    FpLine(FpLine),
    FpCircle(FpCircle),
    FpArc(FpArc),
    TEdit(String),
    TStamp(String),
    Path(String),
    At(At),
    Model(Model),
    Locked,
}

impl Element {
    pub fn bounding_box(&self) -> Option<(f64, f64, f64, f64)> {
        match *self {
            Element::Pad(ref x) => Some(x.bounding_box()),
            Element::FpPoly(ref x) => Some(x.bounding_box()),
            Element::FpLine(ref x) => Some(x.bounding_box()),
            Element::FpCircle(ref x) => Some(x.bounding_box()),
            _ => None,
        }
    }
}

#[derive(Debug,Clone)]
pub struct FpText {
    pub name: String,
    pub value: String,
    pub at: At,
    pub layer: Layer,
    pub effects: Effects,
    pub hide: bool,
}

impl FpText {
    pub fn new(name: String, value: String) -> FpText {
        FpText {
            name: name,
            value: value,
            at: At::new_empty(),
            layer: Layer::default(),
            effects: Effects::default(),
            hide: false
        }
    }
    pub fn set_effects(&mut self, effects: &Effects) {
        self.effects.clone_from(effects)
    }
    pub fn set_layer(&mut self, layer: &Layer) {
        self.layer.clone_from(layer)
    }
}

#[derive(Debug,Clone)]
pub struct At {
    pub x: f64,
    pub y: f64,
    pub rot: f64
}

impl At {
    pub fn new(x:f64 ,y:f64, rot:f64) -> At {
        At { x:x, y:y, rot:rot }
    }
    pub fn new_empty() -> At {
        At { x:0.0, y:0.0, rot:0.0 }
    }
}

#[derive(Debug,Clone)]
pub struct Font {
    pub size: Xy,
    pub thickness: f64,
}

impl Default for Font {
    fn default() -> Font {
        Font { size: Xy::new(0.0, 0.0, XyType::Size), thickness: 0.0 }
    }
}

#[derive(Debug,Clone)]
pub struct Effects {
    pub font: Font,
    pub justify:Option<Justify>,
}


#[derive(Debug,Clone)]
pub enum Justify {
    Mirror,
}

impl Default for Effects {
    fn default() -> Effects {
        Effects { font: Font::default(), justify:None }
    }
}

impl Effects {
    pub fn from_font(font: Font, justify: Option<Justify>) -> Effects {
        Effects { font: font, justify:justify }
    }
}
#[derive(Debug,Clone,PartialEq)]
pub enum XyType {
    Xy,
    Start,
    End,
    Size,
    Center,
    RectDelta,
}

#[derive(Debug,Clone)]
pub struct Xy {
    pub x: f64,
    pub y: f64,
    pub t: XyType,
}

#[derive(Debug,Clone)]
pub struct Pts {
    pub elements: Vec<Xy>
}

impl Xy {
    pub fn new(x: f64, y: f64, t: XyType) -> Xy {
        Xy { x:x, y:y, t:t }
    }
    pub fn new_empty(t: XyType) -> Xy {
        Xy { x:0.0, y:0.0, t:t }
    }
}

impl Default for Pts {
    fn default() -> Pts { Pts { elements:vec![] } }
}

#[derive(Clone,Debug,Default)]
pub struct Drill {
    pub shape:Option<String>,
    pub width:f64,
    pub height:f64,
    pub offset_x:f64,
    pub offset_y:f64,
}

#[derive(Debug)]
pub enum Part {
    At(At),
    Layer(Layer),
    Hide,
    Effects(Effects),
    Layers(Layers),
    Width(f64),
    Angle(f64),
    Xy(Xy),
    Pts(Pts),
    Thickness(f64),
    Net(Net),
    Drill(Drill),
    SolderPasteMargin(f64),
    SolderMaskMargin(f64),
    Clearance(f64),
}

#[derive(Debug,Clone)]
pub enum PadType {
    Smd,
    Pth,
    NpPth,
}

impl PadType {
    pub fn from_string(s:&str) -> Result<PadType> {
        match s {
            "smd" => Ok(PadType::Smd),
            "thru_hole" => Ok(PadType::Pth),
            "np_thru_hole" => Ok(PadType::NpPth),
            x => str_error(format!("unknown PadType {}", x))
        }
    }
}

#[derive(Debug,Clone)]
pub enum PadShape {
    Rect,
    Circle,
    Oval,
    Trapezoid,
    // TODO
}

impl PadShape {
    pub fn from_string(s:&str) -> Result<PadShape> {
        match s {
            "rect" => Ok(PadShape::Rect),
            "circle" => Ok(PadShape::Circle),
            "oval" => Ok(PadShape::Oval),
            "trapezoid" => Ok(PadShape::Trapezoid),
            x => str_error(format!("unknown PadShape: {}", x))
        }
    }
}

#[derive(Debug,Clone)]
pub enum LayerSide {
    Front,
    Back,
    Dwgs,
    Cmts,
    Eco1,
    Eco2,
    Edge,
    Both,
    In1,
    In2,
    None,
}

#[derive(Debug,Clone)]
pub enum LayerType {
    Cu,
    Paste,
    Mask,
    SilkS,
    User,
    Adhes,
    Cuts,
    CrtYd,
    Fab,
    Margin,
    Other(String),
}

#[derive(Debug,Clone)]
pub struct Layer {
    pub side: LayerSide,
    pub t: LayerType,
}

impl Default for Layer {

    fn default() -> Layer {
        Layer { side:LayerSide::Front, t:LayerType::Cu }
    }
}
    
impl Layer {
    
    pub fn from_string(s: String) -> Result<Layer> {
        let sp:Vec<&str> = s.split('.').collect();
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
                x => return str_error(format!("unknown layer side {}", x)),
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
        Ok(Layer { side:side, t:t, })
    }
    fn default() -> Layer {
        Layer { side: LayerSide::Front, t: LayerType::Cu }
    }
}

#[derive(Debug,Clone)]
pub struct Layers {
    pub layers: Vec<Layer>,
}

impl Default for Layers {
    fn default() -> Layers {
        Layers {
            layers: vec![]
        }
    }
}
    
impl Layers {
    pub fn append(&mut self, layer: &Layer) {
        self.layers.push(layer.clone())
    }
}

#[derive(Debug,Clone)]
pub struct Pad {
    pub name: String,
    pub t: PadType,
    pub shape: PadShape,
    pub size: Xy,
    pub rect_delta: Option<Xy>,
    pub at: At,
    pub layers: Layers,
    pub net: Option<Net>,
    pub drill: Option<Drill>,
    pub solder_paste_margin: Option<f64>,
    pub solder_mask_margin: Option<f64>,
    pub clearance: Option<f64>,
}

impl Pad {
    pub fn new(name: String, t:PadType, shape: PadShape) -> Pad {
        Pad {
            name: name,
            t: t,
            shape: shape,
            size: Xy::new_empty(XyType::Size),
            rect_delta:None,
            at: At::new_empty(),
            layers: Layers::default(),
            net:None,
            drill:None,
            solder_paste_margin:None,
            solder_mask_margin:None,
            clearance:None,
        }
    }

    pub fn rename_net(&mut self, old_name:&str, new_name:&str) {
        let new_net = if let Some(ref net) = self.net {
            if &net.name == old_name {
                Some(Net { name:new_name.to_string(), .. *net })
            } else {
                Some(net.clone())
            }
        } else { None } ;
        self.net = new_net
    }

    pub fn set_net(&mut self, net:&Net) {
        self.net = Some(net.clone())
    }
    
    pub fn set_drill(&mut self, drill:&Drill) {
        self.drill = Some(drill.clone())
    }

    pub fn bounding_box(&self) -> (f64,f64,f64,f64) {
        let x = self.at.x;
        let y = self.at.y;
        let (dx, dy) = if self.at.rot < 0.1 {
            (self.size.x, self.size.y)
        } else {
            (self.size.y, self.size.x)
        };
        (x-dx/2.0, y-dy/2.0, x+dx/2.0, y+dy/2.0)
    }
}

#[derive(Debug,Clone)]
pub struct FpPoly {
    pub pts:Pts,
    pub width:f64,
    pub layer:Layer,
}

impl Default for FpPoly {
    
    fn default() -> FpPoly {
        FpPoly { pts:Pts::default(), width:0.0, layer:Layer::default() }
    }
}

impl FpPoly {
    
    pub fn bounding_box(&self) -> (f64,f64,f64,f64) {
        let mut x1 = 10000.0_f64;
        let mut y1 = 10000.0_f64;
        let mut x2 = 0.0_f64;
        let mut y2 = 0.0_f64;
        for p in &self.pts.elements {
            x1 = x1.min(p.x);
            y1 = y1.min(p.y);
            x2 = x2.max(p.x);
            y2 = y2.max(p.y);
        }
        (x1,y2,x2,y2)
    }
}

#[derive(Debug,Clone)]
pub struct FpLine {
    pub start:Xy,
    pub end:Xy,
    pub layer:Layer,
    pub width:f64,
}

impl Default for FpLine {
    fn default() -> FpLine {
        FpLine { start:Xy::new_empty(XyType::Start), end:Xy::new_empty(XyType::End), layer:Layer::default(), width:0.0 }
    }
}
    
impl FpLine {
    pub fn bounding_box(&self) -> (f64,f64,f64,f64) {
        let mut x1 = 10000.0_f64;
        let mut y1 = 10000.0_f64;
        let mut x2 = 0.0_f64;
        let mut y2 = 0.0_f64;
        x1 = x1.min(self.start.x);
        y1 = y1.min(self.start.y);
        x2 = x2.max(self.end.x);
        y2 = y2.max(self.end.y);
        (x1,y1,x2,y2)
    }
}

#[derive(Debug,Clone)]
pub struct FpCircle {
    pub center:Xy,
    pub end:Xy,
    pub layer:Layer,
    pub width:f64,
}

impl Default for FpCircle {
    fn default() -> FpCircle {
        FpCircle { center:Xy::new_empty(XyType::Center), end:Xy::new_empty(XyType::End), layer:Layer::default(), width:0.0 }
    }
}
    
impl FpCircle {
    pub fn bounding_box(&self) -> (f64,f64,f64,f64) {
        let mut x1 = 10000.0_f64;
        let mut y1 = 10000.0_f64;
        let mut x2 = 0.0_f64;
        let mut y2 = 0.0_f64;
        let dx = self.center.x - self.end.x;
        let dy = self.center.y - self.end.y;
        let d2 = dx*dx + dy*dy;
        let d = d2.sqrt();
        x1 = x1.min(self.center.x-d);
        y1 = y1.min(self.center.y-d);
        x2 = x2.max(self.center.x+d);
        y2 = y2.max(self.center.y+d);
        (x1,y1,x2,y2)
    }
}

#[derive(Debug,Clone)]
pub struct FpArc {
    pub start:Xy,
    pub end:Xy,
    pub angle:f64,
    pub layer:Layer,
    pub width:f64,
}

impl Default for FpArc {
    fn default() -> FpArc {
        FpArc { start:Xy::new_empty(XyType::Start), end:Xy::new_empty(XyType::End), angle:0.0, layer:Layer::default(), width:0.0 }
    }
}

#[derive(Debug,Clone)]
pub struct Net {
    pub num: i64,
    pub name: String,
}

#[derive(Debug,Clone)]
pub struct Model {
    pub name: String,
    pub at: Xyz,
    pub scale: Xyz,
    pub rotate: Xyz,
}

#[derive(Debug,Clone)]
pub struct Xyz {
    pub x:f64,
    pub y:f64,
    pub z:f64,
}

impl Xyz {
    pub fn new(x:f64, y:f64, z:f64) -> Xyz {
        Xyz { x:x, y:y, z:z, }
    }
}
