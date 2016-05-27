// (c) 2015-2016 Joost Yervante Damad <joost@productize.be>

// extension: .kicad_mod
// format: new-style

use std::fmt;
use std::result;

// get from parent
use Result;
use str_error;
use Sexp;
use symbolic_expressions;
use symbolic_expressions::IntoSexp;

pub use footprint::data::*;

// TODO: get rid of it

pub fn display_string(s:&str) -> String {
    if s.contains('(') || s.contains(' ') || s.is_empty() {
        format!("\"{}\"", s)
    } else {
        s.to_string()
    }
}

pub trait FromSexp {
    fn from_sexp(&Sexp) -> Self;
}

impl fmt::Display for Module {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        try!(write!(f, "(module {}\n", self.name));
        for e in &self.elements {
            try!(write!(f, "    {}\n", e))
        };
        write!(f, ")")
    }
}

impl IntoSexp for Module {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("module"));
        for e in &self.elements {
            v.push(e.into_sexp())
        }
        Sexp::new_list(v)
    }
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match *self {
            Element::Layer(ref s) => write!(f, "(layer {})", s),
            Element::Descr(ref s) => write!(f, "(descr {})", display_string(s)),
            Element::Tags(ref s) => write!(f, "(tags {})", display_string(s)),
            Element::Attr(ref s) => write!(f, "(attr {})", display_string(s)),
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

impl IntoSexp for Element {
    fn into_sexp(&self) -> Sexp {
        match *self {
            Element::Layer(ref s) => Sexp::new_named("layer", s),
            Element::Descr(ref s) => Sexp::new_named("descr", s),
            Element::Tags(ref s) => Sexp::new_named("tags", s),
            Element::Attr(ref s) => Sexp::new_named("attr", s),
            Element::FpText(ref p) => p.into_sexp(),
            Element::Pad(ref pad) => pad.into_sexp(),
            Element::FpPoly(ref p) => p.into_sexp(),
            Element::FpLine(ref p) => p.into_sexp(),
            Element::FpCircle(ref p) => p.into_sexp(),
            Element::TEdit(ref p) => Sexp::new_named("tedit", p),
            Element::TStamp(ref p) => Sexp::new_named("tstamp", p),
            Element::Path(ref p) => Sexp::new_named("path", p),
            Element::At(ref p) => p.into_sexp(),
            Element::Model(ref p) => p.into_sexp(),
            Element::Locked => Sexp::new_string("locked"),
        }
    }
}

impl fmt::Display for FpText {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "(fp_text {} {} {} (layer {}){} {})", self.name, display_string(&self.value), self.at, self.layer, if self.hide { " hide" } else { "" }, self.effects)
    }
}

impl IntoSexp for FpText {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("fp_text"));
        v.push(Sexp::new_string(&self.name));
        v.push(Sexp::new_string(&self.value));
        v.push(self.at.into_sexp());
        v.push(Sexp::new_named("layer", &self.layer));
        if self.hide {
            v.push(Sexp::new_string("hide"));
        }
        v.push(self.effects.into_sexp());
        Sexp::new_list(v)
    }
}

impl fmt::Display for At {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        if self.rot == 0.0 {
            write!(f, "(at {} {})", self.x, self.y)
        } else {
            write!(f, "(at {} {} {})", self.x, self.y, self.rot)
        }
    }
}
impl IntoSexp for At {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("at"));
        v.push(Sexp::new_string(self.x));
        v.push(Sexp::new_string(self.y));
        if self.rot != 0.0 {
            v.push(Sexp::new_string(self.rot));
        }
        Sexp::new_list(v)
    }
}

impl fmt::Display for Font {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "(font (size {} {}) (thickness {}))", self.size.x, self.size.y, self.thickness)
    }
}

impl IntoSexp for Font {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("font"));
        let mut v1 = vec![];
        v1.push(Sexp::new_string("size"));
        v1.push(Sexp::new_string(self.size.x));
        v1.push(Sexp::new_string(self.size.y));
        v.push(Sexp::new_list(v1));
        v.push(Sexp::new_named("thickness", self.thickness));
        Sexp::new_list(v)
    }
}

