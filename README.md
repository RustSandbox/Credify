# LinkedIn Profile Validator

[![Crates.io](https://img.shields.io/crates/v/linkedin-profile-validator.svg)](https://crates.io/crates/linkedin-profile-validator)
[![Documentation](https://docs.rs/linkedin-profile-validator/badge.svg)](https://docs.rs/linkedin-profile-validator)
[![CI](https://github.com/RustSandbox/linkedin_profile_validator/workflows/CI/badge.svg)](https://github.com/RustSandbox/linkedin_profile_validator/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

A robust Rust library to validate LinkedIn profile URLs, checking both format and profile existence.

## Features

- **Format validation** - Check if a URL follows LinkedIn profile URL pattern
- **Existence validation** - Verify the profile actually exists (doesn't redirect to 404)
- **Blocking and async APIs** - Choose based on your application needs
- **Detailed error types** - Know exactly why validation failed
- **Fast and reliable** - Built with reqwest and proper error handling

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
linkedin-profile-validator = "0.1.0"
```

Or use cargo add:

```bash
cargo add linkedin-profile-validator
```

## Usage

### Quick format check (no network calls)

```rust
use linkedin_profile_validator::is_valid_linkedin_profile_format;

fn main() {
    let url = "https://www.linkedin.com/in/johndoe";
    if is_valid_linkedin_profile_format(url) {
        println!("Valid LinkedIn profile URL format!");
    }
}
```

### Full validation with network check (blocking)

```rust
use linkedin_profile_validator::LinkedInValidator;

fn main() {
    let validator = LinkedInValidator::new();
    let url = "https://www.linkedin.com/in/johndoe";
    
    match validator.is_valid_linkedin_profile_url(url) {
        Ok(_) => println!("Profile exists!"),
        Err(e) => println!("Invalid: {}", e),
    }
}
```

### Async validation

```rust
use linkedin_profile_validator::validate_linkedin_url_async;

#[tokio::main]
async fn main() {
    let url = "https://www.linkedin.com/in/johndoe";
    
    match validate_linkedin_url_async(url).await {
        Ok(_) => println!("Profile exists!"),
        Err(e) => println!("Invalid: {}", e),
    }
}
```

## Error Types

The library provides detailed error information:

- `InvalidUrl` - The URL format is invalid
- `NotLinkedInUrl` - The URL is not from linkedin.com domain
- `NotProfileUrl` - The URL is not a profile URL (e.g., it's a company page)
- `ProfileNotFound` - The profile doesn't exist (redirects to 404)
- `NetworkError` - Network request failed
- `AuthenticationRequired` - LinkedIn requires authentication to verify the profile

## Important Notes

### LinkedIn's Anti-Bot Protection

LinkedIn actively prevents automated profile checking and may:
- Return status code 999 for suspected bot traffic
- Redirect to an authentication wall
- Rate limit requests

When the library encounters these protections, it returns an `AuthenticationRequired` error. This doesn't necessarily mean the profile doesn't exist, just that LinkedIn is preventing automated verification.

### Rate Limiting

To avoid being blocked by LinkedIn:
- Don't make too many requests in a short time period
- Consider adding delays between requests in your application
- Use the format validation (`is_valid_linkedin_profile_format`) when you don't need to verify existence

## Valid URL Patterns

Valid LinkedIn profile URLs follow this pattern:
- `https://www.linkedin.com/in/username`
- `https://linkedin.com/in/username`
- `https://www.linkedin.com/in/user-name-123/`

## Development

```bash
# Run tests
cargo test

# Check formatting and linting
cargo fmt -- --check
cargo clippy -- -D warnings

# Run the example
cargo run --example basic

# Build documentation
cargo doc --open
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate. See [CONTRIBUTING.md](CONTRIBUTING.md) for more details.

## License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions. 