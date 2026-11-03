use super::*;

#[test]
fn parse_bcp47_handles_chinese_variants() {
    for tag in ["zh-CN", "zh-Hans-CN", "zh", "zh_TW", "  zh-Hant  "] {
        assert_eq!(Locale::parse_bcp47(tag), Some(Locale::ZhCn), "tag={tag}");
    }
}

#[test]
fn parse_bcp47_handles_english_variants() {
    for tag in ["en", "en-US", "en_GB"] {
        assert_eq!(Locale::parse_bcp47(tag), Some(Locale::En), "tag={tag}");
    }
}

#[test]
fn parse_bcp47_rejects_unknown() {
    assert_eq!(Locale::parse_bcp47(""), None);
    assert_eq!(Locale::parse_bcp47("ja"), None);
    assert_eq!(Locale::parse_bcp47("fr-FR"), None);
}

#[test]
fn resolve_explicit_overrides_system() {
    assert_eq!(
        resolve_locale(Some(LanguagePreference::En), Some("zh-CN")),
        Locale::En
    );
    assert_eq!(
        resolve_locale(Some(LanguagePreference::Zh), Some("en-US")),
        Locale::ZhCn
    );
}

#[test]
fn resolve_system_falls_back_to_en_when_unknown() {
    assert_eq!(
        resolve_locale(Some(LanguagePreference::System), Some("ja-JP")),
        Locale::En
    );
    assert_eq!(
        resolve_locale(Some(LanguagePreference::System), None),
        Locale::En
    );
}

#[test]
fn missing_preference_defaults_to_zh() {
    assert_eq!(resolve_locale(None, Some("en-US")), Locale::ZhCn);
    assert_eq!(resolve_locale(None, None), Locale::ZhCn);
}
