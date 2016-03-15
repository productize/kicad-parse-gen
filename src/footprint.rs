// (c) 2015-2016 Joost Yervante Damad <joost@productize.be>

use std::fmt;

// get from parent
use ERes;
use err;

extern crate rustysexp;
use self::rustysexp::Sexp;

pub trait FromSexp {
    fn from_sexp(&Sexp) -> Self;
}

#[derive(Debug,Clone)]
pub struct Module {
    pub name: String,
    pub elements: Vec<Element>
}

impl Module {
    fn new(name: String) -> Module {
        Module { name: name, elements: vec![] }
    }
    fn append(&mut self, e: Element) {
        self.elements.push(e)
    }
    pub fn is_reference(&self, reference:&String) -> bool {
        for element in &self.elements[..] {
            match *element {
                Element::FpText(ref fp_text) => {
                    if fp_text.name == "reference" && fp_text.value == *reference {
                        return true
                    }
                }
                _ => ()
            }
        }
        return false
    }
    pub fn set_reference(&mut self, reference:&String, reference2:&String) {
        //println!("debug: searching '{}'", reference);
        for ref mut element in &mut self.elements[..] {
            match **element {
                Element::FpText(ref mut fp_text) => {
                    if fp_text.name == "reference" && fp_text.value == *reference {
                        fp_text.value.clone_from(reference2);
                    }
                }
                _ => ()
            }
        }
    }
    pub fn at(&self) -> (f64, f64) {
        for element in &self.elements[..] {
            match *element {
                Element::At(ref at) => {
                    return (at.x, at.y)
                }
                _ => ()
            }
        }
        return (0.0, 0.0)
    }

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
        return (x1, y1, x2, y2)
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(f, "(module {}\n", self.name));
        for e in &self.elements {
            try!(write!(f, "    {}\n", e))
        };
        write!(f, ")")
    }
}

#[derive(Debug,Clone)]
pub enum Element {
    Layer(String),
    Descr(String),
    Tags(String),
    Attr(String),
    FpText(FpText),
    Pad(Pad),
    FpPoly(FpPoly),
    FpLine(FpLine),
    FpCircle(FpCircle),
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

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Element::Layer(ref s) => write!(f, "(layer {})", s),
            Element::Descr(ref s) => write!(f, "(descr {})", rustysexp::display_string(s)),
            Element::Tags(ref s) => write!(f, "(tags {})", rustysexp::display_string(s)),
            Element::Attr(ref s) => write!(f, "(attr {})", rustysexp::display_string(s)),
            Element::FpText(ref p) => write!(f, "{}",p),
            Element::Pad(ref pad) => write!(f, "{}", pad),
            Element::FpPoly(ref p) => write!(f, "{}", p),
            Element::FpLine(ref p) => write!(f, "{}", p),
            Element::FpCircle(ref p) => write!(f, "{}", p),
            Element::TEdit(ref p) => write!(f, "(tedit {})", p),
            Element::TStamp(ref p) => write!(f, "(tstamp {})", p),
            Element::Path(ref p) => write!(f, "(path {})", p),
            Element::At(ref p) => write!(f, "{}", p),
            Element::Model(ref p) => write!(f, "{}", p),
            Element::Locked => write!(f, "locked"),
        }
    }
}

#[derive(Debug,Clone)]
pub struct FpText {
    name: String,
    value: String,
    at: At,
    layer: Layer,
    effects: Effects,
    hide: bool,
}

impl FpText {
    fn new(name: String, value: String) -> FpText {
        FpText {
            name: name,
            value: value,
            at: At::new_empty(),
            layer: Layer::default(),
            effects: Effects::new(),
            hide: false
        }
    }
    fn set_effects(&mut self, effects: &Effects) {
        self.effects.clone_from(effects)
    }
    fn set_layer(&mut self, layer: &Layer) {
        self.layer.clone_from(layer)
    }
}

impl fmt::Display for FpText {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "(fp_text {} {} {} (layer {}){} {})", self.name, rustysexp::display_string(&self.value), self.at, self.layer, if self.hide { " hide" } else { "" }, self.effects)
    }
}

