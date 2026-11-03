// compile_fail: t!() requires a string literal, not a variable.
use warp_i18n::t;

fn main() {
    let key = "ui-button-ok";
    let _label = t!(key); //~ ERROR
}
