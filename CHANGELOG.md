# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

* Using criterion for benchmarking support. 
* Added implements of `TryInto<Symbol>` and `TryInto<Production>` to ease parsing.
* Added an implementation of `FromStr` for `ProductionString`. 
* Added an implementation of `FromStr` for `Symbol`. 
* Added an implementation of `FromStr` for `Production`. 

### Changed

* `System::parse_production` has been removed, and now `System::add_production` is used instead.
* Better handling of `Infallible` by `Error`.
* The `derive` argument for the lsystem cli is now called `interpret`. 
* Updated documentation for the lsystem cli.

### Removed

## [5.0.0]

### Added

* `symbols::get_code` and related functions that return a symbol code 
  for a string, or the string / name associated with a code.
* `Symbol::name` returns the name of a symbol. 
* Added trait `SymbolIterable` for collections of `Symbol` instances

### Changed

* Display tokens, strings, productions, and so on, is now substantially easier. They all implement 
  Display.
* TokenKind has been removed. 
* Simplified how tokens are stored in a system. Only the code is now stored.
* The TokenStore implementation for HashMap has been changed to HashSet, and no longer needs arcs.
* The `Token` struct has been renamed `Symbol`.
* `parse_prod_string` moved to the parser namespace. 
* The `parser` module is now a top level module

### Removed
a
* DisplaySystem trait has been removed
 

## [4.0.0]

### Changed

* The `clap` and `ansi_term` dependencies are now optional, and not used by default.
* The `lsystem` binary now requires the `lsystem` feature. 

## [3.0.0]

### Added
* Productions are now context-sensitive. 

### Changed
* the --output flag in `lsystem` now has a default value of `out.svg`.
* Refactored crate::system::interpretation module to crate::interpretation 
* Documentation updates.

### Removed

## [2.0.0]

### Added
- Added ErrorKind::Locking to indicate problems acquiring synchronisation locks.
- Added ErrorKind::Io for passing on handling std::io::Error.
- Added the ability to define families of L-Systems that share common tokens.
- Path is now a fuller collection, with indexing and iterator support
- Minimal SVG support. 
- An interpretation that will save to SVG
- A command line lsystem tool

### Changed
- README updates.
- Using ErrorKind::Locking in more places
- Many documentation updates
- derivation functions now only return a Result, not a Result<Option>.

### Removed
- Edge removed from Geometry, since nothing was using it. 

## [1.0.0] - 2024-05-24

### Added

- More implementation of std::ops for points and vectors.
- Added ProductionStore and TokenStore implementations for RefCell wrappers of collections.
- Edge and Path types to Geometry
- Added a zero function to Point
- Added From implementations to convert between Point and Vector and vice versa. 

### Changed

- Updated README to include links to docs.rs, and some example code. 
- Documentation has generally been updated.
- System now uses a HashMap instead of a BTreeMap.
- parse_production now takes two arguments.
- skia-plant example has been tidied to show how to build
  an interpretation.

### Removed

- Removed the implementation of *AddAssign* for point. Point should be immutable.
- Removed the single argument version of parse_production.

## [0.3.0] - 2024-05-18

Initial release
