//! Example demonstrating LLM-friendly error messages

use credify::{validate_for_llm, validate_for_llm_async, LinkedInUrlError, LinkedInValidator};

fn main() {
    println!("LinkedIn Validator - LLM-Friendly Error Messages Example\n");

    // Create validator
    let validator = match LinkedInValidator::new() {
        Ok(v) => v,
        Err(e) => {
            println!("VALIDATION_RESULT: ERROR");
            println!("ERROR_TYPE: CLIENT_INITIALIZATION_FAILED");
            println!("ERROR_MESSAGE: {e}");
            println!("SUGGESTED_ACTION: Check system resources and retry");
            return;
        }
    };

    let test_cases = vec![
        (
            "https://www.linkedin.com/in/valid-user",
            "Valid LinkedIn profile",
        ),
        (
            "https://www.linkedin.com/in/nonexistent99999",
            "Non-existent profile",
        ),
        ("https://www.google.com/in/someone", "Wrong domain"),
        (
            "https://linkedin.com/company/microsoft",
            "Not a profile URL",
        ),
        ("not-a-url", "Invalid URL format"),
    ];

    for (url, description) in &test_cases {
        println!("\n--- Testing: {description} ---");
        println!("URL: {url}");

        match validator.is_valid_linkedin_profile_url(url) {
            Ok(_) => {
                println!("VALIDATION_RESULT: SUCCESS");
                println!("PROFILE_EXISTS: TRUE");
                println!("SUGGESTED_ACTION: Proceed with profile data extraction");
            }
            Err(e) => {
                println!("VALIDATION_RESULT: ERROR");
                match e {
                    LinkedInUrlError::InvalidUrl(_) => {
                        println!("ERROR_TYPE: INVALID_URL_FORMAT");
                        println!("ERROR_MESSAGE: {e}");
                        println!("SUGGESTED_ACTION: Verify URL format and retry");
                    }
                    LinkedInUrlError::NotLinkedInUrl => {
                        println!("ERROR_TYPE: NOT_LINKEDIN_DOMAIN");
                        println!("ERROR_MESSAGE: {e}");
                        println!("SUGGESTED_ACTION: Ensure URL is from linkedin.com domain");
                    }
                    LinkedInUrlError::NotProfileUrl => {
                        println!("ERROR_TYPE: NOT_PROFILE_URL");
                        println!("ERROR_MESSAGE: {e}");
                        println!("SUGGESTED_ACTION: Use profile URLs in format /in/username");
                    }
                    LinkedInUrlError::ProfileNotFound => {
                        println!("ERROR_TYPE: PROFILE_NOT_FOUND");
                        println!("ERROR_MESSAGE: {e}");
                        println!("SUGGESTED_ACTION: Verify username and check if profile exists");
                    }
                    LinkedInUrlError::AuthenticationRequired => {
                        println!("ERROR_TYPE: AUTH_REQUIRED");
                        println!("ERROR_MESSAGE: {e}");
                        println!("SUGGESTED_ACTION: Cannot verify profile existence - LinkedIn requires authentication. Consider using format validation only or implement authentication");
                    }
                    LinkedInUrlError::NetworkError(_) => {
                        println!("ERROR_TYPE: NETWORK_ERROR");
                        println!("ERROR_MESSAGE: {e}");
                        println!("SUGGESTED_ACTION: Check network connection and retry");
                    }
                    LinkedInUrlError::ClientBuildError(_) => {
                        println!("ERROR_TYPE: CLIENT_BUILD_ERROR");
                        println!("ERROR_MESSAGE: {e}");
                        println!("SUGGESTED_ACTION: Check system resources and retry");
                    }
                }
            }
        }
    }

    println!("\n\n=== Format-Only Validation (No Network Calls) ===");
    println!("This is useful when you just need to check URL format without verifying profile existence.\n");

    use credify::is_valid_linkedin_profile_format;

    for (url, _description) in &test_cases {
        let is_valid = is_valid_linkedin_profile_format(url);
        println!("URL: {url}");
        println!(
            "FORMAT_VALIDATION_RESULT: {}",
            if is_valid { "VALID" } else { "INVALID" }
        );
        println!(
            "SUGGESTED_ACTION: {}\n",
            if is_valid {
                "URL format is correct - can proceed with network validation if needed"
            } else {
                "Fix URL format before attempting network validation"
            }
        );
    }

    println!("\n\n=== Using the New validate_for_llm Function ===");
    println!("This function returns a structured string that's easy to parse:\n");

    // Example with the new function
    let test_url = "https://www.linkedin.com/in/test-user";
    println!("Testing URL: {test_url}\n");

    let result = validate_for_llm(test_url);
    println!("Result from validate_for_llm():");
    println!("{result}");

    // Demonstrate async version
    println!("\n=== Async Version ===");
    let runtime = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    runtime.block_on(async {
        let async_result = validate_for_llm_async("https://linkedin.com/in/async-test").await;
        println!("Result from validate_for_llm_async():");
        println!("{async_result}");
    });
}
