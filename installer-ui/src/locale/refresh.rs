use super::filter::{filter_pairs, to_model};
use super::state::{LocaleLists, LocaleSelection, items_for, selected_code};
use crate::FinixInstaller;
use slint::SharedString;

pub(super) fn refresh(ui: &FinixInstaller, lists: &LocaleLists, selection: &LocaleSelection) {
    let items = items_for(lists, selection.active);
    let filtered = filter_pairs(items, &selection.filter);
    let current_code = selected_code(selection, selection.active);
    let current_index = filtered
        .iter()
        .position(|(code, _)| code == current_code)
        .unwrap_or(0);

    ui.set_locale_match_count(filtered.len() as i32);
    ui.set_locale_selected_index(current_index as i32);
    ui.set_locale_visible_items(to_model(&filtered));
    ui.set_locale_selected_code(SharedString::from(current_code));
    ui.set_locale_active_category(selection.active as i32);
    ui.set_locale_filter_text(SharedString::from(selection.filter.as_str()));

    ui.set_locale_language_value(SharedString::from(selection.language.as_str()));
    ui.set_locale_keyboard_value(SharedString::from(selection.keyboard.as_str()));
    ui.set_locale_timezone_value(SharedString::from(selection.timezone.as_str()));
    ui.set_locale_locale_format_value(SharedString::from(selection.locale_format.as_str()));
}