impl fmt::Display for Effects {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let justify = match self.justify {
            None => String::from(""),
            Some(ref j) => format!(" {}", j),
        };
        write!(f, "(effects (font (size {} {}) (thickness {})){})", self.font.size.x, self.font.size.y, self.font.thickness, justify)
    }
}
impl IntoSexp for Effects {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("effects"));
        v.push(self.font.into_sexp());
        if let Some(ref j) = self.justify {
            v.push(j.into_sexp())
        }
        Sexp::new_list(v)
    }
}

impl fmt::Display for Justify {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match *self {
            Justify::Mirror => write!(f,"(justify mirror)"),
        }
    }
}

impl IntoSexp for Justify {
    fn into_sexp(&self) -> Sexp {
        match *self {
            Justify::Mirror => Sexp::new_named("justify","mirror"),
        }
    }
}

impl fmt::Display for Xy {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self.t {
            XyType::Xy => write!(f,"(xy {} {})", self.x, self.y),
            XyType::Start => write!(f,"(start {} {})", self.x, self.y),
            XyType::End => write!(f,"(end {} {})", self.x, self.y),
            XyType::Size => write!(f,"(size {} {})", self.x, self.y),
            XyType::Center => write!(f,"(center {} {})", self.x, self.y),
        }
    }
}
impl IntoSexp for Xy {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string(match self.t {
            XyType::Xy => "xy",
            XyType::Start => "start",
            XyType::End => "end",
            XyType::Size => "size",
            XyType::Center => "center",
        }));
        v.push(Sexp::new_string(self.x));
        v.push(Sexp::new_string(self.y));
        Sexp::new_list(v)
    }
}

impl fmt::Display for Pts {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        try!(write!(f, "(pts"));
        for x in &self.elements {
            try!(write!(f, " {}", x));
        }
        write!(f,")")
    }
}

impl IntoSexp for Pts {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("pts"));
        for x in &self.elements {
            v.push(x.into_sexp())
        }
        Sexp::new_list(v)
    }
}

impl fmt::Display for Drill {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        try!(write!(f,"(drill "));
        if let Some(ref s) = self.shape {
            try!(write!(f, "{} ", s))
        }
        try!(write!(f,"{}", self.drill));
        if let Some(ref d2) = self.drill2 {
            try!(write!(f, " {}", d2))
        }
        write!(f,")")
    }
}

impl IntoSexp for Drill {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("drill"));
        if let Some(ref s) = self.shape {
            v.push(Sexp::new_string(s))
        }
        v.push(Sexp::new_string(self.drill));
        if let Some(ref s) = self.drill2 {
            v.push(Sexp::new_string(s))
        }
        Sexp::new_list(v)
    }
}

// as parts are intermediate only Display doesn't really matter
/*
impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
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
}*/

impl fmt::Display for PadType {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match *self {
            PadType::Smd => write!(f, "smd"),
            PadType::Pth => write!(f, "thru_hole"),
            PadType::NpPth => write!(f, "np_thru_hole"),
        }
    }
}

impl IntoSexp for PadType {
    fn into_sexp(&self) -> Sexp {
        Sexp::new_string(match *self {
            PadType::Smd => "smd",
            PadType::Pth => "thru_hole",
            PadType::NpPth => "np_thru_hole",
        })
    }
}

impl fmt::Display for PadShape {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match *self {
            PadShape::Rect => write!(f, "rect"),
            PadShape::Circle => write!(f, "circle"),
            PadShape::Oval => write!(f, "oval"),
        }
    }
}

