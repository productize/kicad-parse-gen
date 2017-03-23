// (c) 2016 Productize SPRL <joost@productize.be>

use Sexp;
use symbolic_expressions::IntoSexp;
use footprint::data::*;
use std::f64;

impl IntoSexp for Module {
    fn into_sexp(&self) -> Sexp {
        let mut v = Sexp::start("module");
        v.push(&self.name);
        for e in &self.elements {
            v.push(e.into_sexp())
        }
        v
    }
}

impl IntoSexp for Element {
    fn into_sexp(&self) -> Sexp {
        match *self {
            Element::SolderMaskMargin(ref s) => ("solder_mask_margin", s).into(),
            Element::Layer(ref s) => ("layer", s).into(),
            Element::Descr(ref s) => ("descr", s).into(),
            Element::Tags(ref s) => ("tags", s).into(),
            Element::Attr(ref s) => ("attr", s).into(),
            Element::FpText(ref p) => p.into_sexp(),
            Element::Pad(ref pad) => pad.into_sexp(),
            Element::FpPoly(ref p) => p.into_sexp(),
            Element::FpLine(ref p) => p.into_sexp(),
            Element::FpCircle(ref p) => p.into_sexp(),
            Element::FpArc(ref p) => p.into_sexp(),
            Element::TEdit(ref p) => ("tedit", p).into(),
            Element::TStamp(ref p) => ("tstamp", p).into(),
            Element::Path(ref p) => ("path", p).into(),
            Element::At(ref p) => p.into_sexp(),
            Element::Model(ref p) => p.into_sexp(),
            Element::Clearance(ref s) => ("clearance", s).into(),
            Element::Locked => "locked".into(),
        }
    }
}

impl IntoSexp for FpText {
    fn into_sexp(&self) -> Sexp {
        let mut v = Sexp::start("fp_text");
        v.push(&self.name);
        v.push(&self.value);
        v.push(self.at.into_sexp());
        v.push(("layer", &self.layer));
        if self.hide {
            v.push("hide");
        }
        v.push(self.effects.into_sexp());
        v
    }
}

impl IntoSexp for At {
    fn into_sexp(&self) -> Sexp {
        let mut v = Sexp::start("at");
        v.push(self.x);
        v.push(self.y);
        if self.rot != 0.0 {
            v.push(self.rot);
        }
        v
    }
}

impl IntoSexp for Font {
    fn into_sexp(&self) -> Sexp {
        let mut v = Sexp::start("font");
        if self.size.x > 0.0 || self.size.y > 0.0 {
            let mut v1 = Sexp::start("size");
            v1.push(self.size.x);
            v1.push(self.size.y);
            v.push(v1);
        }
        v.push(("thickness", &self.thickness));
        if self.italic {
            v.push("italic")
        }
        v
    }
}

impl IntoSexp for Effects {
    fn into_sexp(&self) -> Sexp {
        let mut v = Sexp::start("effects");
        v.push(self.font.into_sexp());
        if let Some(ref j) = self.justify {
            v.push(j.into_sexp())
        }
        v
    }
}

impl IntoSexp for Justify {
    fn into_sexp(&self) -> Sexp {
        match *self {
            Justify::Mirror => ("justify", &"mirror").into(),
            Justify::Left => ("justify", &"left").into(),
            Justify::Right => ("justify", &"right").into(),
        }
    }
}

impl IntoSexp for Xy {
    fn into_sexp(&self) -> Sexp {
        let mut v = Sexp::start(match self.t {
            XyType::Xy => "xy",
            XyType::Start => "start",
            XyType::End => "end",
            XyType::Size => "size",
            XyType::Center => "center",
            XyType::RectDelta => "rect_delta",
        });
        v.push(self.x);
        v.push(self.y);
        v
    }
}

impl IntoSexp for Pts {
    fn into_sexp(&self) -> Sexp {
        let mut v = Sexp::start("pts");
        for x in &self.elements {
            v.push(x.into_sexp())
        }
        v
    }
}

// TODO: kicad doesn't output drill section at all when all is zero?
impl IntoSexp for Drill {
    fn into_sexp(&self) -> Sexp {
        let mut v = Sexp::start("drill");
        if let Some(ref s) = self.shape {
            v.push(s)
        }
        if self.width > 0.0 {
            v.push(self.width);
        }
        if self.height > 0.0 && (self.height - self.width).abs() > f64::EPSILON {
            v.push(self.height);
        }
        if self.offset_x != 0.0 || self.offset_y != 0.0 {
            let mut v2 = Sexp::start("offset");
            v2.push(self.offset_x);
            v2.push(self.offset_y);
            v.push(v2);
        }
        v
    }
}

