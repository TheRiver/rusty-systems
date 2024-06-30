//! Tools for parsing L-Systems from text strings.
//!
//! Important functions:
//! * [`parse_prod_string`]
//! * [`parse_production`]

use crate::error::{Error, ErrorKind};
use crate::prelude::*;
use crate::productions::{Production, ProductionBody, ProductionHead};
use crate::Result;

/// Parse the body of a production rule.
///
/// For example, in the string `A -> B C`, the `B C` after the arrow
/// is the rule's body.
pub fn parse_production_body(body: &str) -> Result<ProductionBody> {
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

        body_tokens.push(Symbol::build(term)?);
    }

    match chance {
        None => Ok(ProductionBody::new(ProductionString::from(body_tokens))),
        Some(chance) => ProductionBody::try_with_chance(chance, ProductionString::from(body_tokens))
    }
}

/// Parse the head of a production rule.
pub fn parse_production_head(head: &str) -> Result<ProductionHead> {
    let head = head.trim();

    if head.is_empty() {
        return Err(Error::new(ErrorKind::Parse, "no head in production string"));
    }

    let tokens : Vec<_> = head.split_ascii_whitespace().collect();
    let split: Vec<_> = tokens.splitn(2, |s| *s == "<").collect();

    let mut left : Option<&[&str]> = None;
    let mut right : Option<&[&str]> = None;


    let remains = if split.len() == 2 {
        left = Some(split[0]);
        split[1]
    } else {
        split[0]
    };

    let split : Vec<_> = remains.splitn(2, |s| *s == ">").collect();
    let remains = if split.len() == 2 {
        right = Some(split[1]);
        split[0]
    } else {
        split[0]
    };

    if remains.len() != 1 {
        return Err(Error::new(ErrorKind::Parse, "There should be exactly one token as the head target"))
    }

    let center = remains[0];
    let head_token = Symbol::build(center)?;

    let left = parse_head_context(left);
    if let Some(Err(e)) = left {
        return Err(e);
    }

    let left = left.map(|d| d.unwrap());

    let right = parse_head_context(right);
    if let Some(Err(e)) = right {
        return Err(e);
    }

    let right = right.map(|d| d.unwrap());

    ProductionHead::build(
        left,
        head_token,
        right)
}

fn parse_head_context(strings: Option<&[&str]>) -> Option<Result<ProductionString>> {
    strings.map(|strings| {
        let iter = strings.iter().map(|s| Symbol::build(*s));
        let error = iter.clone().find(|t| t.is_err());

        if let Some(Err(e)) = error {
            return Err(e);
        }

        let tokens: Vec<_> = iter.map(|t| t.unwrap()).collect();

        Ok(ProductionString::from(tokens))
    })
}


// Parse a production string.
//
// Most of the time you likely want to do this using [`System::parse_production`],
// which is also thread safe. If you want to use this (note that this is not thread safe),
// you can do so using your own implementations of:
//
// * [`ProductionStore`], which stores productions.
// * [`SymbolStore`], which stores and generates unique tokens.
//
// Default implementations exist for
//
// * [`std::cell::RefCell<Vec<Production>>`] implements [`ProductionStore`].
// * [`std::cell::RefCell<std::collections::HashMap<String, prelude::Token>>`] implements [`SymbolStore`].
//
// These are easy to use by just wrapping your collections in RefCell. Please note that doing this
// is not thread safe:
//
// ```
// use std::cell::RefCell;
// use std::collections::{HashMap, HashSet};
// use std::sync::Arc;
// use rusty_systems::productions::Production;
// use rusty_systems::parser::parse_production;
// use rusty_systems::symbols::Symbol;
//
// // Create your token and production collections at some point
// // in your code.
// let tokens : HashSet<u32> = HashSet::new();
// let productions : Vec<Production> = Vec::new();
//
// // ... Do a lot of other stuff. Call functions. Have fun!
//
// // Borrow your collections.
// let token_cell = RefCell::new(tokens);
// let production_cell = RefCell::new(productions);
//
// // Now we can parse a production. Note that because of the underlying
// // stores, this is not thread safe.
// let result = parse_production(&token_cell, &production_cell, "Name -> first surname");
// // Check for errors, etc: result.is_err(), and so on.
//
// // Get your collections back from the cells.
// // You can now look at the added tokens and productions in these collections.
// let tokens = token_cell.take();
// let productions = production_cell.take();
//
// ```
pub fn parse_production(production: &str) -> Result<Production> {
    let production = production.trim();
    if production.is_empty() {
        return Err(Error::new(ErrorKind::Parse,
                              String::from("production string should not be an empty string")));
    }

    let index = production
        .find("->")
        .ok_or_else(|| Error::new(ErrorKind::Parse,
                                  String::from("supplied string is not a production: ") + production))?;

    let head_str = &production[0..index];
    let body_str = &production[index + 2..];

    let head = parse_production_head(head_str)?;
    let body = parse_production_body(body_str)?;

    Ok(Production::new(head, body))
}



/// Allows you to parse a text string into a string of [`Symbol`] objects
/// to then rewrite using a [`System`]
pub fn parse_prod_string(string: &str) -> Result<ProductionString> {
    let mut result = ProductionString::default();

    let items = string.trim().split_ascii_whitespace();


    for term in items {
        result.push_symbol(Symbol::build(term)?);
    }

    Ok(result)
}




#[cfg(test)]
mod test {
    use crate::parser::{parse_production_body, parse_production_head};
    use crate::symbols::{get_code};

    #[test]
    fn can_parse_empty_body() {
        let body = parse_production_body("");

        assert!(body.unwrap().is_empty());
    }

    #[test]
    fn can_parse_body_without_chance() {
        let body = parse_production_body("A B").unwrap();

        assert_eq!(body.len(), 2);
        assert!(body.chance().is_derived());
    }

    #[test]
    fn can_parse_body_with_chance() {
        let body = parse_production_body("0.3 A B").unwrap();

        assert_eq!(body.len(), 2);
        assert!(body.chance().is_user_set());
        assert_eq!(body.chance().unwrap(), 0.3);
    }

    #[test]
    fn parsing_production_head() {
        let head = parse_production_head("A").unwrap();
        assert_eq!(get_code("A").unwrap(), head.target().code());

        let head = parse_production_head("Pre < A > Post").unwrap();
        assert_eq!(get_code("A").unwrap(), head.target().code());

        let left = head.pre_context().unwrap();
        assert_eq!(left.len(), 1);
        assert_eq!(get_code("Pre").unwrap(), left[0].code());

        let right = head.post_context().unwrap();
        assert_eq!(right.len(), 1);
        assert_eq!(get_code("Post").unwrap(), right[0].code());
    }
}

