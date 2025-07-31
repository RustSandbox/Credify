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

**🎯 New in v0.4.0**: Enhanced LinkedIn 404 detection with comprehensive apostrophe encoding support and complete Rig framework integration examples with real-world patterns.

## 🚀 Real-World Use Cases

### AI Recruiting Assistant
```rust
// Validate candidate profiles before outreach
let result = rig_validate_json("https://linkedin.com/in/john-doe").await;
// Returns: {"valid": true, "username": "john-doe", "confidence": 95, ...}
```

### Lead Generation Bot
```rust
// Quick validation for scraped LinkedIn URLs
if rig_is_valid(potential_lead_url).await {
    add_to_crm(potential_lead_url);
}
```

### Professional Network Analyzer
```rust
// Get human-readable validation for chat interfaces
let message = rig_validate_text(profile_url).await;
// Returns: "✅ Valid profile @jane-smith (90% confidence)"
```

### Data Enrichment Pipeline
```rust
// Batch validate URLs with confidence scoring
let results = urls.iter().map(|url| rig_validate(url)).collect().await;
// Filter by confidence level for data quality
```

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
credify = "0.4.0"
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
// Complete Rig tool implementation
#[derive(Deserialize)]
struct ValidateLinkedInArgs {
    url: String,
}

#[derive(Deserialize, Serialize)]
struct LinkedInValidator;

#[async_trait]
impl Tool for LinkedInValidator {
    const NAME: &'static str = "validate_linkedin_profile";
    type Args = ValidateLinkedInArgs;
    type Output = String;
    type Error = ToolError;

