use super::{category::Category, lists::Pairs};

macro_rules! by_category {
    ($category:expr, $language:expr, $keyboard:expr, $timezone:expr, $locale_format:expr) => {
        match $category {
            Category::Language => $language,
            Category::Keyboard => $keyboard,
            Category::Timezone => $timezone,
            Category::LocaleFormat => $locale_format,
        }
    };
}

pub(super) struct LocaleLists {
    pub(super) language: Pairs,
    pub(super) keyboard: Pairs,
    pub(super) timezone: Pairs,
    pub(super) locale_format: Pairs,
}

pub(super) struct LocaleSelection {
    pub(super) language: String,
    pub(super) keyboard: String,
    pub(super) timezone: String,
    pub(super) locale_format: String,
    pub(super) active: Category,
    pub(super) filter: String,
}

pub(super) fn items_for(lists: &LocaleLists, category: Category) -> &[(String, String)] {
    by_category!(
        category,
        &lists.language,
        &lists.keyboard,
        &lists.timezone,
        &lists.locale_format
    )
}

pub(super) fn selected_code(selection: &LocaleSelection, category: Category) -> &str {
    by_category!(
        category,
        &selection.language,
        &selection.keyboard,
        &selection.timezone,
        &selection.locale_format
    )
}

pub(super) fn set_selected_code(selection: &mut LocaleSelection, category: Category, code: String) {
    *by_category!(
        category,
        &mut selection.language,
        &mut selection.keyboard,
        &mut selection.timezone,
        &mut selection.locale_format
    ) = code;
}
