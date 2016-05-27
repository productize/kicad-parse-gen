// (c) 2016 Productize SPRL <joost@productize.be>

// extension: .kicad_pcb
// format: new-style

use std::fmt;
use std::result;

// from parent
use Result;
use str_error as err;
use footprint;
use footprint::FromSexp;
use footprint::wrap;
use Sexp;
use symbolic_expressions;
use str_error;

pub use layout::data::Layout;

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        try!(writeln!(f, "(kicad_pcb (version {}) (host {} \"{}\")", self.version, self.host.tool, self.host.build));
        //let mut i = 0;
        try!(writeln!(f, "  {}", self.general));
        try!(writeln!(f, "  (page {})", self.page));
        try!(writeln!(f, "  {}", self.setup));
        try!(writeln!(f, "(layers "));
        for layer in &self.layers[..] {
            try!(writeln!(f, "  {}", layer));
        }
        try!(writeln!(f, ")"));
        for element in &self.elements[..] {
            try!(writeln!(f, "  {}", element));
            try!(writeln!(f, ""));
            // kludge to put setup at the right order in the file
            //if i == 3 {
            //    try!(writeln!(f, "  {}", self.setup));
            //}
            //i+=1;
        }
        writeln!(f, ")")
    }
}

impl fmt::Display for General {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        try!(writeln!(f, "(general"));
        try!(writeln!(f, "  (links {})", self.links));
        try!(writeln!(f, "  {}", self.area));
        try!(writeln!(f, "  (thickness {})", self.thickness));
        try!(writeln!(f, "  (drawings {})", self.drawings));
        try!(writeln!(f, "  (tracks {})", self.tracks));
        try!(writeln!(f, "  (zones {})", self.zones));
        try!(writeln!(f, "  (modules {})", self.modules));
        try!(writeln!(f, "  (nets {})", self.nets));
        writeln!(f, ")")
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        writeln!(f, "(area {} {} {} {})", self.x1, self.y1, self.x2, self.y2)
    }
}


impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match *self {
            Element::Other(ref s) => write!(f, "{}", s),
            Element::Module(ref s) => Ok(()),//write!(f, "{}", s), TODO
            Element::Net(ref s) => write!(f, "{}", s),
            Element::NetClass(ref s) => write!(f, "{}", s),
            Element::Graphics(ref s) => write!(f, "{}", s),
        }
    }
}

impl fmt::Display for Net {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        if self.name.contains('(') || self.name.contains(')') || self.name.is_empty() {
            write!(f, "(net {} \"{}\")", self.num, self.name)
        } else {
            write!(f, "(net {} {})", self.num, self.name)
        }
    }
}
impl fmt::Display for NetClass {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        try!(write!(f, "(net_class {} {} ", self.name, display_string(&self.desc)));
        try!(write!(f, "(clearance {}) ", self.clearance));
        try!(write!(f, "(trace_width {}) ", self.trace_width));
        try!(write!(f, "(via_dia {}) ", self.via_dia));
        try!(write!(f, "(via_drill {}) ", self.via_drill));
        try!(write!(f, "(uvia_dia {}) ", self.uvia_dia));
        try!(write!(f, "(uvia_drill {}) ", self.uvia_drill));
        for net in &self.nets {
            if net.contains('(') || net.contains(')') {
                try!(write!(f, "(add_net \"{}\")", net))
            } else {
                try!(write!(f, "(add_net {})", net))
            }
        }
        write!(f, ")")
        
    }
}

// (0 F.Cu signal)
impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        if !self.hide {
            write!(f, "({} {} {})", self.num, self.layer, self.layer_type)
        } else {
            write!(f, "({} {} {} hide)", self.num, self.layer, self.layer_type)
        }
            
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


impl fmt::Display for SetupElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match self.value2 {
            Some(ref x) => writeln!(f, "   ({} {} {})", self.name, self.value1, x),
            None => writeln!(f, "   ({} {})", self.name, self.value1),
        }
    }
}

impl fmt::Display for Setup {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        try!(writeln!(f, "(setup"));
        for ref k in &self.elements {
            try!(writeln!(f, "   {}", k));
        }
        try!(writeln!(f, " (pcbplotparams"));
        for ref k in &self.pcbplotparams {
            try!(writeln!(f, "     {}", k));
        }
        writeln!(f, "))")
    }
}

impl fmt::Display for Graphics {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match *self {
            Graphics::GrText(ref x) => write!(f, "{}", x),
            Graphics::GrLine(ref x) => write!(f, "{}", x),
            Graphics::GrArc(ref x) => write!(f, "{}", x),
            Graphics::Dimension(ref x) => write!(f, "{}", x),
            _ => write!(f, "(TODO)"),
        }
    }
}


impl fmt::Display for GrText {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        try!(writeln!(f,"(gr_text {} {} (layer {})", display_string(&self.value), self.at, self.layer));
        try!(writeln!(f,"    {}", self.effects));
        write!(f,")")
    }
}

impl fmt::Display for GrLine {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "(gr_line {} {} (angle {}) (layer {}) (width {}))", self.start, self.end, self.angle, self.layer, self.width)
    }
}

impl fmt::Display for GrArc {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "(gr_arc {} {} (angle {}) (layer {}) (width {}))", self.start, self.end, self.angle, self.layer, self.width)
    }
}

impl fmt::Display for Dimension {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        try!(writeln!(f, "(dimension {} (width {}) (layer {})", display_string(&self.name), self.width, self.layer));
        try!(writeln!(f, "{}", self.text));
        try!(writeln!(f, "(feature1 {})", self.feature1));
        try!(writeln!(f, "(feature2 {})", self.feature2));
        try!(writeln!(f, "(crossbar {})", self.crossbar));
        try!(writeln!(f, "(arrow1a {})", self.arrow1a));
        try!(writeln!(f, "(arrow1b {})", self.arrow1b));
        try!(writeln!(f, "(arrow2a {})", self.arrow2a));
        try!(writeln!(f, "(arrow2b {})", self.arrow2b));
        writeln!(f, ")")
    }
}

pub fn parse(s: &str) -> Result<Layout> {
    match symbolic_expressions::parser::parse_str(s) {
        Ok(s) => Result::from_sexp(&s),
        Err(x) => str_error(format!("ParseError: {:?}", x)),
    }
}

mod data;
mod de;