#[derive(Debug,Clone)]
pub struct At {
    x: f64,
    y: f64,
    rot: f64
}

impl At {
    fn new(x:f64 ,y:f64, rot:f64) -> At {
        At { x:x, y:y, rot:rot }
    }
    fn new_empty() -> At {
        At { x:0.0, y:0.0, rot:0.0 }
    }
}

impl fmt::Display for At {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        if self.rot == 0.0 {
            write!(f, "(at {} {})", self.x, self.y)
        } else {
            write!(f, "(at {} {} {})", self.x, self.y, self.rot)
        }
    }
}

#[derive(Debug,Clone)]
pub struct Font {
    size: Xy,
    thickness: f64,
}

impl Font {
    fn new() -> Font {
        Font { size: Xy::new(0.0, 0.0, XyType::Size), thickness: 0.0 }
    }
}


impl fmt::Display for Font {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "(font (size {} {}) (thickness {}))", self.size.x, self.size.y, self.thickness)
    }
}

#[derive(Debug,Clone)]
pub struct Effects {
    font: Font,
    justify:Option<Justify>,
}


#[derive(Debug,Clone)]
pub enum Justify {
    Mirror,
}

impl Effects {
    fn new() -> Effects {
        Effects { font: Font::new(), justify:None }
    }
    fn from_font(font: Font, justify: Option<Justify>) -> Effects {
        Effects { font: font, justify:justify }
    }
}

impl fmt::Display for Effects {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let justify = match self.justify {
            None => String::from(""),
            Some(ref j) => format!(" {}", j),
        };
        write!(f, "(effects (font (size {} {}) (thickness {})){})", self.font.size.x, self.font.size.y, self.font.thickness, justify)
    }
}

impl fmt::Display for Justify {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Justify::Mirror => write!(f,"(justify mirror)"),
        }
    }
}

#[derive(Debug,Clone,PartialEq)]
pub enum XyType {
    Xy,
    Start,
    End,
    Size,
    Center,
}

#[derive(Debug,Clone)]
pub struct Xy {
    x: f64,
    y: f64,
    t: XyType,
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

impl Pts {
    fn new() -> Pts { Pts { elements:vec![] } }
}


impl fmt::Display for Xy {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.t {
            XyType::Xy => write!(f,"(xy {} {})", self.x, self.y),
            XyType::Start => write!(f,"(start {} {})", self.x, self.y),
            XyType::End => write!(f,"(end {} {})", self.x, self.y),
            XyType::Size => write!(f,"(size {} {})", self.x, self.y),
            XyType::Center => write!(f,"(center {} {})", self.x, self.y),
        }
    }
}
impl fmt::Display for Pts {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(f, "(pts"));
        for x in &self.elements {
            try!(write!(f, " {}", x));
        }
        write!(f,")")
    }
}

#[derive(Clone,Debug)]
pub struct Drill {
    shape:Option<String>,
    drill:f64,
    drill2:Option<f64>,
}

impl fmt::Display for Drill {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(f,"(drill "));
        match self.shape {
            Some(ref s) => try!(write!(f, "{} ", s)),
            None => (),
        }
        try!(write!(f,"{}", self.drill));
        match self.drill2 {
            Some(ref d2) => try!(write!(f, " {}", d2)),
            None => (),
        }
        write!(f,")")
    }
}

#[derive(Debug)]
enum Part {
    At(At),
    Layer(Layer),
    Hide,
    Effects(Effects),
    Layers(Layers),
    Width(f64),
    Xy(Xy),
    Pts(Pts),
    Thickness(f64),
    Net(Net),
    Drill(Drill),
    SolderPasteMargin(f64),
}

