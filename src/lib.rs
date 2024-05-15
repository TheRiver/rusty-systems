//! A crate for procedurally generating content using L-systems.
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
//! The code repository is available on [GitHub](https://github.com/TheRiver/rusty-grammar/) and
//! is distributed under an [MIT license](https://github.com/TheRiver/rusty-grammar/blob/main/LICENSE).

pub mod error;
pub mod tokens;
pub mod productions;
pub mod strings;
pub mod system;

pub mod prelude {
    pub use super::error::Error;
    pub use super::strings::ProductionString;
    pub use super::tokens::Token;
    pub use super::system::System;
}

use prelude::*;

/// A result type for functions that can return errors.
pub type Result<T> = std::result::Result<T, Error>;


