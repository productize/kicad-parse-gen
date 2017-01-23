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
        let mut v = Sexp::start("kicad_pcb");

        v.push(("version", &self.version));

        let mut v2 = Sexp::start("host");
        v2.push(&self.host.tool);
        v2.push(&self.host.build);
        v.push(v2);

        v.push(self.general.into_sexp());

        v.push(("page", &self.page));


        let mut v2 = Sexp::start("layers");
        for layer in &self.layers {
            v2.push(layer.into_sexp());
        }
        v.push(v2);

        v.push(self.setup.into_sexp());

        for element in &self.elements {
            v.push(element.into_sexp());
        }
        v
    }
}

impl IntoSexp for General {
    fn into_sexp(&self) -> Sexp {
        let mut v = Sexp::start("general");
        v.push(("links", &self.links));
        v.push(("no_connects", &self.no_connects));
        v.push(self.area.into_sexp());
        v.push(("thickness", &self.thickness));
        v.push(("drawings", &self.drawings));
        v.push(("tracks", &self.tracks));
        v.push(("zones", &self.zones));
        v.push(("modules", &self.modules));
        v.push(("nets", &self.nets));
        v
    }
}

impl IntoSexp for Area {
    fn into_sexp(&self) -> Sexp {
        let mut v = Sexp::start("area");
        v.push(self.x1);
        v.push(self.y1);
        v.push(self.x2);
        v.push(self.y2);
        v
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
        let mut v = Sexp::start("zone");
        v.push(("net", &self.net));
        v.push(("net_name", &self.net_name));
        for o in &self.other {
            v.push(o.clone());
        }
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

impl IntoSexp for NetClass {
    fn into_sexp(&self) -> Sexp {
        let mut v = Sexp::start("net_class");
        v.push(&self.name);
        v.push(&self.desc);
        v.push(("clearance", &self.clearance));
        v.push(("trace_width", &self.trace_width));
        v.push(("via_dia", &self.via_dia));
        v.push(("via_drill", &self.via_drill));
        v.push(("uvia_dia", &self.uvia_dia));
        v.push(("uvia_drill", &self.uvia_drill));
        if let Some(diff_pair_gap) = self.diff_pair_gap {
            v.push(("diff_pair_gap", &diff_pair_gap));
        }
        if let Some(diff_pair_width) = self.diff_pair_width {
            v.push(("diff_pair_width", &diff_pair_width));
        }
        for net in &self.nets {
            v.push(("add_net", net));
        }
        v
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
        let mut v = Sexp::start("setup");
        for ref k in &self.elements {
            v.push(k.into_sexp())
        }
        let mut v2 = Sexp::start("pcbplotparams");
        for ref k in &self.pcbplotparams {
            v2.push(k.into_sexp())
        }
        v.push(v2);
        v
    }
}

impl IntoSexp for GrText {
    fn into_sexp(&self) -> Sexp {
        let mut v = Sexp::start("gr_text");
        v.push(&self.value);
        v.push(self.at.into_sexp());
        v.push(("layer", &self.layer));
        v.push(self.effects.into_sexp());
        v
    }
}

impl IntoSexp for GrLine {
    fn into_sexp(&self) -> Sexp {
        let mut v = Sexp::start("gr_line");
        v.push(self.start.into_sexp());
        v.push(self.end.into_sexp());
        v.push(("angle", &self.angle));
        v.push(("layer", &self.layer));
        v.push(("width", &self.width));
        v
    }
}

impl IntoSexp for GrArc {
    fn into_sexp(&self) -> Sexp {
        let mut v = Sexp::start("gr_arc");
        v.push(self.start.into_sexp());
        v.push(self.end.into_sexp());
        v.push(("angle", &self.angle));
        v.push(("layer", &self.layer));
        v.push(("width", &self.width));
        v
    }
}

impl IntoSexp for Dimension {
    fn into_sexp(&self) -> Sexp {
        let mut v = Sexp::start("dimension");
        v.push(&self.name);
        v.push(("width", &self.width));
        v.push(("layer", &self.layer));
        match self.tstamp {
            None => (),
            Some(ref tstamp) => {
                v.push(("tstamp", tstamp));
            }
        }
        v.push(self.text.into_sexp());
        v.push(("feature1", self.feature1.into_sexp()));
        v.push(("feature2", self.feature2.into_sexp()));
        v.push(("crossbar", self.crossbar.into_sexp()));
        v.push(("arrow1a", self.arrow1a.into_sexp()));
        v.push(("arrow1b", self.arrow1b.into_sexp()));
        v.push(("arrow2a", self.arrow2a.into_sexp()));
        v.push(("arrow2b", self.arrow2b.into_sexp()));
        v
    }
}
