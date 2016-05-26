// (c) 2016 Productize SPRL <joost@productize.be>

use std::error;
use std::io;
use std::fmt;
use std::result;
use symbolic_expressions;

#[derive(Debug)]
pub enum Error {
    Parse(String),
    Other(String),
    Io(io::Error),
    Symbolic(symbolic_expressions::Error),
}


impl error::Error for Error {
    
    fn description(&self) -> &str {
        match *self {
            Error::Other(ref s) => {
                format!("Other Error: {}", s).as_str()
            },
            Error::Parse(ref s) => &format!("Parse Error: {}", s),
            Error::Io(ref error) => error::Error::description(error),
            Error::Symbolic(ref error) => error.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Io(ref error) => Some(error),
            Error::Symbolic(ref error) => Some(error),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        Error::Io(error)
    }
}

impl From<symbolic_expressions::Error> for Error {
    fn from(error: symbolic_expressions::Error) -> Error {
        Error::Symbolic(error)
    }
}

impl From<String> for Error {
    fn from(error: String) -> Error {
        Error::Other(error)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        match *self {
            Error::Other(ref s) => {
                write!(fmt, "Error:{}", s)
            }
            Error::Parse(ref s) => {
                write!(fmt, "Parse Error:{}", s)
            }
            Error::Io(ref error) => fmt::Display::fmt(error, fmt),
            Error::Symbolic(ref error) => fmt::Display::fmt(error, fmt),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;

pub fn str_error<T>(s:String) -> Result<T> {
    Err(Error::Other(s))
}

pub fn parse_error<T>(s:String) -> Result<T> {
    Err(Error::Parse(s))
}