impl IntoSexp for PadShape {
    fn into_sexp(&self) -> Sexp {
        Sexp::new_string(match *self {
            PadShape::Rect => "rect",
            PadShape::Circle => "circle",
            PadShape::Oval => "oval",
        })
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
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

impl IntoSexp for Layer {
    fn into_sexp(&self) -> Sexp {
        Sexp::new_string(&self)
    }
} 

impl fmt::Display for Layers {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
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

impl IntoSexp for Layers {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("layers"));
        for layer in &self.layers {
            v.push(layer.into_sexp())
        }
        Sexp::new_list(v)
    }
}

impl fmt::Display for Pad {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
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

impl IntoSexp for Pad {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("pad"));
        v.push(Sexp::new_string(&self.name));
        v.push(self.t.into_sexp());
        v.push(self.shape.into_sexp());
        v.push(self.size.into_sexp());
        v.push(self.at.into_sexp());
        v.push(self.layers.into_sexp());
        if let Some(ref net) = self.net {
            v.push(net.into_sexp());
        }
        if let Some(ref drill) = self.drill {
            v.push(drill.into_sexp());
        }
        if let Some(ref spm) = self.solder_paste_margin {
            v.push(Sexp::new_named("solder_paste_margin", spm));
        }
        Sexp::new_list(v)
    }
}

impl fmt::Display for FpPoly {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let l = self.pts.elements.len();
        if l == 0 {
            return Ok(())
        }
        try!(write!(f, "(fp_poly {}", self.pts));
        write!(f, " (layer {}) (width {}))", self.layer, self.width)
    }
}

impl IntoSexp for FpPoly {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("fp_poly"));
        v.push(self.pts.into_sexp());
        v.push(Sexp::new_named("layer", &self.layer));
        v.push(Sexp::new_named("width", self.width));
        Sexp::new_list(v)
    }
}

impl fmt::Display for FpLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "(fp_line {} {} (layer {}) (width {}))", self.start, self.end, self.layer, self.width)
    }
}

impl IntoSexp for FpLine {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("fp_line"));
        v.push(self.start.into_sexp());
        v.push(self.end.into_sexp());
        v.push(Sexp::new_named("layer", &self.layer));
        v.push(Sexp::new_named("width", self.width));
        Sexp::new_list(v)
    }
}

impl fmt::Display for FpCircle {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "(fp_circle {} {} (layer {}) (width {}))", self.center, self.end, self.layer, self.width)
    }
}

impl IntoSexp for FpCircle {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("fp_circle"));
        v.push(self.center.into_sexp());
        v.push(self.end.into_sexp());
        v.push(Sexp::new_named("layer", &self.layer));
        v.push(Sexp::new_named("width", self.width));
        Sexp::new_list(v)
    }
}

impl fmt::Display for Net {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "(net {} \"{}\")", self.num, self.name)
    }
}

impl IntoSexp for Net {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("net"));
        v.push(Sexp::new_string(self.num));
        v.push(Sexp::new_string(&self.name));
        Sexp::new_list(v)
    }
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "(model {} (at {}) (scale {}) (rotate {}))", self.name, self.at, self.scale, self.rotate)
    }
}

impl IntoSexp for Model {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("model"));
        v.push(Sexp::new_string(&self.name));
        v.push(Sexp::new_named_sexp("at", &self.at));
        v.push(Sexp::new_named_sexp("scale", &self.scale));
        v.push(Sexp::new_named_sexp("rotate", &self.rotate));
        Sexp::new_list(v)
    }
}

impl fmt::Display for Xyz {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "(xyz {} {} {})", self.x, self.y, self.z)
    }
}

impl IntoSexp for Xyz {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("xyz"));
        v.push(Sexp::new_string(self.x));
        v.push(Sexp::new_string(self.y));
        v.push(Sexp::new_string(self.z));
        Sexp::new_list(v)
    }
}


// (at 0.0 -4.0) (at -2.575 -1.625 180)
impl FromSexp for Result<At> {
    fn from_sexp(s:&Sexp) -> Result<At> {
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
            _ => str_error("at with wrong length".to_string())
        }
    }
}

pub fn wrap<X,Y,F,G>(s:&Sexp, make:F, wrapper:G) -> Result<Y>
    where F:Fn(&Sexp) -> Result<X>, G:Fn(X) -> Y
{
    Ok(wrapper(try!(make(s))))
}


