//! A crate for procedurally generating content using L-systems.
//!
//! **NOTE:** this crate is still under early development, and might change rapidly.
//! The next released version will likely have breaking changes.
//!
//! # Introduction
//!
//! This crate currently supports producing strings using *context-free* and
//! *stochastic* [L-Systems][wiki].
//!
//! # Parsing and Derivation
//!
//! The easiest way to parse:
//!
//! ```
//! use rusty_grammar::prelude::ProductionString;
//! use rusty_grammar::system::{RunSettings, System};
//! let mut system = System::new();
//!
//! system.parse_production("CompanyName -> Surname Surname").unwrap();
//! let starting_axiom = system.parse_prod_string("CompanyName").unwrap();
//! let result = system.derive(starting_axiom, RunSettings::default()).unwrap().unwrap();
//!
//! println!("The resulting string is:\n{}", system.format(&result).unwrap());
//!
//! ```
//!
//! If you would like to parse without using a [`System`] instance,
//! you can use the following underlying functions:
//!
//! * [`system::parser::parse_production`] to parse individual productions.
//! 
//! See [`system::parser`] for more information.
//!
//! # Examples
//!
//! The crate's example directory has various examples:
//!
//! * [Vector graphics plant][skia-plant]
//!
//!   This example uses two rules to produce a small plant. The tokens
//!   are interpreted using a classic [logo turtle][logo-turtle] interpretation
//!   to produce vector graphics. While the example uses [tiny skia][tiny-skia],
//!   this can be replaced with any vector graphic library.
//!
//!   If you clone the code repository, you can run this using:
//!
//!   ```cargo run --example skia-plant ```
//!
//! # Learn more about L-Systems
//!
//! If you would like to learn more about L-Systems, the original *Algorithmic Beauty of Plants*
//! book, by Prusinkiewicz and Lindenmayer, is [available for free, online][abop].
//!
//! # Code repository, license, and versioning.
//!
//! The code repository is available on [GitHub](https://github.com/TheRiver/rusty-grammar/) and
//! is distributed under an [MIT license](https://github.com/TheRiver/rusty-grammar/blob/main/LICENSE).
//!
//! This crate versioning uses [semantic versioning][semver].
//!
//! [wiki]: https://en.wikipedia.org/wiki/L-system
//! [abop]: http://algorithmicbotany.org/papers/#abop
//! [skia-plant]: https://github.com/TheRiver/rusty-grammar/blob/main/examples/skia-plant/main.rs
//! [logo-turtle]: https://en.wikipedia.org/wiki/Logo_(programming_language)
//! [tiny-skia]: https://github.com/RazrFalcon/tiny-skia
//! [semver]: https://semver.org/

pub mod error;
pub mod tokens;
pub mod productions;
pub mod strings;
pub mod system;
pub mod geometry;

pub mod prelude {
    pub use super::error::Error;
    pub use super::strings::ProductionString;
    pub use super::tokens::Token;
    pub use super::system::System;
}

use std::collections::HashMap;
use prelude::*;

/// A result type for functions that can return errors.
pub type Result<T> = std::result::Result<T, Error>;


pub trait DisplaySystem {
    fn format(&self, names: &HashMap<Token, String>) -> Result<String>;
}
