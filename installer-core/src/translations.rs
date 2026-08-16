use crate::languages;
use rust_i18n::{set_locale, t};

pub struct UiStrings {
    pub window_title: String,
    pub welcome_tagline: String,
    pub welcome_subtitle: String,
    pub welcome_install: String,
    pub welcome_reboot: String,
    pub welcome_exit: String,
    pub step_locale: String,
    pub step_network: String,
    pub step_providers: String,
    pub step_disk: String,
    pub step_account: String,
    pub step_summary: String,
    pub locale_language_category: String,
    pub locale_keyboard_category: String,
    pub locale_timezone_category: String,
    pub locale_format_category: String,
    pub locale_back: String,
    pub locale_continue: String,
    pub locale_no_matches: String,
}

pub fn ui_strings() -> UiStrings {
    UiStrings {
        window_title: t!("welcome.title").to_string(),
        welcome_tagline: t!("welcome.tagline").to_string(),
        welcome_subtitle: t!("welcome.subtitle").to_string(),
        welcome_install: t!("welcome.install").to_string(),
        welcome_reboot: t!("welcome.reboot").to_string(),
        welcome_exit: t!("welcome.exit").to_string(),
        step_locale: t!("step.locale").to_string(),
        step_network: t!("step.network").to_string(),
        step_providers: t!("step.providers").to_string(),
        step_disk: t!("step.disk").to_string(),
        step_account: t!("step.account").to_string(),
        step_summary: t!("step.summary").to_string(),
        locale_language_category: t!("locale.category.language").to_string(),
        locale_keyboard_category: t!("locale.category.keyboard").to_string(),
        locale_timezone_category: t!("locale.category.timezone").to_string(),
        locale_format_category: t!("locale.category.format").to_string(),
        locale_back: t!("locale.back").to_string(),
        locale_continue: t!("locale.continue").to_string(),
        locale_no_matches: t!("locale.no_matches").to_string(),
    }
}

pub fn system_line(kernel: &str, tty: &str) -> String {
    t!("welcome.system_line", kernel = kernel, tty = tty).to_string()
}

pub fn version_line(version: &str) -> String {
    t!("welcome.version_line", version = version).to_string()
}

pub fn matches_label(count: usize) -> String {
    match count {
        0 => t!("locale.matches.zero").to_string(),
        1 => t!("locale.matches.one", count = count).to_string(),
        _ => t!("locale.matches.many", count = count).to_string(),
    }
}

pub fn current_locale() -> String {
    rust_i18n::locale().to_string()
}

pub fn set_ui_locale(locale: &str) -> bool {
    if !languages::installer_languages()
        .iter()
        .any(|(code, _)| *code == locale)
    {
        return false;
    }
    set_locale(locale);
    true
}

pub fn detect_locale() -> String {
    if let Ok(lang) = std::env::var("LANG") {
        let base = lang
            .split('.')
            .next()
            .unwrap_or("")
            .replace('_', "-")
            .to_lowercase();
        if base == "pl" || base.starts_with("pl-") {
            return "pl".to_string();
        }
        if base == "en-gb" || base.starts_with("en-gb") {
            return "en-GB".to_string();
        }
        if base == "en" || base.starts_with("en-") {
            return "en-US".to_string();
        }
    }
    "en-US".to_string()
}