impl FromSexp for Result<Layer> {
    fn from_sexp(s:&Sexp) -> Result<Layer> {
        let v = try!(s.slice_atom("layer"));
        let layer = try!(v[0].string());
        let layer = try!(Layer::from_string(layer.clone()));
        Ok(layer)
    }
}

impl FromSexp for Result<Effects> {
    fn from_sexp(s:&Sexp) -> Result<Effects> {
        //let v = try!(s.slice_atom_num("effects", 1));
        // TODO investigate why the above doesn't work !?
        let v = try!(s.slice_atom("effects"));
        if v.len() < 1 {
            return str_error(format!("Expected at least one element in {}", s))
        }
        let font = try!(Result::from_sexp(&v[0]));
        let justify = if v.len() > 1 {
            Some(try!(Result::from_sexp(&v[1])))
        } else {
            None
        };
        Ok(Effects::from_font(font, justify))
    }
}

impl FromSexp for Result<Justify> {
    fn from_sexp(s:&Sexp) -> Result<Justify> {
        let v = try!(s.slice_atom("justify"));
        if v.len() < 1 {
            return str_error(format!("Expected at least one element in {}", s))
        }
        let s = try!(v[0].string());
        match &s[..] {
            "mirror" => Ok(Justify::Mirror),
            _ => str_error(format!("unknown justify: {}", s))
        }
    }
}

impl FromSexp for Result<Font> {
    fn from_sexp(s:&Sexp) -> Result<Font> {
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
                ref x => Err(format!("unknown element in font: {:?}", x))
            })
        }
        Ok(font)
    }
}

impl FromSexp for Result<Layers> {
    fn from_sexp(s:&Sexp) -> Result<Layers> {
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

fn parse_part_float<F>(e: &Sexp, make:F) -> Result<Part>
    where F:Fn(f64) -> Part
{
    let v = try!(e.list());
    if v.len() < 2 {
        return str_error(format!("not enough elements in {}", e))
    }
    let f = try!(v[1].f());
    Ok(make(f))
}

impl FromSexp for Result<Vec<Xy> > {
    fn from_sexp(s:&Sexp) -> Result<Vec<Xy> > {
        let v = try!(s.slice_atom("pts"));
        let mut pts = vec![];
        for e in &v[1..] {
            let p = try!(Result::from_sexp(e));
            pts.push(p)
        }
        Ok(pts)
    }
}

impl FromSexp for Result<Xy> {
    fn from_sexp(s: &Sexp) -> Result<Xy> {
        let name = try!(s.list_name());
        let t = try!(match &name[..] {
            "xy" => Ok(XyType::Xy),
            "start" => Ok(XyType::Start),
            "end" => Ok(XyType::End),
            "size" => Ok(XyType::Size),
            "center" => Ok(XyType::Center),
            ref x => str_error(format!("unknown XyType {}", x)),
        });
        let v = try!(s.slice_atom_num(&name, 2));
        let x = try!(v[0].f());
        let y = try!(v[1].f());
        Ok(Xy::new(x,y,t))
    }
}

impl FromSexp for Result<Pts> {
    fn from_sexp(s: &Sexp) -> Result<Pts> {
        let v = try!(s.slice_atom("pts"));
        let mut r = vec![];
        for x in v {
            let xy = try!(Result::from_sexp(x));
            r.push(xy)
        }
        Ok(Pts { elements:r })
    }
}


impl FromSexp for Result<Xyz> {
    fn from_sexp(s: &Sexp) -> Result<Xyz> {
        let v = try!(s.slice_atom_num("xyz", 3));
        let x = try!(v[0].f());
        let y = try!(v[1].f());
        let z = try!(v[2].f());
        Ok(Xyz::new(x,y,z))
    }
}

impl FromSexp for Result<Net> {
    fn from_sexp(s: &Sexp) -> Result<Net> {
        let v = try!(s.slice_atom_num("net", 2));
        let num = try!(v[0].i());
        let name = try!(v[1].string());
        Ok(Net { num:num, name:name.clone(), })
    }
}

impl FromSexp for Result<Drill> {
    fn from_sexp(s: &Sexp) -> Result<Drill> {
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
            str_error("unknown drill format".to_string())
        }
    }
}

impl FromSexp for Result<Part> {
    fn from_sexp(s:&Sexp) -> Result<Part> {
        match s.string() {
            Ok(ref sx) => match &sx[..] {
                "hide" => Ok(Part::Hide),
                x => str_error(format!("unknown part in element: {}", x))
            },
            _ => {
                let name = &try!(s.list_name())[..];
                match name {
                    "at" => wrap(s, Result::from_sexp, Part::At),
                    "layer" => wrap(s, Result::from_sexp, Part::Layer),
                    "effects" => wrap(s, Result::from_sexp, Part::Effects),
                    "layers" => wrap(s, Result::from_sexp, Part::Layers),
                    "width" => parse_part_float(s, Part::Width),
                    "start" | "end" | "size" | "center" => wrap(s, Result::from_sexp, Part::Xy),
                    "pts" => wrap(s, Result::from_sexp, Part::Pts),
                    "thickness" => parse_part_float(s, Part::Thickness),
                    "net" => wrap(s, Result::from_sexp, Part::Net),
                    "drill" => wrap(s, Result::from_sexp, Part::Drill),
                    "solder_paste_margin" => parse_part_float(s, Part::SolderPasteMargin),
                    x => str_error(format!("unknown part {}", x))
                }
            }
        }
    }        
}

fn parse_parts(v: &[Sexp]) -> Result<Vec<Part>> {
    let mut res = Vec::new();
    for e in v {
        let p = try!(Result::from_sexp(e));
        res.push(p);
    }
    Ok(res)
}

fn parse_string_element(s:&Sexp) -> Result<String> {
    let name = try!(s.list_name());
    let v = try!(s.slice_atom_num(&name, 1));
    let s = try!(v[0].string());
    Ok(s.clone())
}

impl FromSexp for Result<FpText> {
    fn from_sexp(s:&Sexp) -> Result<FpText> {
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
                    return str_error(format!("fp_text: unknown {:?}", x))
                }
            }
        }
        Ok(fp)
    }
}

