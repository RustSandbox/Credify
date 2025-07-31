use credify::{is_valid_linkedin_profile_format, validate_linkedin_url_async, LinkedInValidator};

#[tokio::main]
async fn main() {
    println!("LinkedIn URL Validator Example\n");

    let test_urls = vec![
        "https://www.linkedin.com/in/hamze/",        // Valid profile
        "https://www.linkedin.com/in/hamzeghalebi/", // Also valid profile
        "https://www.linkedin.com/in/this-profile-definitely-does-not-exist-123456789/", // Invalid profile (404)
        "https://www.google.com/in/johndoe",      // Not LinkedIn
        "https://linkedin.com/company/microsoft", // Not a profile URL
        "not-a-url",                              // Invalid URL format
    ];

    println!("=== Format Validation (no network calls) ===");
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

    println!("\n=== Full Validation with Network Check (blocking) ===");
    // Run blocking code in a spawn_blocking task to avoid runtime conflict
    let test_urls_clone = test_urls.clone();
    match tokio::task::spawn_blocking(move || {
        let validator = match LinkedInValidator::new() {
            Ok(v) => v,
            Err(e) => {
                eprintln!("[CLIENT_BUILD_ERROR] Failed to create validator: {e}");
                return;
            }
        };

        for url in &test_urls_clone[..2] {
            match validator.is_valid_linkedin_profile_url(url) {
                Ok(_) => println!("{url}: ✓ Valid and exists"),
                Err(e) => println!("{url}: ✗ Error: {e}"),
            }
        }
    })
    .await
    {
        Ok(_) => (),
        Err(e) => eprintln!("[TASK_ERROR] Failed to run blocking task: {e}"),
    };

    println!("\n=== Async Validation Example ===");
    match validate_linkedin_url_async(test_urls[0]).await {
        Ok(_) => println!("{}: ✓ Valid and exists (async)", test_urls[0]),
        Err(e) => println!("{}: ✗ Error: {}", test_urls[0], e),
    }
}
