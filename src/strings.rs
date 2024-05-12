use crate::{Token};

/// Represents strings in our L-system. Strings
/// are made up of a list of [`Token`] objects. 
#[derive(Debug, Clone)]
pub struct ProductionString {
    tokens: Vec<Token>
}

impl ProductionString {
    /// Create an empty string
    pub fn new() -> Self {
        ProductionString {
            tokens: Vec::new()
        }
    }

    /// Access the tokens of this production string.
    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }
}

impl Default for ProductionString {
    fn default() -> Self {
        ProductionString::new()
    }
}

impl From<Vec<Token>> for ProductionString {
    fn from(value: Vec<Token>) -> Self {
        ProductionString {
            tokens: value
        }
    }
}


impl From<Token> for ProductionString {
    fn from(value: Token) -> Self {
        ProductionString {
            tokens: vec![value]
        }
    }
}