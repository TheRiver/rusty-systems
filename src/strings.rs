//! The string of [`Token`] instances which are rewritten using [`crate::productions::Production`]
//! rules of a [`System`].

use std::collections::HashMap;
use std::iter::Cloned;
use std::slice::Iter;
use crate::DisplaySystem;
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

    /// Iterate over the tokens.
    #[inline]
    pub fn iter(&self) -> Iter<'_, Token> {
        self.tokens.iter()
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

impl<'a> IntoIterator for &'a ProductionString {
    type Item = Token;
    type IntoIter = Cloned<Iter<'a, Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        self.tokens.iter().cloned()
    }
}

impl DisplaySystem for ProductionString {
    fn format(&self, names: &HashMap<Token, String>) -> crate::Result<String> {
        let mut error = false;
        let string : Vec<String> = self.tokens()
            .iter()
            .map(|t| names.get(t))
            .map(|o| o.ok_or_else(|| Error::general("Some tokens have no name")))
            .filter_map(|r| r.map_err(|_| error = true).ok())
            .cloned()
            .collect();

        if error {
            return Err(Error::general("Unable to find names for all tokens"));
        }

        Ok(string.join(" "))
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