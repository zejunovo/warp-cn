// compile_fail: t!() requires a string literal, not a function call.
use warp_i18n::t;

fn get_key() -> &'static str {
    "ui-button-ok"
}

fn main() {
    let _label = t!(get_key()); //~ ERROR
}
