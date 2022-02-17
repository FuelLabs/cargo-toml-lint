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

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
