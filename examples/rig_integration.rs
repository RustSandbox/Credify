//! Example of using Credify with Rig framework for AI-powered lead generation
//!
//! This example demonstrates the AI-optimized API designed specifically
//! for function calling with LLM agents.

use credify::{AIDecision, AIValidationResult, ai_validate_json, ai_validate_json_async};
use serde::{Deserialize, Serialize};
use serde_json::json;

// Note: This is a mock implementation of Rig tool trait
// In real usage, you would import from rig crate

#[derive(Deserialize)]
struct ValidateLinkedInArgs {
    url: String,
}

#[derive(Debug, thiserror::Error)]
#[error("LinkedIn validation error")]
struct LinkedInValidatorError(String);

#[derive(Deserialize, Serialize)]
struct LinkedInValidator;

// Mock implementation of Rig Tool trait
impl LinkedInValidator {
    const NAME: &'static str = "LinkedInValidator";

    fn definition(&self) -> serde_json::Value {
        json!({
            "name": "LinkedInValidator",
            "description": "Validates LinkedIn profile URLs and returns structured data for AI decision making",
            "parameters": {
                "type": "object",
                "properties": {
                    "url": {
                        "type": "string",
                        "description": "The LinkedIn URL to validate"
                    }
                },
                "required": ["url"]
            }
        })
    }

    async fn call(&self, args: ValidateLinkedInArgs) -> Result<String, LinkedInValidatorError> {
        // Use the async version of AI-optimized JSON function
        // This avoids blocking runtime issues
        let json_result = ai_validate_json_async(&args.url).await;
        Ok(json_result)
    }
}

// Example system prompt optimized for LinkedIn lead generation
fn lead_gen_system_prompt() -> &'static str {
    r#"
You are an expert LinkedIn lead generator. When searching for LinkedIn profiles:

1. Use web search to find actual LinkedIn URLs - don't make them up
2. When you find a URL, ALWAYS validate it using the LinkedInValidator tool
3. Interpret the validation results:
   - is_valid: true with confidence >= 0.9 = DEFINITELY a real profile
   - is_valid: true with confidence >= 0.7 = LIKELY a real profile
   - decision: Accept = Use this URL
   - decision: Retry = Try validating again after a few seconds
   - decision: Reject = Search for a different URL
4. AUTH_REQUIRED errors usually mean the profile EXISTS - treat these as valid
5. Extract the username from the result for your records

Always provide the validated LinkedIn URL to the user along with the confidence level.
"#
}

#[tokio::main]
async fn main() {
    println!("=== Credify + Rig Framework Integration Example ===\n");

    // Demonstrate the AI-optimized validation
    let test_urls = vec![
        "https://www.linkedin.com/in/satyanadella",
        "https://linkedin.com/in/fake-user-12345",
        "https://www.google.com/in/someone",
        "not-a-url",
    ];

    let validator = LinkedInValidator;

    for url in test_urls {
        println!("Testing URL: {}", url);

        let args = ValidateLinkedInArgs {
            url: url.to_string(),
        };

        match validator.call(args).await {
            Ok(json_result) => {
                // Parse the JSON result
                match serde_json::from_str::<AIValidationResult>(&json_result) {
                    Ok(result) => {
                        println!("  ✓ Validation complete:");
                        println!("    - Valid: {}", result.is_valid);
                        println!("    - Confidence: {:.1}%", result.confidence * 100.0);
                        println!("    - Decision: {:?}", result.decision);
                        println!("    - Reason: {}", result.reason);

                        if let Some(username) = &result.username {
                            println!("    - Username: {}", username);
                        }

                        // Show AI agent interpretation
                        match result.decision {
                            AIDecision::Accept => {
                                println!("    🤖 AI: Use this LinkedIn URL");
                            }
                            AIDecision::Retry => {
                                println!("    🤖 AI: Network issue - retry in a few seconds");
                            }
                            AIDecision::Reject => {
                                println!("    🤖 AI: Invalid URL - search for another");
                            }
                        }
                    }
                    Err(e) => {
                        println!("  ✗ Failed to parse result: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("  ✗ Validation error: {}", e);
            }
        }

        println!();
    }

    // Show the system prompt
    println!("\n=== Recommended System Prompt for Rig ===");
    println!("{}", lead_gen_system_prompt());

    // Demonstrate JSON output format
    println!("\n=== JSON Output Format Example ===");
    let example_json = ai_validate_json("https://linkedin.com/in/example");
    println!("{}", example_json);
}