impl FromSexp for Result<Pad> {
    fn from_sexp(s:&Sexp) -> Result<Pad> {
        let v = try!(s.slice_atom("pad"));
        if v.len() < 3 {
            return str_error("not enough elements in pad".to_string())
        }
        let name = try!(v[0].string()).clone();
        let t = try!(v[1].string());
        let t = try!(PadType::from_string(&t));
        let shape = try!(v[2].string());
        let shape = try!(PadShape::from_string(&shape));
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
                ref x => return str_error(format!("pad: unknown {:?}", x)),
            }
        }
        Ok(pad)
    }
}

impl FromSexp for Result<FpPoly> {
    fn from_sexp(s:&Sexp) -> Result<FpPoly> {
        let v = try!(s.slice_atom("fp_poly"));
        let mut fp_poly = FpPoly::new();
        let parts = try!(parse_parts(&v));
        for part in &parts[..] {
            match *part {
                Part::Pts(ref pts) => fp_poly.pts.clone_from(pts),
                Part::Width(w) => fp_poly.width = w,
                Part::Layer(ref layer) => fp_poly.layer.clone_from(layer),
                ref x => println!("fp_poly: ignoring {:?}", x),
            }
        } 
        Ok(fp_poly)
    }
}

impl FromSexp for Result<FpLine> {
    fn from_sexp(s:&Sexp) -> Result<FpLine> {
        let v = try!(s.slice_atom("fp_line"));
        let mut fp_line = FpLine::new();
        let parts = try!(parse_parts(&v));
        for part in &parts[..] {
            match *part {
                Part::Xy(ref xy) if xy.t == XyType::Start => fp_line.start.clone_from(xy),
                Part::Xy(ref xy) if xy.t == XyType::End => fp_line.end.clone_from(xy),
                Part::Layer(ref layer) => fp_line.layer.clone_from(layer),
                Part::Width(w) => fp_line.width = w,
                ref x => return str_error(format!("fp_line: unknown {:?}", x)),
            }
        }
        Ok(fp_line)
    }
}

