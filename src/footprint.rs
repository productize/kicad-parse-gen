// (c) 2015-2016 Joost Yervante Damad <joost@productize.be>

use std::fmt;

// get from parent
use ERes;
use err;
use read_file;

extern crate rustysexp;
use self::rustysexp::Sexp;

macro_rules! fail {
    ($expr:expr) => (
        return Err(::std::error::FromError::from_error($expr));
        )
}

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
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Element::Layer(ref s) => write!(f, "(layer {})", s),
            Element::Descr(ref s) => write!(f, "(descr {})", rustysexp::display_string(s)),
            Element::Tags(ref s) => write!(f, "(tags {})", rustysexp::display_string(s)),
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
    font: Font
}

impl Effects {
    fn new() -> Effects {
        Effects { font: Font::new() }
    }
    fn from_font(font: Font) -> Effects {
        Effects { font: font }
    }
}

impl fmt::Display for Effects {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "(effects (font (size {} {}) (thickness {})))", self.font.size.x, self.font.size.y, self.font.thickness)
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

impl Xy {
    fn new(x: f64, y: f64, t: XyType) -> Xy {
        Xy { x:x, y:y, t:t }
    }
    fn new_empty(t: XyType) -> Xy {
        Xy { x:0.0, y:0.0, t:t }
    }
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
    Pts(Vec<Xy>),
    Thickness(f64),
    Net(Net),
    Drill(Drill),
    SolderPasteMargin(f64),
}