impl IntoSexp for PadType {
    fn into_sexp(&self) -> Sexp {
        match *self {
                PadType::Smd => "smd",
                PadType::Pth => "thru_hole",
                PadType::NpPth => "np_thru_hole",
            }
            .into()
    }
}

impl IntoSexp for PadShape {
    fn into_sexp(&self) -> Sexp {
        match *self {
                PadShape::Rect => "rect",
                PadShape::Circle => "circle",
                PadShape::Oval => "oval",
                PadShape::Trapezoid => "trapezoid",
            }
            .into()
    }
}

impl IntoSexp for Layer {
    fn into_sexp(&self) -> Sexp {
        format!("{}", self).into()
    }
}

impl IntoSexp for Layers {
    fn into_sexp(&self) -> Sexp {
        let mut v = Sexp::start("layers");
        for layer in &self.layers {
            v.push(layer.into_sexp())
        }
        v
    }
}

impl IntoSexp for Pad {
    fn into_sexp(&self) -> Sexp {
        let mut v = Sexp::start("pad");
        v.push(&self.name);
        v.push(self.t.into_sexp());
        v.push(self.shape.into_sexp());
        v.push(self.at.into_sexp());
        v.push(self.size.into_sexp());
        if let Some(ref drill) = self.drill {
            v.push(drill.into_sexp());
        }
        if let Some(ref rect_delta) = self.rect_delta {
            v.push(rect_delta.into_sexp());
        }
        v.push(self.layers.into_sexp());
        if let Some(ref net) = self.net {
            v.push(net.into_sexp());
        }
        if let Some(ref spm) = self.solder_paste_margin {
            v.push(("solder_paste_margin", spm));
        }
        if let Some(ref spm) = self.solder_mask_margin {
            v.push(("solder_mask_margin", spm));
        }
        if let Some(ref spm) = self.clearance {
            v.push(("clearance", spm));
        }
        if let Some(ref spm) = self.thermal_gap {
            v.push(("thermal_gap", spm));
        }
        v
    }
}

impl IntoSexp for FpPoly {
    fn into_sexp(&self) -> Sexp {
        let mut v = Sexp::start("fp_poly");
        v.push(self.pts.into_sexp());
        v.push(("layer", &self.layer));
        v.push(("width", &self.width));
        v
    }
}

impl IntoSexp for FpLine {
    fn into_sexp(&self) -> Sexp {
        let mut v = Sexp::start("fp_line");
        v.push(self.start.into_sexp());
        v.push(self.end.into_sexp());
        v.push(("layer", &self.layer));
        v.push(("width", &self.width));
        v
    }
}

impl IntoSexp for FpCircle {
    fn into_sexp(&self) -> Sexp {
        let mut v = Sexp::start("fp_circle");
        v.push(self.center.into_sexp());
        v.push(self.end.into_sexp());
        v.push(("layer", &self.layer));
        v.push(("width", &self.width));
        v
    }
}

impl IntoSexp for FpArc {
    fn into_sexp(&self) -> Sexp {
        let mut v = Sexp::start("fp_arc");
        v.push(self.start.into_sexp());
        v.push(self.end.into_sexp());
        v.push(("angle", &self.angle));
        v.push(("layer", &self.layer));
        v.push(("width", &self.width));
        v
    }
}

impl IntoSexp for Net {
    fn into_sexp(&self) -> Sexp {
        let mut v = Sexp::start("net");
        v.push(self.num);
        v.push(&self.name);
        v
    }
}


impl IntoSexp for Model {
    fn into_sexp(&self) -> Sexp {
        let mut v = Sexp::start("model");
        v.push(&self.name);
        v.push(("at", self.at.into_sexp()));
        v.push(("scale", self.scale.into_sexp()));
        v.push(("rotate", self.rotate.into_sexp()));
        v
    }
}

impl IntoSexp for Xyz {
    fn into_sexp(&self) -> Sexp {
        let mut v = Sexp::start("xyz");
        v.push(self.x);
        v.push(self.y);
        v.push(self.z);
        v
    }
}
