// (c) 2016-2017 Productize SPRL <joost@productize.be>

use Sexp;
use footprint;
use footprint::data::*;
use wrap;
use symbolic_expressions::iteratom::*;
use symbolic_expressions::SexpError;

// TODO: get rid of Part
#[derive(Debug)]
enum Part {
    Thickness(f64),
    Net(footprint::Net),
    SolderPasteMargin(f64),
    SolderMaskMargin(f64),
    Clearance(f64),
    ThermalGap(f64),
    ZoneConnect(i64),
}

struct Offset(f64, f64);

impl FromSexp for Offset {
    fn from_sexp(s: &Sexp) -> Result<Offset, SexpError> {
        let mut i = IterAtom::new(s, "offset")?;
        let x = i.f("x")?;
        let y = i.f("y")?;
        i.close(Offset(x, y))
    }
}

// (at 0.0 -4.0) (at -2.575 -1.625 180)
impl FromSexp for At {
    fn from_sexp(s: &Sexp) -> Result<At, SexpError> {
        let mut i = IterAtom::new(s, "at")?;
        let x = i.f("x")?;
        let y = i.f("y")?;
        let rot = i.maybe_f().unwrap_or(0.0);
        i.close(At::new(x, y, rot))
    }
}

impl FromSexp for Layer {
    fn from_sexp(s: &Sexp) -> Result<Layer, SexpError> {
        let mut i = IterAtom::new(s, "layer")?;
        let layer = i.s("layername")?;
        let layer = Layer::from_string(&layer)?;
        i.close(layer)
    }
}

impl FromSexp for Effects {
    fn from_sexp(s: &Sexp) -> Result<Effects, SexpError> {
        let mut i = IterAtom::new(s, "effects")?;
        let font = i.t("font")?;
        let justify = i.maybe_t();
        i.close(Effects::from_font(font, justify))
    }
}

