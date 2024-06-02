//! Collection of tools for defining a group of [`Production`] rules on strings
//! of tokens.
//!
//! # Creating systems
//!
//! ## Using [`System`]
//!
//! * TODO Talk about [`System::new`] and [`System::default`].
//! * TODO Talk about [`System::of_family`].
//!
//! ## Collections of [`Token`] and [`Production`]
//!
//! TODO talk about collections of tokens and productions.
//!
//! # Families
//!
//! TODO talk about families
//!
//! # Thread safety
//!
//! TODO Talk about thread safety
//!
//! # See also
//! * [`Token`]
//! * [`Production`]
//! * [`ProductionString`]
//! * [`SystemFamily`]

use std::collections::HashMap;
use std::ops::Deref;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::RwLock;

use parser::determine_kind;

use crate::error::{Error, ErrorKind};
use crate::prelude::*;
use crate::productions::{Production, ProductionStore};
use crate::system::family::TryIntoFamily;
use crate::tokens::{TokenKind, TokenStore};

use super::{DisplaySystem, Result};

pub mod parser;
pub mod family;

/// Represents an L-system. This is the base for running the
/// production rules.
///
/// This struct is convenient for defining and storing tokens
/// and productions without having to create your own collections.
///
/// See the [system namespace](crate::system) to for information more broadly.
///
/// * Productions can be parsed via strings.
/// * Productions can be programmatically created.
///
/// This is thread safe, and is [`Sync`] and [`Send`].
#[derive(Debug)]
pub struct System {
    tokens: RwLock<HashMap<String, Token>>,
    productions: RwLock<Vec<Production>>,
    token_id: AtomicU32
}

impl System {
    pub fn new() -> Self {
        System {
            tokens: RwLock::new(HashMap::new()),
            productions: RwLock::new(Vec::new()),
            token_id: AtomicU32::new(100)
        }
    }

    /// Given a previously defined family, this returns a new system
    /// using the defined tokens / alphabet / words of that family of systems.
    ///
    /// ```
    /// use rusty_systems::system::{family, System};
    /// use rusty_systems::interpretation::abop;
    ///
    /// family::register(abop::abop_family()).expect("Unable to register the family");
    /// let system = System::of_family("ABOP").expect("Unable to find system");
    /// ```
    pub fn of_family<F: TryIntoFamily>(family: F) -> Result<Self> {
        let family = family.into_family()?;
        let system = System::default();

        for token in family.tokens() {
            system.add_token(token.name.as_str(), token.kind)?;
        }

        Ok(system)
    }

    /// Parse a string as a production and add it to the system.
    ///
    /// * Empty bodies are allowed. This is how to write productions that lead
    ///   to the empty string.
    pub fn parse_production(&self, production: &str) -> Result<Production> {
        parser::parse_production(self, self, production)
    }

    /// Format [`Token`], [`ProductionString`], [`Production`] as strings.
    pub fn format<T: DisplaySystem>(&self, item: &T) -> Result<String> {
        let code_to_string = {
            let mut map = HashMap::new();

            if let Ok(tokens) = self.tokens.read() {
                tokens.iter()
                    .for_each(|(i, val)| {
                        map.insert(*val, i.clone());
                    });
            } else {
                return Err(Error::general("Poisoned lock when accessing tokens"));
            }

            map
        };

        item.format(&code_to_string)
    }


    /// Run a single iteration of the productions on the given string.
    /// Returns [`None`] if an empty string is produced.
    pub fn derive_once(&self, string: ProductionString) -> Result<ProductionString> {
        if string.is_empty() {
            return Ok(ProductionString::empty())
        }

        if let Ok(productions) = self.productions.read() {
            return derive_once(string, productions.deref());
        }

        Err(Error::general("Poisoned lock on production list"))
    }

    pub fn derive(&self, string: ProductionString, settings: RunSettings) -> Result<ProductionString> {
        if string.is_empty() {
            return Ok(ProductionString::empty())
        }

        if let Ok(productions) = self.productions.read() {
            return derive(string, productions.deref(), settings);
        }

        Err(Error::general("Poisoned lock on production list"))
    }

    pub fn parse_prod_string(&self, string: &str) -> Result<ProductionString> {
        let mut result = ProductionString::default();

        let items = string.trim().split_ascii_whitespace();

        for term in items {
            let kind = determine_kind(term)
                .ok_or_else(|| Error::new(ErrorKind::Parse, "Unable to determine token kind"))?;
            result.push_token(self.add_token(term, kind)?);
        }

        Ok(result)
    }
    
    /// Returns the number of production rules in the system.
    pub fn production_len(&self) -> usize {
        self.productions.read().unwrap().len()
    }

    /// Returns the number of tokens (terminal and production tokens) registered with the system
    pub fn token_len(&self) -> usize {
        self.tokens.read().unwrap().len()
    }

    /// Returns the number of terminal tokens registered with the system
    pub fn terminal_tokens_len(&self) -> usize {
        self.tokens.read().unwrap()
            .values()
            .filter(|t| t.is_terminal())
            .count()
    }

    /// Returns the number of terminal tokens registered with the system
    pub fn prod_tokens_len(&self) -> usize {
        self.tokens.read().unwrap()
            .values()
            .filter(|t| t.is_production())
            .count()
    }
}