// as parts are intermediate only Display doesn't really matter
impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Part::At(ref at)           => write!(f, "{}", at),
            Part::Layer(ref layer)     => write!(f, "(layer {})", layer),
            Part::Hide                 => write!(f, "hide"),
            Part::Effects(ref effects) => write!(f, "{}", effects),
            Part::Layers(_)            => write!(f, "(layers TODO"),
            Part::Width(ref w)         => write!(f, "(width {}", w),
            Part::Xy(ref xy)           => write!(f, "{}", xy),
            Part::Pts(_)               => write!(f, "(pts TODO)"),
            Part::Thickness(ref x)     => write!(f, "(thickness {})", x),
            Part::Net(ref x)           => write!(f, "{}", x),
            Part::Drill(ref x)         => write!(f, "(drill {})", x),
            Part::SolderPasteMargin(ref x) => write!(f, "(solder_paste_margin {})", x),
            
        }
    }
}


#[derive(Debug,Clone)]
pub enum PadType {
    Smd,
    Pth,
    NpPth,
}

impl PadType {
    fn from_string(s: String) -> ERes<PadType> {
        match &s[..] {
            "smd" => Ok(PadType::Smd),
            "thru_hole" => Ok(PadType::Pth),
            "np_thru_hole" => Ok(PadType::NpPth),
            x => Err(format!("unknown PadType {}", x))
        }
    }
}

impl fmt::Display for PadType {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            PadType::Smd => write!(f, "smd"),
            PadType::Pth => write!(f, "thru_hole"),
            PadType::NpPth => write!(f, "np_thru_hole"),
        }
    }
}

#[derive(Debug,Clone)]
pub enum PadShape {
    Rect,
    Circle,
    Oval,
    // TODO
}

impl PadShape {
    fn from_string(s: String) -> ERes<PadShape> {
        match &s[..] {
            "rect" => Ok(PadShape::Rect),
            "circle" => Ok(PadShape::Circle),
            "oval" => Ok(PadShape::Oval),
            x => Err(format!("unknown PadShape: {}", x))
        }
    }
}

impl fmt::Display for PadShape {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            PadShape::Rect => write!(f, "rect"),
            PadShape::Circle => write!(f, "circle"),
            PadShape::Oval => write!(f, "oval"),
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
    side: LayerSide,
    t: LayerType,
}

impl Layer {

    pub fn new() -> Layer {
        Layer { side:LayerSide::Front, t:LayerType::Cu }
    }
    
    pub fn from_string(s: String) -> ERes<Layer> {
        let sp:Vec<&str> = s.split('.').collect();
        let mut side = LayerSide::None;
        let mut s_t = sp[0];
        if sp.len() == 2 {
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
                x => return Err(format!("unknown layer side {}", x)),
            };
            s_t = sp[1];
        }
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

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(match self.side {
            LayerSide::Front => write!(f, "F."),
            LayerSide::Back  => write!(f, "B."),
            LayerSide::Dwgs  => write!(f, "Dwgs."),
            LayerSide::Cmts  => write!(f, "Cmts."),
            LayerSide::Eco1  => write!(f, "Eco1."),
            LayerSide::Eco2  => write!(f, "Eco2."),
            LayerSide::Edge  => write!(f, "Edge."),
            LayerSide::In1   => write!(f, "In1."),
            LayerSide::In2   => write!(f, "In2."),
            LayerSide::Both  => write!(f, "*."),
            LayerSide::None  => Ok(()),
        });
        match self.t {
            LayerType::Cu    => write!(f,"Cu"),
            LayerType::Paste => write!(f,"Paste"),
            LayerType::Mask  => write!(f,"Mask"),
            LayerType::SilkS => write!(f,"SilkS"),
            LayerType::User  => write!(f,"User"),
            LayerType::Adhes => write!(f,"Adhes"),
            LayerType::Cuts  => write!(f,"Cuts"),
            LayerType::CrtYd => write!(f,"CrtYd"),
            LayerType::Fab   => write!(f,"Fab"),
            LayerType::Margin => write!(f,"Margin"),
            LayerType::Other(ref x) => write!(f, "{}", x),
        }
    }
}

#[derive(Debug,Clone)]
pub struct Layers {
    layers: Vec<Layer>,
}

impl Layers {
    fn new() -> Layers {
        Layers {
            layers: vec![]
        }
    }
    fn append(&mut self, layer: &Layer) {
        self.layers.push(layer.clone())
    }
}

