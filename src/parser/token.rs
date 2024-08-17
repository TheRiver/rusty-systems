use std::fmt::{Display, Formatter, Write};
use crate::error::Error;
use crate::prelude::Symbol;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TokenKind {
    Symbol,
    Arrow,
    ContextLeft,
    ContextRight,
    Terminator
}

impl From<&str> for TokenKind {
    fn from(value: &str) -> Self {
        match value {
            ";" | ""    => TokenKind::Terminator,
            "->"        => TokenKind::Arrow,
            ">"         => TokenKind::ContextRight,
            "<"         => TokenKind::ContextLeft,
            _           => TokenKind::Symbol
        }
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Symbol => f.write_str("Symbol"),
            TokenKind::Arrow => f.write_str("->"),
            TokenKind::ContextLeft => f.write_str("<"),
            TokenKind::ContextRight => f.write_str(">"),
            TokenKind::Terminator => f.write_char(';'),
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

    #[inline]
    pub fn is_terminal(&self) -> bool {
        matches!(self.kind, TokenKind::Terminator)
    }
}

impl Display for Token<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text = if self.text.is_empty() { "EOS" } else { self.text };
        f.write_str(text)
    }
}

impl<'a> AsRef<str> for Token<'a> {
    fn as_ref(&self) -> &str {
        self.text
    }
}

impl<'a> TryFrom<Token<'a>> for Symbol {
    type Error = Error;

    fn try_from(token: Token) -> Result<Self, Self::Error> {
        if token.kind != TokenKind::Symbol {
            return Err(Error::parse_error("token is not a Symbol"));
        }

        token.text.try_into()
    }
}