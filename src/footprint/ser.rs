// (c) 2016 Productize SPRL <joost@productize.be>

use Sexp;
use symbolic_expressions::IntoSexp;
use footprint::data::*;
use std::f64;

impl IntoSexp for Module {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("module"));
        v.push(Sexp::new_string(&self.name));
        for e in &self.elements {
            v.push(e.into_sexp())
        }
        Sexp::new_list(v)
    }
}

impl IntoSexp for Element {
    fn into_sexp(&self) -> Sexp {
        match *self {
            Element::SolderMaskMargin(ref s) => Sexp::new_named("solder_mask_margin", s),
            Element::Layer(ref s) => Sexp::new_named("layer", s),
            Element::Descr(ref s) => Sexp::new_named("descr", s),
            Element::Tags(ref s) => Sexp::new_named("tags", s),
            Element::Attr(ref s) => Sexp::new_named("attr", s),
            Element::FpText(ref p) => p.into_sexp(),
            Element::Pad(ref pad) => pad.into_sexp(),
            Element::FpPoly(ref p) => p.into_sexp(),
            Element::FpLine(ref p) => p.into_sexp(),
            Element::FpCircle(ref p) => p.into_sexp(),
            Element::FpArc(ref p) => p.into_sexp(),
            Element::TEdit(ref p) => Sexp::new_named("tedit", p),
            Element::TStamp(ref p) => Sexp::new_named("tstamp", p),
            Element::Path(ref p) => Sexp::new_named("path", p),
            Element::At(ref p) => p.into_sexp(),
            Element::Model(ref p) => p.into_sexp(),
            Element::Locked => Sexp::new_string("locked"),
        }
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

impl IntoSexp for Font {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("font"));
        if self.size.x > 0.0 || self.size.y > 0.0 {
            let mut v1 = vec![];
            v1.push(Sexp::new_string("size"));
            v1.push(Sexp::new_string(self.size.x));
            v1.push(Sexp::new_string(self.size.y));
            v.push(Sexp::new_list(v1));
        }
        v.push(Sexp::new_named("thickness", self.thickness));
        Sexp::new_list(v)
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

impl IntoSexp for Justify {
    fn into_sexp(&self) -> Sexp {
        match *self {
            Justify::Mirror => Sexp::new_named("justify","mirror"),
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
            XyType::RectDelta => "rect_delta",
        }));
        v.push(Sexp::new_string(self.x));
        v.push(Sexp::new_string(self.y));
        Sexp::new_list(v)
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

// TODO: kicad doesn't output drill section at all when all is zero?
impl IntoSexp for Drill {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("drill"));
        if let Some(ref s) = self.shape {
            v.push(Sexp::new_string(s))
        }
        if self.width > 0.0 {
            v.push(Sexp::new_string(self.width));
        }
        if self.height > 0.0 && (self.height - self.width).abs() > f64::EPSILON {
            v.push(Sexp::new_string(self.height));
        }
        if self.offset_x != 0.0 || self.offset_y != 0.0 {
            let mut v2 = vec![];
            v2.push(Sexp::new_string("offset"));
            v2.push(Sexp::new_string(self.offset_x));
            v2.push(Sexp::new_string(self.offset_y));
            v.push(Sexp::new_list(v2));
        }
        Sexp::new_list(v)
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

impl IntoSexp for PadShape {
    fn into_sexp(&self) -> Sexp {
        Sexp::new_string(match *self {
            PadShape::Rect => "rect",
            PadShape::Circle => "circle",
            PadShape::Oval => "oval",
            PadShape::Trapezoid => "trapezoid",
        })
    }
}

impl IntoSexp for Layer {
    fn into_sexp(&self) -> Sexp {
        Sexp::new_string(&self)
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

impl IntoSexp for Pad {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("pad"));
        v.push(Sexp::new_string(&self.name));
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
            v.push(Sexp::new_named("solder_paste_margin", spm));
        }
        if let Some(ref spm) = self.solder_mask_margin {
            v.push(Sexp::new_named("solder_mask_margin", spm));
        }
        if let Some(ref spm) = self.clearance {
            v.push(Sexp::new_named("clearance", spm));
        }
        Sexp::new_list(v)
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

impl IntoSexp for FpArc {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("fp_arc"));
        v.push(self.start.into_sexp());
        v.push(self.end.into_sexp());
        v.push(Sexp::new_named("angle", self.angle));
        v.push(Sexp::new_named("layer", &self.layer));
        v.push(Sexp::new_named("width", self.width));
        Sexp::new_list(v)
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
