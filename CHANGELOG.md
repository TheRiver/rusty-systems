# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- More implementation of std::ops in the geometry points and vectors. 

### Changed

- Updated README to include links to docs.rs, and some example code. 
- Documentation has generally been updated.
- System now uses a HashMap instead of a BTreeMap.

### Removed

- Removed the implementation of *AddAssign* for point. Point should be immutable.

## [0.3.0] - 2024-05-18

Initial release
