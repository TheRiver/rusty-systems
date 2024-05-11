use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Error {
    General(String),
    /// Errors related to defining systems, production rules,
    /// and so on. 
    Definitions(String)
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self { 
            Error::General(message) => f.write_str(message),
            Error::Definitions(message) => {
                write!(f, "Definition problem: {message}")
            }
        }
    }
}

impl std::error::Error for Error { }


impl Error {
    pub fn general<T : ToString>(message: T) -> Self {
        Error::General(message.to_string())
    }
    
    pub fn definition<T : ToString>(message: T) -> Self {
        Error::Definitions(message.to_string())
    }
}