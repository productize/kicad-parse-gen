// (c) 2016-2017 Productize SPRL <joost@productize.be>

use Sexp;
use footprint::data::*;
use Part;
use wrap;
use symbolic_expressions::iteratom::*;

struct Offset(f64, f64);

impl FromSexp for Offset {
    fn from_sexp(s: &Sexp) -> SResult<Offset> {
        let mut i = IterAtom::new(s, "offset")?;
        let x = i.f("x")?;
        let y = i.f("y")?;
        i.close(Offset(x, y))
    }
}

// (at 0.0 -4.0) (at -2.575 -1.625 180)
impl FromSexp for At {
    fn from_sexp(s: &Sexp) -> SResult<At> {
        let mut i = IterAtom::new(s, "at")?;
        let x = i.f("x")?;
        let y = i.f("y")?;
        let rot = i.maybe_f().unwrap_or(0.0);
        Ok(At::new(x, y, rot))
    }
}

impl FromSexp for Layer {
    fn from_sexp(s: &Sexp) -> SResult<Layer> {
        let mut i = IterAtom::new(s, "layer")?;
        let layer = i.s("layername")?;
        let layer = Layer::from_string(&layer)?;
        Ok(layer)
    }
}

impl FromSexp for Effects {
    fn from_sexp(s: &Sexp) -> SResult<Effects> {
        let mut i = IterAtom::new(s, "effects")?;
        let font = i.t("font")?;
        let justify = i.maybe_t();
        Ok(Effects::from_font(font, justify))
    }
}

impl FromSexp for Justify {
    fn from_sexp(s: &Sexp) -> SResult<Justify> {
        let mut i = IterAtom::new(s, "justify")?;
        let s = i.s("mirror")?;
        match &s[..] {
            "mirror" => Ok(Justify::Mirror),
            "left" => Ok(Justify::Left),
            "right" => Ok(Justify::Right),
            _ => Err(format!("unknown justify: {}", s).into()),
        }
    }
}

