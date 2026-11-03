//! Leak-once string interning for cases where downstream APIs require
//! `&'static str`. The first time a given content is requested it is leaked
//! into the heap and cached; subsequent requests return the same pointer.
//!
//! Memory cost is bounded by the number of unique rendered strings across
//! all locales (≈ keys × locales). Locale switches re-enter and may add a
//! second `'static` per key (one per locale that has been active), which is
//! acceptable for the fork's two-locale design.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};

static INTERN: OnceLock<Mutex<HashMap<String, &'static str>>> = OnceLock::new();

pub fn intern(s: String) -> &'static str {
    let cell = INTERN.get_or_init(|| Mutex::new(HashMap::new()));
    let mut map = cell.lock().unwrap();
    if let Some(&existing) = map.get(&s) {
        return existing;
    }
    let leaked: &'static str = Box::leak(s.clone().into_boxed_str());
    map.insert(s, leaked);
    leaked
}
