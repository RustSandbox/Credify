# Contributing to Credify

First off, thank you for considering contributing to Credify! It's people like you that make Credify such a great tool.

## Code of Conduct

This project and everyone participating in it is governed by our Code of Conduct. By participating, you are expected to uphold this code. Please be respectful and considerate of others.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check existing issues as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

* **Use a clear and descriptive title** for the issue to identify the problem.
* **Describe the exact steps which reproduce the problem** in as many details as possible.
* **Provide specific examples to demonstrate the steps**. Include links to files or GitHub projects, or copy/pasteable snippets.
* **Describe the behavior you observed after following the steps** and point out what exactly is the problem with that behavior.
* **Explain which behavior you expected to see instead and why.**
* **Include error messages** and stack traces which show the problem.

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

* **Use a clear and descriptive title** for the issue to identify the suggestion.
* **Provide a step-by-step description of the suggested enhancement** in as many details as possible.
* **Provide specific examples to demonstrate the steps**.
* **Describe the current behavior** and **explain which behavior you expected to see instead** and why.
* **Explain why this enhancement would be useful** to most Credify users.

### Pull Requests

1. Fork the repo and create your branch from `main`.
2. If you've added code that should be tested, add tests.
3. If you've changed APIs, update the documentation.
4. Ensure the test suite passes.
5. Make sure your code lints.
6. Issue that pull request!

## Development Process

1. **Setup your development environment:**
   ```bash
   git clone https://github.com/hamzeghalebi/credify.git
   cd credify
   cargo build
   ```

2. **Run tests:**
   ```bash
   cargo test
   ```

3. **Run linting:**
   ```bash
   cargo clippy -- -D warnings
   cargo fmt -- --check
   ```

4. **Run examples:**
   ```bash
   cargo run --example basic
   cargo run --example batch_validation
   cargo run --example llm_friendly
   ```

## Style Guidelines

### Rust Style

* Follow the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
* Use `cargo fmt` to format your code
* Use `cargo clippy` to catch common mistakes
* Write documentation for all public APIs
* Include examples in documentation when appropriate

### Commit Messages

* Use the present tense ("Add feature" not "Added feature")
* Use the imperative mood ("Move cursor to..." not "Moves cursor to...")
* Limit the first line to 72 characters or less
* Reference issues and pull requests liberally after the first line
* Consider starting the commit message with an applicable emoji:
    * 🎨 `:art:` when improving the format/structure of the code
    * 🐎 `:racehorse:` when improving performance
    * 📝 `:memo:` when writing docs
    * 🐛 `:bug:` when fixing a bug
    * 🔥 `:fire:` when removing code or files
    * ✅ `:white_check_mark:` when adding tests
    * 🔒 `:lock:` when dealing with security
    * ⬆️ `:arrow_up:` when upgrading dependencies
    * ⬇️ `:arrow_down:` when downgrading dependencies

### Documentation

* Use rustdoc comments (`///`) for public APIs
* Include examples in documentation
* Keep documentation up to date with code changes
* Use proper markdown formatting

## Testing

* Write tests for new functionality
* Ensure all tests pass before submitting PR
* Include both unit tests and integration tests where appropriate
* Test error cases, not just happy paths

## Versioning

We use [SemVer](http://semver.org/) for versioning. For the versions available, see the [tags on this repository](https://github.com/hamzeghalebi/credify/tags).

## License

By contributing to Credify, you agree that your contributions will be licensed under its MIT OR Apache-2.0 license.

## Questions?

Feel free to open an issue with your question or contact the maintainers directly.

Thank you for contributing! 🎉