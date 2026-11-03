//! Property-based tests for locale resolution and tag normalization.
//!
//! Covers spec requirements:
//! - (h) locale resolution determinism
//! - (i) tag normalization

use proptest::prelude::*;
use warp_i18n::locale::{LanguagePreference, Locale, resolve_locale};

// (h) resolve_locale is deterministic: same input -> same output in {En, ZhCn}.
proptest! {
    #[test]
    fn resolve_locale_is_deterministic(
        pref in prop_oneof![
            Just(None),
            Just(Some(LanguagePreference::Zh)),
            Just(Some(LanguagePreference::En)),
            Just(Some(LanguagePreference::System)),
        ],
        sys_tag in prop::option::of(".*"),
    ) {
        let results: Vec<Locale> = (0..100)
            .map(|_| resolve_locale(pref, sys_tag.as_deref()))
            .collect();
        let first = results[0];
        for r in &results {
            prop_assert_eq!(*r, first);
        }
        prop_assert!(matches!(first, Locale::ZhCn | Locale::En));
    }
}

// (h) Falsification: proptest with OneOf x Option<String> snapshot comparison.
proptest! {
    #[test]
    fn resolve_locale_snapshot_consistent(
        pref in prop_oneof![
            Just(None),
            Just(Some(LanguagePreference::Zh)),
            Just(Some(LanguagePreference::En)),
            Just(Some(LanguagePreference::System)),
        ],
        sys_tag in prop::option::of("[a-zA-Z0-9_-]{0,20}"),
    ) {
        let result = resolve_locale(pref, sys_tag.as_deref());
        prop_assert!(matches!(result, Locale::ZhCn | Locale::En));
    }
}

// (i) Tag normalization: parse_bcp47 lowercased prefix matching.
proptest! {
    #[test]
    fn parse_bcp47_normalization_consistent(
        tag in prop_oneof![
            Just("zh".to_string()),
            Just("zh-CN".to_string()),
            Just("zh_Hans_CN".to_string()),
            Just("zh_Hant_TW".to_string()),
            Just("en".to_string()),
            Just("en-US".to_string()),
            Just("ZH".to_string()),
            "[a-zA-Z]{0,12}([-_][a-zA-Z0-9]{0,12}){0,4}",
        ],
    ) {
        let result = Locale::parse_bcp47(&tag);
        let lower = tag.to_ascii_lowercase();
        if lower.trim().is_empty() {
            prop_assert!(result.is_none());
        } else if lower.trim().starts_with("zh") {
            prop_assert_eq!(result, Some(Locale::ZhCn));
        } else if lower.trim().starts_with("en") {
            prop_assert_eq!(result, Some(Locale::En));
        } else {
            prop_assert!(result.is_none());
        }
    }
}

// (i) Falsification with proptest regex-generated tags.
proptest! {
    #[test]
    fn parse_bcp47_idempotent(
        tag in "[a-zA-Z]{1,8}([-_][a-zA-Z0-9]{0,8}){0,4}",
    ) {
        let r1 = Locale::parse_bcp47(&tag);
        let r2 = Locale::parse_bcp47(&tag);
        prop_assert_eq!(r1, r2);
    }
}
