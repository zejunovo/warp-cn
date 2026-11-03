use serde_json::json;
use settings_value::SettingsValue;

use super::InterfaceLanguage;

#[test]
fn language_codes_round_trip_in_native_and_file_preferences() {
    for (language, code) in [
        (InterfaceLanguage::English, "en"),
        (InterfaceLanguage::SimplifiedChinese, "zh-CN"),
    ] {
        let value = json!(code);
        assert_eq!(serde_json::to_value(language).unwrap(), value);
        assert_eq!(language.to_file_value(), value);
        assert_eq!(
            serde_json::from_value::<InterfaceLanguage>(value.clone()).unwrap(),
            language
        );
        assert_eq!(
            InterfaceLanguage::from_file_value(&value).unwrap(),
            language
        );
    }
}

#[test]
fn unsupported_languages_are_rejected() {
    for value in [json!("system"), json!("fr"), json!(true), json!(null)] {
        assert!(serde_json::from_value::<InterfaceLanguage>(value.clone()).is_err());
        assert!(InterfaceLanguage::from_file_value(&value).is_none());
    }
}