impl fmt::Display for Layers {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let len = self.layers.len();
        if len == 0 {
            return write!(f,"(layers)"); // or nothing?
        }
        try!(write!(f, "(layers"));
        for layer in &self.layers[..] {
            try!(write!(f, " {}", layer))
        }
        write!(f, ")")
    }
}

#[derive(Debug,Clone)]
pub struct Pad {
    pub name: String,
    pub t: PadType,
    pub shape: PadShape,
    pub size: Xy,
    pub at: At,
    pub layers: Layers,
    pub net: Option<Net>,
    pub drill: Option<Drill>,
    pub solder_paste_margin: Option<f64>,
}

impl Pad {
    fn new(name: String, t:PadType, shape: PadShape) -> Pad {
        Pad {
            name: name,
            t: t,
            shape: shape,
            size: Xy::new_empty(XyType::Size),
            at: At::new_empty(),
            layers: Layers::new(),
            net:None,
            drill:None,
            solder_paste_margin:None,
        }
    }

    fn set_net(&mut self, net:&Net) {
        self.net = Some(net.clone())
    }
    fn set_drill(&mut self, drill:&Drill) {
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

impl fmt::Display for Pad {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let net = match self.net {
            None => String::from(""),
            Some(ref net) => format!(" {}", net),
        };
        let drill = match self.drill {
            None => String::from(""),
            Some(ref drill) => format!(" {}", drill),
        };
        let spm = match self.solder_paste_margin {
            None => String::from(""),
            Some(spm_f) => format!(" (solder_paste_margin {})", spm_f),
        };
        write!(f, "(pad {} {} {} {} {} {}{}{}{})", self.name, self.t, self.shape, self.size, self.at, self.layers, net, drill, spm)
    }
}

#[derive(Debug,Clone)]
pub struct FpPoly {
    pts:Pts,
    width:f64,
    layer:Layer,
}

impl FpPoly {
    fn new() -> FpPoly {
        FpPoly { pts:Pts::new(), width:0.0, layer:Layer::default() }
    }

    fn bounding_box(&self) -> (f64,f64,f64,f64) {
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

impl fmt::Display for FpPoly {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let l = self.pts.elements.len();
        if l == 0 {
            return Ok(())
        }
        try!(write!(f, "(fp_poly {}", self.pts));
        write!(f, " (layer {}) (width {}))", self.layer, self.width)
    }
}

#[derive(Debug,Clone)]
pub struct FpLine {
    start:Xy,
    end:Xy,
    layer:Layer,
    width:f64,
}

impl FpLine {
    fn new() -> FpLine {
        FpLine { start:Xy::new_empty(XyType::Start), end:Xy::new_empty(XyType::End), layer:Layer::default(), width:0.0 }
    }
    
    fn bounding_box(&self) -> (f64,f64,f64,f64) {
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


impl fmt::Display for FpLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "(fp_line {} {} (layer {}) (width {}))", self.start, self.end, self.layer, self.width)
    }
}

impl FpCircle {
    fn new() -> FpCircle {
        FpCircle { center:Xy::new_empty(XyType::Center), end:Xy::new_empty(XyType::End), layer:Layer::default(), width:0.0 }
    }
    fn bounding_box(&self) -> (f64,f64,f64,f64) {
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
pub struct FpCircle {
    center:Xy,
    end:Xy,
    layer:Layer,
    width:f64,
}

impl fmt::Display for FpCircle {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "(fp_circle {} {} (layer {}) (width {}))", self.center, self.end, self.layer, self.width)
    }
}

#[derive(Debug,Clone)]
pub struct Net {
    pub num: i64,
    pub name: String,
}

impl fmt::Display for Net {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "(net {} \"{}\")", self.num, self.name)
    }
}

#[derive(Debug,Clone)]
pub struct Model {
    name: String,
    at: Xyz,
    scale: Xyz,
    rotate: Xyz,
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "(model {} (at {}) (scale {}) (rotate {}))", self.name, self.at, self.scale, self.rotate)
    }
}

