use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Symbol,
    Arrow,
    ContextLeft,
    ContextRight
}

impl From<&str> for TokenKind {
    fn from(value: &str) -> Self {
        match value {
            "->" => TokenKind::Arrow,
            ">" => TokenKind::ContextRight,
            "<" => TokenKind::ContextLeft,
            _ => TokenKind::Symbol
        }
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Symbol => f.write_str("Symbol"),
            TokenKind::Arrow => f.write_str("->"),
            TokenKind::ContextLeft => f.write_str("<"),
            TokenKind::ContextRight => f.write_str(">")
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Token<'a> {
    pub text: &'a str,
    pub start: usize,
    pub end: usize,
    pub kind: TokenKind
}

impl<'a> Token<'a> {
    pub fn new(text: &'a str, start: usize, end: usize) -> Self {
        Token {
            text, start, end,
            kind: TokenKind::from(text)
        }
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.text)
    }
}

impl<'a> AsRef<str> for Token<'a> {
    fn as_ref(&self) -> &str {
        self.text
    }
}