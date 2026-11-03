use std::sync::Arc;
use std::thread;

use warp_i18n::{I18n, Locale};

#[test]
fn concurrent_switches_always_render_a_complete_supported_translation() {
    let i18n = Arc::new(I18n::new(Locale::ZhCn).expect("bundles load"));
    let mut handles = Vec::new();

    for _ in 0..4 {
        let i18n = Arc::clone(&i18n);
        handles.push(thread::spawn(move || {
            for locale in [Locale::ZhCn, Locale::En].into_iter().cycle().take(50) {
                i18n.set_locale(locale);
            }
        }));
    }

    for _ in 0..8 {
        let i18n = Arc::clone(&i18n);
        handles.push(thread::spawn(move || {
            for _ in 0..50 {
                assert!(matches!(
                    i18n.render("ui-button-ok", None).as_str(),
                    "确定" | "OK"
                ));
            }
        }));
    }

    for handle in handles {
        handle.join().expect("locale worker finishes");
    }
}
