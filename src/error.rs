use std::error;
use std::fmt;
use std::io;
use std::num::ParseFloatError;

#[derive(Debug, Clone)]
pub enum ProcessError {
    IOError(String),
    ParseError(String),
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let error_kind = match self {
            ProcessError::IOError(msg) => {
                format!("io error: {}", msg)
            },
            ProcessError::ParseError(msg) => {
                format!("parse topography value error {}", msg)
            }
        };

    write!(f, "Error while processing topography data: {}", error_kind)
    }
}

impl error::Error for ProcessError {
    fn description(&self) -> &str {
        "Error while processing topography data"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

impl From<io::Error> for ProcessError {
    fn from(err: io::Error) -> ProcessError {
        ProcessError::IOError(err.to_string())
    }
}

impl From<ParseFloatError> for ProcessError {
    fn from(err: ParseFloatError) -> ProcessError {
        ProcessError::ParseError(err.to_string())
    }
}
