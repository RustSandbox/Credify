//! Example comparing different API levels in Credify
//!
//! This shows how to choose the right function for your use case.

use credify::{
    LinkedInValidator, ai_validate_async, is_valid_linkedin_profile_format, rig_is_valid,
    rig_validate, rig_validate_json, rig_validate_text, validate_for_llm_async,
};

#[tokio::main]
async fn main() {
    let url = "https://linkedin.com/in/satyanadella";

    println!("=== Credify API Comparison ===\n");
    println!("Testing URL: {}\n", url);

    // Level 1: Format Check Only (no network)
    println!("1️⃣ Format Check (is_valid_linkedin_profile_format):");
    println!("   Result: {}", is_valid_linkedin_profile_format(url));
    println!("   Use case: Quick input validation\n");

    // Level 2: Simple Boolean (Rig helper)
    println!("2️⃣ Simple Boolean (rig_is_valid):");
    println!("   Result: {}", rig_is_valid(url).await);
    println!("   Use case: Basic true/false checks in AI agents\n");

    // Level 3: One-line Text (Rig helper)
    println!("3️⃣ Human-Readable Text (rig_validate_text):");
    println!("   Result: {}", rig_validate_text(url).await);
    println!("   Use case: Chat responses, user feedback\n");

    // Level 4: Structured Data (Rig helper)
    println!("4️⃣ Structured Data (rig_validate):");
    let rig_result = rig_validate(url).await;
    println!("   Valid: {}", rig_result.valid);
    println!("   Status: {}", rig_result.status);
    println!("   Action: {}", rig_result.action);
    println!("   Confidence: {}%", rig_result.confidence);
    if let Some(username) = &rig_result.username {
        println!("   Username: @{}", username);
    }
    println!("   Use case: Rig tools needing structured responses\n");

    // Level 5: JSON Output (Rig helper)
    println!("5️⃣ JSON Output (rig_validate_json):");
    let json = rig_validate_json(url).await;
    println!("{}", indent_json(&json));
    println!("   Use case: Tool responses in Rig framework\n");

    // Level 6: AI Validation (detailed)
    println!("6️⃣ AI Validation (ai_validate_async):");
    let ai_result = ai_validate_async(url).await;
    println!("   is_valid: {}", ai_result.is_valid);
    println!("   confidence: {:.2}", ai_result.confidence);
    println!("   decision: {:?}", ai_result.decision);
    println!("   reason: {}", ai_result.reason);
    println!("   Use case: Complex AI decision making\n");

    // Level 7: LLM Verbose Report
    println!("7️⃣ LLM Verbose Report (validate_for_llm_async):");
    let llm_report = validate_for_llm_async(url).await;
    // Show first few lines
    for line in llm_report.lines().take(5) {
        println!("   {}", line);
    }
    println!("   ... (truncated)");
    println!("   Use case: Detailed reports for LLM analysis\n");

    // Level 8: Traditional API
    println!("8️⃣ Traditional API (LinkedInValidator):");
    if let Ok(validator) = LinkedInValidator::new() {
        match validator.is_valid_linkedin_profile_url(url) {
            Ok(_) => println!("   Result: Profile exists"),
            Err(e) => println!("   Error: {}", e),
        }
    }
    println!("   Use case: Direct control over validation\n");

    println!("🎯 Recommendation: For Rig/AI agents, use levels 2-5 (rig_* functions)");
}

fn indent_json(json: &str) -> String {
    json.lines()
        .map(|line| format!("   {}", line))
        .collect::<Vec<_>>()
        .join("\n")
}
