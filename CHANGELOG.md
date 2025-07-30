# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-01-30

### Added
- Initial release of linkedin-profile-validator
- Format validation for LinkedIn profile URLs without network calls
- Full validation with network requests to check profile existence
- Both synchronous (blocking) and asynchronous APIs
- Comprehensive error handling for different failure scenarios:
  - Invalid URL format
  - Non-LinkedIn domains
  - Non-profile URLs (e.g., company pages)
  - Profile not found (404)
  - Authentication required (LinkedIn's anti-bot protection)
- Support for handling LinkedIn's rate limiting (status 999)
- Example usage in main.rs
- Comprehensive test suite

### Technical Details
- Built with reqwest for HTTP requests
- Uses regex for URL pattern matching
- Implements proper error types with thiserror
- Supports Rust 2021 edition (minimum Rust 1.70)