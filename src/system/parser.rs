//! Tools for parsing simple L-Systems
//!
//! These are the functions that [`crate::prelude::System`] itself uses, but they
//! are generic enough that you can make use of them yourself and if you wish to
//! avoid using [`crate::prelude::System`].

use crate::error::{Error, ErrorKind};
use crate::prelude::*;
use crate::productions::{Production, ProductionBody, ProductionHead, ProductionStore};
use crate::tokens::{TokenKind, TokenStore};
use crate::Result;

/// Parse the body of a production rule.
///
/// For example, in the string `A -> B C`, the `B C` after the arrow
/// is the rule's body.
pub fn parse_production_body<S>(store: &S, body: &str) -> Result<ProductionBody>
    where S: TokenStore {
    let body = body.trim();
    if body.is_empty() {
        return Ok(ProductionBody::empty());
    }

    let body = body.split_ascii_whitespace();

    let mut body_tokens = Vec::new();
    let mut chance : Option<f32> = None;

    for (index, term) in body.enumerate() {
        if index == 0 {
            if let Ok(val) = term.parse() {
                chance = Some(val);
                continue;
            }
        }

        let kind = determine_kind(term).ok_or_else(|| Error::new(ErrorKind::Parse,"unable to determine token type"))?;
        body_tokens.push(store.add_token(term, kind)?);
    }

    match chance {
        None => Ok(ProductionBody::new(ProductionString::from(body_tokens))),
        Some(chance) => ProductionBody::try_with_chance(chance, ProductionString::from(body_tokens))
    }
}

pub fn parse_production_head<S>(store: &S, head: &str) -> Result<ProductionHead>
    where S: TokenStore
{
    let head = head.trim();

    if head.is_empty() {
        return Err(Error::new(ErrorKind::Parse, "no head in production string"));
    }

    let is_production = determine_kind(head)
        .map(|kind| kind.is_production())
        .unwrap_or(false);

    if !is_production {
        return Err(Error::new(ErrorKind::Parse,
                              "production tokes should start with a capitalised letter"));
    }

    let head_token = store.add_token(head, TokenKind::Production)?;
    ProductionHead::build(head_token)
}

/// Parse a production string.
///
/// Most of the time you likely want to do this using [`System::parse_production`],
/// which is also thread safe. If you want to use this (note that this is not thread safe),
/// you can do so using your own implementations of:
///
/// * [`ProductionStore`], which stores productions.
/// * [`TokenStore`], which stores and generates unique tokens.
///
/// Default implementations exist for
///
/// * [`std::cell::RefCell<Vec<Production>>`] implements [`ProductionStore`].
/// * [`std::cell::RefCell<std::collections::HashMap<String, prelude::Token>>`] implements [`TokenStore`].
///
/// These are easy to use by just wrapping your collections in RefCell. Please note that doing this
/// is not thread safe:
///
/// ```
/// use std::cell::RefCell;
/// use std::collections::HashMap;
/// use rusty_systems::productions::Production;
/// use rusty_systems::system::parser::parse_production;
/// use rusty_systems::tokens::Token;
///
/// // Create your token and production collections at some point
/// // in your code.
/// let tokens : HashMap<String, Token> = HashMap::new();
/// let productions : Vec<Production> = Vec::new();
///
/// // ... Do a lot of other stuff. Call functions. Have fun!
///
/// // Borrow your collections.
/// let token_cell = RefCell::new(tokens);
/// let production_cell = RefCell::new(productions);
///
/// // Now we can parse a production. Note that because of the underlying
/// // stores, this is not thread safe.
/// let result = parse_production(&token_cell, &production_cell, "Name -> first surname");
/// // Check for errors, etc: result.is_err(), and so on.
///
/// // Get your collections back from the cells.
/// // You can now look at the added tokens and productions in these collections.
/// let tokens = token_cell.take();
/// let productions = production_cell.take();
///
/// ```
pub fn parse_production<T, P>(token_store: &T,
                              prod_store: &P,
                              production: &str) -> Result<Production>
    where   T: TokenStore,
            P: ProductionStore
{
    let production = production.trim();
    if production.is_empty() {
        return Err(Error::new(ErrorKind::Parse, "no terms in production string"));
    }

    let index = production
        .find("->")
        .ok_or_else(|| Error::new(ErrorKind::Parse, "supplied string is not a production"))?;

    let head_str = &production[0..index];
    let body_str = &production[index + 2..];

    let head = parse_production_head(token_store, head_str)?;
    let body = parse_production_body(token_store, body_str)?;

    prod_store.add_production(Production::new(head, body))
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

    if string.contains('(') || string.contains(')') || string.contains(' ') {
        return None;
    }

    if string.parse::<f32>().is_ok() {
        return None;
    }

    let first = string.chars().next()?;
    if first.is_ascii_uppercase() {
        return Some(TokenKind::Production)
    }

    Some(TokenKind::Terminal)
}



#[cfg(test)]
mod test {
    use crate::system::parser::{determine_kind, parse_production_body};
    use crate::system::System;
    use crate::tokens::TokenKind;

    #[test]
    fn can_parse_empty_body() {
        let mut store = System::default();
        let body = parse_production_body(&mut store, "");

        assert!(body.unwrap().is_empty());
    }

    #[test]
    fn can_parse_body_without_chance() {
        let mut store = System::default();
        let body = parse_production_body(&mut store, "A B").unwrap();

        assert_eq!(body.len(), 2);
        assert!(body.chance().is_derived());
    }

    #[test]
    fn can_parse_body_with_chance() {
        let mut store = System::default();
        let body = parse_production_body(&mut store, "0.3 A B").unwrap();

        assert_eq!(body.len(), 2);
        assert!(body.chance().is_user_set());
        assert_eq!(body.chance().unwrap(), 0.3);
    }

    #[test]
    fn can_determine_token_kind() {
        assert_eq!(determine_kind("bob").unwrap(), TokenKind::Terminal);
        assert_eq!(determine_kind("Bob").unwrap(), TokenKind::Production);
        assert!(determine_kind("Bo b").is_none());
        assert!(determine_kind("Bo(b)").is_none());
    }
}

