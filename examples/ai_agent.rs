//! Example demonstrating how to use Credify with AI agents for lead generation
//!
//! This example shows how the validate_for_llm function is designed to work
//! with AI agents that need to validate LinkedIn URLs found during searches.

use credify::validate_for_llm;

/// Simulated AI agent tool for LinkedIn profile validation
struct LinkedInValidatorTool;

impl LinkedInValidatorTool {
    /// Validate a LinkedIn URL and return AI-friendly response
    fn validate(&self, url: &str) -> String {
        // The validate_for_llm function returns verbose structured data
        // but does NOT print to terminal - perfect for AI agents
        validate_for_llm(url)
    }
}

fn main() {
    println!("LinkedIn Profile Validator for AI Agents Example\n");

    let tool = LinkedInValidatorTool;

    // Simulate URLs that an AI agent might find during lead generation
    let test_urls = vec![
        (
            "https://www.linkedin.com/in/johnsmith",
            "Valid profile format",
        ),
        ("https://linkedin.com/in/jane-doe-123", "Valid with hyphens"),
        ("https://www.google.com/in/someone", "Wrong domain"),
        (
            "https://linkedin.com/company/microsoft",
            "Company page, not profile",
        ),
        ("not-a-url", "Invalid URL format"),
    ];

    for (url, description) in test_urls {
        println!("=== Testing: {} ===", description);
        println!("URL: {}\n", url);

        // Call the tool - this returns structured data but doesn't print
        let result = tool.validate(url);

        // Parse the result to demonstrate what an AI agent would see
        let is_success = result.contains("VALIDATION_RESULT: SUCCESS");
        let is_auth_required = result.contains("ERROR_TYPE: AUTH_REQUIRED");

        if is_success {
            println!("✅ AI Agent Decision: VALID PROFILE - Proceed with this URL");
        } else if is_auth_required {
            println!("✅ AI Agent Decision: LIKELY VALID - LinkedIn is blocking checks but URL format is correct");
            println!("   Recommendation: Treat as valid profile (LinkedIn auth walls usually mean real profiles)");
        } else {
            println!("❌ AI Agent Decision: INVALID - Search for a different URL");
        }

        // Show key information the AI would parse
        for line in result.lines() {
            if line.starts_with("VALIDATION_RESULT:")
                || line.starts_with("ERROR_TYPE:")
                || line.starts_with("RECOMMENDED_NEXT_STEP:")
            {
                println!("   {}", line);
            }
        }

        println!("\n---\n");
    }

    // Demonstrate the authentication case specifically
    println!("=== Special Case: LinkedIn Authentication Required ===\n");
    println!("When LinkedIn returns AUTH_REQUIRED:");
    println!("- The URL format is valid");
    println!("- The profile LIKELY EXISTS");
    println!("- LinkedIn is just blocking automated checks");
    println!("- AI agents should ACCEPT these URLs as valid");
    println!("\nThis is the most common response for real LinkedIn profiles!");
}
