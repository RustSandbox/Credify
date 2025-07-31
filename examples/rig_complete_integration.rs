// Complete example showing how to integrate credify with Rig-style function calling
// This example demonstrates the patterns without requiring external AI providers

use async_trait::async_trait;
use credify::{rig_validate, rig_validate_json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;

// Tool trait similar to Rig's design
#[async_trait]
trait Tool: Send + Sync {
    type Args: for<'de> Deserialize<'de> + Send;
    type Output: Serialize;
    type Error: std::error::Error + Send + Sync;

    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn parameters(&self) -> serde_json::Value;

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error>;
}

// LinkedIn validator tool implementation
#[derive(Debug, Clone)]
struct LinkedInValidator;

#[derive(Debug, Deserialize)]
struct LinkedInValidatorArgs {
    url: String,
    #[serde(default)]
    detailed: bool,
}

#[derive(Debug, Serialize)]
struct LinkedInValidatorOutput {
    valid: bool,
    username: Option<String>,
    confidence: u8,
    status: String,
    action: String,
    raw_response: Option<String>,
}

#[derive(Debug, thiserror::Error)]
enum LinkedInValidatorError {
    #[error("Validation failed: {0}")]
    ValidationError(String),
}

#[async_trait]
impl Tool for LinkedInValidator {
    type Args = LinkedInValidatorArgs;
    type Output = LinkedInValidatorOutput;
    type Error = LinkedInValidatorError;

    fn name(&self) -> &str {
        "validate_linkedin_profile"
    }

    fn description(&self) -> &str {
        "Validates LinkedIn profile URLs and returns structured information about validity, confidence, and recommended actions"
    }

    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "The LinkedIn profile URL to validate"
                },
                "detailed": {
                    "type": "boolean",
                    "description": "Whether to include raw JSON response",
                    "default": false
                }
            },
            "required": ["url"]
        })
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        println!("🔍 [LinkedInValidator] Processing URL: {}", args.url);

        // Get structured validation result
        let result = rig_validate(&args.url).await;

        // Get raw JSON if requested
        let raw_response = if args.detailed {
            Some(rig_validate_json(&args.url).await)
        } else {
            None
        };

        Ok(LinkedInValidatorOutput {
            valid: result.valid,
            username: result.username.clone(),
            confidence: result.confidence,
            status: result.status.clone(),
            action: result.action.clone(),
            raw_response,
        })
    }
}

// Mock agent that uses tools
struct Agent {
    tools: HashMap<String, Box<dyn std::any::Any + Send + Sync>>,
}

impl Agent {
    fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    fn with_tool<T: Tool + 'static>(mut self, tool: T) -> Self {
        self.tools.insert(tool.name().to_string(), Box::new(tool));
        self
    }

    async fn process_query(&self, query: &str) -> String {
        // Simple query processing - in real implementation this would use an LLM
        if query.to_lowercase().contains("linkedin") || query.to_lowercase().contains("profile") {
            // Simulate extracting URL from query
            let urls = extract_urls(query);

            if urls.is_empty() {
                return "I couldn't find any LinkedIn URLs in your query. Please provide a LinkedIn profile URL to validate.".to_string();
            }

            // Use the LinkedIn validator tool
            if let Some(tool) = self.tools.get("validate_linkedin_profile") {
                if let Some(validator) = tool.downcast_ref::<LinkedInValidator>() {
                    let mut responses = Vec::new();

                    for url in urls {
                        let args = LinkedInValidatorArgs {
                            url: url.clone(),
                            detailed: false,
                        };

                        match validator.call(args).await {
                            Ok(result) => {
                                let response = format_validation_response(&result);
                                responses.push(response);
                            }
                            Err(e) => {
                                responses.push(format!("Error validating {}: {}", url, e));
                            }
                        }
                    }

                    return responses.join("\n\n");
                }
            }
        }

        "I can help you validate LinkedIn profile URLs. Please provide a URL to check.".to_string()
    }
}

// Helper functions
fn extract_urls(text: &str) -> Vec<String> {
    text.split_whitespace()
        .filter(|word| word.contains("linkedin.com") || word.starts_with("http"))
        .map(|s| s.to_string())
        .collect()
}

fn format_validation_response(result: &LinkedInValidatorOutput) -> String {
    if result.valid {
        format!(
            "✅ Valid LinkedIn Profile\n\
             👤 Username: @{}\n\
             📊 Confidence: {}%\n\
             📝 Status: {}\n\
             ➡️  Action: {}",
            result.username.as_ref().unwrap_or(&"unknown".to_string()),
            result.confidence,
            result.status,
            result.action
        )
    } else {
        format!(
            "❌ Invalid LinkedIn Profile\n\
             📝 Status: {}\n\
             ➡️  Action: {}",
            result.status, result.action
        )
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 LinkedIn Profile Validation Agent Demo\n");

    // Create an agent with the LinkedIn validator tool
    let agent = Agent::new().with_tool(LinkedInValidator);

    // Test queries
    let queries = vec![
        "Is https://linkedin.com/in/satyanadella a valid LinkedIn profile?",
        "Check these profiles: https://linkedin.com/in/hamze and https://linkedin.com/in/elonmusk",
        "Validate https://not-linkedin.com/in/fake",
        "Find information about John Doe's LinkedIn",
        "https://www.linkedin.com/in/billgates - is this real?",
    ];

    for query in queries {
        println!("💬 Query: {}", query);
        println!("{}", "-".repeat(70));

        let response = agent.process_query(query).await;
        println!("{}\n", response);
    }

    // Demonstrate direct tool usage
    println!("\n🔧 Direct Tool Usage Example:");
    println!("{}", "=".repeat(70));

    let validator = LinkedInValidator;

    // Show tool metadata
    println!("Tool: {}", validator.name());
    println!("Description: {}", validator.description());
    println!(
        "Parameters: {}",
        serde_json::to_string_pretty(&validator.parameters())?
    );

    // Example with detailed response
    println!("\n📋 Detailed Validation Example:");
    let detailed_args = LinkedInValidatorArgs {
        url: "https://linkedin.com/in/satyanadella".to_string(),
        detailed: true,
    };

    match validator.call(detailed_args).await {
        Ok(result) => {
            println!("Result: {}", serde_json::to_string_pretty(&result)?);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }

    Ok(())
}
