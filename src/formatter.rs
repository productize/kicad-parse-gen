// (c) 2016 Productize SPRL <joost@productize.be>

use std::io;

use Sexp;
use symbolic_expressions::Result;
use symbolic_expressions::Formatter;

// custom symbolic_expressions formatter that aims to be
// kicad compatible

#[derive(PartialEq)]
enum Indent {
    Not,
    Before,
    BeforeAfter,
    BeforeDouble,
    BeforeDoubleAfter,
    BeforeDoubleAfterDouble,
}

pub struct KicadFormatter {
    indent:i64,
    stack:Vec<Option<(String,Indent)>>,
    ind:Vec<u8>,
    pts_xy_count:i64,
}

impl KicadFormatter {
    
    pub fn new(initial_indent_level:i64) -> KicadFormatter {
        KicadFormatter {
            indent:initial_indent_level,
            stack:vec![],
            ind:vec![b' ',b' '], // two spaces
            pts_xy_count:0,
        }
    }

    fn is(&self, what:&'static str) -> bool {
        self.stack.iter().any(
            |x:&Option<(String,Indent)>| {
                if let Some((ref y,_)) = *x {
                    y == what
                } else {
                    false
                }
            })
    }
    
    fn parent_is(&self, what:&'static str) -> bool {
        if let Some(s) = self.stack.last() {
            if let Some((ref t,_)) = *s {
                return t == what
            }
        } 
        false
    }
    
    fn indent<W:io::Write>(&self, writer:&mut W, nls:i64) -> Result<()> {
        for _ in 0..nls {
            try!(writer.write_all(b"\n"));
        }
        for _ in 0..self.indent {
            try!(writer.write_all(&self.ind));
        }
        Ok(())
    }

    fn indent_plus<W:io::Write>(&mut self, writer:&mut W, nls:i64) -> Result<()> {
        self.indent+=1;
        let res = self.indent(writer, nls);
        self.indent-=1;
        res
    }

    fn want_indent_module(&self, ele:&str) -> Indent {
        if !self.is("module") {
            return Indent::Not
        }
        if self.parent_is("module") {
            match ele {
                "at" | "descr" | "fp_line" | "fp_poly" |
                "pad" => return Indent::Before,
                "model" | "fp_text" => return Indent::BeforeAfter,
                _ => (),
            }
        } 
        if self.parent_is("fp_text") | self.parent_is("gr_text") {
            if let "effects" = ele {
                return Indent::Before
            }
        }
        if self.parent_is("pts") {
            if let "xy" = ele {
                if self.pts_xy_count > 0 && self.pts_xy_count % 4 == 0 {
                    return Indent::Before
                }
            }
        }
        if self.parent_is("model") {
            match ele {
                "at" | "scale" | "rotate" => {
                    return Indent::Before
                },
                _ => (),
            }
        }
        Indent::Not
    }
    
    fn want_indent_layout(&self, ele:&str, e:&Sexp) -> Indent {
        if !self.is("kicad_pcb") {
            return Indent::Not
        }
        if self.parent_is("kicad_pcb") {
            match ele {
                "page" |
                "module" |
                "gr_arc"  | "gr_circle"
                    => return Indent::BeforeDouble,
                "net" | "gr_line" | "segment" | "via"
                    => return Indent::Before,
                "layers" | "gr_text" | "dimension" | "zone"
                    => return Indent::BeforeAfter,
                "setup"
                    => return Indent::BeforeDoubleAfterDouble,
                "general" | "net_class"
                    => return Indent::BeforeDoubleAfter,
                _ => (),
            }
        }
        if self.parent_is("general") {
            return Indent::Before
        }
        if self.parent_is("layers") {
            return Indent::Before
        }
        if self.parent_is("setup") {
            return Indent::Before
        }
        if self.parent_is("pcbplotparams") {
            return Indent::Before
        }
        if self.parent_is("net_class") {
            return Indent::Before
        }
        if self.parent_is("dimension") {
            match ele {
                "gr_text" | "feature1" |
                "feature2" | "crossbar" |
                "arrow1a" | "arrow1b" |
                "arrow2a" | "arrow2b" => {
                    return Indent::Before
                },
                _ => (),
            }
        }
        if self.parent_is("zone") {
            match ele {
                "connect_pads" | "min_thickness" | "fill" |
                "polygon" | "filled_polygon"
                    => return Indent::Before,
                _ => (),
            }
        }
        if self.parent_is("polygon") | self.parent_is("filled_polygon") {
            return Indent::Before
        }
        Indent::Not
    }
    
