use std::error::Error as StdError;
use std::fmt;

/// Errors that can happen
#[derive(Debug)]
pub enum Error {
    /// Unable to create window
    WindowCreateFailed(String),
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::WindowCreateFailed(_) => "Failed to create window",
        }
    }

    fn cause(&self) -> Option<&StdError> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let extra = match *self {
            Error::WindowCreateFailed(ref e) => e,
        };
        write!(fmt, "{} {:?}", self.description(), extra)
    }
}