#[derive(Debug,Clone)]
pub struct Xyz {
    x:f64,
    y:f64,
    z:f64,
}

impl Xyz {
    fn new(x:f64, y:f64, z:f64) -> Xyz {
        Xyz { x:x, y:y, z:z, }
    }
}


impl fmt::Display for Xyz {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "(xyz {} {} {})", self.x, self.y, self.z)
    }
}

// (at 0.0 -4.0) (at -2.575 -1.625 180)
impl FromSexp for ERes<At> {
    fn from_sexp(s:&Sexp) -> ERes<At> {
        let v = try!(s.slice_atom("at"));
        match v.len() {
            2 => {
                let x = try!(v[0].f());
                let y = try!(v[1].f());
                Ok(At::new(x, y, 0.0))
            }
            3 => {
                let x = try!(v[0].f());
                let y = try!(v[1].f());
                let rot = try!(v[2].f());
                Ok(At::new(x, y, rot))
            }
            _ => err("at with wrong length")
        }
    }
}

pub fn wrap<X,Y,F,G>(s:&Sexp, make:F, wrapper:G) -> ERes<Y>
    where F:Fn(&Sexp) -> ERes<X>, G:Fn(X) -> Y
{
    Ok(wrapper(try!(make(s))))
}


impl FromSexp for ERes<Layer> {
    fn from_sexp(s:&Sexp) -> ERes<Layer> {
        let v = try!(s.slice_atom("layer"));
        let layer = try!(v[0].string());
        let layer = try!(Layer::from_string(layer.clone()));
        Ok(layer)
    }
}

impl FromSexp for ERes<Effects> {
    fn from_sexp(s:&Sexp) -> ERes<Effects> {
        //let v = try!(s.slice_atom_num("effects", 1));
        // TODO investigate why the above doesn't work !?
        let v = try!(s.slice_atom("effects"));
        if v.len() < 1 {
            return Err(format!("Expected at least one element in {}", s))
        }
        let font = try!(ERes::from_sexp(&v[0]));
        let justify = if v.len() > 1 {
            Some(try!(ERes::from_sexp(&v[1])))
        } else {
            None
        };
        Ok(Effects::from_font(font, justify))
    }
}

impl FromSexp for ERes<Justify> {
    fn from_sexp(s:&Sexp) -> ERes<Justify> {
        let v = try!(s.slice_atom("justify"));
        if v.len() < 1 {
            return Err(format!("Expected at least one element in {}", s))
        }
        let s = try!(v[0].string());
        match &s[..] {
            "mirror" => Ok(Justify::Mirror),
            _ => Err(format!("unknown justify: {}", s))
        }
    }
}

impl FromSexp for ERes<Font> {
    fn from_sexp(s:&Sexp) -> ERes<Font> {
        let v = try!(s.slice_atom("font"));
        let parts = try!(parse_parts(&v));
        let mut font = Font::new();
        for part in &parts[..] {
            //println!("part: {}", part);
            try!(match *part {
                Part::Xy(ref xy) if xy.t == XyType::Size => {
                    font.size.x = xy.x;
                    font.size.y = xy.y;
                    Ok(())
                }
                Part::Thickness(ref t) => {
                    font.thickness = *t;
                    Ok(())
                }
                ref x => Err(format!("unknown element in font: {}", x))
            })
        }
        Ok(font)
    }
}

impl FromSexp for ERes<Layers> {
    fn from_sexp(s:&Sexp) -> ERes<Layers> {
        let mut l = Layers::new();
        let v = try!(s.slice_atom("layers"));
        for v1 in v {
            let x = try!(v1.string());
            let layer = try!(Layer::from_string(x.clone()));
            l.append(&layer)
        }
        Ok(l)
    }
}

fn parse_part_float<F>(e: &Sexp, make:F) -> ERes<Part>
    where F:Fn(f64) -> Part
{
    let v = try!(e.list());
    if v.len() < 2 {
        return Err(format!("not enough elements in {}", e))
    }
    let f = try!(v[1].f());
    Ok(make(f))
}

