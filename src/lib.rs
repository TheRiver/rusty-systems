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
//! The [system module](system) contains the primary tools for defining
//! and running these grammars. As a convenience, the [`geometry`] module
//! contains types for easily handling the interpretation of the grammar's output
//! in a 2D space. Note that the geometry tools are not meant to be complete or high performance —
//! it's meant to only be utilitarian.
//!
//! # Parsing and Derivation
//!
//! The easiest way to parse:
//!
//! ```
//! use rusty_systems::prelude::*;
//! use rusty_systems::system::parser;
//!
//! let system = System::new();
//! system.parse_production("CompanyName -> Surname Surname").unwrap();
//!
//! let starting_axiom = parser::parse_prod_string("CompanyName").unwrap();
//! let result = system.derive(starting_axiom, RunSettings::default()).unwrap();
//!
//! println!("The resulting string is:\n{result}");
//!
//! ```
//!
//! If you would like to parse without using a [`System`] instance,
//! you can use the following underlying functions:
//!
//! * [`system::parser::parse_production`] to parse individual productions.
//! 
//! See [`system::parser`] for more information, and for generic parsing
//! functions that do not need you to use [`System`].
//!
//! # Features
//!
//! Some features that you might find useful:
//!
//! * **Some support for interpreting L-Systems as defined in [the Algorithmic Beauty of Plants (ABOP)][abop].**
//!   See the [abop module](interpretation::abop) documentation.
//! * **Some native, limited support for geometric primitives.**
//!   See the [geometry module](geometry). This is not meant as a replacement for libraries
//!   such as [nalgebra][nalgebra], just as something convenient to use.
//! * **Native support for creating and outputting SVGs.**
//!   See the [svg module](interpretation::svg).
//! * **A command line app, `lsystem`, for creating SVGs of systems from ABOP.**
//!   You can read about using this tool [here][lsystem-tool]
//!
//! # Examples
//!
//! The crate's example directory has various examples:
//!
//! * [Vector graphics plant][skia-plant]
//!
//!   This example uses two rules to produce a small plant. The symbols
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
//! book by Prusinkiewicz and Lindenmayer is [available for free, online][abop].
//!
//! # Code repository, license, and versioning.
//! 
//! This crate has a website available at 
//! [https://theriver.github.io/rusty-systems/][website].
//!
//! The code repository is hosted on [GitHub](https://github.com/TheRiver/rusty-systems/) and
//! is distributed under an [MIT license][license]. A [changelog][changelog] is also available.
//!
//! This crate versioning uses [semantic versioning][semver].
//!
//! [wiki]: https://en.wikipedia.org/wiki/L-system
//! [abop]: http://algorithmicbotany.org/papers/#abop
//! [skia-plant]: https://github.com/TheRiver/rusty-systems/blob/main/examples/skia-plant/main.rs
//! [logo-turtle]: https://en.wikipedia.org/wiki/Logo_(programming_language)
//! [tiny-skia]: https://github.com/RazrFalcon/tiny-skia
//! [semver]: https://semver.org/
//! [docs]: https://docs.rs/rusty-systems/latest/rusty_systems/
//! [license]: https://github.com/TheRiver/rusty-systems/blob/main/LICENSE
//! [changelog]: https://github.com/TheRiver/rusty-systems/blob/main/CHANGELOG.md
//! [website]: https://theriver.github.io/rusty-systems/
//! [nalgebra]: https://nalgebra.org/
//! [lsystem-tool]: https://theriver.github.io/rusty-systems/lsystem/

pub mod error;
pub mod symbols;
pub mod productions;
pub mod strings;
pub mod system;
pub mod geometry;
pub mod interpretation;


/// Some commonly used members of the crate re-exported for easy access. 
/// 
/// Use it like so:
/// 
/// ```rust
/// use rusty_systems::prelude::*;
/// ```
pub mod prelude {
    pub use super::error::Error;
    pub use super::strings::ProductionString;
    pub use super::symbols::Symbol;
    pub use super::system::System;
    pub use super::system::RunSettings;
    pub use super::system::family::SystemFamily;
    pub use crate::interpretation::Interpretation;
}

use prelude::*;

/// A result type for functions that can return errors.
pub type Result<T> = std::result::Result<T, Error>;
