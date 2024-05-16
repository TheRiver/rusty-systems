use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum ErrorKind {
    General,
    Parse,
    /// Errors related to defining systems, production rules,
    /// and so on.
    Definitions,
    /// Errors related to running a system. See [`crate::system::System`].
    Execution
}


/// Errors that might be thrown when using this library.
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