impl FromSexp for ERes<Vec<Xy> > {
    fn from_sexp(s:&Sexp) -> ERes<Vec<Xy> > {
        let v = try!(s.slice_atom("pts"));
        let mut pts = vec![];
        for e in &v[1..] {
            let p = try!(ERes::from_sexp(e));
            pts.push(p)
        }
        Ok(pts)
    }
}

impl FromSexp for ERes<Xy> {
    fn from_sexp(s: &Sexp) -> ERes<Xy> {
        let name = try!(s.list_name());
        let t = try!(match &name[..] {
            "xy" => Ok(XyType::Xy),
            "start" => Ok(XyType::Start),
            "end" => Ok(XyType::End),
            "size" => Ok(XyType::Size),
            "center" => Ok(XyType::Center),
            ref x => Err(format!("unknown XyType {}", x)),
        });
        let v = try!(s.slice_atom_num(&name, 2));
        let x = try!(v[0].f());
        let y = try!(v[1].f());
        Ok(Xy::new(x,y,t))
    }
}

impl FromSexp for ERes<Pts> {
    fn from_sexp(s: &Sexp) -> ERes<Pts> {
        let v = try!(s.slice_atom("pts"));
        let mut r = vec![];
        for x in v {
            let xy = try!(ERes::from_sexp(x));
            r.push(xy)
        }
        Ok(Pts { elements:r })
    }
}


impl FromSexp for ERes<Xyz> {
    fn from_sexp(s: &Sexp) -> ERes<Xyz> {
        let v = try!(s.slice_atom_num("xyz", 3));
        let x = try!(v[0].f());
        let y = try!(v[1].f());
        let z = try!(v[2].f());
        Ok(Xyz::new(x,y,z))
    }
}

impl FromSexp for ERes<Net> {
    fn from_sexp(s: &Sexp) -> ERes<Net> {
        let v = try!(s.slice_atom_num("net", 2));
        let num = try!(v[0].i());
        let name = try!(v[1].string());
        Ok(Net { num:num, name:name.clone(), })
    }
}

impl FromSexp for ERes<Drill> {
    fn from_sexp(s: &Sexp) -> ERes<Drill> {
        let v = try!(s.slice_atom("drill"));
        if v.len() == 1 {
            let drill = try!(v[0].f());
            Ok(Drill { shape:None, drill:drill, drill2:None })
                
        } else if v.len() == 3 {
            let shape = try!(v[0].string());
            let drill = try!(v[1].f());
            let drill2 = try!(v[2].f());
            Ok(Drill { shape:Some(shape.clone()), drill:drill, drill2:Some(drill2) })
        } else {
            Err(format!("unknown drill format"))
        }
    }
}

impl FromSexp for ERes<Part> {
    fn from_sexp(s:&Sexp) -> ERes<Part> {
        match s.string() {
            Ok(ref sx) => match &sx[..] {
                "hide" => Ok(Part::Hide),
                x => Err(format!("unknown part in element: {}", x))
            },
            _ => {
                let name = &try!(s.list_name())[..];
                match name {
                    "at" => wrap(s, ERes::from_sexp, Part::At),
                    "layer" => wrap(s, ERes::from_sexp, Part::Layer),
                    "effects" => wrap(s, ERes::from_sexp, Part::Effects),
                    "layers" => wrap(s, ERes::from_sexp, Part::Layers),
                    "width" => parse_part_float(s, Part::Width),
                    "start" => wrap(s, ERes::from_sexp, Part::Xy),
                    "end" => wrap(s, ERes::from_sexp, Part::Xy),
                    "size" => wrap(s, ERes::from_sexp, Part::Xy),
                    "center" => wrap(s, ERes::from_sexp, Part::Xy),
                    "pts" => wrap(s, ERes::from_sexp, Part::Pts),
                    "thickness" => parse_part_float(s, Part::Thickness),
                    "net" => wrap(s, ERes::from_sexp, Part::Net),
                    "drill" => wrap(s, ERes::from_sexp, Part::Drill),
                    "solder_paste_margin" => parse_part_float(s, Part::SolderPasteMargin),
                    x => Err(format!("unknown part {}", x))
                }
            }
        }
    }        
}

