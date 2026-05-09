# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.1](https://github.com/Quantumlyy/osdp-rs/compare/v0.3.0...v0.3.1) - 2026-05-09

## [0.3.0](https://github.com/Quantumlyy/osdp-rs/compare/v0.2.1...v0.3.0) - 2026-05-05

### Added

- *(secure)* Zeroize Session and SessionKeys on drop
- *(driver)* Enforce SQN echo per spec table 2

### Changed

- Replace stringly-typed length errors with typed variants
- *(tests)* Share helpers via tests/common, add VecTransport::shuffle_to
- *(driver)* Dedupe ACU receive loop, name magic numbers
- *(secure)* Dedupe Session<S> state-transition boilerplate

### Documentation

- Drop Session:: qualifier from handshake state diagram
- Add runnable doctests for the high-traffic public APIs
- *(readme)* Add architecture + handshake diagrams; refresh stale stats
- Add mermaid diagrams for the four core state machines
- Wire up aquamarine + docs.rs metadata

### Testing

- *(reply)* Add unit tests for every reply body
- *(command)* Add unit tests for every command body

## [0.2.1](https://github.com/Quantumlyy/osdp-rs/compare/v0.2.0...v0.2.1) - 2026-05-04

### Changed

- *(cipher)* Construct complement ICV functionally

### Testing

- *(cipher)* Derive test keys/IVs via computed fixture

## [0.2.0](https://github.com/Quantumlyy/osdp-rs/compare/v0.1.22...v0.2.0) - 2026-05-04

### Added

- Add ascii utils

### Documentation

- Docs gen

### Testing

- Property + integration coverage; rewrite README

### V0.2.0

- Rewrite for OSDP v2.2 compliance