impl FromSexp for Result<FpCircle> {
    fn from_sexp(s:&Sexp) -> Result<FpCircle> {
        let v = try!(s.slice_atom("fp_circle"));
        let mut fp_circle = FpCircle::new();
        let parts = try!(parse_parts(&v));
        for part in &parts[..] {
            match *part {
                Part::Xy(ref xy) if xy.t == XyType::Center => fp_circle.center.clone_from(xy),
                Part::Xy(ref xy) if xy.t == XyType::End => fp_circle.end.clone_from(xy),
                Part::Layer(ref layer) => fp_circle.layer.clone_from(layer),
                Part::Width(w) => fp_circle.width = w,
                ref x => return str_error(format!("fp_circle: unexpected {:?}", x)),
            }
        }
        Ok(fp_circle)
    }
}

fn parse_sublist<X>(s:&Sexp, name:&'static str) -> Result<X>
    where Result<X>:FromSexp {
    let x = &try!(s.slice_atom_num(name, 1))[0];
    Result::from_sexp(x)
}


impl FromSexp for Result<Model> {
    fn from_sexp(s:&Sexp) -> Result<Model> {
        let v = try!(s.slice_atom_num("model", 4));
        let name = try!(v[0].string()).clone();
        let at = try!(parse_sublist(&v[1], "at"));
        let scale = try!(parse_sublist(&v[2], "scale"));
        let rotate = try!(parse_sublist(&v[3], "rotate"));
        let m = Model {name:name, at:at, scale:scale, rotate:rotate};
        Ok(m)
    }
}

impl FromSexp for Result<Element> {
    fn from_sexp(s:&Sexp) -> Result<Element> {
        match *s {
            Sexp::String(ref s) => {
                match &s[..] {
                    "locked" => Ok(Element::Locked),
                    _ => str_error(format!("unknown element in module: {}", s))
                }
            },
            Sexp::List(_) => {
                let name = try!(s.list_name());
                match &name[..] {
                    "layer" => wrap(s, parse_string_element, Element::Layer),
                    "descr" => wrap(s, parse_string_element, Element::Descr),
                    "tags" => wrap(s, parse_string_element, Element::Tags),
                    "attr" => wrap(s, parse_string_element, Element::Attr),
                    "fp_text" => wrap(s, Result::from_sexp, Element::FpText),
                    "pad" => wrap(s, Result::from_sexp, Element::Pad),
                    "fp_poly" => wrap(s, Result::from_sexp, Element::FpPoly),
                    "fp_line" => wrap(s, Result::from_sexp, Element::FpLine),
                    "fp_circle" => wrap(s, Result::from_sexp, Element::FpCircle),
                    "tedit" => wrap(s, parse_string_element, Element::TEdit),
                    "tstamp" => wrap(s, parse_string_element, Element::TStamp),
                    "path" => wrap(s, parse_string_element, Element::Path),
                    "at" => wrap(s, Result::from_sexp, Element::At),
                    "model" => wrap(s, Result::from_sexp, Element::Model),
                    _ => str_error(format!("unknown element in module: {}", name))
                }
            },
            Sexp::Empty => unreachable!(),
        }
    }
}

impl FromSexp for Result<Module> {
    fn from_sexp(s:&Sexp) -> Result<Module> {
        let v = try!(s.slice_atom("module"));
        if v.len() < 1 {
            return str_error("no name in module".to_string());
        }
        let name = try!(v[0].string());
        let mut module = Module::new(name.clone());
        for e in &v[1..] {
            let el = try!(Result::from_sexp(&e));
            module.append(el)
        }
        Ok(module)
    }
}

pub fn parse(s: &str) -> Result<Module> {
    match symbolic_expressions::parser::parse_str(s) {
        Ok(s) => Result::from_sexp(&s),
        Err(x) => str_error(format!("IOError: {:?}", x))
    }
}

mod data;
