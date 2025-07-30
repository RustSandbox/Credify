# Contributing to linkedin-profile-validator

First off, thank you for considering contributing to linkedin-profile-validator! It's people like you that make this tool better for everyone.

## Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code. Please be respectful and considerate in all interactions.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

* **Use a clear and descriptive title** for the issue to identify the problem.
* **Describe the exact steps which reproduce the problem** in as many details as possible.
* **Provide specific examples to demonstrate the steps**.
* **Describe the behavior you observed after following the steps** and point out what exactly is the problem with that behavior.
* **Explain which behavior you expected to see instead and why.**
* **Include Rust version** (`rustc --version`).

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

* **Use a clear and descriptive title** for the issue to identify the suggestion.
* **Provide a step-by-step description of the suggested enhancement** in as many details as possible.
* **Provide specific examples to demonstrate the steps**.
* **Describe the current behavior** and **explain which behavior you expected to see instead** and why.
* **Explain why this enhancement would be useful** to most users.

### Pull Requests

1. Fork the repo and create your branch from `main`.
2. If you've added code that should be tested, add tests.
3. If you've changed APIs, update the documentation.
4. Ensure the test suite passes (`cargo test`).
5. Make sure your code follows the project style (`cargo fmt` and `cargo clippy`).
6. Issue that pull request!

## Development Process

1. **Setup your development environment:**
   ```bash
   git clone https://github.com/hamzeghalebi/linkedin-profile-validator.git
   cd linkedin-profile-validator
   cargo build
   ```

2. **Run tests:**
   ```bash
   cargo test
   ```

3. **Check formatting and linting:**
   ```bash
   cargo fmt -- --check
   cargo clippy -- -D warnings
   ```

4. **Run the example:**
   ```bash
   cargo run --example basic
   ```

## Testing Guidelines

* Write tests for any new functionality
* Ensure all tests pass before submitting PR
* Include both positive and negative test cases
* Mock external API calls when possible to avoid rate limiting

## Documentation

* Document all public APIs with doc comments
* Include examples in doc comments where appropriate
* Update README.md if you change how the library is used
* Update CHANGELOG.md following the Keep a Changelog format

## Commit Messages

* Use the present tense ("Add feature" not "Added feature")
* Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
* Limit the first line to 72 characters or less
* Reference issues and pull requests liberally after the first line

## Questions?

Feel free to open an issue with your question or reach out to the maintainers directly.

Thank you for contributing! 🎉