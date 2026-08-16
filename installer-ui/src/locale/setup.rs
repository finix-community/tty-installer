use super::filter::filter_pairs;
use super::refresh::refresh;
use super::state::{LocaleLists, LocaleSelection, items_for, selected_code, set_selected_code};
use super::{category::Category, lists};
use crate::FinixInstaller;
use installer_core::system_lists;
use slint::ComponentHandle;
use std::cell::RefCell;
use std::rc::Rc;

pub fn setup(ui: &FinixInstaller) -> Result<(), slint::PlatformError> {
    let mut load_error = None;
    let language = lists::locale_pairs(lists::load_or_warn(
        "locales",
        system_lists::list_locales(),
        &mut load_error,
    ));
    let keyboard = lists::keyboard_pairs(lists::load_or_warn(
        "keyboard layouts",
        system_lists::list_keyboard_layouts(),
        &mut load_error,
    ));
    let timezone = lists::timezone_pairs(lists::load_or_warn(
        "timezones",
        system_lists::list_timezones(),
        &mut load_error,
    ));
    let locale_format = language.clone();
    ui.set_locale_load_error(load_error.unwrap_or_default().into());

    let selection = Rc::new(RefCell::new(LocaleSelection {
        language: String::new(),
        keyboard: String::new(),
        timezone: String::new(),
        locale_format: String::new(),
        active: Category::Language,
        filter: String::new(),
    }));

    let lists = Rc::new(LocaleLists {
        language,
        keyboard,
        timezone,
        locale_format,
    });

    refresh(ui, &lists, &selection.borrow());

    {
        let ui_weak = ui.as_weak();
        let lists = lists.clone();
        let selection = selection.clone();
        ui.on_locale_category_selected(move |category| {
            if let Some(ui) = ui_weak.upgrade() {
                let mut selection = selection.borrow_mut();
                selection.active = Category::from_i32(category);
                selection.filter.clear();
                refresh(&ui, &lists, &selection);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let lists = lists.clone();
        let selection = selection.clone();
        ui.on_locale_filter_char_typed(move |ch| {
            if let Some(ui) = ui_weak.upgrade() {
                let Some(c) = installer_core::input::printable_char(ch.as_str()) else {
                    return;
                };
                let mut selection = selection.borrow_mut();
                selection.filter.push(c);
                refresh(&ui, &lists, &selection);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let lists = lists.clone();
        let selection = selection.clone();
        ui.on_locale_filter_backspace(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let mut selection = selection.borrow_mut();
                selection.filter.pop();
                refresh(&ui, &lists, &selection);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let lists = lists.clone();
        let selection = selection.clone();
        ui.on_locale_item_selected(move |code| {
            if let Some(ui) = ui_weak.upgrade() {
                let mut selection = selection.borrow_mut();
                let active = selection.active;
                set_selected_code(&mut selection, active, code.to_string());
                if active != Category::LocaleFormat {
                    selection.active = active.next();
                    selection.filter.clear();
                }
                refresh(&ui, &lists, &selection);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let lists = lists.clone();
        let selection = selection.clone();
        ui.on_locale_move_selection(move |delta| {
            if let Some(ui) = ui_weak.upgrade() {
                let mut selection = selection.borrow_mut();
                let active = selection.active;
                let items = items_for(&lists, active);
                let filtered = filter_pairs(items, &selection.filter);
                if filtered.is_empty() {
                    return;
                }
                let current_code = selected_code(&selection, active).to_string();
                let current_index = if current_code.is_empty() {
                    -1
                } else {
                    filtered
                        .iter()
                        .position(|(code, _)| *code == current_code)
                        .unwrap_or(0) as i32
                };
                let last_index = filtered.len() as i32 - 1;
                let new_index = (current_index + delta).clamp(0, last_index) as usize;
                let new_code = filtered[new_index].0.clone();
                set_selected_code(&mut selection, active, new_code);
                refresh(&ui, &lists, &selection);
            }
        });
    }
    ui.on_locale_continue_pressed(|| {
        println!("continue requested - network step not implemented yet");
    });

    Ok(())
}