impl TokenStore for System {
    fn add_token(&self, name: &str, kind: TokenKind) -> Result<Token> {
        if name.is_empty() {
            return Err(Error::general("name should not be an empty string"));
        }

        let mut map = self.tokens.write()?;

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
    /// Note that this does not create any new tokens to the system.
    fn get_token(&self, name: &str) -> Option<Token> {
        let tokens = self.tokens.read().ok()?;
        return tokens.get(name).copied();
    }
}

impl ProductionStore for System {
    fn add_production(&self, production: Production) -> Result<Production> {
        let lock = self.productions.write();
        if let Ok(mut productions) = lock {
            let head = production.head().clone();

            match productions.iter_mut().find(|p| (*p.head()).eq(&head)) {
                None => productions.push(production),
                Some(found) => {
                    found.merge(production);
                }
            }

            return Ok(productions.iter().find(|p| (*p.head()).eq(&head)).unwrap().clone())
        }

        Err(Error::new(ErrorKind::Locking, "production lock is poisoned"))
    }
}


impl Default for System {
    fn default() -> Self {
        System::new()
    }
}

/// Defines constraints on deriving strings from an [`System`].
#[derive(Debug, Clone)]
pub struct RunSettings {
    /// The maximum number of iterations allowed for a derivation.
    pub max_iterations: usize
}

impl RunSettings {
    pub fn for_max_iterations(max_iterations: usize) -> Self {
        RunSettings { max_iterations }
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
pub fn derive_once(string: ProductionString, productions: &[Production]) -> Result<ProductionString> {
    if string.is_empty() {
        return Ok(ProductionString::empty())
    }

    let mut result = ProductionString::default();

    for (index, token) in string.tokens().iter().enumerate() {
        if token.is_terminal() {
            result.push_token(*token);
            continue;
        }

        if let Some(production) = find_matching(productions, &string, index) {
            let body = production.body()?;

            body.string()
                .tokens()
                .iter()
                .copied()
                .for_each(|token| result.push_token(token));
            continue;
        } else {
            result.push_token(*token);
        }


    }

    match result.len() {
        0 => Ok(ProductionString::empty()),
        _ => Ok(result)
    }
}

pub fn derive(string: ProductionString, productions: &[Production], settings: RunSettings) -> Result<ProductionString> {
    if string.is_empty() {
        return Ok(ProductionString::empty())
    }

    let mut current = string;
    for _ in 0..settings.max_iterations {
        current = derive_once(current, productions)?;
    }

    Ok(current)
}





#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    #[test]
    fn handles_empty_production() {
        let system = System::new();
        assert!(system.parse_production("").is_err());
    }

    #[test]
    fn handles_no_head() {
        let system = System::new();
        assert!(system.parse_production("-> surname surname").is_err());
    }

    #[test]
    fn handles_no_body() {
        let system = System::new();
        assert!(system.parse_production("Company ->").is_ok());
    }

    #[test]
    fn handles_correct() {
        let system = System::new();
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
        let system = System::new();
        system.parse_production("Company -> surname Company").expect("Unable to add production");
        let string = system.parse_prod_string("Company").expect("Unable to create string");
        let result = system.derive_once(string).unwrap();

        assert_eq!(result. len(), 2);
    }

    #[test]
    fn can_derive_multiple_times() {
        let system = System::new();
        system.parse_production("Company -> surname Company").expect("Unable to add production");
        let string = system.parse_prod_string("Company").expect("Unable to create string");
        let result = system.derive(string, RunSettings::for_max_iterations(2)).expect("Unable to derive");

        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_format() {
        let system = System::default();

        let token = system.add_token("a", TokenKind::Terminal).unwrap();
        assert_eq!(system.format(&token).unwrap(), "a");

        let string = system.parse_prod_string("a b c").unwrap();
        assert_eq!(system.format(&string).unwrap(), "a b c");
    }
    
    #[test]
    fn test_counting_tokens() {
        let system = System::default();
        assert_eq!(system.token_len(), 0);
        
        system.add_token("a", TokenKind::Terminal).unwrap();
        assert_eq!(system.token_len(), 1);
        assert_eq!(system.prod_tokens_len(), 0);
        assert_eq!(system.terminal_tokens_len(), 1);
        assert_eq!(system.production_len(), 0);

        // Nothing should change
        system.add_token("a", TokenKind::Terminal).unwrap();
        assert_eq!(system.token_len(), 1);

        system.add_token("b", TokenKind::Terminal).unwrap();
        assert_eq!(system.token_len(), 2);

        system.add_token("c", TokenKind::Production).unwrap();
        assert_eq!(system.token_len(), 3);
        assert_eq!(system.prod_tokens_len(), 1);
        assert_eq!(system.terminal_tokens_len(), 2);
        assert_eq!(system.production_len(), 0);
    }

    #[test]
    fn test_counting_productions() {
        let system = System::default();
        assert_eq!(system.token_len(), 0);
        assert_eq!(system.production_len(), 0);

        system.parse_production("F -> F F").unwrap();
        assert_eq!(system.token_len(), 1);
        assert_eq!(system.prod_tokens_len(), 1);
        assert_eq!(system.terminal_tokens_len(), 0);
        assert_eq!(system.production_len(), 1);
    }

}