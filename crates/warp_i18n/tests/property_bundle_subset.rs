//! Property-based tests for bundle key subset relations.
//!
//! Covers spec requirement:
//! - (j) bundle subset relation

use proptest::prelude::*;
use warp_i18n::loader::Bundles;

// (j) Key missing in both locales returns placeholder.
proptest! {
    #[test]
    fn missing_key_is_placeholder(
        en_pairs in prop::collection::vec(
            (r"[a-z][a-z0-9-]{0,20}", r"[a-zA-Z0-9 ]{1,20}"),
            2..10,
        ),
        zh_pairs in prop::collection::vec(
            (r"[a-z][a-z0-9-]{0,20}", r"[a-zA-Z0-9 ]{1,20}"),
            1..10,
        ),
    ) {
        let mut en_ftl = String::new();
        for (k, v) in &en_pairs {
            en_ftl.push_str(&format!("{k} = {v}\n"));
        }
        let mut zh_ftl = String::new();
        for (k, v) in &zh_pairs {
            zh_ftl.push_str(&format!("{k} = {v}\n"));
        }

        let Ok(bundles) = Bundles::from_sources(&en_ftl, &zh_ftl) else {
            return Ok(());
        };

        let result = bundles.render(warp_i18n::Locale::ZhCn, "nonexistent-key", None);
        prop_assert_eq!(result, "{nonexistent-key}");
    }
}

// (j) Keys defined in zh-CN render their zh-CN value, not en.
proptest! {
    #[test]
    fn zh_keys_render_zh_value(
        en_pairs in prop::collection::vec(
            (r"[a-z][a-z0-9-]{0,20}", r"[A-Za-z0-9 ]{1,20}"),
            3..8,
        ),
        zh_pairs in prop::collection::vec(
            (r"[a-z][a-z0-9-]{0,20}", r"[一-龥a-zA-Z0-9]{1,20}"),
            3..8,
        ),
    ) {
        let mut en_ftl = String::new();
        for (k, v) in &en_pairs {
            en_ftl.push_str(&format!("{k} = {v}\n"));
        }
        let mut zh_ftl = String::new();
        for (k, v) in &zh_pairs {
            zh_ftl.push_str(&format!("{k} = {v}\n"));
        }

        let Ok(bundles) = Bundles::from_sources(&en_ftl, &zh_ftl) else {
            return Ok(());
        };

        for (key, zh_val) in &zh_pairs {
            let rendered = bundles.render(warp_i18n::Locale::ZhCn, key, None);
            prop_assert_eq!(rendered, zh_val.as_str());
        }
    }
}
