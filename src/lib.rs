use productions::{ Production, ProductionBuilder};

pub mod error;
pub mod productions;

pub mod prelude {
    pub use super::System;
    pub use super::error::Error;
}


/// A result type for functions that can return errors.
pub type Result<T> = std::result::Result<T, error::Error>;


#[derive(Debug, Clone)]
pub enum Token {
    Terminal(String),
    Production(String)
}

#[derive(Debug, Clone)]
pub struct System {
    terminals: Vec<Token>,
    productions: Vec<Production>
}

#[derive(Debug, Clone)]
pub struct Builder {
    terminals: Vec<String>
}


impl Builder {
    pub fn terminal<T: ToString>(mut self, name: T) -> Self {
        self.terminals.push(name.to_string());
        self
    }
    
    pub fn build(self) -> System {
        System::from(self)
    }
}



impl System {
    pub fn define() -> Builder {
        Builder { terminals: Vec::new() }
    }

    pub fn add_terminal<T: ToTerminal>(&mut self, terminal: T) -> &Self {
        self.terminals.push(terminal.to_terminal());
        self
    }

    /// Start defining a new production to add to the system.
    ///
    /// See [`Production`]
    pub fn production(&mut self) -> ProductionBuilder {
        ProductionBuilder::new(self)
    }
}


impl From<Builder> for System {
    fn from(value: Builder) -> Self {
        System {
            terminals: value.terminals.into_iter().map(Token::Terminal).collect(),
            productions: Vec::new()
        }
    }
}

impl From<String> for Token {
    fn from(value: String) -> Self {
        Token::Terminal(value)
    }
}

impl From<&str> for Token {
    fn from(value: &str) -> Self {
        Token::Terminal(value.to_string())
    }
}

pub trait ToTerminal {
    fn to_terminal(self) -> Token;
}

impl ToTerminal for String {
    fn to_terminal(self) -> Token {
        Token::from(self)
    }
}

impl ToTerminal for &str {
    fn to_terminal(self) -> Token {
        Token::from(self.to_string())
    }
}

impl ToTerminal for Token {
    fn to_terminal(self) -> Token {
        self
    }
}



