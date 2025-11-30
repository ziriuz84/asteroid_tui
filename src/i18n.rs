//! # Internationalization
//!
//! Simple i18n support for the application

use crate::settings::Settings;

/// Supported languages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    /// English language
    English,
    /// Italian language
    Italian,
}

impl Language {
    /// Get language from string code
    pub fn from_code(code: &str) -> Self {
        match code {
            "it" => Language::Italian,
            _ => Language::English,
        }
    }

    /// Get language code
    pub fn code(self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Italian => "it",
        }
    }

    /// Get current language from settings
    pub fn current() -> Self {
        Settings::new()
            .map(|s| Language::from_code(s.get_lang()))
            .unwrap_or(Language::English)
    }
}

/// Translation keys
pub mod keys {
    /// Main menu title key
    pub const MAIN_MENU_TITLE: &str = "main_menu_title";
    /// Settings key
    pub const SETTINGS: &str = "settings";
    /// Scheduling key
    pub const SCHEDULING: &str = "scheduling";
    /// Quit key
    pub const QUIT: &str = "quit";
    /// Back key
    pub const BACK: &str = "back";
    /// Select option key
    pub const SELECT_OPTION: &str = "select_option";
    /// Invalid option key
    pub const INVALID_OPTION: &str = "invalid_option";
    /// Settings menu title key
    pub const SETTINGS_MENU_TITLE: &str = "settings_menu_title";
    /// General key
    pub const GENERAL: &str = "general";
    /// Observatory key
    pub const OBSERVATORY: &str = "observatory";
    /// Scheduling menu title key
    pub const SCHEDULING_MENU_TITLE: &str = "scheduling_menu_title";
    /// Weather forecast key
    pub const WEATHER_FORECAST: &str = "weather_forecast";
    /// Sun and moon times key
    pub const SUN_MOON_TIMES: &str = "sun_moon_times";
    /// Observing target list key
    pub const OBSERVING_TARGET_LIST: &str = "observing_target_list";
}

/// Get translated string
pub fn t(key: &str) -> String {
    let lang = Language::current();
    translate(key, lang)
}

/// Translate a key for a specific language
fn translate(key: &str, lang: Language) -> String {
    match (key, lang) {
        // Main menu
        (keys::MAIN_MENU_TITLE, Language::English) => "Main Menu".to_string(),
        (keys::MAIN_MENU_TITLE, Language::Italian) => "Menu Principale".to_string(),
        (keys::SETTINGS, Language::English) => "Settings".to_string(),
        (keys::SETTINGS, Language::Italian) => "Impostazioni".to_string(),
        (keys::SCHEDULING, Language::English) => "Scheduling".to_string(),
        (keys::SCHEDULING, Language::Italian) => "Pianificazione".to_string(),
        (keys::QUIT, Language::English) => "Quit".to_string(),
        (keys::QUIT, Language::Italian) => "Esci".to_string(),
        (keys::BACK, Language::English) => "Back".to_string(),
        (keys::BACK, Language::Italian) => "Indietro".to_string(),
        (keys::SELECT_OPTION, Language::English) => "Select an option:".to_string(),
        (keys::SELECT_OPTION, Language::Italian) => "Seleziona un'opzione:".to_string(),
        (keys::INVALID_OPTION, Language::English) => "Invalid option".to_string(),
        (keys::INVALID_OPTION, Language::Italian) => "Opzione non valida".to_string(),
        
        // Settings menu
        (keys::SETTINGS_MENU_TITLE, Language::English) => "Settings Menu".to_string(),
        (keys::SETTINGS_MENU_TITLE, Language::Italian) => "Menu Impostazioni".to_string(),
        (keys::GENERAL, Language::English) => "General".to_string(),
        (keys::GENERAL, Language::Italian) => "Generale".to_string(),
        (keys::OBSERVATORY, Language::English) => "Observatory".to_string(),
        (keys::OBSERVATORY, Language::Italian) => "Osservatorio".to_string(),
        
        // Scheduling menu
        (keys::SCHEDULING_MENU_TITLE, Language::English) => "Scheduling Menu".to_string(),
        (keys::SCHEDULING_MENU_TITLE, Language::Italian) => "Menu Pianificazione".to_string(),
        (keys::WEATHER_FORECAST, Language::English) => "Weather Forecast".to_string(),
        (keys::WEATHER_FORECAST, Language::Italian) => "Previsioni Meteo".to_string(),
        (keys::SUN_MOON_TIMES, Language::English) => "Sun and moon times".to_string(),
        (keys::SUN_MOON_TIMES, Language::Italian) => "Ore di sole e luna".to_string(),
        (keys::OBSERVING_TARGET_LIST, Language::English) => "Observing target list".to_string(),
        (keys::OBSERVING_TARGET_LIST, Language::Italian) => "Lista obiettivi osservazione".to_string(),
        
        // Default: return key if not found
        _ => key.to_string(),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_language_from_code() {
        assert_eq!(Language::from_code("en"), Language::English);
        assert_eq!(Language::from_code("it"), Language::Italian);
        assert_eq!(Language::from_code("unknown"), Language::English);
    }

    #[test]
    fn test_translate() {
        assert_eq!(translate(keys::SETTINGS, Language::English), "Settings");
        assert_eq!(translate(keys::SETTINGS, Language::Italian), "Impostazioni");
    }
}
