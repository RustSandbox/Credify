<div align="center">
  <img src="credify_logo.png" alt="Credify Logo" width="200">
  
  # Credify
  
  [![Crates.io](https://img.shields.io/crates/v/credify.svg)](https://crates.io/crates/credify)
  [![Documentation](https://docs.rs/credify/badge.svg)](https://docs.rs/credify)
  [![CI](https://github.com/RustSandbox/Credify/workflows/CI/badge.svg)](https://github.com/RustSandbox/Credify/actions)
  [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
  [![License: Apache 2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
</div>

A robust Rust library for validating LinkedIn profile URLs with AI-first design. Built for the era of AI agents and LLMs, Credify provides both traditional validation APIs and specialized functions optimized for AI tool calling, especially with frameworks like [Rig](https://github.com/0xPlaygrounds/rig).

**🎯 New in v0.3.0**: Ergonomic Rig framework integration with ultra-simple async helpers that prevent runtime panics and provide clean, structured responses perfect for AI agents.

## 🌟 Key Features

- **🤖 AI-First Design** - Multiple API levels from simple booleans to rich structured data
- **🎯 Rig Framework Optimized** - Ergonomic helpers designed specifically for Rig tools
- **⚡ Async & Sync APIs** - Full async support to prevent blocking runtime panics
- **📊 Structured Responses** - `AIValidationResult` with confidence scores and decisions
- **🔍 Smart Validation** - Format checking, existence verification, and intelligent fallbacks
- **📝 Rich Error Context** - Detailed explanations with actionable suggestions
- **🛡️ Never Panics** - Comprehensive error handling throughout
- **🚀 High Performance** - Optimized for concurrent operations

## 📦 Installation

```toml
[dependencies]
credify = "0.3.0"
```

Or use cargo add:

```bash
cargo add credify
```

## 🚀 Quick Start

### For AI Agents & Rig Framework (Recommended)

```rust
use credify::{rig_is_valid, rig_validate_text};

// Ultra-simple validation
if rig_is_valid("https://linkedin.com/in/johndoe").await {
    println!("Valid LinkedIn profile!");
}

// Get a human-readable response
let message = rig_validate_text("https://linkedin.com/in/johndoe").await;
// Returns: "✅ Valid profile @johndoe (95% confidence)"
```

### For Rig Tool Implementation

```rust
impl Tool for LinkedInValidator {
    async fn call(&self, args: Args) -> Result<String, Error> {
        // Just one line! No runtime panics, perfect for Rig
        Ok(credify::rig_validate_json(&args.url).await)
    }
}
```

## 📚 API Overview

### 🎯 Ergonomic Rig Helpers (New!)

| Function | Returns | Use Case |
|----------|---------|----------|
| `rig_is_valid()` | `bool` | Quick true/false checks |
| `rig_validate_text()` | `String` | One-line human-readable responses |
| `rig_validate_json()` | `String` | Clean JSON for tool responses |
| `rig_validate()` | `RigValidationResult` | Structured data with all details |

### 🤖 AI-Optimized Functions

| Function | Returns | Use Case |
|----------|---------|----------|
| `ai_validate()` | `AIValidationResult` | Full structured data |
| `ai_validate_json()` | `String` | JSON for AI consumption |
| `validate_for_llm()` | `String` | Verbose text reports |

### 🔧 Traditional API

| Function | Returns | Use Case |
|----------|---------|----------|
| `is_valid_linkedin_profile_format()` | `bool` | Format checking only |
| `LinkedInValidator::is_valid_linkedin_profile_url()` | `Result<bool>` | Full validation |

## 💡 Usage Examples

### 1. Rig Framework Integration (Recommended)

```rust
use credify::{rig_validate, RigValidationResult};
use rig::tool::Tool;

#[derive(Deserialize, Serialize)]
struct LinkedInChecker;

impl Tool for LinkedInChecker {
    const NAME: &'static str = "linkedin_checker";
    type Args = CheckArgs;
    type Output = String;
    type Error = MyError;

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // One line - that's it!
        Ok(credify::rig_validate_json(&args.url).await)
    }
}

// Or use structured data
async fn check_with_details(url: &str) {
    let result: RigValidationResult = credify::rig_validate(url).await;
    
    println!("Valid: {}", result.valid);
    println!("Status: {}", result.status);
    println!("Action: {}", result.action);
    println!("Confidence: {}%", result.confidence);
    
    if let Some(username) = result.username {
        println!("Username: @{}", username);
    }
}
```

### 2. AI Agent Integration

```rust
use credify::{ai_validate, AIDecision};

async fn validate_for_ai(url: &str) {
    let result = ai_validate(url).await;
    
    // Simple boolean check
    if result.is_valid {
        println!("Profile is valid!");
    }
    
    // Use confidence for nuanced decisions
    if result.confidence >= 0.9 {
        println!("High confidence validation");
    }
    
    // AI-friendly decision enum
    match result.decision {
        AIDecision::Accept => {
            // Use the profile
            println!("Accepted: {}", result.username.unwrap_or_default());
        }
        AIDecision::Retry => {
            // Network issue, try again
            println!("Temporary issue, retry in a moment");
        }
        AIDecision::Reject => {
            // Invalid URL
            println!("Invalid: {}", result.reason);
        }
    }
}
```

### 3. Quick Validation

```rust
use credify::is_valid_linkedin_profile_format;

// Format check only (no network calls)
if is_valid_linkedin_profile_format("https://linkedin.com/in/johndoe") {
    println!("Format is valid!");
}

// Full validation with network check
use credify::LinkedInValidator;

let validator = LinkedInValidator::new()?;
match validator.is_valid_linkedin_profile_url(url) {
    Ok(true) => println!("Profile exists!"),
    Ok(false) => println!("Profile not found"),
    Err(e) => println!("Error: {}", e),
}
```

### 4. Async Operations

```rust
use credify::validate_linkedin_url_async;

// Async validation
let is_valid = validate_linkedin_url_async(url).await?;

// Async with AI response
let json = credify::ai_validate_json_async(url).await;
```

## ⚠️ Important: Async Usage

When using Credify in async contexts (like web servers or AI frameworks), **always use the async versions** to avoid runtime panics:

```rust
// ❌ WRONG - Can cause panic in async context
async fn my_tool() {
    let result = credify::ai_validate_json(url); // Panic!
}

// ✅ CORRECT - Use async version
async fn my_tool() {
    let result = credify::ai_validate_json_async(url).await; // Works!
}

// ✅ BEST - Use Rig helpers (always async)
async fn my_tool() {
    let result = credify::rig_validate_json(url).await; // Perfect!
}
```

## 📊 Response Types

### RigValidationResult

```rust
pub struct RigValidationResult {
    pub valid: bool,              // Simple pass/fail
    pub username: Option<String>, // LinkedIn username if found
    pub confidence: u8,           // 0-100 percentage
    pub status: String,           // Human-readable status
    pub action: String,           // Suggested action for AI
}
```

### AIValidationResult

```rust
pub struct AIValidationResult {
    pub is_valid: bool,
    pub confidence: f32,          // 0.0 to 1.0
    pub decision: AIDecision,     // Accept/Retry/Reject
    pub username: Option<String>,
    pub reason: String,
    pub metadata: ValidationMetadata,
}
```

## 🤖 Why AI-Friendly Validation Matters

Traditional validation returns simple true/false or error codes. AI agents need rich context to make intelligent decisions:

- **Context-Rich Responses**: Understand why validation failed
- **Confidence Scores**: Make nuanced decisions based on certainty
- **Actionable Suggestions**: Know what to do next
- **Structured Data**: Easy to parse and reason about

## 🛠️ Advanced Features

### Custom User Agent

```rust
let validator = LinkedInValidator::new_with_user_agent(
    "MyBot/1.0 (https://mybot.com)"
)?;
```

### Handling LinkedIn Authentication

LinkedIn often returns AUTH_REQUIRED (999 status) for valid profiles. Credify intelligently handles this:

```rust
// AUTH_REQUIRED is treated as a valid profile
let result = rig_validate(url).await;
if result.valid && result.status.contains("auth required") {
    println!("Profile likely exists but LinkedIn is blocking checks");
}
```

## 📖 More Examples

Check out the `examples/` directory for:

- `basic.rs` - Simple validation examples
- `rig_ergonomic.rs` - Ergonomic Rig API showcase
- `rig_integration.rs` - Full Rig framework integration
- `batch_validator.rs` - Validate multiple URLs concurrently
- `llm_simple.rs` - LLM-friendly validation

Run examples with:

```bash
cargo run --example rig_ergonomic
```

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture
```

## 📄 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 🙏 Acknowledgments

Built with ❤️ for the AI agent community. Special thanks to the [Rig framework](https://github.com/0xPlaygrounds/rig) team for inspiring the ergonomic API design.

---

<div align="center">
  <sub>Built by <a href="https://github.com/hghalebi">Hamze Ghalebi</a></sub>
</div>