// (c) 2016 Productize SPRL <joost@productize.be>

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
        v.push(Sexp::new_string("kicad_pcb"));

        v.push(Sexp::new_named("version", self.version));

        let mut v2 = vec![];
        v2.push(Sexp::new_string("host"));
        v2.push(Sexp::new_string(self.host.tool.clone()));
        v2.push(Sexp::new_string(self.host.build.clone()));
        v.push(Sexp::new_list(v2));

        v.push(self.general.into_sexp());

        v.push(Sexp::new_named("page", self.page.clone()));


        let mut v2 = vec![];
        v2.push(Sexp::new_string("layers"));
        for layer in &self.layers {
            v2.push(layer.into_sexp());
        }
        v.push(Sexp::new_list(v2));

        v.push(self.setup.into_sexp());

        for element in &self.elements {
            v.push(element.into_sexp());
        }
        Sexp::new_list(v)
    }
}

impl IntoSexp for General {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("general"));
        v.push(Sexp::new_named("links", self.links));
        v.push(Sexp::new_named("no_connects", self.no_connects));
        v.push(self.area.into_sexp());
        v.push(Sexp::new_named("thickness", self.thickness));
        v.push(Sexp::new_named("drawings", self.drawings));
        v.push(Sexp::new_named("tracks", self.tracks));
        v.push(Sexp::new_named("zones", self.zones));
        v.push(Sexp::new_named("modules", self.modules));
        v.push(Sexp::new_named("nets", self.nets));
        Sexp::new_list(v)
    }
}

impl IntoSexp for Area {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("area"));
        v.push(Sexp::new_string(self.x1));
        v.push(Sexp::new_string(self.y1));
        v.push(Sexp::new_string(self.x2));
        v.push(Sexp::new_string(self.y2));
        Sexp::new_list(v)
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
        v.push(Sexp::new_string("zone"));
        v.push(Sexp::new_named("net", self.net));
        v.push(Sexp::new_named("net_name", &self.net_name));
        for o in &self.other {
            v.push(o.clone());
        }
        Sexp::new_list(v)
    }
}

impl IntoSexp for Net {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("net"));
        v.push(Sexp::new_string(self.num));
        v.push(Sexp::new_string(self.name.clone()));
        Sexp::new_list(v)
    }
}

impl IntoSexp for NetClass {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("net_class"));
        v.push(Sexp::new_string(self.name.clone()));
        v.push(Sexp::new_string(self.desc.clone()));
        v.push(Sexp::new_named("clearance", self.clearance));
        v.push(Sexp::new_named("trace_width", self.trace_width));
        v.push(Sexp::new_named("via_dia", self.via_dia));
        v.push(Sexp::new_named("via_drill", self.via_drill));
        v.push(Sexp::new_named("uvia_dia", self.uvia_dia));
        v.push(Sexp::new_named("uvia_drill", self.uvia_drill));
        for net in &self.nets {
            v.push(Sexp::new_named("add_net", net));
        }
        Sexp::new_list(v)
    }
}

impl IntoSexp for Layer {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string(self.num));
        v.push(Sexp::new_string(self.layer.clone()));
        v.push(Sexp::new_string(self.layer_type.clone()));
        if self.hide {
            v.push(Sexp::new_string("hide"));
        }
        Sexp::new_list(v)
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
        v.push(Sexp::new_string(self.name.clone()));
        v.push(Sexp::new_string(self.value1.clone()));
        if let Some(ref x) = self.value2 {
            v.push(Sexp::new_string(x))
        }
        Sexp::new_list(v)
    }
}

impl IntoSexp for Setup {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("setup"));
        for ref k in &self.elements {
            v.push(k.into_sexp())
        }
        let mut v2 = vec![];
        v2.push(Sexp::new_string("pcbplotparams"));
        for ref k in &self.pcbplotparams {
            v2.push(k.into_sexp())
        }
        v.push(Sexp::new_list(v2));
        Sexp::new_list(v)
    }
}

impl IntoSexp for GrText {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("gr_text"));
        v.push(Sexp::new_string(self.value.clone()));
        v.push(self.at.into_sexp());
        v.push(Sexp::new_named_sexp("layer", &self.layer));
        v.push(self.effects.into_sexp());
        Sexp::new_list(v)
    }
}

impl IntoSexp for GrLine {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("gr_line"));
        v.push(self.start.into_sexp());
        v.push(self.end.into_sexp());
        v.push(Sexp::new_named("angle", self.angle));
        v.push(Sexp::new_named_sexp("layer", &self.layer));
        v.push(Sexp::new_named("width", self.width));
        Sexp::new_list(v)
    }
}

impl IntoSexp for GrArc {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("gr_arc"));
        v.push(self.start.into_sexp());
        v.push(self.end.into_sexp());
        v.push(Sexp::new_named("angle", self.angle));
        v.push(Sexp::new_named_sexp("layer", &self.layer));
        v.push(Sexp::new_named("width", self.width));
        Sexp::new_list(v)
    }
}

impl IntoSexp for Dimension {
    fn into_sexp(&self) -> Sexp {
        let mut v = vec![];
        v.push(Sexp::new_string("dimension"));
        v.push(Sexp::new_string(self.name.clone()));
        v.push(Sexp::new_named("width", self.width));
        v.push(Sexp::new_named_sexp("layer", &self.layer));
        match self.tstamp {
            None => (),
            Some(ref tstamp) => {
                v.push(Sexp::new_named("tstamp", tstamp));
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
        Sexp::new_list(v)
    }
}
