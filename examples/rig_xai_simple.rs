// Simplified version that focuses on the core functionality
// without requiring the full rig crate

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize)]
struct ValidateUrlArgs {
    url: String,
}

#[derive(Debug, Serialize)]
struct LinkedInValidatorTool;

impl LinkedInValidatorTool {
    fn name() -> &'static str {
        "validate_linkedin_profile"
    }

    fn description() -> &'static str {
        "Validates if a URL is a valid LinkedIn profile URL and returns detailed information"
    }

    fn parameters() -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "The LinkedIn profile URL to validate"
                }
            },
            "required": ["url"]
        })
    }

    async fn call(args: ValidateUrlArgs) -> String {
        println!("🔍 Validating LinkedIn URL: {}", &args.url);

        // Use credify's Rig-optimized validation
        let result = credify::rig_validate_json(&args.url).await;

        // Parse and display result
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&result) {
            if let Some(valid) = parsed.get("valid").and_then(|v| v.as_bool()) {
                if valid {
                    if let Some(username) = parsed.get("username").and_then(|v| v.as_str()) {
                        println!("✅ Valid profile found: @{}", username);
                    }
                } else {
                    println!("❌ Invalid LinkedIn URL");
                }
            }
        }

        result
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 LinkedIn Profile Validator Demo\n");

    // Test URLs
    let test_urls = vec![
        "https://linkedin.com/in/satyanadella",
        "https://www.linkedin.com/in/hamze",
        "https://linkedin.com/in/invalid-user-12345678900000",
        "https://not-linkedin.com/in/fake",
        "invalid-url",
    ];

    println!("Testing LinkedIn URL validation:\n");

    for url in test_urls {
        println!("Testing: {}", url);
        println!("{}", "-".repeat(60));

        let args = ValidateUrlArgs {
            url: url.to_string(),
        };

        let result = LinkedInValidatorTool::call(args).await;

        // Pretty print the JSON result
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&result) {
            println!("Result: {}", serde_json::to_string_pretty(&parsed)?);
        } else {
            println!("Result: {}", result);
        }

        println!();
    }

    // Demonstrate tool definition
    println!("\nTool Definition for AI Agents:");
    println!("{}", "-".repeat(60));
    println!("Name: {}", LinkedInValidatorTool::name());
    println!("Description: {}", LinkedInValidatorTool::description());
    println!(
        "Parameters: {}",
        serde_json::to_string_pretty(&LinkedInValidatorTool::parameters())?
    );

    Ok(())
}
