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
    BeforeOnly,
    BeforeAfter,
    BeforeAfterDouble,
}

pub struct KicadFormatter {
    indent:i64,
    stack:Vec<Option<(String,Indent)>>,
    ind:Vec<u8>,
    poly_xy_count:i64,
}

impl KicadFormatter {
    
    pub fn new(initial_indent_level:i64) -> KicadFormatter {
        KicadFormatter {
            indent:initial_indent_level,
            stack:vec![],
            ind:vec![b' ',b' '], // two spaces
            poly_xy_count:0,
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
        if self.parent_is("module") {
            match ele {
                "at" | "descr" | "fp_line" | "fp_poly" |
                "pad" => return Indent::BeforeOnly,
                "model" | "fp_text" => return Indent::BeforeAfter,
                _ => (),
            }
        } 
        if self.parent_is("fp_text") {
            if let "effects" = ele {
                return Indent::BeforeOnly
            }
        }
        if self.parent_is("pts") {
            if let "xy" = ele {
                if self.poly_xy_count == 4 {
                    return Indent::BeforeOnly
                }
            }
        }
        if self.parent_is("model") {
            match ele {
                "at" | "scale" | "rotate" => {
                    return Indent::BeforeOnly
                },
                _ => (),
            }
        }
        Indent::Not
    }
    
    fn want_indent_layout(&self, ele:&str) -> Indent {
        if self.parent_is("kicad_pcb") {
            match ele {
                "general" | "page" |
                "setup" | "net" | "net_class" |
                "module" | "gr_text" | "gr_line" |
                "gr_arc"  | "gr_circle" |
                "dimension" | "segment" | "via" |
                "zone"
                    => return Indent::BeforeAfterDouble,
                "layers"
                    => return Indent::BeforeAfter,
                _ => (),
            }
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
            let i = self.want_indent_layout(ele);
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
            try!(self.indent(writer, 1));
        }
        
        // special handling for breaking after 4 elements of xy
        // within fp_poly
        if self.parent_is("module") {
            if let "fp_poly" = &ele[..] {
                self.poly_xy_count = 0;
            }
        }
        if self.is("fp_poly") {
            if let "xy" = &ele[..] {
                self.poly_xy_count += 1;
                if self.poly_xy_count == 5 {
                    self.poly_xy_count = 1;
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
            writer.write_all(b" ").map_err(From::from)
        } else {
            Ok(())
        }
        
    }
    
    fn close<W>(&mut self, writer: &mut W) -> Result<()>
        where W: io::Write
    {
        if let Some(Some((s, want_indent))) = self.stack.pop() {
            if want_indent != Indent::Not {
                self.indent -= 1
            }
            match want_indent {
                Indent::Not | Indent::BeforeOnly => (),
                Indent::BeforeAfter => {
                    try!(self.indent_plus(writer, 1));
                }
                Indent::BeforeAfterDouble => {
                    try!(self.indent_plus(writer, 2));
                }
                // special handling to add another newline before ')'
            }
            /*
            if self.is("module") {
                match &s[..] {
                    "fp_text" | "model" => {
                        try!(self.indent_plus(writer, want_indent));
                    },
                    _ => (),
                }
            }
            if let "module" = &s[..] {
                try!(self.indent(writer, 1));
            }*/
        }
        writer.write_all(b")").map_err(From::from)
    }
}

