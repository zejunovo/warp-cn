use std::sync::OnceLock;

use crate::{Bundles, FluentValue, I18n, Locale, args_from, init, intern, t};

fn bundles() -> &'static Bundles {
    static BUNDLES: OnceLock<Bundles> = OnceLock::new();
    BUNDLES.get_or_init(|| Bundles::load().expect("production bundles must load"))
}

#[test]
fn renders_simplified_chinese() {
    assert_eq!(bundles().render(Locale::ZhCn, "ui-button-ok", None), "确定");
}

#[test]
fn renders_english() {
    assert_eq!(bundles().render(Locale::En, "ui-button-ok", None), "OK");
}

#[test]
fn billing_suffixes_preserve_leading_spaces_without_displaying_quotes() {
    assert_eq!(
        bundles().render(Locale::En, "settings-billing-to-continue", None),
        " to continue."
    );
    assert_eq!(
        bundles().render(Locale::ZhCn, "settings-billing-to-continue", None),
        " 以继续。"
    );
}

#[test]
fn billing_conjunction_preserves_both_spaces_without_displaying_quotes() {
    assert_eq!(
        bundles().render(Locale::En, "settings-billing-or-byok", None),
        " or "
    );
    assert_eq!(
        bundles().render(Locale::ZhCn, "settings-billing-or-byok", None),
        " 或 "
    );
}

#[test]
fn quotation_marks_in_visible_text_are_preserved() {
    let bundles = Bundles::from_sources(
        "instruction = Click \"Save\".\n",
        "instruction = 点击\"保存\"。\n",
    )
    .expect("test bundles load");
    assert_eq!(
        bundles.render(Locale::En, "instruction", None),
        "Click \"Save\"."
    );
    assert_eq!(
        bundles.render(Locale::ZhCn, "instruction", None),
        "点击\"保存\"。"
    );
}

#[test]
fn missing_chinese_translation_falls_back_to_english() {
    let bundles = Bundles::from_sources(
        "fallback-only = Fallback only string\n",
        "other-key = 其他字符串\n",
    )
    .expect("test bundles load");
    assert_eq!(
        bundles.render(Locale::ZhCn, "fallback-only", None),
        "Fallback only string"
    );
}

#[test]
fn unknown_key_has_a_visible_placeholder() {
    assert_eq!(
        bundles().render(Locale::En, "does-not-exist-anywhere", None),
        "{does-not-exist-anywhere}"
    );
}

#[test]
fn locale_switch_can_round_trip_without_reloading_bundles() {
    let i18n = I18n::new(Locale::ZhCn).expect("bundles load");
    assert_eq!(i18n.render("ui-button-cancel", None), "取消");
    i18n.set_locale(Locale::En);
    assert_eq!(i18n.render("ui-button-cancel", None), "Cancel");
    i18n.set_locale(Locale::ZhCn);
    assert_eq!(i18n.render("ui-button-cancel", None), "取消");
}

#[test]
fn watchers_observe_the_published_locale() {
    let i18n = I18n::new(Locale::ZhCn).expect("bundles load");
    let mut receiver = i18n.subscribe();
    i18n.set_locale(Locale::En);
    assert!(receiver.has_changed().expect("sender remains alive"));
    assert_eq!(*receiver.borrow_and_update(), Locale::En);
    assert_eq!(i18n.current(), Locale::En);
    assert_eq!(i18n.render("ui-button-save", None), "Save");
}

#[test]
fn late_subscribers_observe_changes_made_without_receivers() {
    let i18n = I18n::new(Locale::ZhCn).expect("bundles load");
    i18n.set_locale(Locale::En);
    assert_eq!(*i18n.subscribe().borrow(), Locale::En);
}

#[test]
fn chinese_named_arguments_preserve_the_count() {
    let args = args_from([("n", FluentValue::from(3))]);
    assert_eq!(
        bundles().render(Locale::ZhCn, "tabs-close-confirm", Some(&args)),
        "关闭 3 个标签页"
    );
}

#[test]
fn english_plural_rules_distinguish_one_and_many() {
    let one = args_from([("n", FluentValue::from(1))]);
    let many = args_from([("n", FluentValue::from(5))]);
    assert_eq!(
        bundles().render(Locale::En, "tabs-close-confirm", Some(&one)),
        "Close 1 tab"
    );
    assert_eq!(
        bundles().render(Locale::En, "tabs-close-confirm", Some(&many)),
        "Close 5 tabs"
    );
}

#[test]
fn interpolated_user_content_is_preserved() {
    let bundles = Bundles::from_sources(
        "open-file = Open { $path }\n",
        "open-file = 打开 { $path }\n",
    )
    .expect("test bundles load");
    let args = args_from([("path", FluentValue::from("C:\\项目\\README.md --help"))]);
    assert_eq!(
        bundles.render(Locale::ZhCn, "open-file", Some(&args)),
        "打开 C:\\项目\\README.md --help"
    );
}

#[test]
fn duplicate_resource_keys_are_rejected() {
    assert!(Bundles::from_sources("duplicate = One\nduplicate = Two\n", "other = 其他\n").is_err());
}

#[test]
fn malformed_resources_are_rejected() {
    assert!(Bundles::from_sources("broken = {\n", "other = 其他\n").is_err());
}

#[test]
fn interning_reuses_equal_static_labels() {
    let first = intern("固定界面标签".to_owned());
    let second = intern("固定界面标签".to_owned());
    assert!(std::ptr::eq(first, second));
}

#[test]
fn global_initialization_is_idempotent_and_macro_lookup_works() {
    let first = init(Locale::ZhCn).expect("bundles load");
    let second = init(Locale::En).expect("existing initialization");
    assert!(std::ptr::eq(first, second));
    assert_eq!(first.current(), Locale::ZhCn);
    assert_eq!(t!("ui-button-ok"), "确定");
}
