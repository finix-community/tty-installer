use installer_core::system_lists::{
    KeyboardLayoutEntry, LocaleEntry, SystemListError, TimezoneEntry,
};

pub(super) type Pairs = Vec<(String, String)>;

pub(super) fn language_pairs(entries: &[(&str, &str)]) -> Pairs {
    entries
        .iter()
        .map(|(code, label)| (code.to_string(), label.to_string()))
        .collect()
}

pub(super) fn load_or_warn<T>(
    label: &str,
    result: Result<Vec<T>, SystemListError>,
    first_error: &mut Option<String>,
) -> Vec<T> {
    match result {
        Ok(items) => items,
        Err(err) => {
            eprintln!("locale: couldn't load {label}: {err}");
            first_error.get_or_insert_with(|| format!("{label} could not be loaded: {err}"));
            Vec::new()
        }
    }
}

pub(super) fn locale_pairs(entries: Vec<LocaleEntry>) -> Pairs {
    entries
        .into_iter()
        .map(|e| (e.name.clone(), e.name))
        .collect()
}

pub(super) fn timezone_pairs(entries: Vec<TimezoneEntry>) -> Pairs {
    entries
        .into_iter()
        .map(|e| {
            let label = format!("{} ({})", e.name, e.country_code);
            (e.name, label)
        })
        .collect()
}

pub(super) fn keyboard_pairs(entries: Vec<KeyboardLayoutEntry>) -> Pairs {
    entries
        .into_iter()
        .map(|e| {
            let label = format!("{} - {}", e.code, e.description);
            (e.code, label)
        })
        .collect()
}
