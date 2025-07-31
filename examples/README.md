# Credify Examples for Rig Framework Integration

This directory contains examples demonstrating how to use Credify with various AI frameworks and use cases.

## Rig Framework Integration Examples

### 1. Simple Validation (`rig_xai_simple.rs`)

Basic example showing LinkedIn URL validation without external dependencies:

```bash
cargo run --example rig_xai_simple
```

Features:
- Direct validation of multiple URLs
- JSON response parsing
- Tool definition for AI agents

### 2. Complete Integration (`rig_complete_integration.rs`)

Full example demonstrating Rig-style tool implementation:

```bash
cargo run --example rig_complete_integration
```

Features:
- Complete Tool trait implementation
- Mock agent system
- Query processing
- Batch validation
- Error handling

### 3. Mock xAI Integration (`rig_mock_xai.rs`)

Demonstrates the xAI/Grok integration pattern without requiring API keys:

```bash
cargo run --example rig_mock_xai
```

Features:
- Mock Rig framework types
- xAI client simulation
- Tool calling demonstration
- System prompts for AI guidance

### 4. Working Rig Pattern (`rig_working_example.rs`)

Complete working example demonstrating the Rig tool pattern:

```bash
cargo run --example rig_working_example
```

Features:
- Full tool implementation following Rig's pattern
- Mock agent system for testing
- Direct tool usage examples
- No external API dependencies

### 5. Real xAI Integration (`rig_xai_integration.rs`)

Full integration with xAI's Grok model (requires XAI_API_KEY):

```bash
# Set your API key
export XAI_API_KEY='your-api-key-here'

# Or use a .env file
echo "XAI_API_KEY=your-api-key-here" > .env

# Run the example
cargo run --example rig_xai_integration
```

Features:
- Real xAI/Grok integration
- LinkedIn profile search
- Tool-based validation
- Error handling with helpful messages

## Other Examples

### Basic Validation (`basic.rs`)

Simple synchronous validation:

```bash
cargo run --example basic
```

### Batch Validation (`batch_validator.rs`)

Validate multiple URLs concurrently:

```bash
cargo run --example batch_validator
```

### LLM-Friendly Output (`llm_simple.rs`)

Get verbose, structured output for LLMs:

```bash
cargo run --example llm_simple
```

### AI Agent Demo (`ai_agent_demo.rs`)

Complete AI agent implementation with decision logic:

```bash
cargo run --example ai_agent_demo
```

## Key Patterns

### 1. Tool Definition

```rust
impl Tool for LinkedInChecker {
    const NAME: &'static str = "validate_linkedin_profile";
    
    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Validates LinkedIn profiles".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "url": {
                        "type": "string",
                        "description": "LinkedIn URL"
                    }
                },
                "required": ["url"]
            }),
        }
    }
    
    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        Ok(credify::rig_validate_json(&args.url).await)
    }
}
```

### 2. Agent Integration

```rust
let agent = client
    .agent("gpt-4")
    .preamble("You are a LinkedIn profile validator")
    .tool(LinkedInChecker)
    .build();

let response = agent
    .prompt("Validate https://linkedin.com/in/johndoe")
    .await?;
```

### 3. Error Handling

```rust
match validator.call(args).await {
    Ok(result) => {
        // Process successful validation
        println!("Valid: {}", result.valid);
    }
    Err(e) => {
        // Handle errors gracefully
        eprintln!("Validation error: {}", e);
    }
}
```

## Tips

1. **Always use async**: All Rig helpers are async to prevent blocking
2. **Handle errors**: Check for network issues, auth requirements, and invalid URLs
3. **Use confidence scores**: Make decisions based on the confidence level
4. **Batch processing**: Process multiple URLs concurrently for better performance
5. **Mock testing**: Use the mock examples to test without API keys