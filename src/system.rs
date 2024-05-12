use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::RwLock;

use crate::error::{Error, ErrorKind};
use crate::prelude::*;
use crate::tokens::TokenKind;

use super::Result;

pub struct System {
    tokens: RwLock<BTreeMap<String, Token>>,
    token_id: AtomicU32
}

impl System {
    pub fn new() -> Self {
        System {
            tokens: RwLock::new(BTreeMap::new()),
            token_id: AtomicU32::default()
        }
    }

    /// Parse a string as a production and add it to the system.
    ///
    /// * Empty bodies are allowed. This is how to write productions that lead
    ///   to the empty string.
    pub fn add_production(&mut self, production: &str) -> Result<()> {
        let terms: Vec<&str> = production
            .trim()
            .split_ascii_whitespace()
            .filter(|s| !s.is_empty())
            .collect();

        if terms.is_empty() {
            return Err(Error::new(ErrorKind::Parse, "no terms in production string"));
        }

        let index = terms.iter()
            .enumerate()
            .find(|t| *t.1 == "->")
            .map(|(i, _)| i)
            .ok_or_else(|| Error::new(ErrorKind::Parse, "Unable to find \"->\" term"))?;

        let head = &terms[0..index];
        let body = &terms[index+1..];

        if head.is_empty() {
            return Err(Error::new(ErrorKind::Parse, "no head in production string"));
        }

        if head.len() != 1 {
            return Err(Error::new(ErrorKind::Parse, "productions should be context free"));
        }

        let head = head[0];
        if let Some(ch) = head.chars().next() {
           if !ch.is_ascii_uppercase() {
               return Err(Error::new(ErrorKind::Parse,
                                     "production tokes should start with a capitalised letter"));
           }
        }


        let head_token = self.add_token(head, TokenKind::Production)?;
        // let mut bodies = Vec::new();
        //
        // for bod in body {
        //     bodies.push(self.add_token(&bod, TokenKind::Terminal)?)
        // }

        println!("Head is {:?} with token {head_token:?}", head);
        println!("tail is {:?}", body);

        Ok(())
    }


    fn add_token(&self, name: &str, kind: TokenKind) -> Result<Token> {
        if name.is_empty() {
            return Err(Error::general("name should not be an empty string"));
        }

        let map = self.tokens.write();
        if let Err(e) = map {
            let message = format!("Error accessing token cache: {}", e);
            return Err(Error::general(message));
        }


        let mut map = map.unwrap();

        // If it already exists, return it.
        if let Some(value) = map.get(name) {
            return Ok(*value);
        }

        // Safely create a new token.
        let atomic = self.token_id.fetch_add(1, Ordering::Relaxed);
        let token = Token::new(kind, atomic);
        map.insert(name.to_string(), token);
        return Ok(map.get(name).copied().unwrap())
    }
}

impl Default for System {
    fn default() -> Self {
        System::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn handles_empty_production() {
        let mut system = System::new();
        assert!(system.add_production("").is_err());
    }

    #[test]
    fn handles_no_head() {
        let mut system = System::new();
        assert!(system.add_production("-> surname surname").is_err());
    }

    #[test]
    fn handles_no_body() {
        let mut system = System::new();
        assert!(system.add_production("Company ->").is_ok());
    }

    #[test]
    fn handles_correct() {
        let mut system = System::new();
        assert!(system.add_production("Company -> surname surname").is_ok());
    }

    #[test]
    fn increments_for_distinct_names() {
        let system = System::new();
        let token1 = system.add_token("one", TokenKind::Terminal).unwrap();
        let token2 = system.add_token("two", TokenKind::Terminal).unwrap();

        assert_ne!(token1.code(), token2.code());
    }

    #[test]
    fn no_increments_for_equal_names() {
        let system = System::new();
        let token1 = system.add_token("one", TokenKind::Terminal).unwrap();
        let token2 = system.add_token("one", TokenKind::Terminal).unwrap();

        assert_eq!(token1.code(), token2.code());
    }
}