    fn want_indent(&self, value:&Sexp) -> Indent {
        let first = match *value {
            Sexp::List(ref l) => {
                if l.is_empty() {
                    return Indent::Not
                }
                (&l[0]).clone()
            },
            Sexp::Empty => return Indent::Not,
            Sexp::String(ref l) => Sexp::String(l.clone()),
        };
        if let Sexp::String(ref ele) = first {
            let i = self.want_indent_module(ele);
            if i != Indent::Not {
                return i
            }
            let i = self.want_indent_layout(ele, value);
            if i != Indent::Not {
                return i
            }
        }
        Indent::Not
    }
}

impl Formatter for KicadFormatter {
    
    fn open<W>(&mut self, writer: &mut W, value:Option<&Sexp>) -> Result<()>
        where W: io::Write
    {
        let mut ele = String::new();
        // if first element is string
        if let Some(ref sexp) = value {
            if let Sexp::String(ref s) = **sexp {
                ele.push_str(s);
            }
        }
        let exp = Sexp::String(ele.clone());
        let want_indent = self.want_indent(&exp);
        if want_indent != Indent::Not {
            self.indent += 1;
            if want_indent == Indent::BeforeDouble
            || want_indent == Indent::BeforeDoubleAfterDouble
            || want_indent == Indent::BeforeDoubleAfter {
                try!(self.indent(writer, 2));
            } else {
                try!(self.indent(writer, 1));
            }
        }
        
        // special handling for breaking after 4 elements of xy
        if let "pts" = &ele[..] {
            self.pts_xy_count = 0;
        }
        if self.parent_is("pts") {
            if let "xy" = &ele[..] {
                self.pts_xy_count += 1;
                if self.pts_xy_count == 5 {
                    self.pts_xy_count = 1;
                }
            }
        }
        
        if !ele.is_empty() {
            self.stack.push(Some((ele, want_indent)))
        } else {
            self.stack.push(None)
        }
        writer.write_all(b"(").map_err(From::from)
    }
    
    fn element<W>(&mut self, writer: &mut W, value:&Sexp) -> Result<()>
        where W: io::Write
    {
        // get rid of the space if we will be putting a newline next
        if self.want_indent(value) == Indent::Not {
            try!(writer.write_all(b" "));
        } else if let Sexp::String(_) = *value {
            try!(writer.write_all(b" "));
        }
        Ok(())
        
    }
    
    fn close<W>(&mut self, writer: &mut W) -> Result<()>
        where W: io::Write
    {
        if let Some(Some((s, want_indent))) = self.stack.pop() {
            if want_indent != Indent::Not {
                self.indent -= 1
            }
            match want_indent {
                Indent::Not |
                Indent::Before |
                Indent::BeforeDouble => (),
                Indent::BeforeDoubleAfter |
                Indent::BeforeAfter |
                Indent::BeforeDoubleAfterDouble
                    => {
                    try!(self.indent_plus(writer, 1));
                }
            }
            // TODO: remove
            if let "module" = &s[..] {
                try!(self.indent(writer, 1));
            }
            try!(writer.write_all(b")"));
            if want_indent == Indent::BeforeDoubleAfterDouble {
                try!(writer.write_all(b"\n"));
            }
        } else {
            try!(writer.write_all(b")"));
        }
        Ok(())
    }
}

