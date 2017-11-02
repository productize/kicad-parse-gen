// (c) 2016-2017 Productize SPRL <joost@productize.be>

use std::io;
use symbolic_expressions;
use std::env;
use shellexpand;

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

/// create an other error
pub fn str_error<T>(s: String) -> Result<T, KicadError> {
    Err(KicadError::Other(s))
}

/// create a parse error
pub fn parse_error<T>(s: String) -> Result<T, KicadError> {
    Err(KicadError::Parse(s))
}