impl FromSexp for Justify {
    fn from_sexp(s: &Sexp) -> Result<Justify, SexpError> {
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

// (font (size 0.625 0.625) (thickness 0.1))
impl FromSexp for Font {
    fn from_sexp(s: &Sexp) -> Result<Font, SexpError> {
        let mut i = IterAtom::new(s, "font")?;
        let mut font = Font::default();
        if let Some(xy) = i.maybe_t::<Xy>() {
            font.size.x = xy.x;
            font.size.y = xy.y;
        }
        if let Some(thickness) = i.maybe_f_in_list("thickness") {
            font.thickness = thickness;
        }
        if i.maybe_literal_s("italic").is_some() {
            font.italic = true
        }
        i.close(font)
    }
}

impl FromSexp for Layers {
    fn from_sexp(s: &Sexp) -> Result<Layers, SexpError> {
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

fn parse_part_float<F>(e: &Sexp, make: F) -> Result<Part, SexpError>
where
    F: Fn(f64) -> Part,
{
    let v = e.list()?;
    if v.len() < 2 {
        return Err(format!("not enough elements in {}", e).into());
    }
    let f = v[1].f()?;
    Ok(make(f))
}

fn parse_part_int<F>(e: &Sexp, make: F) -> Result<Part, SexpError>
where
    F: Fn(i64) -> Part,
{
    let v = e.list()?;
    if v.len() < 2 {
        return Err(format!("not enough elements in {}", e).into());
    }
    let i = v[1].i()?;
    Ok(make(i))
}

impl FromSexp for Xy {
    fn from_sexp(s: &Sexp) -> Result<Xy, SexpError> {
        let name: &str = &s.list_name()?[..];
        let t: Result<XyType, SexpError> = match name {
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
        i.close(Xy::new(x, y, t))
    }
}

impl FromSexp for Pts {
    fn from_sexp(s: &Sexp) -> Result<Pts, SexpError> {
        let mut i = IterAtom::new(s, "pts")?;
        let r = i.vec()?;
        Ok(Pts { elements: r })
    }
}


impl FromSexp for Xyz {
    fn from_sexp(s: &Sexp) -> Result<Xyz, SexpError> {
        let mut i = IterAtom::new(s, "xyz")?;
        let x = i.f("y")?;
        let y = i.f("y")?;
        let z = i.f("z")?;
        i.close(Xyz::new(x, y, z))
    }
}

impl FromSexp for Net {
    fn from_sexp(s: &Sexp) -> Result<Net, SexpError> {
        let mut i = IterAtom::new(s, "net")?;
        let num = i.i("num")?;
        let name = i.s("name")?;
        i.close(Net {
            num: num,
            name: name.into(),
        })
    }
}

impl FromSexp for Drill {
    fn from_sexp(s: &Sexp) -> Result<Drill, SexpError> {
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
    fn from_sexp(s: &Sexp) -> Result<Part, SexpError> {
        let name = &(s.list_name()?)[..];
        match name {
            "thickness" => parse_part_float(s, Part::Thickness),
            "net" => wrap(s, from_sexp, Part::Net),
            "solder_paste_margin" => parse_part_float(s, Part::SolderPasteMargin),
            "solder_mask_margin" => parse_part_float(s, Part::SolderMaskMargin),
            "clearance" => parse_part_float(s, Part::Clearance),
            "thermal_gap" => parse_part_float(s, Part::ThermalGap),
            "zone_connect" => parse_part_int(s, Part::ZoneConnect),
            x => Err(format!("unknown part {}", x).into()),
        }
    }
}

//   (fp_text value MOSFET-N-GSD (at 0 2.6) (layer F.SilkS) hide (effects (font (size 0.625 0.625) (thickness 0.1))))
impl FromSexp for FpText {
    fn from_sexp(s: &Sexp) -> Result<FpText, SexpError> {
        let mut i = IterAtom::new(s, "fp_text")?;
        let name = i.s("name")?;
        let value = i.s("value")?;
        let mut fp = FpText::new(name, value);
        if let Some(at) = i.maybe_t::<footprint::At>() {
            fp.at = at;
        }
        if let Some(layer) = i.maybe_t::<footprint::Layer>() {
            fp.layer = layer;
        }
        if i.maybe_literal_s("hide").is_some() {
            fp.hide = true
        }
        if let Some(effects) = i.maybe_t::<footprint::Effects>() {
            fp.effects = effects;
        }
        i.close(fp)
    }
}

// (pad 1 smd rect (at -0.95 0.885) (size 0.802 0.972) (layers F.Cu F.Paste F.Mask))
impl FromSexp for Pad {
    fn from_sexp(s: &Sexp) -> Result<Pad, SexpError> {
        let mut i = IterAtom::new(s, "pad")?;
        let name = i.s("name")?;
        let t = i.s("type")?;
        let t = PadType::from_string(&t)?;
        let shape = i.s("shape")?;
        let shape = PadShape::from_string(&shape)?;
        let mut pad = Pad::new(name, t, shape);
        // println!("{}", pad);
        if let Some(at) = i.maybe_t::<At>() {
            pad.at = at;
        }
        if let Some(xy) = i.maybe_t::<Xy>() {
            if xy.t == XyType::Size {
                pad.size = xy;
            } else if xy.t == XyType::RectDelta {
                pad.rect_delta = Some(xy);
            }
            // else ignore
        }
        pad.drill = i.maybe_t::<Drill>();
        if let Some(layers) = i.maybe_t::<Layers>() {
            pad.layers = layers;
        }
        let parts = i.vec()?;
        for part in parts {
            match part {
                Part::Net(n) => pad.set_net(n),
                Part::SolderPasteMargin(n) => pad.solder_paste_margin = Some(n),
                Part::SolderMaskMargin(n) => pad.solder_mask_margin = Some(n),
                Part::Clearance(n) => pad.clearance = Some(n),
                Part::ThermalGap(n) => pad.thermal_gap = Some(n),
                Part::ZoneConnect(n) => pad.zone_connect = Some(n),
                ref x => return Err(format!("pad: unknown {:?}", x).into()),
            }
        }
        Ok(pad)
    }
}

// (fp_poly (pts (xy 0.7 0.65) (xy 0.7 1.15) (xy 1.2 1.15) (xy 1.2 0.65) (xy 0.7 0.65)) (layer Dwgs.User) (width 0.15))
impl FromSexp for FpPoly {
    fn from_sexp(s: &Sexp) -> Result<FpPoly, SexpError> {
        let mut i = IterAtom::new(s, "fp_poly")?;
        let mut fp_poly = FpPoly::default();
        if let Some(pts) = i.maybe_t::<Pts>() {
            fp_poly.pts = pts;
        }
        if let Some(layer) = i.maybe_t::<Layer>() {
            fp_poly.layer = layer;
        }
        if let Some(width) = i.maybe_f_in_list("width") {
            fp_poly.width = width;
        }
        i.close(fp_poly)
    }
}

// (fp_line (start -1.5 -1.5) (end -1.5 1.5) (layer F.SilkS) (width 0.1))
impl FromSexp for FpLine {
    fn from_sexp(s: &Sexp) -> Result<FpLine, SexpError> {
        let mut i = IterAtom::new(s, "fp_line")?;
        let mut fp_line = FpLine::default();
        if let Some(xy) = i.maybe_t::<Xy>() {
            if xy.t == XyType::Start {
                fp_line.start = xy;
            }
        }
        if let Some(xy) = i.maybe_t::<Xy>() {
            if xy.t == XyType::End {
                fp_line.end = xy;
            }
        }
        if let Some(layer) = i.maybe_t::<Layer>() {
            fp_line.layer = layer;
        }
        if let Some(width) = i.maybe_f_in_list("width") {
            fp_line.width = width;
        }
        i.close(fp_line)
    }
}

// (fp_circle (center -2.6 -2.6) (end -2.467417 -2.467417) (layer F.SilkS) (width 0.1875))
impl FromSexp for FpCircle {
    fn from_sexp(s: &Sexp) -> Result<FpCircle, SexpError> {
        let mut i = IterAtom::new(s, "fp_circle")?;
        let mut fp_circle = FpCircle::default();
        if let Some(xy) = i.maybe_t::<Xy>() {
            if xy.t == XyType::Center {
                fp_circle.center = xy;
            }
        }
        if let Some(xy) = i.maybe_t::<Xy>() {
            if xy.t == XyType::End {
                fp_circle.end = xy;
            }
        }
        if let Some(layer) = i.maybe_t::<Layer>() {
            fp_circle.layer = layer;
        }
        if let Some(width) = i.maybe_f_in_list("width") {
            fp_circle.width = width;
        }
        i.close(fp_circle)
    }
}

// (fp_arc (start 4.15 4.25) (end 5.15 4.25) (angle 86.6) (layer F.SilkS) (width 0.1))
impl FromSexp for FpArc {
    fn from_sexp(s: &Sexp) -> Result<FpArc, SexpError> {
        let mut i = IterAtom::new(s, "fp_arc")?;
        let mut fp_arc = FpArc::default();
        if let Some(xy) = i.maybe_t::<Xy>() {
            if xy.t == XyType::Start {
                fp_arc.start = xy;
            }
        }
        if let Some(xy) = i.maybe_t::<Xy>() {
            if xy.t == XyType::End {
                fp_arc.end = xy;
            }
        }
        if let Some(angle) = i.maybe_f_in_list("angle") {
            fp_arc.angle = angle;
        }
        if let Some(layer) = i.maybe_t::<Layer>() {
            fp_arc.layer = layer;
        }
        if let Some(width) = i.maybe_f_in_list("width") {
            fp_arc.width = width;
        }
        i.close(fp_arc)
    }
}

// (model C_0603J.wrl (at (xyz 0 0 0)) (scale (xyz 1 1 1)) (rotate (xyz 0 0 0)))
impl FromSexp for Model {
    fn from_sexp(s: &Sexp) -> Result<Model, SexpError> {
        let mut i = IterAtom::new(s, "model")?;
        let model = Model {
            name: i.s("name")?,
            at: i.t_in_list("at")?,
            scale: i.t_in_list("scale")?,
            rotate: i.t_in_list("rotate")?,
        };
        i.close(model)
    }
}

fn parse_string_element(s: &Sexp) -> Result<String, SexpError> {
    let name = s.list_name()?;
    let mut i = IterAtom::new(s, name)?;
    Ok(i.s("element")?)
}

fn parse_float_element(s: &Sexp) -> Result<f64, SexpError> {
    let name = s.list_name()?;
    let mut i = IterAtom::new(s, name)?;
    Ok(i.f("element")?)
}

fn parse_stamp_element(s: &Sexp) -> Result<i64, SexpError> {
    let name = s.list_name()?;
    let mut i = IterAtom::new(s, name)?;
    let s = i.s("element")?;
    let i = i64::from_str_radix(&s, 16)?;
    Ok(i)
}

impl FromSexp for Element {
    fn from_sexp(s: &Sexp) -> Result<Element, SexpError> {
        match *s {
            Sexp::String(ref s) => match &s[..] {
                "locked" => Ok(Element::Locked),
                _ => Err(format!("unknown element in module: {}", s).into()),
            },
            Sexp::List(_) => {
                let name = s.list_name()?;
                match &name[..] {
                    "solder_mask_margin" => wrap(s, parse_float_element, Element::SolderMaskMargin),
                    "layer" => wrap(s, from_sexp, Element::Layer),
                    "descr" => wrap(s, parse_string_element, Element::Descr),
                    "tags" => wrap(s, parse_string_element, Element::Tags),
                    "attr" => wrap(s, parse_string_element, Element::Attr),
                    "fp_text" => wrap(s, from_sexp, Element::FpText),
                    "pad" => wrap(s, from_sexp, Element::Pad),
                    "fp_poly" => wrap(s, from_sexp, Element::FpPoly),
                    "fp_line" => wrap(s, from_sexp, Element::FpLine),
                    "fp_circle" => wrap(s, from_sexp, Element::FpCircle),
                    "fp_arc" => wrap(s, from_sexp, Element::FpArc),
                    "tedit" => wrap(s, parse_stamp_element, Element::TEdit),
                    "tstamp" => wrap(s, parse_stamp_element, Element::TStamp),
                    "path" => wrap(s, parse_string_element, Element::Path),
                    "at" => wrap(s, from_sexp, Element::At),
                    "model" => wrap(s, from_sexp, Element::Model),
                    "clearance" => wrap(s, parse_float_element, Element::Clearance),
                    _ => Err(format!("unknown element in module: {}", name).into()),
                }
            }
            Sexp::Empty => unreachable!(),
        }
    }
}

impl FromSexp for Module {
    fn from_sexp(s: &Sexp) -> Result<Module, SexpError> {
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
