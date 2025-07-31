//! Example showing proper async usage with Rig framework
//!
//! This demonstrates how to use Credify's async functions to avoid
//! blocking runtime issues in async contexts.
//!
//! To run this example, you need to add rig to your dependencies:
//! ```toml
//! [dependencies]
//! rig = "0.x.x"
//! ```

use rig::completion::ToolDefinition;
use rig::completion::{Completion, Prompt};
use rig::prelude::CompletionClient;
use rig::providers::xai;
use rig::tool::Tool;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize)]
struct Url {
    url: String,
}

#[derive(Debug, thiserror::Error)]
#[error("LinkedIn validation error: {0}")]
struct LinkedInValidatorError(String);

#[derive(Deserialize, Serialize)]
struct LinkedInValidator;

impl Tool for LinkedInValidator {
    const NAME: &'static str = "LinkedInValidator";
    type Error = LinkedInValidatorError;
    type Args = Url;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Validates LinkedIn profile URLs and returns AI-optimized structured data"
                .to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "url": {
                        "type": "string",
                        "description": "The LinkedIn URL to validate"
                    }
                },
                "required": ["url"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        println!("[tool-call] Validating LinkedIn URL: {}", &args.url);

        // IMPORTANT: Use the async version to avoid blocking issues
        let json_result = credify::ai_validate_json_async(&args.url).await;

        // Parse result to show what happened
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&json_result) {
            if let Some(is_valid) = parsed.get("is_valid").and_then(|v| v.as_bool()) {
                if is_valid {
                    println!("[tool-call] ✅ Valid LinkedIn profile");
                } else {
                    println!("[tool-call] ❌ Invalid URL");
                }
            }
        }

        Ok(json_result)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Credify + Rig Async Example ===\n");

    // Get API key
    let xai_api_key =
        std::env::var("XAI_API_KEY").expect("Please set XAI_API_KEY environment variable");

    // Create client
    let client = xai::Client::new(&xai_api_key);

    // Build agent with LinkedIn validator tool
    let agent = client
        .agent(xai::completion::GROK_4)
        .preamble(system_prompt())
        .tool(LinkedInValidator)
        .build();

    // Test with a search query
    println!("Searching for LinkedIn profiles...\n");

    match agent.prompt("Find Hamze Ghalebi's LinkedIn profile").await {
        Ok(response) => {
            println!("✅ Agent Response:\n{}", response);
        }
        Err(e) => {
            eprintln!("❌ Error: {:#?}", e);
        }
    }

    Ok(())
}

fn system_prompt() -> &'static str {
    r#"You are an expert LinkedIn profile finder. Your task is to find and validate LinkedIn profiles.

When searching for LinkedIn profiles:

1. Use web search to find actual LinkedIn URLs - don't make them up
2. When you find a URL, ALWAYS validate it using the LinkedInValidator tool
3. Interpret the validation results:
   - is_valid: true with confidence >= 0.9 = DEFINITELY a real profile
   - is_valid: true with confidence >= 0.7 = LIKELY a real profile
   - decision: "Accept" = Use this URL
   - decision: "Retry" = Try validating again after a few seconds
   - decision: "Reject" = Search for a different URL
4. AUTH_REQUIRED errors usually mean the profile EXISTS - treat these as valid
5. Extract the username from the result for your records

Always provide the validated LinkedIn URL to the user along with the confidence level.
"#
}
