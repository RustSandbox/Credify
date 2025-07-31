# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Ergonomic Rig framework helper functions: `rig_validate()`, `rig_is_valid()`, `rig_validate_text()`, `rig_validate_json()`
- `RigValidationResult` type for cleaner, simpler responses optimized for Rig tools
- ASYNC_GUIDE.md documentation for proper async usage
- rig_async_proper.rs example showing correct async implementation
- rig_ergonomic.rs example demonstrating the new ergonomic API

### Fixed
- Updated examples to use async functions in async contexts to prevent runtime panics
- Added warning in README about using async versions in async contexts

### Changed
- Completely revamped README with AI-first focus and comprehensive examples
- Enhanced crate-level documentation with API levels table
- Updated all documentation to highlight async usage importance

### Documentation
- Added API_REFERENCE.md with complete function reference
- Added MIGRATION_GUIDE.md for upgrading from previous versions
- Updated all examples to use async versions in async contexts

## [0.3.0] - 2025-07-31

### Added
- AI-optimized API with structured types (`AIValidationResult`, `AIDecision`, `ValidationMetadata`)
- `ai_validate()` and `ai_validate_async()` functions returning structured data for AI agents
- `ai_validate_json()` and `ai_validate_json_async()` for direct JSON output
- Confidence levels (0.0 to 1.0) for nuanced AI decision-making
- Decision enum (Accept/Retry/Reject) for clear agent actions
- Comprehensive tests for AI-optimized functions
- Rig framework integration example demonstrating AI tool implementation
- Support for LinkedIn usernames with dots (e.g., first.last)

### Changed
- Enhanced README with AI-optimized API documentation and Rig framework integration guide
- Improved username extraction to properly handle all valid LinkedIn username formats
- Updated to Rust 2024 edition

## [0.2.1] - 2025-07-31

### Fixed
- Repository URLs updated to correct GitHub organization (RustSandbox/Credify)
- GitHub Actions workflow to use correct secret name (CARGO_REGISTRY_TOKEN)
- Code formatting issues resolved with cargo fmt

### Changed
- Made CodeCov optional in CI to prevent build failures
- Improved AUTH_REQUIRED suggested actions for AI agents - now recommends accepting URLs as valid
- Updated detailed explanations to clarify that AUTH_REQUIRED likely means the profile exists

### Added
- New example `ai_agent.rs` demonstrating usage with AI agents for lead generation

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

[Unreleased]: https://github.com/RustSandbox/Credify/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/RustSandbox/Credify/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/RustSandbox/Credify/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/RustSandbox/Credify/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/RustSandbox/Credify/releases/tag/v0.1.0