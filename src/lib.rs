// (c) 2015-2016 Joost Yervante Damad <joost@damad.be>

use std::fmt;
use std::fs::File;
use std::io::Read;

extern crate rustysexp;
use rustysexp::Sexp;
use rustysexp::Atom;

type ERes<T> = Result<T, String>;

fn err<T>(msg: &str) -> ERes<T> {
    Err(String::from(msg))
}

#[derive(Debug)]
pub struct Module {
    name: String,
    elements: Vec<Element>
}

impl Module {
    fn new(name: &String) -> Module {
        Module { name: name.clone(), elements: vec![] }
    }
    fn append(&mut self, e: Element) {
        self.elements.push(e)
    }
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        try!(write!(f, "(module {}\n", self.name));
        for e in &self.elements {
            try!(write!(f, "{}\n", e))
        };
        write!(f, ")")
    }
}

#[derive(Debug)]
pub enum Element {
    Layer(String),
    Descr(String),
    FpText(FpText),
    Pad(Pad),
    FpPoly,
    FpLine,
    FpCircle
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Element::Layer(ref s) => write!(f, "(layer {})", s),
            Element::Descr(ref s) => write!(f, "(descr \"{}\")", s),
            Element::FpText(ref p) => write!(f, "{}", p),
            Element::Pad(ref pad) => write!(f, "{}", pad),
            Element::FpPoly => write!(f, "(fp_poly)"),
            Element::FpLine => write!(f, "(fp_line)"),
            Element::FpCircle => write!(f, "(fp_circle)"),
        }
    }
}

#[derive(Debug)]
pub struct FpText {
    name: String,
    value: String,
    at: At,
    layer: Layer,
    effects: Effects,
    hide: bool,
}

impl FpText {
    fn new(name: &String, value: &String) -> FpText {
        FpText {
            name: name.clone(),
            value: value.clone(),
            at: At::new(0.,0.,0.),
            layer: Layer::default(),
            effects: Effects::new(),
            hide: false
        }
    }
    fn set_at(&mut self, at: &At) {
        self.at.set(at)
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
        write!(f, "(fp_text {} \"{}\" {} (layer {}){} {})", self.name, self.value, self.at, self.layer, if self.hide { " hide" } else { "" }, self.effects)
    }
}

#[derive(Clone)]
#[derive(Debug)]
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
    fn set(&mut self, at: &At) {
        self.x = at.x;
        self.y = at.y;
        self.rot = at.rot;
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

#[derive(Clone)]
#[derive(Debug)]
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

#[derive(Clone)]
#[derive(Debug)]
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

#[derive(PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
pub enum XyType {
    Xy,
    Start,
    End,
    Size,
}

#[derive(Clone)]
#[derive(Debug)]
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
        }
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
}

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
            
        }
    }
}


#[derive(Debug)]
enum PadType {
    Smd,
    Pth,
}

impl PadType {
    fn from_string(s: String) -> ERes<PadType> {
        match &s[..] {
            "smd" => Ok(PadType::Smd),
            "pth" => Ok(PadType::Pth),
            x => Err(format!("unknown PadType {}", x))
        }
    }
}

impl fmt::Display for PadType {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            PadType::Smd => write!(f, "smd"),
            PadType::Pth => write!(f, "pth"), // TODO
        }
    }
}

#[derive(Debug)]
enum PadShape {
    Rect,
    // TODO
}

impl PadShape {
    fn from_string(s: String) -> ERes<PadShape> {
        match &s[..] {
            "rect" => Ok(PadShape::Rect),
            x => Err(format!("unknown PadShape: {}", x))
        }
    }
}

impl fmt::Display for PadShape {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            PadShape::Rect => write!(f, "rect"),
        }
    }
}

#[derive(Clone)]
#[derive(Debug)]
enum LayerSide {
    Front,
    Back,
}

#[derive(Clone)]
#[derive(Debug)]
enum LayerType {
    Cu,
    Paste,
    Mask,
    SilkS,
    // TODO
}

#[derive(Clone)]
#[derive(Debug)]
struct Layer {
    side: LayerSide,
    t: LayerType,
}

impl Layer {
    fn from_string(s: String) -> ERes<Layer> {
        let side = match &s[0..2] {
            "F." => LayerSide::Front,
            "B." => LayerSide::Back,
            x => return Err(format!("unknown layer side {}", x))
        };
        let t = match &s[2..] {
            "Cu" => LayerType::Cu,
            "Paste" => LayerType::Paste,
            "Mask" => LayerType::Mask,
            "SilkS" => LayerType::SilkS,
            x => return Err(format!("unknown layer type {}", x))
        };
        Ok(Layer { side:side, t:t, })
    }
    fn default() -> Layer {
        Layer { side: LayerSide::Front, t: LayerType::Cu }
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let _ = match self.side {
            LayerSide::Front => write!(f, "F."),
            LayerSide::Back  => write!(f, "B."),
        };
        match self.t {
            LayerType::Cu    => write!(f,"Cu"),
            LayerType::Paste => write!(f,"Paste"),
            LayerType::Mask  => write!(f,"Mask"),
            LayerType::SilkS => write!(f,"SilkS"),
        }
    }
}

