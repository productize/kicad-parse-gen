// (c) 2016-2017 Productize SPRL <joost@productize.be>

use std::io;
use symbolic_expressions;
use std::env;
use shellexpand;
use std::error;
use std::fmt;

/// errors that can happen in this library
#[derive(Debug)]
pub enum KicadError {
    /// parse error
    Parse(String),
    /// other error
    Other(String),
    /// IO Error
    Io(io::Error),
    /// env var error
    EnvVar(env::VarError),
    /// shell expand lookup error
    ShellExpand(shellexpand::LookupError<env::VarError>),
    /// symbolic expressions error
    SymbolicExpression(symbolic_expressions::SexpError),
}

impl From<symbolic_expressions::SexpError> for KicadError {
    fn from(e: symbolic_expressions::SexpError) -> KicadError {
        KicadError::SymbolicExpression(e)
    }
}

impl From<shellexpand::LookupError<env::VarError>> for KicadError {
    fn from(e: shellexpand::LookupError<env::VarError>) -> KicadError {
        KicadError::ShellExpand(e)
    }
}

impl From<io::Error> for KicadError {
    fn from(e: io::Error) -> KicadError {
        KicadError::Io(e)
    }
}

impl From<String> for KicadError {
    fn from(s: String) -> KicadError {
        KicadError::Other(s)
    }
}

impl error::Error for KicadError {
    fn description(&self) -> &str {
        "Kicad Error"
    }
}

impl fmt::Display for KicadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            KicadError::Parse(ref s) => write!(f, "Kicad Parse Error: {}", s),
            KicadError::Other(ref s) => write!(f, "Kicad Other Error: {}", s),
            KicadError::Io(ref io) => write!(f, "Kicad IO Error: {}", io),
            KicadError::EnvVar(ref s) => write!(f, "Kicad Env Var Error: {}", s),
            KicadError::ShellExpand(ref s) => write!(f, "Kicad Shell Expand Error: {}", s),
            KicadError::SymbolicExpression(ref s) => write!(f, "Kicad Symbolic Expression Error: {}", s),
        }
    }
}

/// create an other error
pub fn str_error<T>(s: String) -> Result<T, KicadError> {
    Err(KicadError::Other(s))
}

/// create a parse error
pub fn parse_error<T>(s: String) -> Result<T, KicadError> {
    Err(KicadError::Parse(s))
}
