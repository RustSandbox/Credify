# Migration Guide

## Upgrading to v0.3.0

### New Features

v0.3.0 introduces ergonomic Rig framework helpers and fixes async runtime issues.

#### New Rig Helpers

If you're using Credify with Rig framework, migrate to the new ergonomic helpers:

**Before (v0.2.x):**
```rust
impl Tool for LinkedInValidator {
    async fn call(&self, args: Args) -> Result<String, Error> {
        // This could cause runtime panics!
        Ok(credify::ai_validate_json(&args.url))
    }
}
```

**After (v0.3.0):**
```rust
impl Tool for LinkedInValidator {
    async fn call(&self, args: Args) -> Result<String, Error> {
        // Safe and ergonomic
        Ok(credify::rig_validate_json(&args.url).await)
    }
}
```

#### Async Runtime Fix

If you were experiencing "Cannot drop a runtime in a context where blocking is not allowed" panics:

**Before (causing panic):**
```rust
async fn validate(url: &str) {
    let result = credify::ai_validate_json(url); // PANIC!
}
```

**After (fixed):**
```rust
async fn validate(url: &str) {
    // Option 1: Use async version
    let result = credify::ai_validate_json_async(url).await;
    
    // Option 2: Use Rig helpers (recommended)
    let result = credify::rig_validate_json(url).await;
}
```

### New Types

#### `RigValidationResult`

A simpler, cleaner result type for Rig integration:

```rust
pub struct RigValidationResult {
    pub valid: bool,              // Simple pass/fail
    pub username: Option<String>, // LinkedIn username
    pub confidence: u8,           // 0-100 percentage
    pub status: String,           // Human-readable status
    pub action: String,           // Suggested action
}
```

### Function Mapping

| Old Function | New Recommended Function |
|--------------|-------------------------|
| `ai_validate_json()` in async context | `rig_validate_json()` |
| `ai_validate()` for simple checks | `rig_is_valid()` |
| `validate_for_llm()` for one-liners | `rig_validate_text()` |

### Breaking Changes

None! All existing APIs remain unchanged. The new functions are additions.

## Upgrading to v0.2.0

### New LLM Functions

v0.2.0 added verbose LLM-friendly validation functions:

```rust
// New in v0.2.0
let verbose_report = validate_for_llm(url);
let async_report = validate_for_llm_async(url).await;
```

### Enhanced Error Messages

All error messages now include:
- Timestamps
- Severity levels
- Detailed explanations
- 5-7 suggested actions
- Recommended next steps

## Best Practices

### 1. Use Rig Helpers for AI Agents

```rust
// Best practice for Rig tools
impl Tool for Validator {
    async fn call(&self, args: Args) -> Result<String, Error> {
        Ok(rig_validate_json(&args.url).await)
    }
}
```

### 2. Always Use Async in Async Contexts

```rust
// In async functions, always use async versions
async fn my_handler(url: String) {
    let result = rig_validate(&url).await; // Always async
}
```

### 3. Choose the Right API Level

- **Quick boolean check**: `rig_is_valid()`
- **Human-readable response**: `rig_validate_text()`
- **Tool responses**: `rig_validate_json()`
- **Full control**: `ai_validate_async()`
- **Verbose reports**: `validate_for_llm_async()`

### 4. Handle AUTH_REQUIRED Properly

```rust
let result = rig_validate(url).await;
if result.valid && result.status.contains("auth required") {
    // Profile likely exists, LinkedIn is just blocking checks
    use_profile_with_confidence();
}
```