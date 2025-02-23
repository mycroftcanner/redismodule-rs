use crate::RedisError;
use std::error;
use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Generic(GenericError),
    FromUtf8(std::string::FromUtf8Error),
    ParseInt(std::num::ParseIntError),
}

impl Error {
    #[must_use]
    pub fn generic(message: &str) -> Self {
        Self::Generic(GenericError::new(message))
    }
}

impl From<RedisError> for Error {
    fn from(err: RedisError) -> Self {
        Self::generic(err.to_string().as_str())
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(err: std::string::FromUtf8Error) -> Self {
        Self::FromUtf8(err)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            // Both underlying errors already impl `Display`, so we defer to
            // their implementations.
            Error::Generic(ref err) => write!(f, "{}", err),
            Error::FromUtf8(ref err) => write!(f, "{}", err),
            Error::ParseInt(ref err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            // N.B. Both of these implicitly cast `err` from their concrete
            // types (either `&io::Error` or `&num::ParseIntError`)
            // to a trait object `&Error`. This works because both error types
            // implement `Error`.
            Error::Generic(ref err) => Some(err),
            Error::FromUtf8(ref err) => Some(err),
            Error::ParseInt(ref err) => Some(err),
        }
    }
}

#[derive(Debug)]
pub struct GenericError {
    message: String,
}

impl GenericError {
    #[must_use]
    pub fn new(message: &str) -> Self {
        Self {
            message: String::from(message),
        }
    }
}

impl Display for GenericError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Store error: {}", self.message)
    }
}

impl error::Error for GenericError {
    fn description(&self) -> &str {
        self.message.as_str()
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}
