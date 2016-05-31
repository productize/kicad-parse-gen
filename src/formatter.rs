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
}

impl KicadFormatter {
    pub fn new(initial_indent_level:i64) -> KicadFormatter {
        KicadFormatter {
            initial_indent_level:initial_indent_level,
            indent:initial_indent_level,
        }
    }
}

impl Formatter for KicadFormatter {
    fn open<W>(&mut self, writer: &mut W, value:Option<&Sexp>) -> Result<()>
        where W: io::Write
    {
        // if first element is string
        if let Some(ref sexp) = value {
            if let Sexp::String(ref s) = **sexp {
                let s:&str = s;
                match s {
                    _ => (),
                }
            }
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
        writer.write_all(b")").map_err(From::from)
    }
}

