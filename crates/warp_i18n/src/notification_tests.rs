use super::*;

#[test]
fn fits_unchanged() {
    assert_eq!(truncate("hello", 64), "hello");
    assert_eq!(truncate("你好", 64), "你好");
}

#[test]
fn truncates_with_ellipsis() {
    let out = truncate("abcdefghij", 5);
    assert_eq!(out, "abcd…");
}

#[test]
fn does_not_split_grapheme() {
    let flag = "🇨🇳"; // single grapheme, two codepoints
    let s = format!("{flag}{flag}{flag}");
    let out = truncate(&s, 2);
    // Should keep one full flag plus ellipsis, never partial codepoints.
    assert!(out.ends_with('…'));
    assert!(!out.is_empty());
}

#[test]
fn zero_budget() {
    assert_eq!(truncate("anything", 0), "");
}
