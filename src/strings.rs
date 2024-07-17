//! Provides tools for handling strings of  [`Symbol`] objects which are rewritten using [`crate::productions::Production`]
//! rules of a [`System`].
//!
//! The main struct is [`ProductionString`]. These can be parsed from a text string using
//! [`parser::parse_prod_string`](crate::parser::parse_prod_string). See [`ProductionString`]
//! for more details.
//! 
//! # Creating production strings
//! 
//! The simplest way to create a [`ProductionString`] instance is using
//! [`str::parse`]:
//! 
//! ```
//! use rusty_systems::prelude::ProductionString;
//! let string: ProductionString = "F F F F".parse().expect("Unable to parse");
//! ```
//! 
//! This is the same as the following parse function:
//! 
//! ```
//! use rusty_systems::parser;
//! use rusty_systems::prelude::ProductionString;
//! let string = parser::parse_prod_string("F F F F").expect("Unable to parse");
//! ```
//!

use std::fmt::{Display, Formatter, Write};
use std::iter::Cloned;
use std::ops::Index;
use std::slice::Iter;
use std::str::FromStr;

use crate::parser::parse_prod_string;
use crate::prelude::*;

/// Represents strings in our L-system. Strings
/// are made up of a list of [`Symbol`] objects.
///
/// If you would like to parse an instance of this struct
/// from a string, you can use
/// [`parser::parse_prod_string`](crate::parser::parse_prod_string), like so:
///
/// ```
/// use rusty_systems::parser;
/// let axiom = parser::parse_prod_string("Forward Forward Forward").unwrap();
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProductionString {
    symbols: Vec<Symbol>
}

impl ProductionString {
    /// Create an empty string
    pub fn new() -> Self {
        ProductionString {
            symbols: Vec::new()
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
        self.symbols.is_empty()
    }

    /// Returns the length of the production string.
    #[inline]
    pub fn len(&self) -> usize {
        self.symbols.len()
    }

    /// Access the symbols of this production string.
    #[inline]
    pub fn symbols(&self) -> &Vec<Symbol> {
        &self.symbols
    }

    /// Add another symbol to the end of the string.
    #[inline]
    pub fn push_symbol(&mut self, symbol: Symbol) {
        self.symbols.push(symbol);
    }

    /// Iterate over the symbols.
    #[inline]
    pub fn iter(&self) -> Iter<'_, Symbol> {
        self.symbols.iter()
    }
}

impl Default for ProductionString {
    fn default() -> Self {
        ProductionString::new()
    }
}

impl From<Vec<Symbol>> for ProductionString {
    fn from(value: Vec<Symbol>) -> Self {
        ProductionString {
            symbols: value
        }
    }
}

impl Index<usize> for ProductionString {
    type Output = Symbol;

    fn index(&self, index: usize) -> &Self::Output {
        &self.symbols[index]
    }
}


impl From<Symbol> for ProductionString {
    fn from(value: Symbol) -> Self {
        ProductionString {
            symbols: vec![value]
        }
    }
}

impl IntoIterator for ProductionString {
    type Item = Symbol;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.symbols.into_iter()
    }
}

impl<'a> IntoIterator for &'a ProductionString {
    type Item = Symbol;
    type IntoIter = Cloned<Iter<'a, Self::Item>>;

    fn into_iter(self) -> Self::IntoIter {
        self.symbols.iter().cloned()
    }
}

impl Display for ProductionString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for symbol in self.symbols() {
            if !first {
                f.write_char(' ')?;
            } else {
                first = false;
            }
            f.write_str(symbol.to_string().as_str())?;
        }
        
        Ok(())
        
    }
}

impl FromStr for ProductionString {
    type Err = Error;

    #[inline]
    fn from_str(string: &str) -> Result<Self, Self::Err> {
        parse_prod_string(string)
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