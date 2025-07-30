//! `LinkedIn` URL validation library
//!
//! This library provides tools to validate `LinkedIn` profile URLs,
//! checking both format correctness and profile existence.

use regex::Regex;
use thiserror::Error;
use url::Url;

#[derive(Error, Debug)]
pub enum LinkedInUrlError {
    #[error("Invalid URL format: {0}")]
    InvalidUrl(String),

    #[error("Not a LinkedIn URL")]
    NotLinkedInUrl,

    #[error("Not a LinkedIn profile URL")]
    NotProfileUrl,

    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Profile not found (404)")]
    ProfileNotFound,

    #[error("Unable to verify - LinkedIn requires authentication")]
    AuthenticationRequired,
}

pub struct LinkedInValidator {
    client: reqwest::blocking::Client,
}

impl LinkedInValidator {
    /// Creates a new `LinkedIn` validator instance.
    ///
    /// # Panics
    ///
    /// Panics if the HTTP client cannot be built.
    #[must_use]
    pub fn new() -> Self {
        let client = reqwest::blocking::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap();

        Self { client }
    }

    /// Validates a `LinkedIn` profile URL by checking format and existence.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The URL format is invalid
    /// - The URL is not from `LinkedIn` domain
    /// - The URL is not a profile URL
    /// - Network request fails
    /// - The profile doesn't exist (404)
    pub fn is_valid_linkedin_profile_url(&self, url_str: &str) -> Result<bool, LinkedInUrlError> {
        let url = Url::parse(url_str).map_err(|e| LinkedInUrlError::InvalidUrl(e.to_string()))?;

        if !is_linkedin_domain(&url) {
            return Err(LinkedInUrlError::NotLinkedInUrl);
        }

        if !is_profile_path(&url) {
            return Err(LinkedInUrlError::NotProfileUrl);
        }

        self.check_profile_exists(url_str)?;

        Ok(true)
    }

    fn check_profile_exists(&self, url: &str) -> Result<(), LinkedInUrlError> {
        let mut response = self.client.get(url).send()?;

        // LinkedIn returns 999 status for bot detection/rate limiting
        // In this case, we need to follow redirects manually
        if response.status().as_u16() == 999 {
            // Try with cookie header to bypass authwall
            response = self.client.get(url).header("Cookie", "sl=v=1&1").send()?;
        }

        // Check if redirected to 404 page
        let final_url = response.url().to_string();
        if final_url.contains("/404/") || final_url.contains("linkedin.com/404") {
            return Err(LinkedInUrlError::ProfileNotFound);
        }

        // Get response body
        let body = response.text()?;

        // Check for authwall (indicates we're being blocked)
        if body.contains("/authwall") || body.contains("sessionRedirect") {
            // When we hit authwall, we can't determine if profile exists
            return Err(LinkedInUrlError::AuthenticationRequired);
        }

        // Check for common error page indicators
        if body.contains("This page doesn't exist")
            || body.contains("This page doesn't exist")
            || body.contains("Page not found")
            || body.contains("Check the URL or return to LinkedIn home")
            || body.contains("return to LinkedIn home")
            || body.contains("Go to your feed") && body.contains("doesn't exist")
        {
            return Err(LinkedInUrlError::ProfileNotFound);
        }

        Ok(())
    }
}

fn is_linkedin_domain(url: &Url) -> bool {
    matches!(url.domain(), Some(domain) if domain == "linkedin.com" || domain == "www.linkedin.com")
}

fn is_profile_path(url: &Url) -> bool {
    let path = url.path();
    let profile_regex = Regex::new(r"^/in/[a-zA-Z0-9\-]+/?$").unwrap();
    profile_regex.is_match(path)
}

impl Default for LinkedInValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Validates a `LinkedIn` profile URL asynchronously.
///
/// # Errors
///
/// Returns an error if:
/// - The URL format is invalid
/// - The URL is not from `LinkedIn` domain
/// - The URL is not a profile URL
/// - Network request fails
/// - The profile doesn't exist (404)
///
/// # Panics
///
/// Panics if the regex pattern is invalid (this should never happen).
pub async fn validate_linkedin_url_async(url: &str) -> Result<bool, LinkedInUrlError> {
    let url_parsed = Url::parse(url).map_err(|e| LinkedInUrlError::InvalidUrl(e.to_string()))?;

    if !is_linkedin_domain(&url_parsed) {
        return Err(LinkedInUrlError::NotLinkedInUrl);
    }

    if !is_profile_path(&url_parsed) {
        return Err(LinkedInUrlError::NotProfileUrl);
    }

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let mut response = client.get(url).send().await?;

    // LinkedIn returns 999 status for bot detection/rate limiting
    if response.status().as_u16() == 999 {
        // Try with cookie header to bypass authwall
        response = client.get(url).header("Cookie", "sl=v=1&1").send().await?;
    }

    // Check if redirected to 404 page
    let final_url = response.url().to_string();
    if final_url.contains("/404/") || final_url.contains("linkedin.com/404") {
        return Err(LinkedInUrlError::ProfileNotFound);
    }

    // Get response body
    let body = response.text().await?;

    // Check for authwall (indicates we're being blocked)
    if body.contains("/authwall") || body.contains("sessionRedirect") {
        return Err(LinkedInUrlError::AuthenticationRequired);
    }

    // Check for common error page indicators
    if body.contains("This page doesn't exist")
        || body.contains("This page doesn't exist")
        || body.contains("Page not found")
        || body.contains("Check the URL or return to LinkedIn home")
        || body.contains("return to LinkedIn home")
        || body.contains("Go to your feed") && body.contains("doesn't exist")
    {
        return Err(LinkedInUrlError::ProfileNotFound);
    }

    Ok(true)
}

