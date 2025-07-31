// Mock implementation of the xAI integration pattern
// This demonstrates how credify would work with Rig and xAI without requiring API keys

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fmt;

// Mock Rig types to demonstrate the pattern
mod mock_rig {
    use super::*;

    #[derive(Clone)]
    pub struct ToolDefinition {
        pub name: String,
        pub description: String,
        pub parameters: serde_json::Value,
    }

    #[async_trait]
    pub trait Tool: Send + Sync {
        const NAME: &'static str;
        type Error: std::error::Error + Send + Sync;
        type Args: for<'de> Deserialize<'de> + Send;
        type Output: Serialize;

        async fn definition(&self, _prompt: String) -> ToolDefinition {
            ToolDefinition {
                name: Self::NAME.to_string(),
                description: self.description(),
                parameters: self.parameters(),
            }
        }

        fn description(&self) -> String;
        fn parameters(&self) -> serde_json::Value;
        async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error>;
    }

    pub trait Prompt {
        fn prompt(&self, text: &str) -> PromptBuilder;
    }

    pub struct PromptBuilder {
        prompt: String,
        tools: Vec<String>,
    }

    impl PromptBuilder {
        pub async fn complete(self) -> Result<String, Box<dyn std::error::Error>> {
            // Mock completion that simulates using tools
            if self.prompt.to_lowercase().contains("linkedin") {
                Ok(format!(
                    "Based on my search and validation:\n\n\
                    I found the LinkedIn profile you requested. \
                    The LinkedInChecker tool confirmed it's valid with high confidence.\n\n\
                    Query: {}\n\
                    Tools used: {:?}",
                    self.prompt, self.tools
                ))
            } else {
                Ok("I can help you validate LinkedIn profiles. Please provide a LinkedIn URL or ask me to find someone's profile.".to_string())
            }
        }
    }

    pub mod providers {
        pub mod xai {
            pub struct Client {
                api_key: String,
            }

            impl Client {
                pub fn new(api_key: &str) -> Self {
                    Self {
                        api_key: api_key.to_string(),
                    }
                }

                pub fn agent(&self, model: &str) -> AgentBuilder {
                    AgentBuilder {
                        model: model.to_string(),
                        preamble: None,
                        tools: vec![],
                    }
                }
            }

            pub struct AgentBuilder {
                model: String,
                preamble: Option<String>,
                tools: Vec<String>,
            }

            impl AgentBuilder {
                pub fn preamble(mut self, preamble: &str) -> Self {
                    self.preamble = Some(preamble.to_string());
                    self
                }

                pub fn tool<T: super::super::Tool + 'static>(mut self, tool: T) -> Self {
                    self.tools.push(T::NAME.to_string());
                    self
                }

                pub fn build(self) -> Agent {
                    Agent {
                        model: self.model,
                        preamble: self.preamble,
                        tools: self.tools,
                    }
                }
            }

            pub struct Agent {
                model: String,
                preamble: Option<String>,
                tools: Vec<String>,
            }

            impl super::super::Prompt for Agent {
                fn prompt(&self, text: &str) -> super::super::PromptBuilder {
                    println!(
                        "🤖 [{}] Processing prompt with tools: {:?}",
                        self.model, self.tools
                    );
                    if let Some(ref preamble) = self.preamble {
                        println!(
                            "📋 Using preamble: {}",
                            preamble.lines().next().unwrap_or("")
                        );
                    }

                    super::super::PromptBuilder {
                        prompt: text.to_string(),
                        tools: self.tools.clone(),
                    }
                }
            }

            pub mod completion {
                pub const GROK_BETA: &str = "grok-beta";
                pub const GROK_2: &str = "grok-2-latest";
            }
        }
    }
}

use mock_rig::{Prompt, Tool, ToolDefinition, providers};

// LinkedIn checker implementation
#[derive(Deserialize)]
struct Url {
    url: String,
}

#[derive(Debug)]
struct LinkedInCheckerError(String);

impl fmt::Display for LinkedInCheckerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LinkedIn checking error: {}", self.0)
    }
}

impl std::error::Error for LinkedInCheckerError {}

#[derive(Deserialize, Serialize)]
struct LinkedInChecker;

#[async_trait]
impl Tool for LinkedInChecker {
    const NAME: &'static str = "LinkedInChecker";
    type Error = LinkedInCheckerError;
    type Args = Url;
    type Output = String;

    fn description(&self) -> String {
        "Validates if a URL is a valid LinkedIn profile URL and returns detailed information"
            .to_string()
    }

    fn parameters(&self) -> serde_json::Value {
        json!({
            "type": "object",
            "properties": {
                "url": {
                    "type": "string",
                    "description": "The URL to check if it's a valid LinkedIn profile URL"
                }
            },
            "required": ["url"]
        })
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        println!("🔍 [LinkedInChecker] Validating URL: {}", &args.url);

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

        Ok(result)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 LinkedIn Profile Finder with Mock xAI Integration\n");

    // Mock API key (in real usage, this would be from environment)
    let xai_api_key = "mock-api-key";

    println!("🔧 Initializing mock xAI client...");
    let client = providers::xai::Client::new(xai_api_key);

    // Create an agent with the LinkedIn checker tool
    let agent = client
        .agent(providers::xai::completion::GROK_2)
        .preamble(system_prompt())
        .tool(LinkedInChecker)
        .build();

    println!("🤖 Agent ready! Testing LinkedIn profile validation...\n");

    // Test queries
    let queries = vec![
        "Find Hamze Ghalebi's LinkedIn profile",
        "Check if https://linkedin.com/in/satyanadella is valid",
        "Validate these profiles: https://linkedin.com/in/elonmusk and https://linkedin.com/in/billgates",
    ];

    for query in queries {
        println!("📝 Query: {}", query);
        println!("{}", "-".repeat(70));

        let response = agent.prompt(query).complete().await?;
        println!("🎯 Response: {}\n", response);

        // Simulate tool usage for LinkedIn URLs in the query
        let urls: Vec<&str> = query
            .split_whitespace()
            .filter(|word| word.contains("linkedin.com"))
            .collect();

        for url in urls {
            let tool = LinkedInChecker;
            let args = Url {
                url: url.to_string(),
            };

            match tool.call(args).await {
                Ok(result) => {
                    println!("📊 Validation Result:");
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&result) {
                        println!("{}", serde_json::to_string_pretty(&parsed)?);
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error: {}", e);
                }
            }
        }

        println!();
    }

    // Demonstrate error handling
    println!("\n🧪 Error Handling Examples:");
    println!("{}", "=".repeat(70));

    // Test with invalid URL
    let invalid_test = Url {
        url: "not-a-url".to_string(),
    };

    let tool = LinkedInChecker;
    match tool.call(invalid_test).await {
        Ok(result) => {
            println!("Result for invalid URL: {}", result);
        }
        Err(e) => {
            println!("Error (expected): {}", e);
        }
    }

    Ok(())
}

fn system_prompt() -> &'static str {
    r#"You are a professional LinkedIn profile finder assistant powered by advanced validation tools.

When validating LinkedIn profiles:
1. Use web search to find LinkedIn URLs - don't make them up
2. ALWAYS validate URLs with LinkedInChecker before using them
3. Interpret results:
   - valid: true + confidence >= 90 = Definitely real profile
   - valid: true + confidence >= 70 = Likely real profile
   - valid: false = Not a valid LinkedIn profile
   - status contains "auth required" = Profile likely exists (LinkedIn blocking check)
4. Provide clear, actionable responses based on validation results"#
}
