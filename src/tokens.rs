//! Tools for handling the tokens that make up strings in the L-system ([`crate::strings::ProductionString`]).
//!
//! Tokens can be of various kinds:
//!
//! * [`TokenKind::Terminal`] are the strict endpoints of the L-System. No production rule can target them.
//! * [`TokenKind::Production`] are those that can be handled by a production rule.
//!
//! Production rules ([`crate::productions::Production`] will enforce that the target of a
//! production is a token of kind [`crate::TokenKind::Production`].

use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd)]
pub enum TokenKind {
    Terminal,
    Production
}

impl TokenKind {
    pub fn is_terminal(&self) -> bool {
        matches!(self, TokenKind::Terminal)
    }
    pub fn is_production(&self) -> bool {
        matches!(self, TokenKind::Production)
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Terminal => f.write_str("Terminal"),
            TokenKind::Production => f.write_str("Production")
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd)]
pub struct Token {
    kind: TokenKind,
    code: u32
}

impl Token {

    pub fn new(kind: TokenKind, code: u32) -> Self {
        Token {
            kind, code
        }
    }

    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    /// A unique identifier for the token.
    ///
    /// This identifier is set when the token is created (see [`Token::new`]).
    /// The value may not be the same when generated by different instances of [`crate::prelude::System`],
    /// and the value here should not be relied on.
    pub fn code(&self) -> u32 {
        self.code
    }

    #[inline]
    pub fn is_terminal(&self) -> bool {
        self.kind.is_terminal()
    }

    #[inline]
    pub fn is_production(&self) -> bool {
        self.kind.is_production()
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]", self.kind, self.code)
    }
}


/// For the default string parser, this determines the kind
/// of [`Token`] it should be parsed as.
///
/// Please note that the rules this function uses for
/// differentiating between terminals and productions
///
pub fn determine_kind(string: &str) -> Option<TokenKind> {
    let string = string.trim();
    if string.is_empty() { return None }

    let first = string.chars().next()?;
    if first.is_ascii_uppercase() {
        return Some(TokenKind::Production)
    }

    Some(TokenKind::Terminal)
}