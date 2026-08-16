use crate::FinixInstaller;
use installer_core::translations;
use slint::SharedString;

pub fn apply(ui: &FinixInstaller) {
    let s = translations::ui_strings();
    let system_line =
        translations::system_line(ui.get_kernel_version().as_str(), ui.get_tty_name().as_str());
    let version_line = translations::version_line(ui.get_installer_version().as_str());

    ui.set_welcome_title(SharedString::from(s.window_title));
    ui.set_welcome_tagline(SharedString::from(s.welcome_tagline));
    ui.set_welcome_subtitle(SharedString::from(s.welcome_subtitle));
    ui.set_welcome_install_label(SharedString::from(s.welcome_install));
    ui.set_welcome_reboot_label(SharedString::from(s.welcome_reboot));
    ui.set_welcome_exit_label(SharedString::from(s.welcome_exit));
    ui.set_welcome_system_line(SharedString::from(system_line));
    ui.set_welcome_version_line(SharedString::from(version_line));

    ui.set_locale_language_category(SharedString::from(s.locale_language_category));
    ui.set_locale_keyboard_category(SharedString::from(s.locale_keyboard_category));
    ui.set_locale_timezone_category(SharedString::from(s.locale_timezone_category));
    ui.set_locale_format_category(SharedString::from(s.locale_format_category));
    ui.set_locale_no_matches_label(SharedString::from(s.locale_no_matches));
    ui.set_locale_back_label(SharedString::from(s.locale_back));
    ui.set_locale_continue_label(SharedString::from(s.locale_continue));

    ui.set_step_locale(SharedString::from(s.step_locale));
    ui.set_step_network(SharedString::from(s.step_network));
    ui.set_step_providers(SharedString::from(s.step_providers));
    ui.set_step_disk(SharedString::from(s.step_disk));
    ui.set_step_account(SharedString::from(s.step_account));
    ui.set_step_summary(SharedString::from(s.step_summary));

    refresh_matches_label(ui);
}

pub fn refresh_matches_label(ui: &FinixInstaller) {
    let count = ui.get_locale_match_count().max(0) as usize;
    ui.set_locale_matches_label(SharedString::from(translations::matches_label(count)));
}