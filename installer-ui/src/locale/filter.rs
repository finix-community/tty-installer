use crate::ListItem;
use slint::{ModelRc, SharedString, VecModel};

pub(super) fn filter_pairs<'a>(
    items: &'a [(String, String)],
    query: &str,
) -> Vec<&'a (String, String)> {
    if query.is_empty() {
        return items.iter().collect();
    }
    let query = query.to_lowercase();
    let mut starts = Vec::new();
    let mut contains = Vec::new();
    for pair in items {
        let haystack = pair.1.to_lowercase();
        if haystack.starts_with(&query) {
            starts.push(pair);
        } else if haystack.contains(&query) {
            contains.push(pair);
        }
    }
    starts.extend(contains);
    starts
}

pub(super) fn to_model(pairs: &[&(String, String)]) -> ModelRc<ListItem> {
    let items: Vec<ListItem> = pairs
        .iter()
        .map(|(code, label)| ListItem {
            code: SharedString::from(code.as_str()),
            label: SharedString::from(label.as_str()),
        })
        .collect();
    ModelRc::new(VecModel::from(items))
}