#[derive(Clone)]
#[derive(Debug)]
struct Layers {
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
            return Ok(())
        }
        for layer in &self.layers[..(len-1)] {
            try!(write!(f, "{} ", layer))
        }
        write!(f, "{}", self.layers[len-1])
    }
}

#[derive(Debug)]
pub struct Pad {
    name: String,
    t: PadType,
    shape: PadShape,
    size: Xy,
    at: At,
    layers: Layers,
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
        }
    }
    fn set_at(&mut self, at:&At) {
        self.at.clone_from(at)
    }
    fn set_size(&mut self, size:&Xy) {
        self.size.clone_from(size)
    }
    fn set_layers(&mut self, l:&Layers) {
        self.layers.clone_from(l)
    }
}

impl fmt::Display for Pad {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "(pad {} {} {} {} {} {})", self.name, self.t, self.shape, self.size, self.at, self.layers)
    }
}

// (at 0.0 -4.0) (at -2.575 -1.625 180)
fn parse_part_at(v: &Vec<Sexp>) -> ERes<Part> {
    match v.len() {
        3 => {
            let x = try!(try!(v[1].atom()).f());
            let y = try!(try!(v[2].atom()).f());
            Ok(Part::At(At::new(x, y, 0.0)))
        }
        4 => {
            let x = try!(try!(v[1].atom()).f());
            let y = try!(try!(v[2].atom()).f());
            let rot = try!(try!(v[3].atom()).f());
            Ok(Part::At(At::new(x, y, rot)))
        }
        _ => err("at with wrong length")
    }
}

fn parse_part_layer(v: &Vec<Sexp>) -> ERes<Part> {
    let layer = try!(try!(v[1].atom()).string());
    let layer = try!(Layer::from_string(layer));
    Ok(Part::Layer(layer))
}

fn parse_part_effects(v: &Vec<Sexp>) -> ERes<Part> {
    let l = try!(v[1].list());
    //for x in &l[..] {
    //    println!("effects el: {}", x)
    //}
    let f = try!(try!(l[0].atom()).string());
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
    Ok(Part::Hide)
}

fn parse_part_width(v: &Vec<Sexp>) -> ERes<Part> {
    let f = try!(try!(v[1].atom()).f());
    Ok(Part::Width(f))
}

fn parse_part_thickness(v: &Vec<Sexp>) -> ERes<Part> {
    let f = try!(try!(v[1].atom()).f());
    Ok(Part::Thickness(f))
}

fn parse_part_pts(v: &Vec<Sexp>) -> ERes<Part> {
    Ok(Part::Hide)
}

fn parse_part_xy(t: XyType, v: &Vec<Sexp>) -> ERes<Part> {
    let x = try!(try!(v[1].atom()).f());
    let y = try!(try!(v[2].atom()).f());
    Ok(Part::Xy(Xy::new(x,y,t)))
}

fn parse_part_list(v: &Vec<Sexp>) -> ERes<Part> {
    let name = &try!(try!(v[0].atom()).string())[..];
    //println!("name: {}", name);
    match name {
        "at" => parse_part_at(v),
        "layer" => parse_part_layer(v),
        "effects" => parse_part_effects(v),
        "layers" => parse_part_layers(v),
        "width" => parse_part_width(v),
        "start" => parse_part_xy(XyType::Start, v),
        "end" => parse_part_xy(XyType::End, v),
        "size" => parse_part_xy(XyType::Size, v),
        "pts" => parse_part_pts(v),
        "thickness" => parse_part_thickness(v),
        x => Err(format!("unknown part {}", x))
    }
}

fn parse_part(part: &Sexp) -> ERes<Part> {
    match *part {
        Sexp::Atom(Atom::S(ref s)) => {
            match &s[..] {
                "hide" => Ok(Part::Hide),
                x => Err(format!("unknown part in element: {}", x))
            }
        },
        Sexp::List(ref v) => parse_part_list(&v),
        ref x => Err(format!("unknown part in element: {}", x))
    }
}

fn parse_parts(v: &[Sexp]) -> ERes<Vec<Part>> {
    let mut res = Vec::new();
    for e in v {
        let p = try!(parse_part(e));
        println!("{}", p);
        res.push(p);
    }
    Ok(res)
}
    
