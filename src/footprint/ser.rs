// (c) 2016 Productize SPRL <joost@productize.be>

use Sexp;
use symbolic_expressions::IntoSexp;
use footprint::data::*;
use std::f64;

impl IntoSexp for Module {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push("module".into());
        v.push(self.name.clone().into());
        for e in &self.elements {
            v.push(e.into_sexp())
        }
        v.into()
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
            Element::Locked => "locked".into(),
        }
    }
}

impl IntoSexp for FpText {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push("fp_text".into());
        v.push(self.name.clone().into());
        v.push(self.value.clone().into());
        v.push(self.at.into_sexp());
        v.push(("layer", &self.layer).into());
        if self.hide {
            v.push("hide".into());
        }
        v.push(self.effects.into_sexp());
        v.into()
    }
}

impl IntoSexp for At {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push("at".into());
        v.push(self.x.into());
        v.push(self.y.into());
        if self.rot != 0.0 {
            v.push(self.rot.into());
        }
        v.into()
    }
}

impl IntoSexp for Font {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push("font".into());
        if self.size.x > 0.0 || self.size.y > 0.0 {
            let mut v1 = vec![];
            v1.push("size".into());
            v1.push(self.size.x.into());
            v1.push(self.size.y.into());
            v.push(v1.into());
        }
        v.push(("thickness", &self.thickness).into());
        v.into()
    }
}

impl IntoSexp for Effects {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push("effects".into());
        v.push(self.font.into_sexp());
        if let Some(ref j) = self.justify {
            v.push(j.into_sexp())
        }
        v.into()
    }
}

impl IntoSexp for Justify {
    fn into_sexp(&self) -> Sexp {
        match *self {
            Justify::Mirror => ("justify", &"mirror").into(),
        }
    }
}

impl IntoSexp for Xy {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(match self.t {
            XyType::Xy => "xy",
            XyType::Start => "start",
            XyType::End => "end",
            XyType::Size => "size",
            XyType::Center => "center",
            XyType::RectDelta => "rect_delta",
        }.into());
        v.push(self.x.into());
        v.push(self.y.into());
        v.into()
    }
}

impl IntoSexp for Pts {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push("pts".into());
        for x in &self.elements {
            v.push(x.into_sexp())
        }
        v.into()
    }
}

// TODO: kicad doesn't output drill section at all when all is zero?
impl IntoSexp for Drill {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push("drill".into());
        if let Some(ref s) = self.shape {
            v.push(s.clone().into())
        }
        if self.width > 0.0 {
            v.push(self.width.into());
        }
        if self.height > 0.0 && (self.height - self.width).abs() > f64::EPSILON {
            v.push(self.height.into());
        }
        if self.offset_x != 0.0 || self.offset_y != 0.0 {
            let mut v2 = vec![];
            v2.push("offset".into());
            v2.push(self.offset_x.into());
            v2.push(self.offset_y.into());
            v.push(v2.into());
        }
        v.into()
    }
}

impl IntoSexp for PadType {
    fn into_sexp(&self) -> Sexp {
        match *self {
            PadType::Smd => "smd",
            PadType::Pth => "thru_hole",
            PadType::NpPth => "np_thru_hole",
        }.into()
    }
}

impl IntoSexp for PadShape {
    fn into_sexp(&self) -> Sexp {
        match *self {
            PadShape::Rect => "rect",
            PadShape::Circle => "circle",
            PadShape::Oval => "oval",
            PadShape::Trapezoid => "trapezoid",
        }.into()
    }
}

impl IntoSexp for Layer {
    fn into_sexp(&self) -> Sexp {
        format!("{}", self).into()
    }
}

impl IntoSexp for Layers {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push("layers".into());
        for layer in &self.layers {
            v.push(layer.into_sexp())
        }
        v.into()
    }
}

impl IntoSexp for Pad {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push("pad".into());
        v.push(self.name.clone().into());
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
            v.push(("solder_paste_margin", spm).into());
        }
        if let Some(ref spm) = self.solder_mask_margin {
            v.push(("solder_mask_margin", spm).into());
        }
        if let Some(ref spm) = self.clearance {
            v.push(("clearance", spm).into());
        }
        v.into()
    }
}

impl IntoSexp for FpPoly {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push("fp_poly".into());
        v.push(self.pts.into_sexp());
        v.push(("layer", &self.layer).into());
        v.push(("width", &self.width).into());
        v.into()
    }
}

impl IntoSexp for FpLine {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push("fp_line".into());
        v.push(self.start.into_sexp());
        v.push(self.end.into_sexp());
        v.push(("layer", &self.layer).into());
        v.push(("width", &self.width).into());
        v.into()
    }
}

impl IntoSexp for FpCircle {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push("fp_circle".into());
        v.push(self.center.into_sexp());
        v.push(self.end.into_sexp());
        v.push(("layer", &self.layer).into());
        v.push(("width", &self.width).into());
        v.into()
    }
}

impl IntoSexp for FpArc {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push("fp_arc".into());
        v.push(self.start.into_sexp());
        v.push(self.end.into_sexp());
        v.push(("angle", &self.angle).into());
        v.push(("layer", &self.layer).into());
        v.push(("width", &self.width).into());
        v.into()
    }
}

impl IntoSexp for Net {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push("net".into());
        v.push(self.num.into());
        v.push(self.name.clone().into());
        v.into()
    }
}


impl IntoSexp for Model {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push("model".into());
        v.push(self.name.clone().into());
        v.push(("at", &self.at.into_sexp()).into());
        v.push(("scale", &self.scale.into_sexp()).into());
        v.push(("rotate", &self.rotate.into_sexp()).into());
        v.into()
    }
}

impl IntoSexp for Xyz {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push("xyz".into());
        v.push(self.x.into());
        v.push(self.y.into());
        v.push(self.z.into());
        v.into()
    }
}
