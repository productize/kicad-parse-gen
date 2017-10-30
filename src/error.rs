// (c) 2016-2017 Productize SPRL <joost@productize.be>

use std::io;
use symbolic_expressions;
use std::num;
use std::env;
use shellexpand;

error_chain! {

    errors {
/// kicad parse error
        Parse(s: String) {
            description("parse error")
            display("parse error: '{}'", s)
        }
    }

    foreign_links {
        Io(io::Error) #[doc = "IO error"];
        Float(num::ParseFloatError) #[doc = "Float error"];
        Int(num::ParseIntError) #[doc = "Int error"];
        EnvVar(env::VarError) #[doc = "Env Var error"];
        Shellexpand(shellexpand::LookupError<env::VarError>) #[doc = "Shell expand lookup error"];
        SymbolicExpression(symbolic_expressions::Error) #[doc = "Symbolic Expressions error"];
    }
}

/// create an other error
pub fn str_error<T>(s: String) -> Result<T> {
    Err(s.into())
}

/// create a parse error
pub fn parse_error<T>(s: String) -> Result<T> {
    Err(ErrorKind::Parse(s).into())
}
