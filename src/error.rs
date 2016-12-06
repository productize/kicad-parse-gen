// (c) 2016 Productize SPRL <joost@productize.be>

use std::error;
use std::io;
use std::fmt;
use std::result;
use symbolic_expressions;
use serde::{de, ser};

/// encapsulating Error type
#[derive(Debug)]
pub enum Error {
    /// parsing error
    Parse(String),
    /// other error
    Other(String),
    /// IO error
    Io(io::Error),
    /// symbolic-expressions library error
    Symbolic(symbolic_expressions::Error),
    /// decoder error
    Decoder(String),
    /// encoder error
    Encoder(String),
}

impl de::Error for Error {
    fn custom<T: Into<String>>(msg: T) -> Self {
        Error::Decoder(msg.into())
    }

    fn end_of_stream() -> Self {
        Error::Decoder("end_of_stream".into())
    }
}

impl ser::Error for Error {
    fn custom<T: Into<String>>(msg: T) -> Self {
        Error::Encoder(msg.into())
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Decoder(ref s) |
            Error::Encoder(ref s) |
            Error::Other(ref s) |
            Error::Parse(ref s) => s,
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
            Error::Other(ref s) => write!(fmt, "Error:{}", s),
            Error::Decoder(ref s) => write!(fmt, "Decoder Error:{}", s),
            Error::Encoder(ref s) => write!(fmt, "Encoder Error:{}", s),
            Error::Parse(ref s) => write!(fmt, "Parse Error:{}", s),
            Error::Io(ref error) => fmt::Display::fmt(error, fmt),
            Error::Symbolic(ref error) => fmt::Display::fmt(error, fmt),
        }
    }
}

/// result type type alias
pub type Result<T> = result::Result<T, Error>;

/// create an other error
pub fn str_error<T>(s: String) -> Result<T> {
    Err(Error::Other(s))
}

/// create a parse error
pub fn parse_error<T>(s: String) -> Result<T> {
    Err(Error::Parse(s))
}
