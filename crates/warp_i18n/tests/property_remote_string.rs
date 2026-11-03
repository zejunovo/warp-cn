//! Compile-fail tests that keep translation keys separate from dynamic content.
//!
//! t!() proc-macro only accepts string literals (LitStr). Any non-literal expression —
//! including `RemoteString::as_str()` — is rejected at compile time with a clear error.

#[test]
fn t_macro_rejects_non_literal_keys() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/t_macro_rejects_variable.rs");
    t.compile_fail("tests/compile_fail/t_macro_rejects_expr.rs");
}
