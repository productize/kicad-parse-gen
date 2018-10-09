// (c) 2017 Productize SPRL <joost@productize.be>

// filename: sym-lib-table
// format: new-style

use symbolic_expressions;
use symbolic_expressions::{IntoSexp, Sexp, SexpError};
use formatter::KicadFormatter;
use symbolic_expressions::iteratom::*;
use shellexpand;
use error::KicadError;

/// a sym-lib-table
#[derive(Debug, Clone)]
pub struct SymLibTable {
    /// the library references contained in the sym-lib-table
    pub libs: Vec<Lib>,
}

/// a library entry
#[derive(Debug, Clone)]
pub struct Lib {
    /// name of the library
    pub name: String,
    /// type of the library (typically Kicad)
    pub type_: String,
    /// the URI of the library (can contain shell variables)
    pub uri: String,
    /// options
    pub options: String,
    /// description
    pub descr: String,
}

impl Lib {
    /// return the URI with environment variables substituted
    pub fn get_expanded_uri(&self) -> Result<String, KicadError> {
        let s = shellexpand::full(&self.uri)?;
        Ok(s.into())
    }
}

impl IntoSexp for SymLibTable {
    fn into_sexp(&self) -> Sexp {
        let mut v = Sexp::start("sym_lib_table");
        for e in &self.libs {
            v.push(e.into_sexp())
        }
        v
    }
}

impl IntoSexp for Lib {
    fn into_sexp(&self) -> Sexp {
        let mut v = Sexp::start("lib");
        v.push(("name", &self.name));
        v.push(("type", &self.type_));
        v.push(("uri", &self.uri));
        v.push(("options", &self.options));
        v.push(("descr", &self.descr));
        v
    }
}

impl FromSexp for SymLibTable {
    fn from_sexp(s: &Sexp) -> Result<SymLibTable, SexpError> {
        let mut i = IterAtom::new(s, "sym_lib_table")?;
        let libs = i.vec()?;
        Ok(SymLibTable { libs: libs })
    }
}

impl FromSexp for Lib {
    fn from_sexp(s: &Sexp) -> Result<Lib, SexpError> {
        let mut i = IterAtom::new(s, "lib")?;
        let name = i.s_in_list("name")?;
        let type_ = i.s_in_list("type")?;
        let uri = i.s_in_list("uri")?;
        let options = i.s_in_list("options")?;
        let descr = i.s_in_list("descr")?;
        Ok(Lib {
            name: name,
            type_: type_,
            uri: uri,
            options: options,
            descr: descr,
        })
    }
}

impl PartialEq for Lib {
    fn eq(&self, other: &Lib) -> bool {
        if self.name != other.name {
            return false;
        }
        if self.type_ != other.type_ {
            return false;
        }
        if self.uri != other.uri {
            return false;
        }
        if self.options != other.options {
            return false;
        }
        if self.descr != other.descr {
            return false;
        }
        true
    }
}

/// parse a &str to an `SymLibTable`
pub fn parse(s: &str) -> Result<SymLibTable, SexpError> {
    let t = symbolic_expressions::parser::parse_str(s)?;
    let s = symbolic_expressions::from_sexp(&t)?;
    Ok(s)
}

/// convert an `SymLibTable` to a formatted symbolic-expressions String
pub fn to_string(sym_lib_table: &SymLibTable, indent_level: i64) -> Result<String, KicadError> {
    let formatter = KicadFormatter::new(indent_level);
    symbolic_expressions::ser::to_string_with_formatter(&sym_lib_table.into_sexp(), formatter)
        .map_err(From::from)
}
