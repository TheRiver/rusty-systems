use crate::prelude::*;

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

    /// Creates an / the _empty_ production string. This is a
    /// synonym for [`ProductionString::new`].
    #[inline]
    pub fn empty() -> Self {
        ProductionString::new()
    }

    /// Whether the production string is empty or not. i.e., whether this is
    /// the _empty_ string.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    /// Returns the length of the production string.
    #[inline]
    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    /// Access the tokens of this production string.
    #[inline]
    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    /// Add another token to the end of the string.
    #[inline]
    pub fn push_token(&mut self, token: Token) {
        self.tokens.push(token);
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

impl IntoIterator for ProductionString {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.into_iter()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string_is_empty() {
        let empty = ProductionString::empty();

        assert!(empty.is_empty());
        assert_eq!(empty.len(), 0);
    }
}