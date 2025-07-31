# Credify API Reference

## Table of Contents

- [Ergonomic Rig Functions](#ergonomic-rig-functions)
- [AI-Optimized Functions](#ai-optimized-functions)
- [LLM-Friendly Functions](#llm-friendly-functions)
- [Traditional API](#traditional-api)
- [Types and Structs](#types-and-structs)
- [Error Types](#error-types)

## Ergonomic Rig Functions

These functions are specifically designed for Rig framework integration and AI agents. They are all async and provide clean, simple responses.

### `rig_is_valid`

```rust
pub async fn rig_is_valid(url: &str) -> bool
```

Ultra-simple boolean validation check.

**Example:**
```rust
if rig_is_valid("https://linkedin.com/in/johndoe").await {
    println!("Valid profile!");
}
```

### `rig_validate_text`

```rust
pub async fn rig_validate_text(url: &str) -> String
```

Returns a one-line human-readable response perfect for chat interfaces.

**Example responses:**
- `"✅ Valid profile @johndoe (95% confidence)"`
- `"❌ Invalid LinkedIn URL - Search for a different LinkedIn profile URL"`

### `rig_validate_json`

```rust
pub async fn rig_validate_json(url: &str) -> String
```

Returns clean JSON output optimized for tool responses.

**Example:**
```rust
impl Tool for LinkedInValidator {
    async fn call(&self, args: Args) -> Result<String, Error> {
        Ok(rig_validate_json(&args.url).await)
    }
}
```

### `rig_validate`

```rust
pub async fn rig_validate(url: &str) -> RigValidationResult
```

Returns structured validation data with all details.

**Returns:** [`RigValidationResult`](#rigvalidationresult)

## AI-Optimized Functions

These functions provide rich structured data for AI decision-making.

### `ai_validate`

```rust
pub fn ai_validate(url: &str) -> AIValidationResult
```

Synchronous validation returning comprehensive structured data.

**Returns:** [`AIValidationResult`](#aivalidationresult)

### `ai_validate_async`

```rust
pub async fn ai_validate_async(url: &str) -> AIValidationResult
```

Async version of `ai_validate`.

### `ai_validate_json`

```rust
pub fn ai_validate_json(url: &str) -> String
```

Returns AI validation result as JSON string.

### `ai_validate_json_async`

```rust
pub async fn ai_validate_json_async(url: &str) -> String
```

Async version of `ai_validate_json`.

## LLM-Friendly Functions

These functions return verbose text reports designed for LLM consumption.

### `validate_for_llm`

```rust
pub fn validate_for_llm(url: &str) -> String
```

Returns extremely verbose validation report with:
- Timestamps
- Severity levels
- Detailed explanations
- 5-7 suggested actions
- Recommended next steps

### `validate_for_llm_async`

```rust
pub async fn validate_for_llm_async(url: &str) -> String
```

Async version of `validate_for_llm`.

## Traditional API

### `LinkedInValidator`

```rust
pub struct LinkedInValidator { /* private fields */ }
```

Main validator struct for traditional usage.

#### Methods

##### `new`

```rust
pub fn new() -> Result<Self, LinkedInUrlError>
```

Creates a new validator with default settings.

##### `new_with_user_agent`

```rust
pub fn new_with_user_agent(user_agent: &str) -> Result<Self, LinkedInUrlError>
```

Creates a validator with custom user agent.

##### `is_valid_linkedin_profile_url`

```rust
pub fn is_valid_linkedin_profile_url(&self, url: &str) -> Result<bool, LinkedInUrlError>
```

Validates a LinkedIn profile URL with network check.

### Standalone Functions

#### `is_valid_linkedin_profile_format`

```rust
pub fn is_valid_linkedin_profile_format(url: &str) -> bool
```

Checks URL format without network calls.

#### `validate_linkedin_url_async`

```rust
pub async fn validate_linkedin_url_async(url: &str) -> Result<bool, LinkedInUrlError>
```

Async validation with network check.

## Types and Structs

### `RigValidationResult`

```rust
pub struct RigValidationResult {
    pub valid: bool,              // Simple pass/fail
    pub username: Option<String>, // LinkedIn username if found
    pub confidence: u8,           // 0-100 percentage
    pub status: String,           // Human-readable status
    pub action: String,           // Suggested action for AI
}
```

### `AIValidationResult`

```rust
pub struct AIValidationResult {
    pub is_valid: bool,
    pub confidence: f32,          // 0.0 to 1.0
    pub decision: AIDecision,
    pub username: Option<String>,
    pub reason: String,
    pub metadata: ValidationMetadata,
}
```

### `AIDecision`

```rust
pub enum AIDecision {
    Accept,  // Definitely use this URL
    Retry,   // Temporary issue, try again
    Reject,  // Invalid URL, search for another
}
```

### `ValidationMetadata`

```rust
pub struct ValidationMetadata {
    pub url_format_valid: bool,
    pub domain_verified: bool,
    pub profile_pattern_matched: bool,
    pub http_status: Option<u16>,
    pub error_type: Option<String>,
    pub timestamp: String,
}
```

## Error Types

### `LinkedInUrlError`

```rust
pub enum LinkedInUrlError {
    InvalidUrl(String),
    NotLinkedInUrl,
    NotProfileUrl,
    NetworkError(reqwest::Error),
    ProfileNotFound,
    AuthenticationRequired,
}
```

| Error | Description |
|-------|-------------|
| `InvalidUrl` | The URL format is invalid |
| `NotLinkedInUrl` | Not a LinkedIn domain |
| `NotProfileUrl` | LinkedIn URL but not a profile |
| `NetworkError` | Network request failed |
| `ProfileNotFound` | Profile doesn't exist (404) |
| `AuthenticationRequired` | LinkedIn requires auth (999) |

## Usage Patterns

### For Rig Tools

```rust
impl Tool for LinkedInValidator {
    async fn call(&self, args: Args) -> Result<String, Error> {
        // Use ergonomic helper
        Ok(rig_validate_json(&args.url).await)
    }
}
```

### For AI Decision Making

```rust
let result = ai_validate_async(url).await;
match result.decision {
    AIDecision::Accept => use_profile(result.username),
    AIDecision::Retry => schedule_retry(),
    AIDecision::Reject => search_for_another(),
}
```

### For Simple Checks

```rust
if is_valid_linkedin_profile_format(url) {
    // Format is correct, proceed with validation
}
```