use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub enum Token {
    Terminal(String),
    Production(String)
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Terminal(name) => write!(f, "Terminal[{name}]"),
            Token::Production(name) => write!(f, "Production[{name}]")
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
