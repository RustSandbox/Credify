//! Basic example of using linkedin-profile-validator

use linkedin_profile_validator::{
    is_valid_linkedin_profile_format, validate_linkedin_url_async, LinkedInUrlError,
    LinkedInValidator,
};

fn main() {
    println!("LinkedIn Profile Validator - Basic Example\n");

    let test_urls = vec![
        "https://www.linkedin.com/in/williamhgates",
        "https://linkedin.com/in/jeffweiner08",
        "https://www.linkedin.com/in/satyanadella",
        "https://www.linkedin.com/in/fake-profile-99999999",
        "https://www.google.com/in/someone",
        "https://linkedin.com/company/microsoft",
        "invalid-url",
    ];

    // Format validation (no network calls)
    println!("=== Format Validation ===");
    for url in &test_urls {
        let is_valid = is_valid_linkedin_profile_format(url);
        println!(
            "{}: {}",
            url,
            if is_valid {
                "✓ Valid format"
            } else {
                "✗ Invalid format"
            }
        );
    }

    // Full validation with network check (blocking)
    println!("\n=== Full Validation (Blocking) ===");
    let validator = LinkedInValidator::new();

    // Only check the first few to avoid rate limiting
    for url in &test_urls[..3] {
        match validator.is_valid_linkedin_profile_url(url) {
            Ok(_) => println!("{url}: ✓ Profile exists"),
            Err(LinkedInUrlError::ProfileNotFound) => {
                println!("{url}: ✗ Profile not found")
            }
            Err(LinkedInUrlError::AuthenticationRequired) => {
                println!("{url}: ⚠ LinkedIn requires authentication")
            }
            Err(e) => println!("{url}: ✗ Error: {e}"),
        }
    }

    // Async validation example
    println!("\n=== Async Validation Example ===");
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async {
        match validate_linkedin_url_async(test_urls[0]).await {
            Ok(_) => println!("{}: ✓ Profile exists (async)", test_urls[0]),
            Err(LinkedInUrlError::AuthenticationRequired) => {
                println!(
                    "{}: ⚠ LinkedIn requires authentication (async)",
                    test_urls[0]
                )
            }
            Err(e) => println!("{}: ✗ Error: {e}", test_urls[0]),
        }
    });
}
