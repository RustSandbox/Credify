//! Example showing the new ergonomic Rig helper functions
//!
//! This demonstrates how clean and simple the API is when using
//! the Rig-specific helper functions.

use credify::{rig_is_valid, rig_validate, rig_validate_json, rig_validate_text};

#[tokio::main]
async fn main() {
    println!("=== Credify Ergonomic Rig API Examples ===\n");

    let test_urls = vec![
        "https://linkedin.com/in/satyanadella",
        "https://linkedin.com/in/fake-user-12345",
        "https://google.com/in/someone",
        "not-a-url",
    ];

    println!("1. Ultra-simple boolean check with rig_is_valid():");
    println!("─────────────────────────────────────────────────\n");

    for url in &test_urls {
        let valid = rig_is_valid(url).await;
        println!("{}: {}", url, if valid { "✅ Valid" } else { "❌ Invalid" });
    }

    println!("\n2. One-line text responses with rig_validate_text():");
    println!("───────────────────────────────────────────────────\n");

    for url in &test_urls {
        let response = rig_validate_text(url).await;
        println!("{}", response);
    }

    println!("\n3. Structured results with rig_validate():");
    println!("─────────────────────────────────────────\n");

    for url in &test_urls {
        let result = rig_validate(url).await;
        println!("URL: {}", url);
        println!("  Valid: {}", result.valid);
        println!("  Status: {}", result.status);
        println!("  Action: {}", result.action);
        if let Some(username) = &result.username {
            println!("  Username: @{}", username);
        }
        println!("  Confidence: {}%", result.confidence);
        println!();
    }

    println!("4. Clean JSON output with rig_validate_json():");
    println!("─────────────────────────────────────────────\n");

    let json = rig_validate_json("https://linkedin.com/in/example").await;
    println!("{}", json);
}

// Example of how to use in a Rig tool
#[allow(dead_code)]
mod rig_tool_example {
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize)]
    struct ValidateArgs {
        url: String,
    }

    #[derive(Debug, thiserror::Error)]
    #[error("Validation error")]
    struct ValidationError;

    #[derive(Deserialize, Serialize)]
    struct LinkedInValidator;

    // Mock Rig Tool trait implementation
    impl LinkedInValidator {
        async fn call(&self, args: ValidateArgs) -> Result<String, ValidationError> {
            // ONE LINE! That's all you need for Rig integration
            Ok(credify::rig_validate_json(&args.url).await)
        }
    }
}

// Even simpler for basic validation
#[allow(dead_code)]
mod simple_rig_tool {
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct CheckArgs {
        url: String,
    }

    async fn check_linkedin(args: CheckArgs) -> String {
        // Ultra-simple one-liner for text response
        credify::rig_validate_text(&args.url).await
    }
}
