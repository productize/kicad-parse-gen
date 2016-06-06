// (c) 2016 Productize SPRL <joost@productize.be>

use std::io;

use Sexp;
use symbolic_expressions::Result;
use symbolic_expressions::Formatter;

// custom symbolic_expressions formatter that aims to be
// kicad compatible

struct Indent2 {
    newline_before:i64,
    closing_on_newline:bool,
    newline_after:i64,
}

impl Indent2 {
    fn before() -> Indent2 {
        Indent2 {
            newline_before:1,
            closing_on_newline:false,
            newline_after:0,
        }
    }
    fn before_after() -> Indent2 {
        Indent2 {
            newline_before:1,
            closing_on_newline:true,
            newline_after:0,
        }
    }
    fn before_double() -> Indent2 {
        Indent2 {
            newline_before:2,
            closing_on_newline:false,
            newline_after:0,
        }
    }
    fn before_double_after() -> Indent2 {
        Indent2 {
            newline_before:2,
            closing_on_newline:true,
            newline_after:0,
        }
    }
    fn before_double_after_double() -> Indent2 {
        Indent2 {
            newline_before:2,
            closing_on_newline:true,
            newline_after:1,
        }
    }
}


pub struct KicadFormatter {
    indent:i64,
    stack:Vec<Option<(String,Option<Indent2>)>>,
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
        for x in &self.stack {
            if let Some((ref name,_)) = *x {
                if name == what {
                    return true
                }
            }
        }
        false
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

    fn want_indent_module(&self, ele:&str) -> Option<Indent2> {
        //if !self.is("module") {
        //    return None
        //}
        if self.parent_is("module") {
            match ele {
                "at" | "descr" | "fp_line" | "fp_poly" |
                "pad" => return Some(Indent2::before()),
                "model" | "fp_text" => return Some(Indent2::before_after()),
                _ => (),
            }
        } 
        if self.parent_is("fp_text") | self.parent_is("gr_text") {
            if let "effects" = ele {
                return Some(Indent2::before())
            }
        }
        if self.parent_is("pts") {
            if let "xy" = ele {
                if self.pts_xy_count > 0 && self.pts_xy_count % 4 == 0 {
                    return Some(Indent2::before())
                }
            }
        }
        if self.parent_is("model") {
            match ele {
                "at" | "scale" | "rotate" => {
                    return Some(Indent2::before())
                },
                _ => (),
            }
        }
        None
    }
    
    fn want_indent_layout(&self, ele:&str, e:&Sexp) -> Option<Indent2> {
        if !self.is("kicad_pcb") {
            return None
        }
        if self.parent_is("kicad_pcb") {
            match ele {
                "page" |
                "module" |
                "gr_arc"  | "gr_circle"
                    => return Some(Indent2::before_double()),
                "net" | "gr_line" | "segment" | "via"
                    => return Some(Indent2::before()),
                "layers" | "gr_text" | "dimension" | "zone"
                    => return Some(Indent2::before_after()),
                "setup"
                    => return Some(Indent2::before_double_after_double()),
                "general" | "net_class"
                    => return Some(Indent2::before_double_after()),
                _ => (),
            }
        }
        if self.parent_is("general") {
            return Some(Indent2::before())
        }
        if self.parent_is("layers") {
            return Some(Indent2::before())
        }
        if self.parent_is("setup") {
            return Some(Indent2::before())
        }
        if self.parent_is("pcbplotparams") {
            return Some(Indent2::before())
        }
        if self.parent_is("net_class") {
            return Some(Indent2::before())
        }
        if self.parent_is("dimension") {
            match ele {
                "gr_text" | "feature1" |
                "feature2" | "crossbar" |
                "arrow1a" | "arrow1b" |
                "arrow2a" | "arrow2b" => {
                    return Some(Indent2::before())
                },
                _ => (),
            }
        }
        if self.parent_is("zone") {
            match ele {
                "connect_pads" | "min_thickness" | "fill" |
                "polygon" | "filled_polygon"
                    => return Some(Indent2::before()),
                _ => (),
            }
        }
        if self.parent_is("polygon") | self.parent_is("filled_polygon") {
            return Some(Indent2::before())
        }
        None
    }
    
    fn want_indent(&self, value:&Sexp) -> Option<Indent2> {
        let first = match *value {
            Sexp::List(ref l) => {
                if l.is_empty() {
                    return None
                }
                (&l[0]).clone()
            },
            Sexp::Empty => return None,
            Sexp::String(ref l) => Sexp::String(l.clone()),
        };
        if let Sexp::String(ref ele) = first {
            let i = self.want_indent_module(ele);
            if i.is_some() {
                return i
            }
            let i = self.want_indent_layout(ele, value);
            if i.is_some() {
                return i
            }
        }
        None
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
        if let Some(ref want_indent) = want_indent {
            self.indent += 1;
            if want_indent.newline_before > 0 {
                try!(self.indent(writer, want_indent.newline_before));
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
        if self.want_indent(value).is_none() {
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
            if let Some(indent) = want_indent {
                self.indent -= 1;
                if indent.closing_on_newline {
                    try!(self.indent_plus(writer, 1));
                }
                // special handling of toplevel module...
                // which doesn't work, because it is not indented
                if &s == "module" && self.stack.is_empty() {
                    try!(writer.write_all(b"\n"));
                }
                try!(writer.write_all(b")"));
                for _ in 0..indent.newline_after {
                    try!(writer.write_all(b"\n"));
                }
                return Ok(())
            } else {
                if self.stack.is_empty() && &s == "module" {
                    try!(writer.write_all(b"\n"));
                }
            }
        }
        try!(writer.write_all(b")"));
        Ok(())
    }
}

