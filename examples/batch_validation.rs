//! Example of batch validation with rate limiting

use credify::{LinkedInUrlError, LinkedInValidator};
use std::thread;
use std::time::Duration;

fn main() {
    println!("LinkedIn Profile Validator - Batch Validation Example\n");

    let urls_to_check = [
        "https://www.linkedin.com/in/billgates",
        "https://www.linkedin.com/in/elonmusk",
        "https://www.linkedin.com/in/sundarpichai",
        "https://www.linkedin.com/in/tim-cook-1a4b3b5c",
        "https://www.linkedin.com/in/markzuckerberg",
    ];

    let validator = match LinkedInValidator::new() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("[CLIENT_BUILD_ERROR] Failed to create validator: {e}");
            return;
        }
    };
    let delay = Duration::from_secs(2); // 2 second delay between requests

    println!(
        "Checking {} profiles with {}s delay between requests...\n",
        urls_to_check.len(),
        delay.as_secs()
    );

    for (i, url) in urls_to_check.iter().enumerate() {
        print!("[{}/{}] Checking {}: ", i + 1, urls_to_check.len(), url);

        match validator.is_valid_linkedin_profile_url(url) {
            Ok(_) => println!("✓ Valid profile"),
            Err(LinkedInUrlError::ProfileNotFound) => println!("✗ Profile not found"),
            Err(LinkedInUrlError::AuthenticationRequired) => {
                println!("⚠ Authentication required - LinkedIn may be rate limiting");
                println!("    Consider increasing delay between requests or using authentication");
            }
            Err(e) => println!("✗ Error: {e}"),
        }

        // Add delay between requests to avoid rate limiting
        if i < urls_to_check.len() - 1 {
            thread::sleep(delay);
        }
    }

    println!("\nBatch validation complete!");
    println!("\nTip: If you're seeing authentication errors, LinkedIn may be detecting");
    println!("automated requests. Consider:");
    println!("- Increasing the delay between requests");
    println!("- Using format validation only when full validation isn't needed");
    println!("- Implementing proper authentication if available");
}
