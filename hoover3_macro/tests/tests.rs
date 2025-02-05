//! Integration tests for the hoover3_macro crate.

/// Test that the macros expand correctly.
/// After changing the macros, delete affected `*.expanded.rs` files and run this test again.
#[test]
pub fn pass() {
    macrotest::expand("tests/expand/*.rs");
}
