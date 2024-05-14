//! A crate for procedurally generating content using L-systems.
//!
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


