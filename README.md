# `cargo-toml-lint` - a linter for `Cargo.toml`

Features:
* Verify toml syntax
* Run `cargo verify-project`
* Check that `[dependencies]` and `[dev-dependencies]` are sorted alphabetically
* Check that `[[test]]` are sorted by test name
* Check all members of top-level object arrays (like) `[[test]]` are placed contiguously
