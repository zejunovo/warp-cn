//! Smoke test: exercise the t!() / tr!() macros end-to-end so the proc-macro key
//! validation + bundle loading + render pipeline all wire up.

fn main() {
    warp_i18n::init(warp_i18n::Locale::ZhCn).expect("init");

    println!("zh-CN ok     = {}", warp_i18n::t!("ui-button-ok"));
    println!("zh-CN cancel = {}", warp_i18n::t!("ui-button-cancel"));

    warp_i18n::set_locale(warp_i18n::Locale::En);
    println!("en    ok     = {}", warp_i18n::t!("ui-button-ok"));
    println!("en    cancel = {}", warp_i18n::t!("ui-button-cancel"));

    // Dynamic key path
    let key = "ui-button-save".to_owned();
    println!("dyn   save   = {}", warp_i18n::tr!(&key));
}
