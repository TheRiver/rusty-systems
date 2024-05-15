use crate::error::{Error, ErrorKind};
use crate::prelude::{ProductionString};
use crate::productions::{Production, ProductionBody, ProductionHead, ProductionStore};
use crate::tokens::{TokenKind, TokenStore};
use crate::Result;


/// Parse the body of a production rule.
///
/// For example, in the string `A -> B C`, the `B C` after the arrow
/// is the rule's body.
pub fn parse_production_body<S>(store: &mut S, body: &str) -> Result<ProductionBody>
    where S: TokenStore {
    let body = body.trim();
    if body.is_empty() {
        return Ok(ProductionBody::empty());
    }

    let body = body.split_ascii_whitespace();

    let mut body_tokens = Vec::new();
    for term in body {
        let kind = determine_kind(term).ok_or_else(|| Error::new(ErrorKind::Parse,"unable to determine token type"))?;
        body_tokens.push(store.add_token(term, kind)?);
    }

    Ok(ProductionBody::new(ProductionString::from(body_tokens)))
}

pub fn parse_production_head<S>(store: &mut S, head: &str) -> Result<ProductionHead>
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

pub fn parse_production<'a, S>(store: &'a mut S, production: &str) -> Result<&'a Production>
    where S: TokenStore + ProductionStore
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

    let head = parse_production_head(store, head_str)?;
    let body = parse_production_body(store, body_str)?;

    store.add_production(Production::new(head, body))
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
        let body = parse_production_body(&mut store, "A B");

        assert_eq!(body.unwrap().len(), 2);
    }

    #[test]
    fn can_determine_token_kind() {
        assert_eq!(determine_kind("bob").unwrap(), TokenKind::Terminal);
        assert_eq!(determine_kind("Bob").unwrap(), TokenKind::Production);
        assert!(determine_kind("Bo b").is_none());
        assert!(determine_kind("Bo(b)").is_none());
    }
}

