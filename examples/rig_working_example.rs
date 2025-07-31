// Working example that demonstrates the pattern without external crate conflicts

use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize)]
struct ValidateUrlArgs {
    url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct LinkedInValidator;

// Tool implementation that follows Rig's pattern
impl LinkedInValidator {
    const NAME: &'static str = "validate_linkedin_profile";

    fn description(&self) -> String {
        "Validates LinkedIn profile URLs and returns detailed information about the profile"
            .to_string()
    }

    fn parameters(&self) -> serde_json::Value {
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

    async fn call(&self, args: ValidateUrlArgs) -> Result<String, String> {
        println!("[tool-call] Validating LinkedIn URL: {}", &args.url);

        // Use credify's Rig-optimized validation
        let result = credify::rig_validate_json(&args.url).await;

        // Parse and log the result
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&result) {
            if let Some(valid) = parsed.get("valid").and_then(|v| v.as_bool()) {
                if valid {
                    if let Some(username) = parsed.get("username").and_then(|v| v.as_str()) {
                        println!("✅ Valid LinkedIn profile found: @{}", username);
                    }
                } else {
                    println!("❌ Invalid LinkedIn URL");
                }

                if let Some(confidence) = parsed.get("confidence").and_then(|v| v.as_u64()) {
                    println!("📊 Confidence: {}%", confidence);
                }
            }
        }

        Ok(result)
    }
}

// Mock agent to demonstrate the integration pattern
struct Agent {
    tools: Vec<LinkedInValidator>,
    preamble: String,
}

impl Agent {
    fn new(preamble: &str) -> Self {
        Self {
            tools: vec![LinkedInValidator],
            preamble: preamble.to_string(),
        }
    }

    async fn prompt(&self, query: &str) -> Result<String, Box<dyn std::error::Error>> {
        println!(
            "🤖 Processing query with preamble: {}",
            self.preamble.lines().next().unwrap_or("")
        );

        // Extract URLs from the query
        let urls: Vec<&str> = query
            .split_whitespace()
            .filter(|word| word.contains("linkedin.com") || word.starts_with("http"))
            .collect();

        if urls.is_empty() {
            return Ok(
                "I can validate LinkedIn profile URLs. Please provide a LinkedIn URL to check."
                    .to_string(),
            );
        }

        let mut results = Vec::new();

        for url in urls {
            if let Some(tool) = self.tools.first() {
                let args = ValidateUrlArgs {
                    url: url.to_string(),
                };
                match tool.call(args).await {
                    Ok(json_result) => {
                        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&json_result)
                        {
                            let valid = parsed
                                .get("valid")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false);
                            let username = parsed
                                .get("username")
                                .and_then(|v| v.as_str())
                                .unwrap_or("unknown");
                            let confidence = parsed
                                .get("confidence")
                                .and_then(|v| v.as_u64())
                                .unwrap_or(0);
                            let status = parsed
                                .get("status")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown");

                            if valid {
                                results.push(format!(
                                    "✅ {} is a valid LinkedIn profile (@{}) with {}% confidence. Status: {}",
                                    url, username, confidence, status
                                ));
                            } else {
                                results.push(format!(
                                    "❌ {} is not a valid LinkedIn profile. Status: {}",
                                    url, status
                                ));
                            }
                        }
                    }
                    Err(e) => {
                        results.push(format!("Error checking {}: {}", url, e));
                    }
                }
            }
        }

        Ok(results.join("\n"))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 LinkedIn Profile Validator - Rig Pattern Example\n");

    // Initialize environment
    dotenv::dotenv().ok();

    // Create an agent with the LinkedIn validation tool
    let agent = Agent::new(system_prompt());

    println!("🤖 Agent ready with LinkedIn validation capabilities!\n");

    // Test queries
    let queries = vec![
        "Is https://linkedin.com/in/hamze a valid LinkedIn profile?",
        "Check if these are valid: https://linkedin.com/in/elonmusk and https://linkedin.com/in/fake-user-12345",
        "Validate https://not-linkedin.com/in/someone",
        "Can you help me validate LinkedIn profiles?",
        "What about https://www.linkedin.com/in/satyanadella?",
    ];

    for query in queries {
        println!("💬 User: {}", query);
        println!("{}", "-".repeat(70));

        match agent.prompt(query).await {
            Ok(response) => {
                println!("🤖 Assistant:\n{}", response);
            }
            Err(e) => {
                eprintln!("❌ Error: {}", e);
            }
        }

        println!("\n");
    }

    // Demonstrate direct tool usage
    println!("🛠️  Direct Tool Usage Example:");
    println!("{}", "=".repeat(70));

    let validator = LinkedInValidator;

    // Show tool metadata
    println!("📋 Tool Definition:");
    println!("   Name: {}", LinkedInValidator::NAME);
    println!("   Description: {}", validator.description());
    println!(
        "   Parameters: {}",
        serde_json::to_string_pretty(&validator.parameters())?
    );

    // Direct validation
    println!("\n🔧 Direct validation examples:");

    let test_urls = vec![
        "https://linkedin.com/in/rustlang",
        "https://linkedin.com/in/billgates",
        "https://invalid-domain.com/in/user",
    ];

    for url in test_urls {
        println!("\nTesting: {}", url);
        let args = ValidateUrlArgs {
            url: url.to_string(),
        };

        match validator.call(args).await {
            Ok(result) => {
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&result) {
                    println!("Result: {}", serde_json::to_string_pretty(&parsed)?);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    println!("\n✅ Example completed successfully!");
    println!("\n📝 Note: This example demonstrates the Rig tool pattern.");
    println!("For real xAI integration, you would need:");
    println!("1. A valid XAI_API_KEY");
    println!("2. The actual Rig providers::xai client");
    println!("3. Proper error handling for API responses");

    Ok(())
}

fn system_prompt() -> &'static str {
    r#"You are a professional LinkedIn profile validation assistant.

Your capabilities:
- Validate LinkedIn profile URLs
- Provide detailed information about profile validity
- Help users verify LinkedIn profiles

Always be accurate, helpful, and professional in your responses."#
}
