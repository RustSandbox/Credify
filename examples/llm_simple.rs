//! Simple example of using the LLM-friendly validation functions

use credify::{validate_for_llm, validate_for_llm_async};

fn main() {
    // Example URLs to test
    let urls = vec![
        "https://www.linkedin.com/in/valid-user",
        "https://www.google.com/in/someone",
        "not-a-url",
    ];

    println!("=== Synchronous LLM Validation ===\n");
    
    for url in &urls {
        println!("Validating: {url}\n");
        let result = validate_for_llm(url);
        println!("{result}");
        println!("---\n");
    }

    // Async example
    println!("=== Asynchronous LLM Validation ===\n");
    
    let runtime = tokio::runtime::Runtime::new().expect("Failed to create runtime");
    runtime.block_on(async {
        for url in &urls {
            println!("Validating (async): {url}\n");
            let result = validate_for_llm_async(url).await;
            println!("{result}");
            println!("---\n");
        }
    });

    // Example of parsing the result
    println!("=== Parsing Example ===\n");
    
    let result = validate_for_llm("https://linkedin.com/in/test");
    
    // Simple parsing example
    for line in result.lines() {
        if line.starts_with("VALIDATION_RESULT:") {
            let status = line.split(':').nth(1).unwrap_or("").trim();
            println!("Validation status: {status}");
        }
        if line.starts_with("ERROR_TYPE:") {
            let error_type = line.split(':').nth(1).unwrap_or("").trim();
            println!("Error type: {error_type}");
        }
        if line.starts_with("SUGGESTED_ACTION:") {
            let action = line.split(':').nth(1).unwrap_or("").trim();
            println!("Suggested action: {action}");
        }
    }
}