impl FromSexp for Font {
    fn from_sexp(s: &Sexp) -> SResult<Font> {
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
    fn from_sexp(s: &Sexp) -> SResult<Layers> {
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

fn parse_part_float<F>(e: &Sexp, make: F) -> SResult<Part>
    where F: Fn(f64) -> Part
{
    let v = e.list()?;
    if v.len() < 2 {
        return Err(format!("not enough elements in {}", e).into());
    }
    let f = v[1].f()?;
    Ok(make(f))
}

impl FromSexp for Xy {
    fn from_sexp(s: &Sexp) -> SResult<Xy> {
        let name: &str = &s.list_name()?[..];
        let t: SResult<XyType> = match name {
            "xy" => Ok(XyType::Xy),
            "start" => Ok(XyType::Start),
            "end" => Ok(XyType::End),
            "size" => Ok(XyType::Size),
            "center" => Ok(XyType::Center),
            "rect_delta" => Ok(XyType::RectDelta),
            x => Err(format!("unknown XyType {}", x).into()),
        };
        let t = t?;
        let mut i = IterAtom::new(s, name)?;
        let x = i.f("x")?;
        let y = i.f("y")?;
        Ok(Xy::new(x, y, t))
    }
}

impl FromSexp for Pts {
    fn from_sexp(s: &Sexp) -> SResult<Pts> {
        let mut i = IterAtom::new(s, "pts")?;
        let r = i.vec()?;
        Ok(Pts { elements: r })
    }
}


impl FromSexp for Xyz {
    fn from_sexp(s: &Sexp) -> SResult<Xyz> {
        let mut i = IterAtom::new(s, "xyz")?;
        let x = i.f("y")?;
        let y = i.f("y")?;
        let z = i.f("z")?;
        Ok(Xyz::new(x, y, z))
    }
}

impl FromSexp for Net {
    fn from_sexp(s: &Sexp) -> SResult<Net> {
        let mut i = IterAtom::new(s, "net")?;
        let num = i.i("num")?;
        let name = i.s("name")?;
        Ok(Net {
            num: num,
            name: name,
        })
    }
}

impl FromSexp for Drill {
    fn from_sexp(s: &Sexp) -> SResult<Drill> {
        let mut i = IterAtom::new(s, "drill")?;
        let mut drill = Drill::default();
        drill.shape = i.maybe_literal_s("oval");
        drill.width = i.f("width")?;
        drill.height = i.maybe_f().unwrap_or(drill.width);
        let offset: Offset = i.maybe_t().unwrap_or(Offset(0.0, 0.0));
        drill.offset_x = offset.0;
        drill.offset_y = offset.1;
        i.close(drill)
    }
}

impl FromSexp for Part {
    fn from_sexp(s: &Sexp) -> SResult<Part> {
        match s.string() {
            Ok(sx) => {
                match &sx[..] {
                    "hide" => Ok(Part::Hide),
                    x => Err(format!("unknown part in element: {}", x).into()),
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
                    x => Err(format!("unknown part {}", x).into()),
                }
            }
        }
    }
}

impl FromSexp for FpText {
    fn from_sexp(s: &Sexp) -> SResult<FpText> {
        let mut i = IterAtom::new(s, "fp_text")?;
        let name = i.s("name")?;
        let value = i.s("value")?;
        let parts = i.vec()?;
        let mut fp = FpText::new(name.clone(), value.clone());
        for part in &parts[..] {
            match *part {
                Part::At(ref at) => fp.at.clone_from(at),
                Part::Layer(ref layer) => fp.set_layer(layer),
                Part::Hide => fp.hide = true,
                Part::Effects(ref effects) => fp.set_effects(effects),
                ref x => return Err(format!("fp_text: unknown {:?}", x).into()),
            }
        }
        Ok(fp)
    }
}

impl FromSexp for Pad {
    fn from_sexp(s: &Sexp) -> SResult<Pad> {
        let mut i = IterAtom::new(s, "pad")?;
        let name = i.s("name")?;
        let t = i.s("type")?;
        let t = PadType::from_string(&t)?;
        let shape = i.s("shape")?;
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
                ref x => return Err(format!("pad: unknown {:?}", x).into()),
            }
        }
        Ok(pad)
    }
}

impl FromSexp for FpPoly {
    fn from_sexp(s: &Sexp) -> SResult<FpPoly> {
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
    fn from_sexp(s: &Sexp) -> SResult<FpLine> {
        let mut i = IterAtom::new(s, "fp_line")?;
        let mut fp_line = FpLine::default();
        let parts = i.vec()?;
        for part in &parts[..] {
            match *part {
                Part::Xy(ref xy) if xy.t == XyType::Start => fp_line.start.clone_from(xy),
                Part::Xy(ref xy) if xy.t == XyType::End => fp_line.end.clone_from(xy),
                Part::Layer(ref layer) => fp_line.layer.clone_from(layer),
                Part::Width(w) => fp_line.width = w,
                ref x => return Err(format!("fp_line: unknown {:?}", x).into()),
            }
        }
        Ok(fp_line)
    }
}

impl FromSexp for FpCircle {
    fn from_sexp(s: &Sexp) -> SResult<FpCircle> {
        let mut i = IterAtom::new(s, "fp_circle")?;
        let mut fp_circle = FpCircle::default();
        let parts = i.vec()?;
        for part in &parts[..] {
            match *part {
                Part::Xy(ref xy) if xy.t == XyType::Center => fp_circle.center.clone_from(xy),
                Part::Xy(ref xy) if xy.t == XyType::End => fp_circle.end.clone_from(xy),
                Part::Layer(ref layer) => fp_circle.layer.clone_from(layer),
                Part::Width(w) => fp_circle.width = w,
                ref x => return Err(format!("fp_circle: unexpected {:?}", x).into()),
            }
        }
        Ok(fp_circle)
    }
}

impl FromSexp for FpArc {
    fn from_sexp(s: &Sexp) -> SResult<FpArc> {
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
                ref x => return Err(format!("fp_arc: unexpected {:?}", x).into()),
            }
        }
        Ok(fp_arc)
    }
}

// (model C_0603J.wrl (at (xyz 0 0 0)) (scale (xyz 1 1 1)) (rotate (xyz 0 0 0)))
impl FromSexp for Model {
    fn from_sexp(s: &Sexp) -> SResult<Model> {
        let mut i = IterAtom::new(s, "model")?;
        Ok(Model {
            name: i.s("name")?,
            at: i.t_in_list("at")?,
            scale: i.t_in_list("scale")?,
            rotate: i.t_in_list("rotate")?,
        })
    }
}

fn parse_string_element(s: &Sexp) -> SResult<String> {
    let name = s.list_name()?;
    let mut i = IterAtom::new(s, name)?;
    Ok(i.s("element")?)
}

fn parse_float_element(s: &Sexp) -> SResult<f64> {
    let name = s.list_name()?;
    let mut i = IterAtom::new(s, name)?;
    Ok(i.f("element")?)
}


impl FromSexp for Element {
    fn from_sexp(s: &Sexp) -> SResult<Element> {
        match *s {
            Sexp::String(ref s) => {
                match &s[..] {
                    "locked" => Ok(Element::Locked),
                    _ => Err(format!("unknown element in module: {}", s).into()),
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
                    _ => Err(format!("unknown element in module: {}", name).into()),
                }
            }
            Sexp::Empty => unreachable!(),
        }
    }
}

impl FromSexp for Module {
    fn from_sexp(s: &Sexp) -> SResult<Module> {
        let mut i = IterAtom::new(s, "module")?;
        let name = i.s("name")?;
        let mut module = Module::new(name);
        for e in i.iter {
            let el = from_sexp(e)?;
            module.append(el)
        }
        Ok(module)
    }
}
