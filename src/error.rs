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
    Locking
}


/// Errors that might be thrown when using this library. They will be
/// of kind [`ErrorKind`].
#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    message: String
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        if let ErrorKind::Definitions = self.kind {
            f.write_str("Definition error: ")?;
        }

        f.write_str(&self.message)
    }
}

impl std::error::Error for Error { }


impl Error {
    pub fn new<S : ToString>(kind: ErrorKind, message: S) -> Self {
        Error {
            kind,
            message: message.to_string()
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