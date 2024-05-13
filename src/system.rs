use std::collections::{BTreeMap, HashMap};
use std::ops::Deref;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::RwLock;

use crate::error::{Error, ErrorKind};
use crate::prelude::*;
use crate::productions::{Production, ProductionBody, ProductionHead};
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
    productions: RwLock<Vec<Production>>,
    token_id: AtomicU32
}

impl System {
    pub fn new() -> Self {
        System {
            tokens: RwLock::new(BTreeMap::new()),
            productions: RwLock::new(Vec::new()),
            token_id: AtomicU32::new(100)
        }
    }

    /// Parse a string as a production and add it to the system.
    ///
    /// * Empty bodies are allowed. This is how to write productions that lead
    ///   to the empty string.
    pub fn parse_production(&mut self, production: &str) -> Result<&Production> {
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

        let mut body_tokens = Vec::new();
        for term in body {
            let kind = determine_kind(term).ok_or_else(|| Error::new(ErrorKind::Parse,"unable to determine token type"))?;
            body_tokens.push(self.add_token(term, kind)?);
        }

        let body = ProductionBody::new(ProductionString::from(body_tokens));

        let lock = self.productions.get_mut();
        if let Ok(productions) = lock {
            productions.push(Production::new(head, body));
            return Ok(productions.last().unwrap())
        }

        Err(Error::general("Poison error attempting to access productions lock"))
    }

    pub fn to_string(&self, string: &ProductionString) -> crate::Result<String> {


        let mut code_to_string = HashMap::new();
        if let Ok(tokens) = self.tokens.read() {
            tokens.iter()
                .for_each(|(i, val)| {
                    code_to_string.insert(val.code(), i.clone());
                });
        } else {
            return Err(Error::general("Poisoned lock when accessing tokens"));
        }

        let mut result : Vec<String> = Vec::new();
        for token in string.tokens() {
            let name = code_to_string.get(&token.code())
                .cloned()
                .ok_or_else(|| Error::general(format!("Unable to find term for token [{}]", token.code())))?;
            result.push(name);
        }

        Ok(result.join(" "))
    }

    /// Run a single iteration of the productions on the given string.
    /// Returns [`None`] if an empty string is produced.
    pub fn derive_once(&self, string: ProductionString) -> Option<ProductionString> {
        if string.is_empty() {
            return None
        }

        if let Ok(productions) = self.productions.read() {
            return derive_once(string, productions.deref());
        }

        panic!("Poisoned lock on production list");
    }

    pub fn derive(&self, string: ProductionString, settings: RunSettings) -> Option<ProductionString> {
        if string.is_empty() {
            return None
        }

        if let Ok(productions) = self.productions.read() {
            let productions = productions.deref();
            let mut current = string;
            for _ in 0..settings.max_iterations() {
                current = derive_once(current, productions)?;
            }
            return Some(current);
        }

        panic!("Poisoned lock on production list");
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

    /// Return the token that represents the given term, if it exists.
    ///
    ///Note that this does not create any new tokens to the system.
    pub fn get_token(&self, name: &str) -> Option<Token> {
        if let Ok(tokens) = self.tokens.read() {
            return tokens.get(name).copied();
        }

        panic!("Access to the token vector has been poisoned");
    }

    pub fn to_production_string(&self, string: &str) -> crate::Result<ProductionString> {
        let mut result = ProductionString::default();

        let items = string.trim().split_ascii_whitespace();

        for term in items {
            let kind = determine_kind(term)
                .ok_or_else(|| Error::new(ErrorKind::Parse, "Unable to determine token kind"))?;
            result.push_token(self.add_token(term, kind)?);
        }

        Ok(result)
    }
}

impl Default for System {
    fn default() -> Self {
        System::new()
    }
}

#[derive(Debug, Clone)]
pub struct RunSettings {
    max_iterations: usize
}

impl RunSettings {
    pub fn for_max_iterations(max_iterations: usize) -> Self {
        RunSettings { max_iterations }
    }

    /// The maximum number of iterations allowed for a derivation.
    pub fn max_iterations(&self) -> usize {
        self.max_iterations
    }
}


impl From<usize> for RunSettings {
    fn from(value: usize) -> Self {
        RunSettings::for_max_iterations(value)
    }
}

const DEFAULT_MAX_ITERATIONS : usize = 10;

impl Default for RunSettings {
    fn default() -> Self {
        RunSettings {
            max_iterations: DEFAULT_MAX_ITERATIONS
        }
    }
}



/// Given a vector of productions, this returns a reference to a
/// production that matches the string at the given location.
pub fn find_matching<'a>(productions: &'a [Production],
                     string: &ProductionString, index: usize) -> Option<&'a Production> {
    for production in productions {
        if production.matches(string, index) {
            return Some(production)
        }
    }

    None
}

/// Runs one step of an iteration, using the given production rules.
///
/// Most of the time you will want to make use of [`System::derive_once`]
/// instead of trying to call this function directly.  
pub fn derive_once(string: ProductionString, productions: &[Production]) -> Option<ProductionString> {
    if string.is_empty() {
        return None
    }

    let mut result = ProductionString::default();

    for (index, token) in string.tokens().iter().enumerate() {
        if token.is_terminal() {
            result.push_token(*token);
            continue;
        }

        if let Some(production) = find_matching(productions, &string, index) {
            production.body()
                .string()
                .tokens()
                .iter()
                .copied()
                .for_each(|token| result.push_token(token));
            continue;
        }
    }

    match result.len() {
        0 => None,
        _ => Some(result)
    }

}

#[cfg(test)]
mod tests {
    use std::thread;
    use super::*;

    #[test]
    fn handles_empty_production() {
        let mut system = System::new();
        assert!(system.parse_production("").is_err());
    }

    #[test]
    fn handles_no_head() {
        let mut system = System::new();
        assert!(system.parse_production("-> surname surname").is_err());
    }

    #[test]
    fn handles_no_body() {
        let mut system = System::new();
        assert!(system.parse_production("Company ->").is_ok());
    }

    #[test]
    fn handles_correct() {
        let mut system = System::new();
        assert!(system.parse_production("Company -> surname surname").is_ok());
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
        let mut token1 : Option<Token> = None;
        let mut token2 : Option<Token> = None;

        let system = System::new();

        thread::scope(|s| {
            s.spawn(|| {
                token1 = Some(system.add_token("one", TokenKind::Terminal).unwrap());
            });

            s.spawn(|| {
                token2 = Some(system.add_token("two", TokenKind::Terminal).unwrap());
            });
        });

        let token1 = token1.unwrap();
        let token2 = token2.unwrap();

        assert_ne!(token1.code(), token2.code());
    }

    #[test]
    fn can_derive_once() {
        let mut system = System::new();
        system.parse_production("Company -> surname Company").expect("Unable to add production");
        let string = system.to_production_string("Company").expect("Unable to create string");
        let result = system.derive_once(string).expect("Unable to derive");

        assert_eq!(result.len(), 2);
    }
}