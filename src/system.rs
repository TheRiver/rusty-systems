//! Collection of tools for defining a group of [`Production`] rules on strings
//! of symbols.
//!
//! # Creating systems
//!
//! ## Using [`System`]
//!
//! * TODO Talk about [`System::new`] and [`System::default`].
//! * TODO Talk about [`System::of_family`].
//! * TODO define a system
//!
//! A production that produces the empty string can be represented as so:
//!
//! ```
//! # use rusty_systems::system::System;
//! # let system = System::default();
//! let production = system.parse_production("X -> ").unwrap();
//! ```
//! 
//! Context sensitive productions.
//! todo More detail here
//!
//! ```
//! # use rusty_systems::system::System;
//! # let system = System::default();
//! let production = system.parse_production("G < S -> S G").unwrap();
//! let string = system.parse_prod_string("S G S").unwrap();
//! assert!(!production.matches(&string, 0));       // Does not match the first S
//! assert!( production.matches(&string, 2));       // Matches the last S
//! ``` 
//! 
//! * todo discuss stochastic rules
//!
//! ## Collections of [`Symbol`] and [`Production`]
//!
//! TODO talk about collections of symbols and productions.
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
//! * [`Symbol`]
//! * [`Production`]
//! * [`ProductionString`]
//! * [`SystemFamily`]

use std::collections::{HashSet};
use std::ops::Deref;
use std::sync::{RwLock};

use crate::error::{Error, ErrorKind};
use crate::prelude::*;
use crate::productions::{Production, ProductionStore};
use crate::system::family::TryIntoFamily;
use crate::symbols::{get_code, SymbolStore};

use super::{Result, symbols};

pub mod parser;
pub mod family;

/// Represents an L-system. This is the base for running the
/// production rules.
///
/// This struct is convenient for defining and storing symbols
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
    symbols: RwLock<HashSet<u32>>,
    productions: RwLock<Vec<Production>>
}

impl System {
    pub fn new() -> Self {
        System {
            symbols: RwLock::new(HashSet::new()),
            productions: RwLock::new(Vec::new())
        }
    }

    /// Given a previously defined family, this returns a new system
    /// using the defined symbols / alphabet / words of that family of systems.
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

        for symbol in family.symbols() {
            system.add_symbol(symbol.name.as_str())?;
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
            result.push_symbol(self.add_symbol(term)?);
        }

        Ok(result)
    }
    
    /// Returns the number of production rules in the system.
    pub fn production_len(&self) -> usize {
        self.productions.read().unwrap().len()
    }

    /// Returns the number of symbols registered with the system
    pub fn symbol_len(&self) -> usize {
        self.symbols.read().unwrap().len()
    }
}

impl SymbolStore for System {
    fn add_symbol(&self, name: &str) -> Result<Symbol> {
        let code = symbols::get_code(name)?;
        
        let mut map = self.symbols.write()?;
        map.insert(code);
        
        Ok(Symbol::new(code))
    }
    
    /// Return the symbol that represents the given term, if it exists.
    ///
    /// Note that this does not create any new symbols to the system.
    fn get_symbol(&self, name: &str) -> Option<Symbol> {
        let code = get_code(name).ok()?;
        let symbols = self.symbols.read().ok()?;
        
        symbols.get(&code)
            .cloned()
            .map(Symbol::new)
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

    for (index, symbol) in string.symbols().iter().enumerate() {
        if let Some(production) = find_matching(productions, &string, index) {
            let body = production.body()?;

            // println!("body match: {index}: {:?}", body.string().iter().map(|t| t.code()).collect::<Vec<_>>());

            body.string()
                .symbols()
                .iter()
                .cloned()
                .for_each(|symbol| result.push_symbol(symbol));
            continue;
        } else {
            result.push_symbol(*symbol);
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
        let symbol1 = system.add_symbol("one").unwrap();
        let symbol2 = system.add_symbol("two").unwrap();

        assert_ne!(symbol1.code(), symbol2.code());
    }

    #[test]
    fn no_increments_for_equal_names() {
        let system = System::new();
        let symbol1 = system.add_symbol("one").unwrap();
        let symbol2 = system.add_symbol("one").unwrap();

        assert_eq!(symbol1.code(), symbol2.code());
    }

    #[test]
    fn sync_and_send() {
        let mut symbol1 : Option<Symbol> = None;
        let mut symbol2: Option<Symbol> = None;

        let system = System::new();

        thread::scope(|s| {
            s.spawn(|| {
                symbol1 = Some(system.add_symbol("one").unwrap());
            });

            s.spawn(|| {
                symbol2 = Some(system.add_symbol("two").unwrap());
            });
        });

        let symbol1 = symbol1.unwrap();
        let symbol2 = symbol2.unwrap();

        assert_ne!(symbol1.code(), symbol2.code());
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

        let symbol = system.add_symbol("a").unwrap();
        assert_eq!(symbol.to_string(), "a");

        let string = system.parse_prod_string("a b c").unwrap();
        assert_eq!(string.to_string(), "a b c");
    }
    
    #[test]
    fn test_counting_symbols() {
        let system = System::default();
        assert_eq!(system.symbol_len(), 0);
        
        system.add_symbol("a").unwrap();
        assert_eq!(system.symbol_len(), 1);
        assert_eq!(system.production_len(), 0);

        // Nothing should change
        system.add_symbol("a").unwrap();
        assert_eq!(system.symbol_len(), 1);

        system.add_symbol("b").unwrap();
        assert_eq!(system.symbol_len(), 2);

        system.add_symbol("c").unwrap();
        assert_eq!(system.symbol_len(), 3);
        assert_eq!(system.production_len(), 0);
    }

    #[test]
    fn test_counting_productions() {
        let system = System::default();
        assert_eq!(system.symbol_len(), 0);
        assert_eq!(system.production_len(), 0);

        system.parse_production("F -> F F").unwrap();
        assert_eq!(system.symbol_len(), 1);
        assert_eq!(system.production_len(), 1);
    }


    #[test]
    fn testing_context_sensitive() {
        let system = System::default();
        let string = system.parse_prod_string("G S S S X").unwrap();
        system.parse_production("G > S -> ").unwrap();
        system.parse_production("G < S -> S G").unwrap();
        let string = system.derive_once(string).unwrap();

        assert_eq!(string, system.parse_prod_string("S G S S X").unwrap());

        let string = system.derive_once(string).unwrap();
        assert_eq!(string, system.parse_prod_string("S S G S X").unwrap());

        let string = system.derive_once(string).unwrap();
        assert_eq!(string, system.parse_prod_string("S S S G X").unwrap());
    }

}