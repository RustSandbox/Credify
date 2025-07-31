# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.0] - 2025-07-31

### Added
- `validate_for_llm()` function that returns structured string output for LLM consumption
- `validate_for_llm_async()` async version of the LLM validation function
- New example `llm_simple.rs` demonstrating the LLM validation functions
- Chrono dependency for timestamp generation in validation reports

### Changed
- Enhanced `validate_for_llm()` and `validate_for_llm_async()` output to be extremely verbose with:
  - Timestamp and detailed headers
  - Severity levels for each error type
  - Detailed explanations (2-3 sentences per scenario)
  - 5-7 suggested actions for each error type
  - Recommended next steps
  - Additional metadata like HTTP status codes and LinkedIn response types
- Improved documentation with comprehensive examples of the new verbose output format

## [0.1.0] - 2025-07-31

### Added
- Initial release of Credify (formerly linkedin-profile-validator)
- LinkedIn profile URL format validation
- LinkedIn profile existence verification
- Synchronous API with `LinkedInValidator`
- Asynchronous API with `validate_linkedin_url_async`
- LLM-friendly structured error messages
- Comprehensive error handling with no panics
- Examples for basic usage, batch validation, and LLM integration
- Full test coverage
- Documentation and examples

### Changed
- Renamed project from `linkedin-profile-validator` to `credify`
- Refactored error handling to never panic
- Enhanced error messages with structured format for LLM agents
- Updated `LinkedInValidator::new()` to return `Result` type

### Security
- No sensitive data is logged or exposed
- Safe handling of all network errors
- Proper timeout configuration for HTTP requests

[Unreleased]: https://github.com/RustSandbox/Credify/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/RustSandbox/Credify/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/RustSandbox/Credify/releases/tag/v0.1.0