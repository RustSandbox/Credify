//! Comprehensive tests for AI-optimized validation functions
//!
//! These tests ensure that the AI-friendly API provides consistent,
//! structured data that can be reliably used by AI agents.

use credify::{
    AIDecision, AIValidationResult, ai_validate, ai_validate_async, ai_validate_json,
    ai_validate_json_async,
};
use serde_json::Value;

#[test]
fn test_ai_validate_valid_url() {
    let result = ai_validate("https://linkedin.com/in/johndoe");

    // Should always return a result, never panic
    assert!(result.confidence >= 0.0 && result.confidence <= 1.0);
    assert!(!result.reason.is_empty());
    assert!(!result.metadata.timestamp.is_empty());

    // For valid URLs, username should be extracted
    if result.is_valid {
        assert!(result.username.is_some());
        assert_eq!(result.username.unwrap(), "johndoe");
    }
}

#[test]
fn test_ai_validate_invalid_domain() {
    let result = ai_validate("https://google.com/in/johndoe");

    assert!(!result.is_valid);
    assert_eq!(result.confidence, 1.0); // 100% confident it's not LinkedIn
    assert!(matches!(result.decision, AIDecision::Reject));
    assert!(result.reason.contains("LinkedIn"));
    assert!(result.username.is_none());
}

#[test]
fn test_ai_validate_malformed_url() {
    let result = ai_validate("not-a-url");

    assert!(!result.is_valid);
    assert_eq!(result.confidence, 1.0); // 100% confident it's invalid
    assert!(matches!(result.decision, AIDecision::Reject));
    assert!(result.reason.contains("URL"));
}

#[test]
fn test_ai_validate_json_format() {
    let json_str = ai_validate_json("https://linkedin.com/in/test-user");

    // Should be valid JSON
    let parsed: Result<Value, _> = serde_json::from_str(&json_str);
    assert!(parsed.is_ok());

    let json = parsed.unwrap();

    // Check required fields exist
    assert!(json.get("is_valid").is_some());
    assert!(json.get("confidence").is_some());
    assert!(json.get("decision").is_some());
    assert!(json.get("reason").is_some());
    assert!(json.get("metadata").is_some());

    // Check metadata structure
    let metadata = json.get("metadata").unwrap();
    assert!(metadata.get("timestamp").is_some());
    assert!(metadata.get("url_format_valid").is_some());
    assert!(metadata.get("domain_verified").is_some());
    assert!(metadata.get("profile_pattern_matched").is_some());
}

#[test]
fn test_ai_decision_enum_serialization() {
    // Test that decision enums serialize correctly
    let decisions = vec![
        (AIDecision::Accept, "Accept"),
        (AIDecision::Retry, "Retry"),
        (AIDecision::Reject, "Reject"),
    ];

    for (decision, expected) in decisions {
        let json = serde_json::to_string(&decision).unwrap();
        assert_eq!(json, format!("\"{}\"", expected));
    }
}

#[test]
fn test_confidence_levels() {
    // Test various URL patterns and their expected confidence levels
    let test_cases = vec![
        ("https://linkedin.com/in/valid-user", true, 0.7, 1.0),
        ("https://google.com/in/fake", false, 1.0, 1.0),
        ("not-a-url", false, 1.0, 1.0),
        ("https://linkedin.com/company/microsoft", false, 0.9, 1.0),
    ];

    for (url, should_be_valid, min_confidence, max_confidence) in test_cases {
        let result = ai_validate(url);

        if should_be_valid {
            // For valid URLs, confidence might vary based on HTTP response
            assert!(result.confidence >= min_confidence && result.confidence <= max_confidence);
        } else {
            // For invalid URLs, we should be very confident
            assert!(result.confidence >= min_confidence);
        }
    }
}