fn parse_parts(v: &[Sexp]) -> ERes<Vec<Part>> {
    let mut res = Vec::new();
    for e in v {
        let p = try!(ERes::from_sexp(e));
        res.push(p);
    }
    Ok(res)
}

fn parse_string_element(s:&Sexp) -> ERes<String> {
    let name = try!(s.list_name());
    let v = try!(s.slice_atom_num(&name, 1));
    let s = try!(v[0].string());
    Ok(s.clone())
}

impl FromSexp for ERes<FpText> {
    fn from_sexp(s:&Sexp) -> ERes<FpText> {
        let v = try!(s.slice_atom("fp_text"));
        let name = try!(v[0].string());
        let value = try!(v[1].string());
        let parts = try!(parse_parts(&v[2..]));
        let mut fp = FpText::new(name.clone(), value.clone());
        for part in &parts[..] {
            match *part {
                Part::At(ref at) => {
                    fp.at.clone_from(at)
                },
                Part::Layer(ref layer) => {
                    fp.set_layer(layer)
                }
                Part::Hide => {
                    fp.hide = true
                },
                Part::Effects(ref effects) => {
                    fp.set_effects(effects)
                }
                ref x => {
                    return Err(format!("fp_text: unknown {}", x))
                }
            }
        }
        Ok(fp)
    }
}

impl FromSexp for ERes<Pad> {
    fn from_sexp(s:&Sexp) -> ERes<Pad> {
        let v = try!(s.slice_atom("pad"));
        if v.len() < 3 {
            return Err(format!("not enough elements in pad"))
        }
        let name = try!(v[0].string()).clone();
        let t = try!(v[1].string());
        let t = try!(PadType::from_string(t.clone()));
        let shape = try!(v[2].string());
        let shape = try!(PadShape::from_string(shape.clone()));
        let mut pad = Pad::new(name, t, shape);
        //println!("{}", pad);
        let parts = try!(parse_parts(&v[3..]));
        for part in &parts[..] {
            match *part {
                Part::At(ref at) => pad.at.clone_from(at),
                Part::Xy(ref xy) if xy.t == XyType::Size => pad.size.clone_from(xy),
                Part::Layers(ref l) => pad.layers.clone_from(l),
                Part::Net(ref n) => pad.set_net(n),
                Part::Drill(ref n) => pad.set_drill(n),
                Part::SolderPasteMargin(n) => pad.solder_paste_margin = Some(n),
                ref x => return Err(format!("pad: unknown {}", x)),
            }
        }
        Ok(pad)
    }
}

impl FromSexp for ERes<FpPoly> {
    fn from_sexp(s:&Sexp) -> ERes<FpPoly> {
        let v = try!(s.slice_atom("fp_poly"));
        let mut fp_poly = FpPoly::new();
        let parts = try!(parse_parts(&v));
        for part in &parts[..] {
            match *part {
                Part::Pts(ref pts) => fp_poly.pts.clone_from(pts),
                Part::Width(w) => fp_poly.width = w,
                Part::Layer(ref layer) => fp_poly.layer.clone_from(layer),
                ref x => println!("fp_poly: ignoring {}", x),
            }
        } 
        Ok(fp_poly)
    }
}

impl FromSexp for ERes<FpLine> {
    fn from_sexp(s:&Sexp) -> ERes<FpLine> {
        let v = try!(s.slice_atom("fp_line"));
        let mut fp_line = FpLine::new();
        let parts = try!(parse_parts(&v));
        for part in &parts[..] {
            match *part {
                Part::Xy(ref xy) if xy.t == XyType::Start => fp_line.start.clone_from(xy),
                Part::Xy(ref xy) if xy.t == XyType::End => fp_line.end.clone_from(xy),
                Part::Layer(ref layer) => fp_line.layer.clone_from(layer),
                Part::Width(w) => fp_line.width = w,
                ref x => return Err(format!("fp_line: unknown {}", x)),
            }
        }
        Ok(fp_line)
    }
}

