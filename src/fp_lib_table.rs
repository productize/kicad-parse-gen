// (c) 2017 Productize SPRL <joost@productize.be>

// filename: fp-lib-table
// format: new-style

use Result;
use symbolic_expressions;
use symbolic_expressions::{IntoSexp, Sexp};
use formatter::KicadFormatter;

/// a fp-lib-table
#[derive(Debug, Clone)]
pub struct FpLibTable {
    libs: Vec<Lib>,
}

/// a library entry
#[derive(Debug, Clone)]
pub struct Lib {
    name:String,
    type_:String,
    uri:String,
    options:String,
    descr:String,
}

/// convert an `FpLibTable` to a formatted symbolic-expressions String
pub fn to_string(fp_lib_table: &FpLibTable, indent_level: i64) -> Result<String> {
    let formatter = KicadFormatter::new(indent_level);
    symbolic_expressions::ser::to_string_with_formatter(&fp_lib_table.into_sexp(), formatter)
        .map_err(From::from)
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
