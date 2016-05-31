// (c) 2016 Productize SPRL <joost@productize.be>

use std::io;

use Sexp;
use symbolic_expressions::Result;
use symbolic_expressions::Formatter;

// custom symbolic_expressions formatter that aims to be
// kicad compatible

pub struct KicadFormatter {
    initial_indent_level:i64,
    indent:i64,
    stack:Vec<Option<String>>,
    ind:Vec<u8>,
    poly_xy_count:i64,
}

impl KicadFormatter {
    
    pub fn new(initial_indent_level:i64) -> KicadFormatter {
        KicadFormatter {
            initial_indent_level:initial_indent_level,
            indent:initial_indent_level,
            stack:vec![],
            ind:vec![b' ',b' '], // two spaces
            poly_xy_count:0,
        }
    }

    fn is(&self, what:&'static str) -> bool {
        self.stack.iter().find(
            |x:&&Option<String>| {
                if let Some(ref y) = **x {
                    y == what
                } else {
                    false
                }
            }).is_some()
    }
    
    fn parent_is(&self, what:&'static str) -> bool {
        if let Some(s) = self.stack.last() {
            if let Some(ref t) = *s {
                return t == what
            }
        } 
        false
    }
    
    fn indent<W:io::Write>(&self, writer:&mut W) -> Result<()> {
        try!(writer.write_all(b"\n"));
        for _ in 0..self.indent {
            try!(writer.write_all(&self.ind));
        }
        Ok(())
    }

    fn indent_plus<W:io::Write>(&mut self, writer:&mut W) -> Result<()> {
        self.indent+=1;
        let res = self.indent(writer);
        self.indent-=1;
        res
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
        if self.parent_is("module") {
            match &ele[..] {
                "at" | "descr" | "fp_text" |
                "fp_line" | "fp_poly" | "pad" |
                "model"
                    => {
                        try!(self.indent_plus(writer));
                    },
                _ => ()
            }
            match &ele[..] {
                "fp_text" | "fp_poly" | "model" => {
                    self.indent += 1;
                },
                _ => ()
            }
            if let "fp_poly" = &ele[..] {
                self.poly_xy_count = 0;
            }
        }
        if self.is("fp_poly") {
            if let "xy" = &ele[..] {
                self.poly_xy_count += 1;
                if self.poly_xy_count == 5 {
                    try!(self.indent_plus(writer));
                    self.poly_xy_count = 1;
                }
            }
        }
        if self.is("fp_text") {
            if let "effects" = &ele[..] {
                try!(self.indent_plus(writer));
            }
        }
        if !ele.is_empty() {
            self.stack.push(Some(ele))
        } else {
            self.stack.push(None)
        }
        writer.write_all(b"(").map_err(From::from)
    }
    fn element<W>(&mut self, writer: &mut W, _value:&Sexp) -> Result<()>
        where W: io::Write
    {
        writer.write_all(b" ").map_err(From::from)
    }
    
    fn close<W>(&mut self, writer: &mut W) -> Result<()>
        where W: io::Write
    {
        if let Some(Some(s)) = self.stack.pop() {
            if self.is("module") {
                match &s[..] {
                    "fp_text" | "fp_poly" | "model" => {
                        self.indent -= 1
                    },
                    _ => ()
                }
                match &s[..] {
                    "fp_text" | "model" => {
                        try!(self.indent_plus(writer));
                    },
                    _ => (),
                }
            }
            if let "module" = &s[..] {
                try!(self.indent(writer));
            }
        }
        writer.write_all(b")").map_err(From::from)
    }
}