    async fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Validates LinkedIn profile URLs and returns structured information".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "url": {
                        "type": "string",
                        "description": "The LinkedIn profile URL to validate"
                    }
                },
                "required": ["url"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
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

## 🎯 Complete Rig Framework Integration Guide

### Setting Up Credify with Rig

Credify is designed to work seamlessly with the Rig framework for building AI agents. Here's a complete guide to integrating LinkedIn validation into your Rig-powered AI system.

#### 1. Basic Tool Setup

```rust
use credify::rig_validate_json;
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize)]
struct ValidateArgs {
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LinkedInValidator;

#[async_trait::async_trait]
impl Tool for LinkedInValidator {
    const NAME: &'static str = "validate_linkedin_profile";
    type Args = ValidateArgs;
    type Output = String;
    type Error = Box<dyn std::error::Error + Send + Sync>;

    async fn definition(&self) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Validates LinkedIn profile URLs and returns structured data about the profile".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "url": {
                        "type": "string",
                        "description": "The LinkedIn profile URL to validate (e.g., https://linkedin.com/in/username)"
                    }
                },
                "required": ["url"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(rig_validate_json(&args.url).await)
    }
}
```

#### 2. Using with Rig Agents

```rust
use rig::providers::openai;
use rig::completion::{Prompt, ToolDefinition};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the LinkedIn validator tool
    let linkedin_tool = LinkedInValidator;
    
    // Set up your AI client
    let client = openai::Client::new(&std::env::var("OPENAI_API_KEY")?);
    
    // Create an agent with the LinkedIn validation tool
    let agent = client
        .agent("gpt-4")
        .preamble("You are a professional network analyst.")
        .tool(linkedin_tool)
        .build();
    
    // Use the agent to validate profiles
    let response = agent
        .prompt("Check if https://linkedin.com/in/satya-nadella is a valid LinkedIn profile")
        .await?;
    
    println!("Agent response: {}", response);
    Ok(())
}
```

#### 3. Advanced Usage with Multiple Response Types

```rust
use credify::{rig_is_valid, rig_validate, rig_validate_text};

// Quick boolean check for conditional logic
async fn quick_check(url: &str) -> bool {
    rig_is_valid(url).await
}

// Human-readable response for chat interfaces
async fn chat_response(url: &str) -> String {
    rig_validate_text(url).await
}

// Structured data for complex workflows
async fn detailed_check(url: &str) {
    let result = rig_validate(url).await;
    
    match result.confidence {
        90..=100 => println!("High confidence: proceed with profile"),
        70..=89 => println!("Medium confidence: verify manually"),
        _ => println!("Low confidence: search for alternative")
    }
}
```

#### 4. Real-World Use Case: Recruiting Assistant

```rust
#[derive(Debug, Serialize, Deserialize)]
struct RecruitingAssistant {
    linkedin_validator: LinkedInValidator,
}

impl RecruitingAssistant {
    async fn validate_candidate_profiles(&self, profiles: Vec<String>) -> Vec<CandidateStatus> {
        let mut results = Vec::new();
        
        for profile_url in profiles {
            let validation = rig_validate(&profile_url).await;
            
            results.push(CandidateStatus {
                url: profile_url,
                valid: validation.valid,
                username: validation.username,
                confidence: validation.confidence,
                action: match validation.valid {
                    true => "Schedule interview",
                    false => "Request updated profile"
                }.to_string(),
            });
        }
        
        results
    }
}

#[derive(Debug, Serialize)]
struct CandidateStatus {
    url: String,
    valid: bool,
    username: Option<String>,
    confidence: u8,
    action: String,
}
```

#### 5. Error Handling Best Practices

```rust
// Credify's Rig helpers NEVER panic, making them safe for production
async fn safe_validation(url: &str) -> String {
    // This will always return a valid JSON response, even on errors
    let result = rig_validate_json(url).await;
    
    // Parse the result to handle different scenarios
    if let Ok(parsed) = serde_json::from_str::<RigValidationResult>(&result) {
        if parsed.valid {
            format!("Profile @{} is valid", parsed.username.unwrap_or_default())
        } else {
            format!("Invalid profile: {}", parsed.status)
        }
    } else {
        "Validation service temporarily unavailable".to_string()
    }
}
```

#### 6. Batch Processing with Rig

```rust
use futures::future::join_all;

async fn batch_validate(urls: Vec<String>) -> Vec<(String, bool)> {
    let futures = urls.into_iter().map(|url| async move {
        let valid = rig_is_valid(&url).await;
        (url, valid)
    });
    
    join_all(futures).await
}
```

### Function Calling Patterns

#### Pattern 1: Simple Validation Tool

```rust
// Minimal implementation for basic needs
impl Tool for SimpleLinkedInChecker {
    async fn call(&self, args: Args) -> Result<String, Error> {
        if rig_is_valid(&args.url).await {
            Ok("Valid LinkedIn profile".to_string())
        } else {
            Ok("Invalid LinkedIn profile".to_string())
        }
    }
}
```

#### Pattern 2: Detailed Analysis Tool

```rust
// Rich responses for complex AI workflows
impl Tool for DetailedLinkedInAnalyzer {
    async fn call(&self, args: Args) -> Result<String, Error> {
        let result = rig_validate(&args.url).await;
        
        Ok(json!({
            "valid": result.valid,
            "username": result.username,
            "confidence": result.confidence,
            "status": result.status,
            "recommended_action": result.action,
            "profile_url": if result.valid {
                Some(format!("https://linkedin.com/in/{}", 
                    result.username.as_ref().unwrap_or(&"unknown".to_string())))
            } else {
                None
            }
        }).to_string())
    }
}
```

#### Pattern 3: Multi-Step Validation

```rust
// Complex validation with fallback strategies
impl Tool for SmartLinkedInValidator {
    async fn call(&self, args: Args) -> Result<String, Error> {
        // First, try the provided URL
        let result = rig_validate(&args.url).await;
        
        if result.valid {
            return Ok(rig_validate_json(&args.url).await);
        }
        
        // If invalid, try common variations
        if let Some(username) = extract_username(&args.url) {
            let variations = vec![
                format!("https://linkedin.com/in/{}", username),
                format!("https://www.linkedin.com/in/{}", username),
                format!("https://linkedin.com/in/{}/", username),
            ];
            
            for variant in variations {
                if rig_is_valid(&variant).await {
                    return Ok(rig_validate_json(&variant).await);
                }
            }
        }
        
        // Return detailed error information
        Ok(json!({
            "valid": false,
            "error": "Profile not found",
            "suggestions": [
                "Check the username spelling",
                "Verify the profile hasn't been deleted",
                "Try searching by name on LinkedIn"
            ]
        }).to_string())
    }
}
```

### Integration Tips

1. **Always use async**: All Rig helpers are async to prevent blocking
2. **Never panics**: Rig helpers return valid responses even on errors
3. **Structured responses**: Use `rig_validate_json` for consistent AI parsing
4. **Confidence scores**: Use confidence levels to make nuanced decisions
5. **Batch wisely**: Process multiple URLs concurrently for better performance

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
- `rig_integration.rs` - Full Rig framework integration with function calling
- `rig_async_proper.rs` - Advanced async patterns for Rig
- `batch_validator.rs` - Validate multiple URLs concurrently
- `llm_simple.rs` - LLM-friendly validation
- `ai_agent_demo.rs` - Complete AI agent implementation

### Quick Example: Rig Function Calling

```rust
// Define your tool
#[derive(Tool)]
#[tool(
    name = "linkedin_validator",
    description = "Validates LinkedIn profile URLs"
)]
struct LinkedInValidator;

// Implement the tool - just one line!
impl LinkedInValidator {
    async fn validate(&self, url: String) -> String {
        credify::rig_validate_json(&url).await
    }
}

// Use with your AI agent
let agent = client
    .agent("gpt-4")
    .tool(LinkedInValidator)
    .build();

let response = agent
    .prompt("Check these LinkedIn profiles for our hiring pipeline")
    .await?;
```

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