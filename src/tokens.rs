use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd)]
pub enum TokenKind {
    Terminal,
    Production
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Terminal => f.write_str("Termial"),
            TokenKind::Production => f.write_str("Production")
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd)]
pub struct Token {
    kind: TokenKind,
    /// A unique identifier for the token.
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
    
    pub fn code(&self) -> u32 {
        self.code
    }
    
    #[inline]
    pub fn is_terminal(&self) -> bool {
        matches!(self.kind, TokenKind::Terminal)
    }

    #[inline]
    pub fn is_production(&self) -> bool {
        matches!(self.kind, TokenKind::Production)
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}[{}]", self.kind, self.code)
    }
}