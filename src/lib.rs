//! A crate for procedurally generating content using L-systems.
//! 
//! # Introduction
//! 
//! The supported [L-Systems][wiki] are _context-free_ and _stochastic_.  
//! 
//! The code repository is available on [GitHub](https://github.com/TheRiver/rusty-grammar/) and
//! is distributed under an [MIT license](https://github.com/TheRiver/rusty-grammar/blob/main/LICENSE).
//!
//! # Parsing
//!
//! The easiest way to parse
//!
//! ```
//! use rusty_grammar::system::System;
//! let mut system = System::new();
//!
//! system.parse_production("CompanyName -> Surname Surname").unwrap();
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
//! # Learn more
//!
//! If you would like to learn more about L-Systems, the original *Algorithmic Beauty of Plants*
//! book, by Prusinkiewicz and Lindenmayer, is [available for free, online][abop].
//! 
//! [wiki]: https://en.wikipedia.org/wiki/L-system
//! [abop]: http://algorithmicbotany.org/papers/#abop
//! [skia-plant]: https://github.com/TheRiver/rusty-grammar/blob/main/examples/skia-plant/main.rs
//! [logo-turtle]: https://en.wikipedia.org/wiki/Logo_(programming_language)
//! [tiny-skia]: https://github.com/RazrFalcon/tiny-skia

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