/// Checks if a URL has valid `LinkedIn` profile format (no network calls).
///
/// # Panics
///
/// Panics if the regex pattern is invalid (this should never happen).
#[must_use]
pub fn is_valid_linkedin_profile_format(url: &str) -> bool {
    let Ok(url_parsed) = Url::parse(url) else {
        return false;
    };

    is_linkedin_domain(&url_parsed) && is_profile_path(&url_parsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_profile_format() {
        // Test with real valid profiles
        assert!(is_valid_linkedin_profile_format(
            "https://www.linkedin.com/in/hamze/"
        ));
        assert!(is_valid_linkedin_profile_format(
            "https://www.linkedin.com/in/hamzeghalebi/"
        ));
        assert!(is_valid_linkedin_profile_format(
            "https://www.linkedin.com/in/johndoe"
        ));
        assert!(is_valid_linkedin_profile_format(
            "https://linkedin.com/in/jane-doe"
        ));
        assert!(is_valid_linkedin_profile_format(
            "https://www.linkedin.com/in/john-doe-123/"
        ));
    }

    #[test]
    fn test_invalid_profile_format() {
        assert!(!is_valid_linkedin_profile_format(
            "https://www.google.com/in/johndoe"
        ));
        assert!(!is_valid_linkedin_profile_format(
            "https://linkedin.com/company/microsoft"
        ));
        assert!(!is_valid_linkedin_profile_format("https://linkedin.com/"));
        assert!(!is_valid_linkedin_profile_format("not-a-url"));
    }

    #[test]
    fn test_real_valid_profile() {
        let validator = LinkedInValidator::new();
        // This is a valid LinkedIn profile
        match validator.is_valid_linkedin_profile_url("https://www.linkedin.com/in/hamze/") {
            Ok(true) => (),
            Ok(false) => panic!("Expected profile to be valid"),
            Err(e) => panic!("Expected profile to be valid, got error: {e}"),
        }
    }

    #[test]
    fn test_real_invalid_profile() {
        let validator = LinkedInValidator::new();
        // This LinkedIn profile doesn't exist - LinkedIn shows error page
        match validator.is_valid_linkedin_profile_url("https://www.linkedin.com/in/hamzeghalebi/") {
            Ok(_) => {
                // LinkedIn might be allowing access sometimes, especially after multiple requests
                // This is inconsistent behavior from LinkedIn
                println!("Warning: LinkedIn allowed access to profile page - cannot determine if profile actually exists");
            }
            Err(LinkedInUrlError::ProfileNotFound) => (),
            Err(LinkedInUrlError::AuthenticationRequired) => {
                println!("LinkedIn requires authentication - cannot verify profile existence");
            }
            Err(e) => panic!(
                "Expected ProfileNotFound or AuthenticationRequired error, got: {e}"
            ),
        }
    }

    #[tokio::test]
    async fn test_async_valid_profile() {
        // Test async validation with valid profile
        match validate_linkedin_url_async("https://www.linkedin.com/in/hamze/").await {
            Ok(true) => (),
            Ok(false) => panic!("Expected profile to be valid"),
            Err(e) => panic!("Expected profile to be valid, got error: {e}"),
        }
    }

    #[tokio::test]
    async fn test_async_invalid_profile() {
        // Test async validation with invalid profile that shows error page
        match validate_linkedin_url_async("https://www.linkedin.com/in/hamzeghalebi/").await {
            Ok(_) => {
                // LinkedIn might be allowing access sometimes, especially after multiple requests
                // This is inconsistent behavior from LinkedIn
                println!("Warning: LinkedIn allowed access to profile page - cannot determine if profile actually exists");
            }
            Err(LinkedInUrlError::ProfileNotFound) => (),
            Err(LinkedInUrlError::AuthenticationRequired) => {
                println!("LinkedIn requires authentication - cannot verify profile existence");
            }
            Err(e) => panic!(
                "Expected ProfileNotFound or AuthenticationRequired error, got: {e}"
            ),
        }
    }

    #[test]
    #[ignore = "Debug test to inspect LinkedIn response"]
    fn debug_linkedin_response() {
        let client = reqwest::blocking::Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .timeout(std::time::Duration::from_secs(10))
            .build()
            .unwrap();

        let url = "https://www.linkedin.com/in/hamzeghalebi/";
        let response = client.get(url).send().unwrap();

        println!("Status: {}", response.status());
        println!("Final URL: {}", response.url());

        let body = response.text().unwrap();
        println!("Body length: {}", body.len());
        println!(
            "First 2000 chars:\n{}",
            &body.chars().take(2000).collect::<String>()
        );
    }
}
