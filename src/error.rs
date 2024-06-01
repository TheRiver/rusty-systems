//! General error handling tools and utilities

use std::fmt::{Display, Formatter};
use std::sync::PoisonError;

#[derive(Debug, Clone)]
pub enum ErrorKind {
    General,
    Parse,
    /// Errors related to defining systems, production rules,
    /// and so on.
    Definitions,
    /// Returned when attempting to create duplicate items
    Duplicate,
    /// Errors related to running a system. See [`crate::system::System`].
    Execution,
    /// Indicates an error with locks, such as a [`PoisonError`].
    Locking,
    /// An IO Error. 
    Io
}


/// Errors that might be thrown when using this library. They will be
/// of kind [`ErrorKind`].
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String,
    source: Option<Box<dyn std::error::Error>>
}

impl Default for Error {
    fn default() -> Self {
        Self {
            kind: ErrorKind::General,
            message: String::from("An unspecified error occurred"),
            source: None
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        if let ErrorKind::Definitions = self.kind {
            f.write_str("Definition error: ")?;
        }

        f.write_str(&self.message)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| &**e)
    }
}


impl Error {
    pub fn new<S : ToString>(kind: ErrorKind, message: S) -> Self {
        Error {
            kind,
            message: message.to_string(),
            ..Default::default()
        }
    }

    pub fn general<T : ToString>(message: T) -> Self {
        Self::new(ErrorKind::General, message)
    }

    pub fn definition<T : ToString>(message: T) -> Self {
        Self::new(ErrorKind::Definitions, message)
    }

    pub fn execution<T : ToString>(message: T) -> Self {
        Self::new(ErrorKind::Execution, message)
    }
}

impl<T> From<PoisonError<T>> for Error {
    fn from(value: PoisonError<T>) -> Self {
        Error::new(ErrorKind::Locking, value.to_string())
    }
}

impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error {
            kind: ErrorKind::Io,
            message: error.to_string(),
            source: Some(Box::new(error)),
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(error: std::num::ParseIntError) -> Self {
        Error {
            kind: ErrorKind::Parse,
            message: error.to_string(),
            source: Some(Box::new(error)),
        }
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(error: std::num::ParseFloatError) -> Self {
        Error {
            kind: ErrorKind::Parse,
            message: error.to_string(),
            source: Some(Box::new(error)),
        }
    }
}