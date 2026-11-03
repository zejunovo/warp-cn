//! Property-based tests for render purity and side-effect freedom.
//!
//! Covers spec requirements:
//! - (a) render is a pure function: fixed (locale, key, args) -> bitwise equal String
//! - (c) rendering preserves the instance locale and subsequent lookup results

use proptest::prelude::*;
use warp_i18n::loader::Bundles;
use warp_i18n::{FluentValue, I18n, Locale, args_from};

// (a) render with known production keys is deterministic.
proptest! {
    #[test]
    fn render_known_keys_deterministic(
        key in prop::sample::select(&[
            "ui-button-ok", "ui-button-cancel", "ui-button-save",
            "ui-button-discard", "ui-button-close", "ui-button-confirm",
            "ui-button-retry", "ui-button-default",
        ]),
        locale in prop_oneof![Just(Locale::ZhCn), Just(Locale::En)],
    ) {
        let i18n = I18n::new(locale).expect("bundles load");
        let first = i18n.render(key, None);
        for _ in 0..500 {
            prop_assert_eq!(&i18n.render(key, None), &first);
        }
    }
}

// (a) Falsification: with named args, render is deterministic.
proptest! {
    #[test]
    fn render_with_args_deterministic(
        n in 0u32..1000u32,
        locale in prop_oneof![Just(Locale::ZhCn), Just(Locale::En)],
    ) {
        let i18n = I18n::new(locale).expect("bundles load");
        let args = args_from([("n", FluentValue::from(n))]);
        let first = i18n.render("tabs-close-confirm", Some(&args));
        for _ in 0..500 {
            prop_assert_eq!(&i18n.render("tabs-close-confirm", Some(&args)), &first);
        }
    }
}

// (a) render with synthetic bundles is deterministic.
proptest! {
    #[test]
    fn render_deterministic_with_synthetic(
        key in r"[a-z][a-z0-9-]{0,20}",
        en_val in r"[A-Za-z0-9 ]{1,30}",
        zh_val in r"[A-Za-z0-9 ]{1,30}",
        locale in prop_oneof![Just(Locale::ZhCn), Just(Locale::En)],
    ) {
        let en_ftl = format!("{key} = {en_val}\n");
        let zh_ftl = format!("{key} = {zh_val}\n");
        let Ok(bundles) = Bundles::from_sources(&en_ftl, &zh_ftl) else {
            return Ok(());
        };

        let first = bundles.render(locale, &key, None);
        for _ in 0..200 {
            prop_assert_eq!(&bundles.render(locale, &key, None), &first);
        }
    }
}

// (c) render preserves the instance locale.
proptest! {
    #[test]
    fn render_has_no_side_effects(
        key in prop::sample::select(&[
            "ui-button-ok", "ui-button-cancel", "ui-button-save",
        ]),
        locale in prop_oneof![Just(Locale::ZhCn), Just(Locale::En)],
    ) {
        let i18n = I18n::new(locale).expect("bundles load");
        let locale_before = i18n.current();
        let val_before = i18n.render(key, None);

        for _ in 0..300 {
            let _ = i18n.render(key, None);
        }

        prop_assert_eq!(i18n.current(), locale_before);
        prop_assert_eq!(i18n.render(key, None), val_before);
    }
}
