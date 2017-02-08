// (c) 2016-2017 Productize SPRL <joost@productize.be>

use Sexp;
use str_error;
//use Result;
use footprint::data::*;
use FromSexp;
use Part;
use wrap;
use from_sexp;
use Result;
use IterAtom;

// (at 0.0 -4.0) (at -2.575 -1.625 180)
impl FromSexp for At {
    fn from_sexp(s: &Sexp) -> Result<At> {
        let mut i = IterAtom::new(s, "at")?;
        let x = i.f("at", "x")?;
        let y = i.f("at", "y")?;
        let rot = i.opt_f(0.0)?;
        Ok(At::new(x, y, rot))
    }
}

impl FromSexp for Layer {
    fn from_sexp(s: &Sexp) -> Result<Layer> {
        let mut i = IterAtom::new(s, "layer")?;
        let layer = i.s("layer", "layername")?;
        let layer = Layer::from_string(&layer)?;
        Ok(layer)
    }
}

impl FromSexp for Effects {
    fn from_sexp(s: &Sexp) -> Result<Effects> {
        let mut i = IterAtom::new(s,"effects")?;
        let font = i.t("effects", "font")?;
        let justify = i.opt_t()?;
        Ok(Effects::from_font(font, justify))
    }
}

impl FromSexp for Justify {
    fn from_sexp(s: &Sexp) -> Result<Justify> {
        let mut i = IterAtom::new(s, "justify")?;
        let s = i.s("justify", "mirror")?;
        match &s[..] {
            "mirror" => Ok(Justify::Mirror),
            "left" => Ok(Justify::Left),
            "right" => Ok(Justify::Right),
            _ => str_error(format!("unknown justify: {}", s)),
        }
    }
}

impl FromSexp for Font {
    fn from_sexp(s: &Sexp) -> Result<Font> {
        let mut i = IterAtom::new(s, "font")?;
        let parts = i.vec()?;
        let mut font = Font::default();
        for part in &parts[..] {
            // println!("part: {}", part);
            match *part {
                    Part::Xy(ref xy) if xy.t == XyType::Size => {
                        font.size.x = xy.x;
                        font.size.y = xy.y;
                        Ok(())
                    }
                    Part::Thickness(ref t) => {
                        font.thickness = *t;
                        Ok(())
                    }
                    ref x => Err(format!("unknown element in font: {:?}", x)),
                }
                ?
        }
        Ok(font)
    }
}

impl FromSexp for Layers {
    fn from_sexp(s: &Sexp) -> Result<Layers> {
        let mut l = Layers::default();
        let i = IterAtom::new(s, "layers")?;
        for v1 in i.iter {
            let x = v1.string()?;
            let layer = Layer::from_string(x)?;
            l.append(layer)
        }
        Ok(l)
    }
}

fn parse_part_float<F>(e: &Sexp, make: F) -> Result<Part>
    where F: Fn(f64) -> Part
{
    let v = e.list()?;
    if v.len() < 2 {
        return str_error(format!("not enough elements in {}", e));
    }
    let f = v[1].f()?;
    Ok(make(f))
}

impl FromSexp for Vec<Xy> {
    fn from_sexp(s: &Sexp) -> Result<Vec<Xy>> {
        let mut i = IterAtom::new(s, "pts")?;
        let pts = i.vec()?;
        Ok(pts)
    }
}

impl FromSexp for Xy {
    fn from_sexp(s: &Sexp) -> Result<Xy> {
        let name = s.list_name()?;
        let t = match &name[..] {
                "xy" => Ok(XyType::Xy),
                "start" => Ok(XyType::Start),
                "end" => Ok(XyType::End),
                "size" => Ok(XyType::Size),
                "center" => Ok(XyType::Center),
                "rect_delta" => Ok(XyType::RectDelta),
                x => str_error(format!("unknown XyType {}", x)),
            }
            ?;
        let mut i = IterAtom::new(s, name)?;
        let x = i.f(name, "x")?;
        let y = i.f(name, "y")?;
        Ok(Xy::new(x, y, t))
    }
}

impl FromSexp for Pts {
    fn from_sexp(s: &Sexp) -> Result<Pts> {
        let mut i = IterAtom::new(s, "pts")?;
        let r = i.vec()?;
        Ok(Pts { elements: r })
    }
}


impl FromSexp for Xyz {
    fn from_sexp(s: &Sexp) -> Result<Xyz> {
        let mut i = IterAtom::new(s, "xyz")?;
        let x = i.f("xyz", "x")?;
        let y = i.f("xyz", "y")?;
        let z = i.f("xyz", "z")?;
        Ok(Xyz::new(x, y, z))
    }
}

impl FromSexp for Net {
    fn from_sexp(s: &Sexp) -> Result<Net> {
        let mut i = IterAtom::new(s, "net")?;
        let num = i.i("net", "num")?;
        let name = i.s("net", "name")?;
        Ok(Net {
            num: num,
            name: name,
        })
    }
}

