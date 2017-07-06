// (c) 2017 Productize SPRL <joost@productize.be>

// filename: fp-lib-table
// format: new-style

use Result;
use symbolic_expressions;
use symbolic_expressions::{IntoSexp, Sexp};
use formatter::KicadFormatter;
use symbolic_expressions::iteratom::*;
use shellexpand;

/// a fp-lib-table
#[derive(Debug, Clone)]
pub struct FpLibTable {
    libs: Vec<Lib>,
}

/// a library entry
#[derive(Debug, Clone)]
pub struct Lib {
    name: String,
    type_: String,
    uri: String,
    options: String,
    descr: String,
}

impl Lib {
    /// return the URI with environment variables substituted
    pub fn get_expanded_uri(&self) -> Result<String> {
        let s = shellexpand::full(&self.uri)?;
        Ok(s.into())
    }
}

impl IntoSexp for FpLibTable {
    fn into_sexp(&self) -> Sexp {
        let mut v = Sexp::start("fp_lib_table");
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

impl FromSexp for FpLibTable {
    fn from_sexp(s: &Sexp) -> SResult<FpLibTable> {
        let mut i = IterAtom::new(s, "fp_lib_table")?;
        let libs = i.vec()?;
        Ok(FpLibTable { libs: libs })
    }
}

impl FromSexp for Lib {
    fn from_sexp(s: &Sexp) -> SResult<Lib> {
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

/// parse a &str to an `FpLibTable`
pub fn parse(s: &str) -> Result<FpLibTable> {
    let t = symbolic_expressions::parser::parse_str(s)?;
    let s = symbolic_expressions::from_sexp(&t)?;
    Ok(s)
}

/// convert an `FpLibTable` to a formatted symbolic-expressions String
pub fn to_string(fp_lib_table: &FpLibTable, indent_level: i64) -> Result<String> {
    let formatter = KicadFormatter::new(indent_level);
    symbolic_expressions::ser::to_string_with_formatter(&fp_lib_table.into_sexp(), formatter)
        .map_err(From::from)
}
