use std::error;
use std::fmt;
use std::num::ParseIntError;
use std::string::FromUtf8Error;

pub enum ParseError {
    Empty,
    ParseInt(ParseIntError),
    ParseStr(FromUtf8Error),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::Empty => f.write_str("Empty array"),
            ParseError::ParseInt(ref e) => e.fmt(f),
            ParseError::ParseStr(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for ParseError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            ParseError::Empty => None,
            ParseError::ParseInt(ref e) => Some(e),
            ParseError::ParseStr(ref e) => Some(e),
        }
    }
}

impl From<ParseIntError> for ParseError {
    fn from(err: ParseIntError) -> ParseError {
        ParseError::ParseInt(err)
    }
}

impl From<FromUtf8Error> for ParseError {
    fn from(err: FromUtf8Error) -> ParseError {
        ParseError::ParseStr(err)
    }
}

// Implement std::fmt::Debug for AppError
impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}