impl FromSexp for Drill {
    fn from_sexp(s: &Sexp) -> Result<Drill> {
        let mut drill = Drill::default();
        let v = s.slice_atom("drill")?;
        let mut i = 0;
        let len = v.len();
        // optional a shape, which can only be "oval"
        let shape = v[i].string()?;
        if let "oval" = &shape[..] {
            drill.shape = Some(shape.clone());
            i += 1;
        }
        // always at least one drill
        let drill1 = v[i].f()?;
        i += 1;
        drill.width = drill1;
        drill.height = drill1;
        if i == len {
            return Ok(drill);
        }
        match v[i] {
            Sexp::String(_) => {
                drill.height = v[i].f()?;
            }
            Sexp::Empty => (),
            Sexp::List(_) => {
                let mut i2 = IterAtom::new(&v[i], "offset")?;
                drill.offset_x = i2.f("drill/offset", "x")?;
                drill.offset_y = i2.f("drill/offset", "y")?;
            }
        }
        Ok(drill)
    }
}

impl FromSexp for Part {
    fn from_sexp(s: &Sexp) -> Result<Part> {
        match s.string() {
            Ok(sx) => {
                match &sx[..] {
                    "hide" => Ok(Part::Hide),
                    x => str_error(format!("unknown part in element: {}", x)),
                }
            }
            _ => {
                let name = &(s.list_name()?)[..];
                match name {
                    "at" => wrap(s, from_sexp, Part::At),
                    "layer" => wrap(s, from_sexp, Part::Layer),
                    "effects" => wrap(s, from_sexp, Part::Effects),
                    "layers" => wrap(s, from_sexp, Part::Layers),
                    "width" => parse_part_float(s, Part::Width),
                    "angle" => parse_part_float(s, Part::Angle),
                    "start" | "end" | "size" | "center" | "rect_delta" => {
                        wrap(s, from_sexp, Part::Xy)
                    }
                    "pts" => wrap(s, from_sexp, Part::Pts),
                    "thickness" => parse_part_float(s, Part::Thickness),
                    "net" => wrap(s, from_sexp, Part::Net),
                    "drill" => wrap(s, from_sexp, Part::Drill),
                    "solder_paste_margin" => parse_part_float(s, Part::SolderPasteMargin),
                    "solder_mask_margin" => parse_part_float(s, Part::SolderMaskMargin),
                    "clearance" => parse_part_float(s, Part::Clearance),
                    "thermal_gap" => parse_part_float(s, Part::ThermalGap),
                    x => str_error(format!("unknown part {}", x)),
                }
            }
        }
    }
}

fn parse_string_element(s: &Sexp) -> Result<String> {
    let name = s.list_name()?;
    let mut i = IterAtom::new(s, name)?;
    Ok(i.s(name, "element")?)
}

fn parse_float_element(s: &Sexp) -> Result<f64> {
    let name = s.list_name()?;
    let mut i = IterAtom::new(s, name)?;
    Ok(i.f(name, "element")?)
}

impl FromSexp for FpText {
    fn from_sexp(s: &Sexp) -> Result<FpText> {
        let mut i = IterAtom::new(s, "fp_text")?;
        let name = i.s("fp_text", "name")?;
        let value = i.s("fp_text", "value")?;
        let parts = i.vec()?;
        let mut fp = FpText::new(name.clone(), value.clone());
        for part in &parts[..] {
            match *part {
                Part::At(ref at) => fp.at.clone_from(at),
                Part::Layer(ref layer) => fp.set_layer(layer),
                Part::Hide => fp.hide = true,
                Part::Effects(ref effects) => fp.set_effects(effects),
                ref x => return str_error(format!("fp_text: unknown {:?}", x)),
            }
        }
        Ok(fp)
    }
}

impl FromSexp for Pad {
    fn from_sexp(s: &Sexp) -> Result<Pad> {
        let mut i = IterAtom::new(s, "pad")?;
        let name = i.s("pad", "name")?;
        let t = i.s("pad", "type")?;
        let t = PadType::from_string(&t)?;
        let shape = i.s("pad", "shape")?;
        let shape = PadShape::from_string(&shape)?;
        let mut pad = Pad::new(name, t, shape);
        // println!("{}", pad);
        let parts = i.vec()?;
        for part in parts {
            match part {
                Part::At(ref at) => pad.at.clone_from(at),
                Part::Xy(ref xy) if xy.t == XyType::Size => pad.size.clone_from(xy),
                Part::Xy(ref xy) if xy.t == XyType::RectDelta => pad.rect_delta = Some(xy.clone()),
                Part::Layers(ref l) => pad.layers.clone_from(l),
                Part::Net(n) => pad.set_net(n),
                Part::Drill(n) => pad.set_drill(n),
                Part::SolderPasteMargin(n) => pad.solder_paste_margin = Some(n),
                Part::SolderMaskMargin(n) => pad.solder_mask_margin = Some(n),
                Part::Clearance(n) => pad.clearance = Some(n),
                Part::ThermalGap(n) => pad.thermal_gap = Some(n),
                ref x => return str_error(format!("pad: unknown {:?}", x)),
            }
        }
        Ok(pad)
    }
}

