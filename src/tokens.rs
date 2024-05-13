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