fn parse_layer(v: &Vec<Sexp>) -> ERes<Element> {
    match v[1] {
        Sexp::Atom(Atom::S(ref s)) => {
            Ok(Element::Layer(s.clone()))
        }
        ref x => Err(format!("unexpected element in layer: {}", x))
    }
}

fn parse_descr(v: &Vec<Sexp>) -> ERes<Element> {
    match v[1] {
        Sexp::Atom(Atom::Q(ref s)) => {
            Ok(Element::Descr(s.clone()))
        }
        ref x => Err(format!("unexpected element in descr: {}", x))
    }
}

fn parse_fp_text(v: &Vec<Sexp>) -> ERes<Element> {
    // TODO: introduce try with error msg argument/fn
    //let name = try!(try!(v[1].atom()).string());
    //let value = try!(try!(v[1].atom()).string());
    let name = try!(match v[1] {
        Sexp::Atom(Atom::S(ref s)) => Ok(s),
        ref x => err("expecting name for fp_text")
    });
    let value = try!(match v[2] {
        Sexp::Atom(Atom::Q(ref s)) => Ok(s),
        ref x => err("expecting value for fp_text")
    });
    let parts = try!(parse_parts(&v[3..]));
    let mut fp = FpText::new(name, value);
    for part in &parts[..] {
        match *part {
            Part::At(ref at) => {
                fp.set_at(at)
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
                println!("ignoring {}", x)
            }
        }
    }
    Ok(Element::FpText(fp))
}
fn parse_pad(v: &Vec<Sexp>) -> ERes<Element> { 
    let name = try!(try!(v[1].atom()).as_string());
    let t = try!(try!(v[2].atom()).string());
    let t = try!(PadType::from_string(t));
    let shape = try!(try!(v[3].atom()).string());
    let shape = try!(PadShape::from_string(shape));
    let mut pad = Pad::new(name, t, shape);
    //println!("{}", pad);
    let parts = try!(parse_parts(&v[4..]));
    for part in &parts[..] {
        match *part {
            Part::At(ref at) => pad.set_at(at),
            Part::Xy(ref xy) if xy.t == XyType::Size => pad.set_size(xy),
            Part::Layers(ref l) => pad.set_layers(l),
            ref x => println!("ignoring {}", x),
        }
    }
    Ok(Element::Pad(pad))
}

fn parse_fp_poly(v: &Vec<Sexp>) -> ERes<Element> {
    Ok(Element::FpPoly)
}
fn parse_fp_line(v: &Vec<Sexp>) -> ERes<Element> {
    Ok(Element::FpLine)
}
fn parse_fp_circle(v: &Vec<Sexp>) -> ERes<Element> {
    Ok(Element::FpCircle)
}

fn parse_element_list(v: &Vec<Sexp>) -> ERes<Element> {
    match v[0] {
        Sexp::Atom(Atom::S(ref s)) => {
            match &s[..] {
                "layer" => parse_layer(v),
                "descr" => parse_descr(v),
                "fp_text" => parse_fp_text(v),
                "pad" => parse_pad(v),
                "fp_poly" => parse_fp_poly(v),
                "fp_line" => parse_fp_line(v),
                "fp_circle" => parse_fp_circle(v),
                x => Err(format!("unknown element in module: {}", x))
            }
        }
        _ => err("expecting atom")
    }
}

fn parse_element(s: &Sexp) -> ERes<Element> {
    match *s {
        Sexp::List(ref v) => parse_element_list(&v),
        _ => err("expecting element list in module")
    }
}

fn parse_module_list(v: &Vec<Sexp>) -> ERes<Module> {
    let mut module = (match v[0] {
        Sexp::Atom(Atom::S(ref s)) if s == "module" => {
            match v[1] {
                Sexp::Atom(Atom::S(ref s)) => {
                    Ok(Module::new(s))
                }
                ref x => return Err(format!("expecting module name got {}", x))
            }
        }
        _ => err("expecting module")
    }).unwrap();
    for e in &v[2..] {
        let el = try!(parse_element(e));
        module.append(el)
    }
    Ok(module)
}

fn parse_module(s: Sexp) -> ERes<Module> {
    match s {
        Sexp::List(v) => parse_module_list(&v),
        _ => err("expecting top level list")
    }
}

fn parse(s: &str) -> ERes<Module> {
    match rustysexp::parse_str(s) {
        Ok(s) => parse_module(s),
        Err(x) => Err(format!("IOError: {}", x))
    }
}


fn read_file(name: &str) -> Result<String,std::io::Error> {
    let mut f = try!(File::open(name));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}

pub fn parse_str(s: &str) -> Module {
    parse(s).unwrap()
}

pub fn parse_file(name: &str) -> Module {
    let s = read_file(name).unwrap();
    parse(&s[..]).unwrap()
}