impl FromSexp for ERes<FpCircle> {
    fn from_sexp(s:&Sexp) -> ERes<FpCircle> {
        let v = try!(s.slice_atom("fp_circle"));
        let mut fp_circle = FpCircle::new();
        let parts = try!(parse_parts(&v));
        for part in &parts[..] {
            match *part {
                Part::Xy(ref xy) if xy.t == XyType::Center => fp_circle.center.clone_from(xy),
                Part::Xy(ref xy) if xy.t == XyType::End => fp_circle.end.clone_from(xy),
                Part::Layer(ref layer) => fp_circle.layer.clone_from(layer),
                Part::Width(w) => fp_circle.width = w,
                ref x => return Err(format!("fp_circle: unexpected {}", x)),
            }
        }
        Ok(fp_circle)
    }
}

fn parse_sublist<X>(s:&Sexp, name:&'static str) -> ERes<X>
    where ERes<X>:FromSexp {
    let x = &try!(s.slice_atom_num(name, 1))[0];
    ERes::from_sexp(x)
}


impl FromSexp for ERes<Model> {
    fn from_sexp(s:&Sexp) -> ERes<Model> {
        let v = try!(s.slice_atom_num("model", 4));
        let name = try!(v[0].string()).clone();
        let at = try!(parse_sublist(&v[1], "at"));
        let scale = try!(parse_sublist(&v[2], "scale"));
        let rotate = try!(parse_sublist(&v[3], "rotate"));
        let m = Model {name:name, at:at, scale:scale, rotate:rotate};
        Ok(m)
    }
}

impl FromSexp for ERes<Element> {
    fn from_sexp(s:&Sexp) -> ERes<Element> {
        match s.element {
            rustysexp::Element::String(ref s) => {
                match &s[..] {
                    "locked" => Ok(Element::Locked),
                    _ => Err(format!("unknown element in module: {}", s))
                }
            },
            rustysexp::Element::List(_) => {
                let name = try!(s.list_name());
                match &name[..] {
                    "layer" => wrap(s, parse_string_element, Element::Layer),
                    "descr" => wrap(s, parse_string_element, Element::Descr),
                    "tags" => wrap(s, parse_string_element, Element::Tags),
                    "attr" => wrap(s, parse_string_element, Element::Attr),
                    "fp_text" => wrap(s, ERes::from_sexp, Element::FpText),
                    "pad" => wrap(s, ERes::from_sexp, Element::Pad),
                    "fp_poly" => wrap(s, ERes::from_sexp, Element::FpPoly),
                    "fp_line" => wrap(s, ERes::from_sexp, Element::FpLine),
                    "fp_circle" => wrap(s, ERes::from_sexp, Element::FpCircle),
                    "tedit" => wrap(s, parse_string_element, Element::TEdit),
                    "tstamp" => wrap(s, parse_string_element, Element::TStamp),
                    "path" => wrap(s, parse_string_element, Element::Path),
                    "at" => wrap(s, ERes::from_sexp, Element::At),
                    "model" => wrap(s, ERes::from_sexp, Element::Model),
                    _ => Err(format!("unknown element in module: {}", name))
                }
            },
            rustysexp::Element::Empty => unreachable!(),
        }
    }
}

impl FromSexp for ERes<Module> {
    fn from_sexp(s:&Sexp) -> ERes<Module> {
        let v = try!(s.slice_atom("module"));
        if v.len() < 1 {
            return Err(String::from("no name in module"));
        }
        let name = try!(v[0].string());
        let mut module = Module::new(name.clone());
        for e in &v[1..] {
            let el = try!(ERes::from_sexp(&e));
            module.append(el)
        }
        Ok(module)
    }
}

pub fn parse(s: &str) -> ERes<Module> {
    match rustysexp::parse_str(s) {
        Ok(s) => ERes::from_sexp(&s),
        Err(x) => Err(format!("IOError: {}", x))
    }
}
