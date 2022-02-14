# `cargo-toml-lint` - a linter for `Cargo.toml`

Features:

* Verify toml syntax
* Run `cargo verify-project`
* Check that `[dependencies]` and `[dev-dependencies]` are sorted alphabetically
* Check that `[[test]]` are sorted by test name
* Check all members of top-level object arrays (like) `[[test]]` are placed contiguously

This is a best-effort linter. Currently custom parsing is really simplified, so it may:

* Reject some valid files if they are written in some particularly obscure way
* Accept some files that violate the given linting rules

However, any such issues are considered bugs and a fix PR would be appreciated.
