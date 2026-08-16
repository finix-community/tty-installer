use installer_core::languages::installer_languages;

#[test]
fn contains_polish_first_then_english_us_and_uk() {
    let languages = installer_languages();
    assert_eq!(
        languages,
        &[("pl", "Polski"), ("en-US", "English (US)"), ("en-GB", "English (UK)")]
    );
}

#[test]
fn codes_are_unique() {
    let languages = installer_languages();
    let mut codes: Vec<&str> = languages.iter().map(|(code, _)| *code).collect();
    codes.sort();
    codes.dedup();
    assert_eq!(codes.len(), languages.len());
}