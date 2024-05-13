use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::RwLock;

use crate::error::{Error, ErrorKind};
use crate::prelude::*;
use crate::productions::ProductionHead;
use crate::tokens::{determine_kind, TokenKind};

use super::Result;

/// Represents an L-system. This is the base for running the
/// production rules.
///
/// * Productions can be parsed via strings.
/// * Productions can be programmatically created.
///
/// This is thread safe, and is [`Sync`] and [`Send`].
///
/// See [`Token`].
#[derive(Debug)]
pub struct System {
    tokens: RwLock<BTreeMap<String, Token>>,
    token_id: AtomicU32
}

impl System {

    pub fn hello(&self) -> String {
        String::from("hello world")
    }

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
        let is_production = determine_kind(head)
            .map(|kind| kind.is_production())
            .unwrap_or(false);

        if !is_production {
            return Err(Error::new(ErrorKind::Parse,
                                  "production tokes should start with a capitalised letter"));
        }

        let head_token = self.add_token(head, TokenKind::Production)?;
        let head = ProductionHead::build(head_token)?;

        println!("Head is {:?}", head);
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
        let atomic = self.token_id.fetch_add(1, Ordering::SeqCst);
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
    use std::thread;
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

    #[test]
    fn sync_and_send() {
        let token1: RwLock<Option<Token>> = RwLock::new(None);
        let token2: RwLock<Option<Token>> = RwLock::new(None);

        let system = System::new();

        thread::scope(|s| {
            s.spawn(|| {
                let lock = token1.write();
                if let Ok(mut token1) = lock {
                    *token1 = Some(system.add_token("one", TokenKind::Terminal).unwrap());
                }
            });

            s.spawn(|| {
                let lock = token2.write();
                if let Ok(mut token2) = lock {
                    *token2 = Some(system.add_token("two", TokenKind::Terminal).unwrap());
                }
            });
        });

        let token1 = token1.read().unwrap().unwrap();
        let token2 = token2.read().unwrap().unwrap();

        assert_ne!(token1.code(), token2.code());
    }
}