#[test]
fn test_metadata_consistency() {
    let url = "https://linkedin.com/in/testuser";
    let result = ai_validate(url);

    // Metadata should always be populated
    assert!(!result.metadata.timestamp.is_empty());

    // Check boolean flags
    if result.is_valid {
        assert!(result.metadata.url_format_valid);
        assert!(result.metadata.domain_verified);
        assert!(result.metadata.profile_pattern_matched);
    }

    // Check error-specific metadata
    if !result.is_valid {
        assert!(result.metadata.error_type.is_some());
    }
}

#[tokio::test]
async fn test_ai_validate_async() {
    let result = ai_validate_async("https://linkedin.com/in/async-test").await;

    // Async should produce same structure as sync
    assert!(result.confidence >= 0.0 && result.confidence <= 1.0);
    assert!(!result.reason.is_empty());
    assert!(!result.metadata.timestamp.is_empty());
}

#[tokio::test]
async fn test_ai_validate_json_async() {
    let json_str = ai_validate_json_async("https://linkedin.com/in/async-json").await;

    // Should be valid JSON
    let parsed: Result<AIValidationResult, _> = serde_json::from_str(&json_str);
    assert!(parsed.is_ok());

    let result = parsed.unwrap();
    assert!(!result.reason.is_empty());
}

#[test]
fn test_ai_decision_logic() {
    // Test that decisions align with validation results
    let test_cases = vec![
        ("https://linkedin.com/in/valid", AIDecision::Accept, true),
        ("https://google.com/profile", AIDecision::Reject, false),
        ("invalid-url", AIDecision::Reject, false),
    ];

    for (url, expected_decision_type, _) in test_cases {
        let result = ai_validate(url);

        match expected_decision_type {
            AIDecision::Accept => {
                // Accept means valid format at minimum
                if result.is_valid {
                    assert!(matches!(result.decision, AIDecision::Accept));
                }
            }
            AIDecision::Reject => {
                // Reject for definitely invalid URLs
                if !result.is_valid && result.confidence >= 0.9 {
                    assert!(matches!(result.decision, AIDecision::Reject));
                }
            }
            _ => {}
        }
    }
}

#[test]
fn test_username_extraction() {
    let test_cases = vec![
        ("https://linkedin.com/in/john-doe", Some("john-doe")),
        ("https://linkedin.com/in/user123", Some("user123")),
        ("https://linkedin.com/in/first.last", Some("first.last")),
        ("https://google.com/in/notlinkedin", None),
        ("not-a-url", None),
    ];

    for (url, expected_username) in test_cases {
        let result = ai_validate(url);
        assert_eq!(result.username, expected_username.map(String::from));
    }
}

#[test]
fn test_error_handling_never_panics() {
    // Test edge cases that should never cause panics
    let edge_cases = vec![
        "",
        " ",
        "http://",
        "https://",
        "linkedin.com",
        "/in/user",
        "https://linkedin.com",
        "https://linkedin.com/",
        "https://linkedin.com/in/",
        "https://linkedin.com/in",
        "https://linkedin.com/in//double-slash",
        "https://linkedin.com/in/user?with=params",
        "https://linkedin.com/in/user#fragment",
    ];

    for url in edge_cases {
        // Should never panic, always return a result
        let result = ai_validate(url);
        assert!(!result.reason.is_empty());
        assert!(result.confidence >= 0.0 && result.confidence <= 1.0);

        // JSON version should also never panic
        let json_str = ai_validate_json(url);
        assert!(!json_str.is_empty());
        assert!(serde_json::from_str::<Value>(&json_str).is_ok());
    }
}

#[test]
fn test_structured_error_messages() {
    // Ensure error messages are structured and helpful for AI agents
    let result = ai_validate("https://google.com/in/user");

    // Should contain actionable information
    assert!(result.reason.contains("LinkedIn"));
    assert!(result.reason.contains("domain") || result.reason.contains("URL"));

    // Should suggest clear action
    assert!(matches!(result.decision, AIDecision::Reject));
}
