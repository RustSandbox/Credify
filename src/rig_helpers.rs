//! Ergonomic helper functions for Rig framework integration
//!
//! These functions provide a cleaner, more ergonomic API specifically
//! designed for use with the Rig framework's tool system.

use crate::{AIDecision, ai_validate_async};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Ergonomic validation result optimized for Rig tool responses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RigValidationResult {
    /// Simple pass/fail for the tool
    pub valid: bool,
    /// LinkedIn username if found
    pub username: Option<String>,
    /// Confidence score as percentage (0-100)
    pub confidence: u8,
    /// Human-readable status
    pub status: String,
    /// Suggested action for the AI agent
    pub action: String,
}

/// Validate LinkedIn URL with Rig-optimized response format
///
/// This is the most ergonomic function for Rig tool usage.
/// Returns a simple, clean result that's easy to work with.
///
/// # Example
/// ```no_run
/// use credify::rig_validate;
///
/// # async fn example() {
/// let result = rig_validate("https://linkedin.com/in/johndoe").await;
/// if result.valid {
///     println!("Found profile: @{}", result.username.unwrap_or_default());
/// }
/// # }
///
/// ```
pub async fn rig_validate(url: &str) -> RigValidationResult {
    let result = ai_validate_async(url).await;

    // Convert confidence to percentage
    let confidence = (result.confidence * 100.0) as u8;

    // Determine status and action based on result
    let (status, action) = match (&result.decision, result.is_valid, result.confidence) {
        (AIDecision::Accept, true, conf) if conf >= 0.9 => (
            "✅ Verified LinkedIn profile".to_string(),
            "Use this profile with high confidence".to_string(),
        ),
        (AIDecision::Accept, true, _) => (
            "✅ Valid LinkedIn profile (auth required)".to_string(),
            "Accept this profile - LinkedIn is blocking verification but URL is valid".to_string(),
        ),
        (AIDecision::Retry, _, _) => (
            "⏳ Network issue - retry needed".to_string(),
            "Wait a moment and try validating again".to_string(),
        ),
        (AIDecision::Reject, false, _) if result.metadata.domain_verified => (
            "❌ Not a LinkedIn profile URL".to_string(),
            "This is LinkedIn but not a profile - might be company page".to_string(),
        ),
        (AIDecision::Reject, false, _) => (
            "❌ Invalid LinkedIn URL".to_string(),
            "Search for a different LinkedIn profile URL".to_string(),
        ),
        _ => (
            "❓ Unknown status".to_string(),
            "Review the URL manually".to_string(),
        ),
    };

    RigValidationResult {
        valid: result.is_valid,
        username: result.username,
        confidence,
        status,
        action,
    }
}

/// Quick validation check for Rig - returns just true/false
///
/// Ultra-simple for basic validation needs.
///
/// # Example
/// ```no_run
/// use credify::rig_is_valid;
///
/// # async fn example() {
/// if rig_is_valid("https://linkedin.com/in/user").await {
///     println!("Valid profile!");
/// }
/// # }
/// ```
pub async fn rig_is_valid(url: &str) -> bool {
    rig_validate(url).await.valid
}

/// Get validation as a one-line string for Rig tool responses
///
/// Perfect for simple tool responses that need a human-readable string.
///
/// # Example
/// ```no_run
/// use credify::rig_validate_text;
///
/// # async fn example() {
/// let response = rig_validate_text("https://linkedin.com/in/user").await;
/// // Returns: "✅ Valid profile @user (95% confidence)"
/// # }
/// ```
pub async fn rig_validate_text(url: &str) -> String {
    let result = rig_validate(url).await;

    if result.valid {
        if let Some(username) = result.username {
            format!(
                "{} @{} ({}% confidence)",
                result.status, username, result.confidence
            )
        } else {
            format!("{} ({}% confidence)", result.status, result.confidence)
        }
    } else {
        format!("{} - {}", result.status, result.action)
    }
}

/// Rig-optimized JSON validation with clean structure
///
/// Returns minimal, clean JSON perfect for Rig tool responses.
///
/// # Example
/// ```no_run
/// use credify::rig_validate_json;
///
/// # async fn example() {
/// let json = rig_validate_json("https://linkedin.com/in/user").await;
/// // Returns clean JSON with just the essentials
/// # }
/// ```
pub async fn rig_validate_json(url: &str) -> String {
    let result = rig_validate(url).await;
    serde_json::to_string_pretty(&result).unwrap_or_else(|_| {
        json!({
            "valid": false,
            "status": "Error processing request",
            "action": "Try again later"
        })
        .to_string()
    })
}
