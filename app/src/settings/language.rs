use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::macros::define_settings_group;
use settings::{SupportedPlatforms, SyncToCloud};
use warp_errors::report_if_error;
use warp_i18n::Locale;
use warpui::{AppContext, SingletonEntity};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum InterfaceLanguage {
    #[serde(rename = "en")]
    English,
    #[default]
    #[serde(rename = "zh-CN")]
    SimplifiedChinese,
}

impl settings_value::SettingsValue for InterfaceLanguage {}

impl InterfaceLanguage {
    pub const ALL: [Self; 2] = [Self::SimplifiedChinese, Self::English];

    pub fn label(self) -> &'static str {
        match self {
            Self::English => "English",
            Self::SimplifiedChinese => "简体中文",
        }
    }

    pub fn locale(self) -> Locale {
        match self {
            Self::English => Locale::En,
            Self::SimplifiedChinese => Locale::ZhCn,
        }
    }
}

define_settings_group!(LanguageSettings, settings: [
    language: InterfaceLanguageSetting {
        type: InterfaceLanguage,
        default: InterfaceLanguage::SimplifiedChinese,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        surface: settings::SettingSurfaces::GUI,
        private: false,
        storage_key: "InterfaceLanguage",
        toml_path: "appearance.language",
        description: "The interface language. Changes take effect after restarting Warp.",
    },
]);

pub fn initialize_language(ctx: &AppContext) {
    let locale = LanguageSettings::as_ref(ctx).language.locale();
    report_if_error!(warp_i18n::init(locale));
    warp_i18n::set_locale(locale);
}

#[cfg(test)]
#[path = "language_tests.rs"]
mod tests;
