// (c) 2016-2017 Productize SPRL <joost@productize.be>

// extension: .kicad_pcb
// format: new-style

use std::fmt;
use std::result;

// from parent
use Sexp;
use symbolic_expressions::IntoSexp;
use layout::data::*;

impl IntoSexp for Layout {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push("kicad_pcb".into());

        v.push(("version", &self.version).into());

        let mut v2 = vec![];
        v2.push("host".into());
        v2.push(self.host.tool.clone().into());
        v2.push(self.host.build.clone().into());
        v.push(v2.into());

        v.push(self.general.into_sexp());

        v.push(("page", &self.page).into());


        let mut v2 = vec![];
        v2.push("layers".into());
        for layer in &self.layers {
            v2.push(layer.into_sexp());
        }
        v.push(v2.into());

        v.push(self.setup.into_sexp());

        for element in &self.elements {
            v.push(element.into_sexp());
        }
        v.into()
    }
}

impl IntoSexp for General {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push("general".into());
        v.push(("links", &self.links).into());
        v.push(("no_connects", &self.no_connects).into());
        v.push(self.area.into_sexp());
        v.push(("thickness", &self.thickness).into());
        v.push(("drawings", &self.drawings).into());
        v.push(("tracks", &self.tracks).into());
        v.push(("zones", &self.zones).into());
        v.push(("modules", &self.modules).into());
        v.push(("nets", &self.nets).into());
        v.into()
    }
}

impl IntoSexp for Area {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push("area".into());
        v.push(self.x1.into());
        v.push(self.y1.into());
        v.push(self.x2.into());
        v.push(self.y2.into());
        v.into()
    }
}

impl IntoSexp for Element {
    fn into_sexp(&self) -> Sexp {
        match *self {
            Element::Other(ref s) => s.clone(),
            Element::Module(ref s) => s.into_sexp(),
            Element::Net(ref s) => s.into_sexp(),
            Element::NetClass(ref s) => s.into_sexp(),
            Element::GrText(ref s) => s.into_sexp(),
            Element::GrLine(ref s) => s.into_sexp(),
            Element::GrArc(ref s) => s.into_sexp(),
            Element::Dimension(ref s) => s.into_sexp(),
            Element::Zone(ref s) => s.into_sexp(),
        }
    }
}

impl IntoSexp for Zone {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push("zone".into());
        v.push(("net", &self.net).into());
        v.push(("net_name", &self.net_name).into());
        for o in &self.other {
            v.push(o.clone());
        }
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

impl IntoSexp for NetClass {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push("net_class".into());
        v.push(self.name.clone().into());
        v.push(self.desc.clone().into());
        v.push(("clearance", &self.clearance).into());
        v.push(("trace_width", &self.trace_width).into());
        v.push(("via_dia", &self.via_dia).into());
        v.push(("via_drill", &self.via_drill).into());
        v.push(("uvia_dia", &self.uvia_dia).into());
        v.push(("uvia_drill", &self.uvia_drill).into());
        if let Some(diff_pair_gap) = self.diff_pair_gap {
            v.push(("diff_pair_gap", &diff_pair_gap).into());
        }
        if let Some(diff_pair_width) = self.diff_pair_width {
            v.push(("diff_pair_width", &diff_pair_width).into());
        }
        for net in &self.nets {
            v.push(("add_net", net).into());
        }
        v.into()
    }
}

impl IntoSexp for Layer {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(self.num.into());
        v.push(format!("{}", self.layer).into());
        v.push(format!("{}", self.layer_type).into());
        if self.hide {
            v.push("hide".into());
        }
        v.into()
    }
}

impl fmt::Display for LayerType {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        let s = match *self {
            LayerType::Signal => "signal",
            LayerType::User => "user",
        };
        write!(f, "{}", s)
    }
}

impl IntoSexp for SetupElement {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(self.name.clone().into());
        v.push(self.value1.clone().into());
        if let Some(ref x) = self.value2 {
            v.push(x.clone().into())
        }
        v.into()
    }
}

impl IntoSexp for Setup {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push("setup".into());
        for ref k in &self.elements {
            v.push(k.into_sexp())
        }
        let mut v2 = vec![];
        v2.push("pcbplotparams".into());
        for ref k in &self.pcbplotparams {
            v2.push(k.into_sexp())
        }
        v.push(v2.into());
        v.into()
    }
}

impl IntoSexp for GrText {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push("gr_text".into());
        v.push(self.value.clone().into());
        v.push(self.at.into_sexp());
        v.push(("layer", &self.layer).into());
        v.push(self.effects.into_sexp());
        v.into()
    }
}

impl IntoSexp for GrLine {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push("gr_line".into());
        v.push(self.start.into_sexp());
        v.push(self.end.into_sexp());
        v.push(("angle", &self.angle).into());
        v.push(("layer", &self.layer).into());
        v.push(("width", &self.width).into());
        v.into()
    }
}

impl IntoSexp for GrArc {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push("gr_arc".into());
        v.push(self.start.into_sexp());
        v.push(self.end.into_sexp());
        v.push(("angle", &self.angle).into());
        v.push(("layer", &self.layer).into());
        v.push(("width", &self.width).into());
        v.into()
    }
}

impl IntoSexp for Dimension {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push("dimension".into());
        v.push(self.name.clone().into());
        v.push(("width", &self.width).into());
        v.push(("layer", &self.layer).into());
        match self.tstamp {
            None => (),
            Some(ref tstamp) => {
                v.push(("tstamp", tstamp).into());
            }
        }
        v.push(self.text.into_sexp());
        v.push(Sexp::new_named_sexp("feature1", &self.feature1));
        v.push(Sexp::new_named_sexp("feature2", &self.feature2));
        v.push(Sexp::new_named_sexp("crossbar", &self.crossbar));
        v.push(Sexp::new_named_sexp("arrow1a", &self.arrow1a));
        v.push(Sexp::new_named_sexp("arrow1b", &self.arrow1b));
        v.push(Sexp::new_named_sexp("arrow2a", &self.arrow2a));
        v.push(Sexp::new_named_sexp("arrow2b", &self.arrow2b));
        v.into()
    }
}