impl FromSexp for FpPoly {
    fn from_sexp(s: &Sexp) -> Result<FpPoly> {
        let mut i = IterAtom::new(s, "fp_poly")?;
        let mut fp_poly = FpPoly::default();
        let parts = i.vec()?;
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

impl FromSexp for FpLine {
    fn from_sexp(s: &Sexp) -> Result<FpLine> {
        let mut i = IterAtom::new(s, "fp_line")?;
        let mut fp_line = FpLine::default();
        let parts = i.vec()?;
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

impl FromSexp for FpCircle {
    fn from_sexp(s: &Sexp) -> Result<FpCircle> {
        let mut i = IterAtom::new(s, "fp_circle")?;
        let mut fp_circle = FpCircle::default();
        let parts = i.vec()?;
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

impl FromSexp for FpArc {
    fn from_sexp(s: &Sexp) -> Result<FpArc> {
        let mut i = IterAtom::new(s, "fp_arc")?;
        let mut fp_arc = FpArc::default();
        let parts = i.vec()?;
        for part in &parts[..] {
            match *part {
                Part::Xy(ref xy) if xy.t == XyType::Start => fp_arc.start.clone_from(xy),
                Part::Xy(ref xy) if xy.t == XyType::End => fp_arc.end.clone_from(xy),
                Part::Angle(w) => fp_arc.angle = w,
                Part::Layer(ref layer) => fp_arc.layer.clone_from(layer),
                Part::Width(w) => fp_arc.width = w,
                ref x => return str_error(format!("fp_arc: unexpected {:?}", x)),
            }
        }
        Ok(fp_arc)
    }
}


fn parse_sublist<X: FromSexp>(s: &Sexp, name: &'static str) -> Result<X> {
    let x = &(s.slice_atom_num(name, 1)?)[0];
    X::from_sexp(x)
}


impl FromSexp for Model {
    fn from_sexp(s: &Sexp) -> Result<Model> {
        let v = s.slice_atom_num("model", 4)?;
        let name = v[0].string()?.clone();
        let at = parse_sublist(&v[1], "at")?;
        let scale = parse_sublist(&v[2], "scale")?;
        let rotate = parse_sublist(&v[3], "rotate")?;
        let m = Model {
            name: name,
            at: at,
            scale: scale,
            rotate: rotate,
        };
        Ok(m)
    }
}

impl FromSexp for Element {
    fn from_sexp(s: &Sexp) -> Result<Element> {
        match *s {
            Sexp::String(ref s) => {
                match &s[..] {
                    "locked" => Ok(Element::Locked),
                    _ => str_error(format!("unknown element in module: {}", s)),
                }
            }
            Sexp::List(_) => {
                let name = s.list_name()?;
                match &name[..] {
                    "solder_mask_margin" => wrap(s, parse_float_element, Element::SolderMaskMargin),
                    "layer" => wrap(s, parse_string_element, Element::Layer),
                    "descr" => wrap(s, parse_string_element, Element::Descr),
                    "tags" => wrap(s, parse_string_element, Element::Tags),
                    "attr" => wrap(s, parse_string_element, Element::Attr),
                    "fp_text" => wrap(s, from_sexp, Element::FpText),
                    "pad" => wrap(s, from_sexp, Element::Pad),
                    "fp_poly" => wrap(s, from_sexp, Element::FpPoly),
                    "fp_line" => wrap(s, from_sexp, Element::FpLine),
                    "fp_circle" => wrap(s, from_sexp, Element::FpCircle),
                    "fp_arc" => wrap(s, from_sexp, Element::FpArc),
                    "tedit" => wrap(s, parse_string_element, Element::TEdit),
                    "tstamp" => wrap(s, parse_string_element, Element::TStamp),
                    "path" => wrap(s, parse_string_element, Element::Path),
                    "at" => wrap(s, from_sexp, Element::At),
                    "model" => wrap(s, from_sexp, Element::Model),
                    _ => str_error(format!("unknown element in module: {}", name)),
                }
            }
            Sexp::Empty => unreachable!(),
        }
    }
}

impl FromSexp for Module {
    fn from_sexp(s: &Sexp) -> Result<Module> {
        let v = s.slice_atom("module")?;
        if v.len() < 1 {
            return str_error("no name in module".to_string());
        }
        let name = v[0].string()?;
        let mut module = Module::new(name.clone());
        for e in &v[1..] {
            let el = from_sexp(e)?;
            module.append(el)
        }
        Ok(module)
    }
}
