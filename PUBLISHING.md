# Publishing Checklist

This checklist helps ensure a smooth release process for Credify.

## Pre-release Checklist

- [x] All tests pass: `cargo test --all`
- [x] Code is properly formatted: `cargo fmt --all -- --check`
- [x] No clippy warnings: `cargo clippy --all-targets --all-features -- -D warnings`
- [x] Documentation builds: `cargo doc --no-deps`
- [x] Examples run successfully:
  - [x] `cargo run --example basic`
  - [x] `cargo run --example batch_validation`
  - [x] `cargo run --example llm_friendly`
  - [x] `cargo run --example llm_simple`
- [x] CHANGELOG.md is updated with new version
- [x] Version number in Cargo.toml is updated (now v0.2.0)
- [x] README.md badges point to correct URLs
- [x] New LLM validation functions documented
- [x] Verbose output format documented

## Publishing to crates.io

1. **Dry run to check everything:**
   ```bash
   cargo publish --dry-run
   ```

2. **Login to crates.io (if not already):**
   ```bash
   cargo login
   ```

3. **Publish to crates.io:**
   ```bash
   cargo publish
   ```

## Publishing to GitHub

1. **Create and push a git tag:**
   ```bash
   git tag -a v0.2.0 -m "Release version 0.2.0 - Enhanced LLM support with verbose validation reports"
   git push origin v0.2.0
   ```

2. **GitHub Actions will automatically:**
   - Run all tests
   - Create a GitHub release
   - Publish to crates.io (if CRATES_TOKEN secret is set)

## Post-release

- [ ] Verify the crate appears on https://crates.io/crates/credify
- [ ] Check that documentation is available at https://docs.rs/credify
- [ ] Update any dependent projects
- [ ] Announce the release (Twitter, Reddit, etc.)

## Setting up GitHub Secrets

For automatic publishing, add these secrets to your GitHub repository:

1. Go to Settings � Secrets � Actions
2. Add `CRATES_TOKEN` with your crates.io API token

To get your crates.io token:
1. Login at https://crates.io
2. Go to Account Settings � API Tokens
3. Create a new token with publish permissions