impl Part {
    fn xy(&self) -> ERes<Xy> {
        match *self {
            Part::Xy(ref xy) => Ok(xy.clone()),
            ref x => Err(format!("expecting xy got {}", x))
        }
    }
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
enum LayerSide {
    Front,
    Back,
    Dwgs,
    Cmts,
    Eco1,
    Eco2,
    Edge,
    Both,
    None,
}

#[derive(Debug,Clone)]
enum LayerType {
    Cu,
    Paste,
    Mask,
    SilkS,
    User,
    Adhes,
    Cuts,
    CrtYd,
    Fab,
}

#[derive(Debug,Clone)]
struct Layer {
    side: LayerSide,
    t: LayerType,
}

impl Layer {
    fn from_string(s: String) -> ERes<Layer> {
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
            x => return Err(format!("unknown layer type {}", x)),
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
    pts:Vec<Xy>,
    width:f64,
    layer:Layer,
}

impl FpPoly {
    fn new() -> FpPoly {
        FpPoly { pts:vec![], width:0.0, layer:Layer::default() }
    }
}

impl fmt::Display for FpPoly {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let l = self.pts.len();
        if l == 0 {
            return Ok(())
        }
        try!(write!(f, "(fp_poly (pts"));
        for x in &self.pts[..] {
            try!(write!(f, " {}", x))
        }
        write!(f, ") (layer {}) (width {}))", self.layer, self.width)
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
fn parse_at(s: &Sexp) -> ERes<At> {
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

fn wrap<X,Y,F,G>(s:&Sexp, make:F, wrapper:G) -> ERes<Y>
    where F:Fn(&Sexp) -> ERes<X>, G:Fn(X) -> Y
{
    Ok(wrapper(try!(make(s))))
}


fn parse_part_layer(v: &Vec<Sexp>) -> ERes<Part> {
    let layer = try!(v[1].string());
    let layer = try!(Layer::from_string(layer.clone()));
    Ok(Part::Layer(layer))
}

fn parse_part_effects(v: &Vec<Sexp>) -> ERes<Part> {
    let l = try!(v[1].list());
    //for x in &l[..] {
    //    println!("effects el: {}", x)
    //}
    let f = try!(l[0].string());
    if &f[..] != "font" {
        return err("expecting font")
    }
    let parts = try!(parse_parts(&l[1..]));
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
    Ok(Part::Effects(Effects::from_font(font)))
}

fn parse_part_layers(v: &Vec<Sexp>) -> ERes<Part> {
    let mut l = Layers::new();
    for v1 in &v[1..] {
        let x = try!(v1.string());
        let layer = try!(Layer::from_string(x.clone()));
        l.append(&layer)
    }
    Ok(Part::Layers(l))
}

fn parse_part_float<F>(v: &Vec<Sexp>, make:F) -> ERes<Part>
    where F:Fn(f64) -> Part
{
    let f = try!(v[1].f());
    Ok(make(f))
}

fn parse_part_pts(v: &Vec<Sexp>) -> ERes<Part> {
    let mut pts = vec![];
    for e in &v[1..] {
        let p = try!(ERes::from_sexp(e));
        pts.push(p)
    }
    Ok(Part::Pts(pts))
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

impl FromSexp for ERes<Xyz> {
    fn from_sexp(s: &Sexp) -> ERes<Xyz> {
        let v = try!(s.slice_atom_num("xyz", 3));
        let x = try!(v[0].f());
        let y = try!(v[1].f());
        let z = try!(v[2].f());
        Ok(Xyz::new(x,y,z))
    }
}

fn parse_part_net(v: &Vec<Sexp>) -> ERes<Part> {
    let num = try!(v[1].i());
    let name = try!(v[2].string());
    Ok(Part::Net(Net { num:num, name:name.clone(), }))
}

fn parse_drill(v: &Vec<Sexp>) -> ERes<Drill> {
    if v.len() == 2 {
        let drill = try!(v[1].f());
        Ok(Drill { shape:None, drill:drill, drill2:None })
        
    } else if v.len() == 4 {
        let shape = try!(v[1].string());
        let drill = try!(v[2].f());
        let drill2 = try!(v[3].f());
        Ok(Drill { shape:Some(shape.clone()), drill:drill, drill2:Some(drill2) })
    } else {
        Err(format!("unknown drill format"))
    }
}

fn parse_part_list(s:&Sexp, v: &Vec<Sexp>) -> ERes<Part> {
    let name = &try!(v[0].string())[..];
    //println!("name: {}", name);
    match name {
        "at" => wrap(s, parse_at, Part::At),
        "layer" => parse_part_layer(v),
        "effects" => parse_part_effects(v),
        "layers" => parse_part_layers(v),
        "width" => parse_part_float(v, Part::Width),
        "start" => wrap(s, ERes::from_sexp, Part::Xy),
        "end" => wrap(s, ERes::from_sexp, Part::Xy),
        "size" => wrap(s, ERes::from_sexp, Part::Xy),
        "center" => wrap(s, ERes::from_sexp, Part::Xy),
        "pts" => parse_part_pts(v),
        "thickness" => parse_part_float(v, Part::Thickness),
        "net" => parse_part_net(v),
        "drill" => Ok(Part::Drill(try!(parse_drill(v)))),
        "solder_paste_margin" => parse_part_float(v, Part::SolderPasteMargin),
        x => Err(format!("unknown part {}", x))
    }
}

fn parse_part(part: &Sexp) -> ERes<Part> {
    match part.element {
        rustysexp::Element::String(ref s) => {
            match &s[..] {
                "hide" => Ok(Part::Hide),
                x => Err(format!("unknown part in element: {}", x))
            }
        },
        rustysexp::Element::List(ref v) => parse_part_list(part, &v),
        _ => Err(format!("unknown part in element: {}", part))
    }
}

fn parse_parts(v: &[Sexp]) -> ERes<Vec<Part>> {
    let mut res = Vec::new();
    for e in v {
        let p = try!(parse_part(e));
        //println!("{}", p);
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

fn parse_fp_text(s: &Sexp) -> ERes<Element> {
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
    Ok(Element::FpText(fp))
}
fn parse_pad(v: &Vec<Sexp>) -> ERes<Element> { 
    let name = try!(v[1].string()).clone();
    let t = try!(v[2].string());
    let t = try!(PadType::from_string(t.clone()));
    let shape = try!(v[3].string());
    let shape = try!(PadShape::from_string(shape.clone()));
    let mut pad = Pad::new(name, t, shape);
    //println!("{}", pad);
    let parts = try!(parse_parts(&v[4..]));
    for part in &parts[..] {
        match *part {
            Part::At(ref at) => pad.at.clone_from(at),
            Part::Xy(ref xy) if xy.t == XyType::Size => pad.size.clone_from(xy),
            Part::Layers(ref l) => pad.layers.clone_from(l),
            Part::Net(ref n) => pad.set_net(n),
            Part::Drill(ref n) => pad.set_drill(n),
            Part::SolderPasteMargin(n) => pad.solder_paste_margin = Some(n),
            ref x => println!("pad: ignoring {}", x),
        }
    }
    Ok(Element::Pad(pad))
}

fn parse_fp_poly(v: &Vec<Sexp>) -> ERes<Element> {
    let mut fp_poly = FpPoly::new();
    let parts = try!(parse_parts(&v[1..]));
    for part in &parts[..] {
        match *part {
            Part::Pts(ref pts) => fp_poly.pts.clone_from(pts),
            Part::Width(w) => fp_poly.width = w,
            Part::Layer(ref layer) => fp_poly.layer.clone_from(layer),
            ref x => println!("fp_poly: ignoring {}", x),
        }
    } 
    Ok(Element::FpPoly(fp_poly))
}
fn parse_fp_line(v: &Vec<Sexp>) -> ERes<Element> {
    let mut fp_line = FpLine::new();
    let parts = try!(parse_parts(&v[1..]));
    for part in &parts[..] {
        match *part {
            Part::Xy(ref xy) if xy.t == XyType::Start => fp_line.start.clone_from(xy),
            Part::Xy(ref xy) if xy.t == XyType::End => fp_line.end.clone_from(xy),
            Part::Layer(ref layer) => fp_line.layer.clone_from(layer),
            Part::Width(w) => fp_line.width = w,
            ref x => println!("fp_line: ignoring {}", x),
        }
    }
    Ok(Element::FpLine(fp_line))
}
fn parse_fp_circle(v: &Vec<Sexp>) -> ERes<Element> {
    let mut fp_circle = FpCircle::new();
    let parts = try!(parse_parts(&v[1..]));
    for part in &parts[..] {
        match *part {
            Part::Xy(ref xy) if xy.t == XyType::Center => fp_circle.center.clone_from(xy),
            Part::Xy(ref xy) if xy.t == XyType::End => fp_circle.end.clone_from(xy),
            Part::Layer(ref layer) => fp_circle.layer.clone_from(layer),
            Part::Width(w) => fp_circle.width = w,
            ref x => println!("fp_circle: ignoring {}", x),
        }
    }
    Ok(Element::FpCircle(fp_circle))
}

fn parse_sublist<X>(v:&[Sexp], name:&'static str) -> ERes<X>
    where ERes<X>:FromSexp {
    let x = &try!(v[1].slice_atom_num(name, 1))[0];
    ERes::from_sexp(x)
}


fn parse_model_element(s:&Sexp) -> ERes<Element> {
    let v = try!(s.slice_atom_num("model", 4));
    let name = try!(v[0].string()).clone();
    let at = try!(parse_sublist(v, "at"));
    let scale = try!(parse_sublist(v, "scale"));
    let rotate = try!(parse_sublist(v, "rotate"));
    let m = Model {name:name, at:at, scale:scale, rotate:rotate};
    Ok(Element::Model(m))
}

impl FromSexp for ERes<Element> {
    fn from_sexp(s:&Sexp) -> ERes<Element> {
        let name = try!(s.list_name());
        match &name[..] {
            "layer" => wrap(s, parse_string_element, Element::Layer),
            "descr" => wrap(s, parse_string_element, Element::Descr),
            "tags" => wrap(s, parse_string_element, Element::Tags),
            "fp_text" => parse_fp_text(s),
            //"pad" => parse_pad(v),
            //"fp_poly" => parse_fp_poly(v),
            //"fp_line" => parse_fp_line(v),
            //"fp_circle" => parse_fp_circle(v),
            "tedit" => wrap(s, parse_string_element, Element::TEdit),
            "tstamp" => wrap(s, parse_string_element, Element::TStamp),
            "path" => wrap(s, parse_string_element, Element::Path),
            "at" => wrap(s, parse_at, Element::At),
            "model" => parse_model_element(s),
            _ => Err(format!("unknown element in module: {}", name))
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

fn parse(s: &str) -> ERes<Module> {
    match rustysexp::parse_str(s) {
        Ok(s) => ERes::from_sexp(&s),
        Err(x) => Err(format!("IOError: {}", x))
    }
}

pub fn parse_str(s: &str) -> ERes<Module> {
    parse(s)
}

pub fn parse_file(name: &str) -> ERes<Module> {
    let s = try!(match read_file(name) {
        Ok(s) => Ok(s),
        Err(x) => Err(format!("io error: {}", x))
    });
    parse(&s[..])
}
