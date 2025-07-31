# Async Usage Guide for Credify

## Why Async Matters

When using Credify in async contexts (like web servers, AI agents, or async applications), it's crucial to use the async versions of functions to avoid runtime panics.

## The Problem

Using synchronous functions in async contexts can cause this error:
```
thread 'main' panicked at .../tokio-.../runtime/blocking/shutdown.rs:51:21:
Cannot drop a runtime in a context where blocking is not allowed.
```

This happens because synchronous functions create blocking HTTP clients that conflict with async runtimes like Tokio.

## The Solution

Always use the `*_async` versions of functions in async contexts:

### ❌ Wrong (causes panic)
```rust
impl Tool for LinkedInValidator {
    async fn call(&self, args: Args) -> Result<String, Error> {
        // This will panic in async context!
        Ok(credify::ai_validate_json(&args.url))
    }
}
```

### ✅ Correct
```rust
impl Tool for LinkedInValidator {
    async fn call(&self, args: Args) -> Result<String, Error> {
        // Use async version
        Ok(credify::ai_validate_json_async(&args.url).await)
    }
}
```

## Available Async Functions

Credify provides async versions for all validation functions:

| Synchronous | Asynchronous |
|-------------|--------------|
| `validate_for_llm()` | `validate_for_llm_async()` |
| `ai_validate()` | `ai_validate_async()` |
| `ai_validate_json()` | `ai_validate_json_async()` |
| `validate_linkedin_url()` | `validate_linkedin_url_async()` |

## Complete Rig Framework Example

```rust
use credify::ai_validate_json_async;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
struct ValidateArgs {
    url: String,
}

#[derive(Debug, thiserror::Error)]
#[error("Validation error: {0}")]
struct ValidationError(String);

#[derive(Deserialize, Serialize)]
struct LinkedInValidator;

impl Tool for LinkedInValidator {
    const NAME: &'static str = "LinkedInValidator";
    type Error = ValidationError;
    type Args = ValidateArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> rig::completion::ToolDefinition {
        rig::completion::ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Validates LinkedIn profile URLs".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "url": {
                        "type": "string",
                        "description": "LinkedIn URL to validate"
                    }
                },
                "required": ["url"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // Always use async version in async contexts
        Ok(ai_validate_json_async(&args.url).await)
    }
}
```

## Best Practices

1. **Always use async in async contexts**: If your function is `async`, use Credify's async functions
2. **Use sync for CLI tools**: For command-line tools, the sync versions are fine
3. **Web servers need async**: FastAPI, Actix, Axum, etc. all need async versions
4. **AI frameworks need async**: Rig, LangChain-rust, etc. need async versions

## Performance Note

The async versions use Tokio-compatible async HTTP clients and are optimized for concurrent operations. They're just as fast as sync versions but play nicely with async runtimes.