use installer_core::languages::installer_languages;
use installer_core::translations;
use std::sync::Mutex;

static LOCALE_GUARD: Mutex<()> = Mutex::new(());

#[test]
fn detect_locale_maps_system_lang() {
    unsafe {
        std::env::set_var("LANG", "pl_PL.UTF-8");
        assert_eq!(translations::detect_locale(), "pl");
        std::env::set_var("LANG", "en_GB.UTF-8");
        assert_eq!(translations::detect_locale(), "en-GB");
        std::env::set_var("LANG", "en_US.UTF-8");
        assert_eq!(translations::detect_locale(), "en-US");
        std::env::remove_var("LANG");
    }
    assert_eq!(translations::detect_locale(), "en-US");
}

#[test]
fn rejects_unsupported_locale() {
    let _guard = LOCALE_GUARD.lock().unwrap();
    assert!(!translations::set_ui_locale("de"));
    assert!(translations::set_ui_locale("pl"));
}

#[test]
fn all_available_languages_translate() {
    let _guard = LOCALE_GUARD.lock().unwrap();
    for (code, _) in installer_languages() {
        assert!(translations::set_ui_locale(code));
        let s = translations::ui_strings();
        assert!(
            !s.window_title.is_empty() && !s.locale_back.is_empty() && !s.step_summary.is_empty(),
            "missing strings for {code}"
        );
    }
}

#[test]
fn locale_dependent_strings() {
    let _guard = LOCALE_GUARD.lock().unwrap();
    translations::set_ui_locale("pl");
    let s = translations::ui_strings();
    assert_eq!(s.locale_back, "< wstecz");
    assert_eq!(s.welcome_install, "instaluj >");
    assert_eq!(s.window_title, "instalator finix");
    assert_eq!(translations::matches_label(0), "brak dopasowań");
    assert_eq!(translations::matches_label(1), "1 dopasowanie");
    assert_eq!(translations::matches_label(3), "3 dopasowań");
    assert_eq!(
        translations::system_line("7.1.4", "tty1"),
        "linux 7.1.4 · tty1"
    );

    translations::set_ui_locale("en-US");
    let s = translations::ui_strings();
    assert_eq!(s.locale_back, "< back");
    assert_eq!(s.window_title, "finix installer");
    assert_eq!(translations::matches_label(1), "1 match");
    assert_eq!(translations::matches_label(4), "4 matches");
}
