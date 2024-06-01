# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Added ErrorKind::Locking to indicate problems acquiring synchronisation locks.
- Added the ability to define families of L-Systems that share common tokens.
- Path is now a fuller collection, with indexing and iterator support
- Minimal SVG support. 
- An interpretation that will save to SVG
- A command line lsystem tool

### Changed
- README updates.
- Using ErrorKind::Locking in more places
- Many documentation updates

### Removed

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
