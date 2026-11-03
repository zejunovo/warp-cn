use std::fmt;

use serde::{Deserialize, Serialize};
use unic_langid::{LanguageIdentifier, langid};

/// Supported UI locales.
///
/// Forks add new variants here and update the loader fallback chain in `loader.rs`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Locale {
    #[serde(rename = "zh-CN")]
    ZhCn,
    #[serde(rename = "en")]
    En,
}

impl Locale {
    /// Default locale for this fork: simplified Chinese.
    pub const DEFAULT: Self = Self::ZhCn;

    pub fn as_langid(&self) -> LanguageIdentifier {
        match self {
            Self::ZhCn => langid!("zh-CN"),
            Self::En => langid!("en"),
        }
    }

    pub fn as_bcp47(&self) -> &'static str {
        match self {
            Self::ZhCn => "zh-CN",
            Self::En => "en",
        }
    }

    /// Parse a BCP-47 tag into one of the supported locales.
    ///
    /// Normalisation rules: leading whitespace and trailing region/script subtags are tolerated;
    /// any `zh*` tag maps to `ZhCn`; `en*` maps to `En`. Unknown tags return `None`.
    pub fn parse_bcp47(tag: &str) -> Option<Self> {
        let trimmed = tag.trim();
        if trimmed.is_empty() {
            return None;
        }
        let primary = trimmed.split(['-', '_']).next().unwrap_or(trimmed);
        match primary.to_ascii_lowercase().as_str() {
            "zh" => Some(Self::ZhCn),
            "en" => Some(Self::En),
            _ => None,
        }
    }
}

impl fmt::Display for Locale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_bcp47())
    }
}

/// Locale resolution policy for callers of the runtime. The desktop GUI
/// persists its two explicit choices using `InterfaceLanguage` in the app crate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LanguagePreference {
    #[serde(rename = "zh-CN")]
    Zh,
    #[serde(rename = "en")]
    En,
    System,
}

impl Default for LanguagePreference {
    /// Default to Simplified Chinese.
    fn default() -> Self {
        Self::Zh
    }
}

/// Resolve the active locale from a user preference plus the host system tag.
///
/// `system_tag` should come from `sys-locale::get_locale()`; pass `None` to opt out
/// of system detection entirely (test paths, headless processes).
pub fn resolve_locale(pref: Option<LanguagePreference>, system_tag: Option<&str>) -> Locale {
    match pref.unwrap_or_default() {
        LanguagePreference::Zh => Locale::ZhCn,
        LanguagePreference::En => Locale::En,
        LanguagePreference::System => system_tag
            .and_then(Locale::parse_bcp47)
            .unwrap_or(Locale::En),
    }
}

/// Detect the host system locale via `sys-locale`. Cached lookup is safe for repeat callers.
pub fn detect_system_locale() -> Option<String> {
    sys_locale::get_locale()
}

#[cfg(test)]
#[path = "locale_tests.rs"]
mod tests;
