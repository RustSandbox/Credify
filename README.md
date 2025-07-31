<div align="center">
  <img src="credify_logo.png" alt="Credify Logo" width="200">
  
  # Credify
  
  [![Crates.io](https://img.shields.io/crates/v/credify.svg)](https://crates.io/crates/credify)
  [![Documentation](https://docs.rs/credify/badge.svg)](https://docs.rs/credify)
  [![CI](https://github.com/RustSandbox/Credify/workflows/CI/badge.svg)](https://github.com/RustSandbox/Credify/actions)
  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
  [![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
</div>

A robust Rust library for validating LinkedIn profile URLs with a focus on accuracy and data quality. Perfect for data validation pipelines, lead generation tools, and any application that needs to verify LinkedIn profile URLs.

**New in v0.2.0**: Enhanced LLM support with extremely verbose validation reports including timestamps, severity levels, detailed explanations, and comprehensive action suggestions.

## 🚀 Features

- **🔍 Format Validation** - Quickly check if a URL follows the LinkedIn profile pattern
- **✅ Existence Verification** - Verify that profiles actually exist (with intelligent fallback)
- **🤖 LLM-Friendly Output** - Structured validation reports designed for AI/LLM consumption
- **📝 Verbose Validation Reports** - Detailed explanations with timestamps, severity levels, and action suggestions
- **⚡ Async & Sync APIs** - Choose based on your application architecture
- **🛡️ No Panics** - Comprehensive error handling throughout
- **📊 Detailed Error Types** - Know exactly why validation failed
- **💡 Smart Suggestions** - Get 5-7 actionable suggestions for each validation scenario

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
credify = "0.2.1"
```

Or use cargo add:

```bash
cargo add credify
```

## 🔧 Usage

### Quick Format Check (No Network Calls)

Perfect for validating user input before making network requests:

```rust
use credify::is_valid_linkedin_profile_format;

fn main() {
    let urls = vec![
        "https://www.linkedin.com/in/johndoe",
        "https://linkedin.com/in/jane-smith",
        "https://www.linkedin.com/company/acme",  // Not a profile URL
        "https://google.com/in/someone",          // Wrong domain
    ];

    for url in urls {
        if is_valid_linkedin_profile_format(url) {
            println!("✅ {} - Valid format", url);
        } else {
            println!("❌ {} - Invalid format", url);
        }
    }
}
```

### Full Validation with Existence Check (Blocking)

Verify that a LinkedIn profile actually exists:

```rust
use credify::{LinkedInValidator, LinkedInUrlError};

fn main() {
    let validator = LinkedInValidator::new().expect("Failed to create validator");
    let url = "https://www.linkedin.com/in/johndoe";
    
    match validator.is_valid_linkedin_profile_url(url) {
        Ok(_) => println!("✅ Profile exists!"),
        Err(LinkedInUrlError::ProfileNotFound) => println!("❌ Profile not found"),
        Err(LinkedInUrlError::AuthenticationRequired) => {
            println!("⚠️  LinkedIn requires authentication - cannot verify")
        },
        Err(e) => println!("❌ Error: {}", e),
    }
}
```

### Async Validation

For async applications using Tokio:

```rust
use credify::validate_linkedin_url_async;

#[tokio::main]
async fn main() {
    let url = "https://www.linkedin.com/in/johndoe";
    
    match validate_linkedin_url_async(url).await {
        Ok(_) => println!("✅ Profile exists!"),
        Err(e) => println!("❌ Invalid: {}", e),
    }
}
```

### LLM-Friendly Validation Functions (New in v0.2.0!)

The library provides dedicated functions that return extremely verbose, structured validation reports perfect for LLM/AI agent consumption:

```rust
use credify::{validate_for_llm, validate_for_llm_async};

// Synchronous validation
let result = validate_for_llm("https://www.linkedin.com/in/johndoe");
println!("{}", result);
// Output:
// === LINKEDIN PROFILE VALIDATION REPORT ===
//
// TIMESTAMP: 2025-07-31T13:10:54.691091+00:00
// INPUT_URL: https://www.linkedin.com/in/johndoe
//
// VALIDATION_RESULT: SUCCESS
// VALIDATION_STATUS: PASSED
// PROFILE_EXISTS: TRUE
// URL_FORMAT: VALID
// DOMAIN_VERIFIED: TRUE
// PROFILE_ACCESSIBLE: TRUE
// LINKEDIN_USERNAME: johndoe
//
// DETAILED_EXPLANATION:
// The provided URL has been successfully validated. The LinkedIn profile exists and is accessible. 
// The URL follows the correct LinkedIn profile format and the domain has been verified as authentic. 
// The profile page returned a successful response, confirming the profile is active and publicly viewable.
//
// SUGGESTED_ACTIONS:
// 1. Proceed with profile data extraction using LinkedIn API or web scraping tools
// 2. Cache this validation result to avoid repeated network requests
// 3. Store the profile URL in your database as a verified LinkedIn profile
// 4. Consider extracting additional profile metadata (name, headline, etc.)
// 5. Set up monitoring to periodically re-validate the profile existence
//
// RECOMMENDED_NEXT_STEP: Extract profile data using appropriate LinkedIn data extraction methods
//
// === END OF VALIDATION REPORT ===

// Asynchronous validation
let result = validate_for_llm_async("https://www.linkedin.com/in/johndoe").await;
// Returns identical verbose format
```

The enhanced validation reports include:
- **Header with Timestamp**: ISO 8601 timestamp for tracking
- **Comprehensive Status Fields**: Multiple status indicators for different aspects
- **Severity Levels**: ERROR_SEVERITY field for prioritization
- **Detailed Explanations**: 2-3 sentences explaining what happened and why
- **Multiple Suggested Actions**: 5-7 specific, actionable suggestions per scenario
- **Recommended Next Steps**: Clear guidance on the immediate next action
- **Additional Metadata**: HTTP status codes, actual domains, LinkedIn-specific responses

## 🤖 Why LLM-Friendly Validation Matters

In the age of AI agents and autonomous systems, validation tools need to provide more than just boolean results. The `validate_for_llm` functions are specifically designed to empower LLM agents with:

### 1. **Context-Rich Decision Making**
Traditional validation returns simple true/false or error codes. LLM agents need context to understand:
- **Why** validation failed (detailed explanations)
- **What** exactly went wrong (specific error types with severity)
- **When** the validation occurred (timestamps for tracking)
- **How** to fix the issue (multiple suggested actions)

### 2. **Actionable Intelligence**
Each validation report includes 5-7 suggested actions, enabling LLM agents to:
- Automatically retry with different strategies
- Provide meaningful feedback to users
- Make intelligent decisions about next steps
- Handle edge cases appropriately

### 3. **Planning Support**
The verbose output helps LLM agents in their planning phase by providing:
- **Severity levels** to prioritize issues
- **Recommended next steps** for immediate action
- **Alternative approaches** when primary validation fails
- **Detailed metadata** for debugging and logging

### Example: How LLMs Benefit

```rust
let result = validate_for_llm("https://linkedin.com/in/johndoe");

// An LLM can parse this structured output to:
// 1. Understand if the profile exists
// 2. Get specific error details if it doesn't
// 3. Receive multiple suggestions for resolution
// 4. Make informed decisions about retries or alternatives
// 5. Provide helpful feedback to end users
```

### 🚀 AI-Optimized API (New in v0.2.1!)

For even deeper AI agent integration, Credify now offers structured data types specifically designed for AI decision-making:

```rust
use credify::{ai_validate, ai_validate_json, AIDecision};

// Get structured validation result
let result = ai_validate("https://linkedin.com/in/johndoe");

// Simple boolean for quick decisions
if result.is_valid {
    println!("Valid profile!");
}

// Confidence level (0.0 to 1.0) for nuanced decisions
if result.confidence >= 0.8 {
    println!("High confidence: {}", result.confidence);
}

// AI-friendly decision enum
match result.decision {
    AIDecision::Accept => {
        // Use the profile URL
        println!("Profile accepted: {:?}", result.username);
    }
    AIDecision::Retry => {
        // Network issue - try again later
        println!("Temporary issue, retry suggested");
    }
    AIDecision::Reject => {
        // Invalid URL - search for another
        println!("Invalid URL: {}", result.reason);
    }
}

// Get JSON for direct AI consumption
let json = ai_validate_json("https://linkedin.com/in/johndoe");
// Returns pretty-printed JSON with all fields
```

#### Integration with Rig Framework

Credify is optimized for use with the [Rig](https://github.com/0xPlaygrounds/rig) Rust framework for building AI agents:

```rust
// Define the tool for Rig
#[derive(Deserialize, Serialize)]
struct LinkedInValidator;

impl Tool for LinkedInValidator {
    const NAME: &'static str = "LinkedInValidator";
    
    async fn call(&self, args: ValidateLinkedInArgs) -> Result<String, Error> {
        // Use the AI-optimized JSON function
        Ok(credify::ai_validate_json(&args.url))
    }
}

// System prompt for LinkedIn lead generation
const SYSTEM_PROMPT: &str = r#"
When validating LinkedIn profiles:
1. Use web search to find LinkedIn URLs - don't make them up
2. ALWAYS validate URLs with LinkedInValidator before using them
3. Interpret results:
   - is_valid: true + confidence >= 0.9 = Definitely real profile
   - decision: Accept = Use this URL
   - decision: Retry = Try again in a few seconds
   - decision: Reject = Search for a different URL
4. AUTH_REQUIRED usually means the profile EXISTS
"#;
```

Common LLM agent scenarios:
- **Lead Generation**: Validate profiles before adding to CRM
- **Data Enrichment**: Verify profiles before fetching additional data
- **Automation Workflows**: Make decisions based on profile validity
- **Error Recovery**: Intelligently handle validation failures

## 📊 Error Types

The library provides detailed, LLM-friendly error messages:

| Error Type | Description | Example Message |
|------------|-------------|-----------------|
| `InvalidUrl` | URL parsing failed | `[INVALID_URL_FORMAT] The provided URL is not a valid URL: {details}` |
| `NotLinkedInUrl` | Wrong domain | `[NOT_LINKEDIN_DOMAIN] The URL is not from linkedin.com or www.linkedin.com domain` |
| `NotProfileUrl` | Not a profile page | `[NOT_PROFILE_URL] The URL is not a LinkedIn profile URL (expected format: /in/username)` |
| `ProfileNotFound` | Profile doesn't exist | `[PROFILE_NOT_FOUND] The LinkedIn profile does not exist (404)` |
| `AuthenticationRequired` | LinkedIn auth wall | `[AUTH_REQUIRED] LinkedIn requires authentication to verify this profile` |
| `NetworkError` | Network issues | `[NETWORK_ERROR] Failed to connect to LinkedIn: {details}` |
| `ClientBuildError` | HTTP client error | `[CLIENT_BUILD_ERROR] Failed to create HTTP client: {details}` |

## ⚠️ Important Notes

### LinkedIn's Anti-Bot Protection

LinkedIn actively prevents automated profile checking and may:
- Return status code 999 for suspected bot traffic
- Show an authentication wall (authwall)
- Rate limit requests from the same IP

When encountering these protections, the library returns an `AuthenticationRequired` error. This doesn't mean the profile doesn't exist - it means LinkedIn is preventing automated verification.

### Best Practices

1. **Rate Limiting**: Add delays between requests (2-5 seconds recommended)
2. **Batch Processing**: Process URLs in batches with delays
3. **Format-First**: Use format validation when existence checking isn't critical
4. **Error Handling**: Always handle the `AuthenticationRequired` case gracefully

## 📚 Examples

The library includes several examples demonstrating different use cases:

```bash
# Basic usage example
cargo run --example basic

# Batch validation with rate limiting
cargo run --example batch_validation

# LLM-friendly output format
cargo run --example llm_friendly

# Simple LLM validation example
cargo run --example llm_simple
```

## 🛠️ Development

```bash
# Run tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Check code quality
cargo clippy -- -D warnings

# Format code
cargo fmt

# Build documentation
cargo doc --open

# Run all checks before committing
cargo check && cargo test && cargo clippy && cargo fmt
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to:
1. Update tests as appropriate
2. Run `cargo fmt` and `cargo clippy`
3. Update documentation for new features
4. Add examples if applicable

## 📄 License

This project is licensed under either of:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

## 🙏 Acknowledgments

Built with:
- [reqwest](https://github.com/seanmonstar/reqwest) - HTTP client
- [url](https://github.com/servo/rust-url) - URL parsing
- [regex](https://github.com/rust-lang/regex) - Pattern matching
- [tokio](https://github.com/tokio-rs/tokio) - Async runtime

---

Made with ❤️ by [Hamze Ghalebi](https://github.com/hghalebi)