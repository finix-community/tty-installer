use installer_core::system_lists::{
    KeyboardLayoutEntry, LocaleEntry, SystemListError, TimezoneEntry, parse_supported_locales,
    parse_xkb_layouts, parse_zone1970,
};

#[test]
fn parses_zone1970_ignoring_comments_and_blank_lines() {
    let sample = "# comment line, ignored\n\
PL\t+5215+02100\tEurope/Warsaw\n\
US,CA\t+4740-07331\tAmerica/New_York\tEastern time\n\
\n";
    let entries = parse_zone1970(sample);
    assert_eq!(
        entries,
        vec![
            TimezoneEntry {
                name: "Europe/Warsaw".into(),
                country_code: "PL".into()
            },
            TimezoneEntry {
                name: "America/New_York".into(),
                country_code: "US".into()
            },
        ]
    );
}

#[test]
fn parses_supported_locales_slash_format() {
    let sample = "# This file names the currently supported and somewhat tested locales.\n\
SUPPORTED-LOCALES=\\\
\n\
en_US.UTF-8/UTF-8 \\\
\n\
pl_PL.UTF-8/UTF-8 \\\
\n";
    let entries = parse_supported_locales(sample);
    assert_eq!(
        entries,
        vec![
            LocaleEntry {
                name: "en_US.UTF-8".into(),
                charmap: "UTF-8".into()
            },
            LocaleEntry {
                name: "pl_PL.UTF-8".into(),
                charmap: "UTF-8".into()
            },
        ]
    );
}

#[test]
fn parses_supported_locales_space_format() {
    let sample = "en_US.UTF-8 UTF-8\npl_PL.UTF-8 UTF-8\n# comment\n";
    let entries = parse_supported_locales(sample);
    assert_eq!(
        entries,
        vec![
            LocaleEntry {
                name: "en_US.UTF-8".into(),
                charmap: "UTF-8".into()
            },
            LocaleEntry {
                name: "pl_PL.UTF-8".into(),
                charmap: "UTF-8".into()
            },
        ]
    );
}

#[test]
fn parses_xkb_layout_section_only() {
    let sample = "! model\n\
  pc105          Generic 105-key PC\n\
\n\
! layout\n\
  us             English (US)\n\
  pl             Polish\n\
\n\
! variant\n\
  dvorak         English (Dvorak)\n";
    let entries = parse_xkb_layouts(sample);
    assert_eq!(
        entries,
        vec![
            KeyboardLayoutEntry {
                code: "us".into(),
                description: "English (US)".into()
            },
            KeyboardLayoutEntry {
                code: "pl".into(),
                description: "Polish".into()
            },
        ]
    );
}

#[test]
fn missing_env_var_is_a_clear_error_not_a_silent_default() {
    let err = SystemListError::MissingEnvVar("FINIX_TZDATA_ZONE1970");
    assert!(err.to_string().contains("FINIX_TZDATA_ZONE1970"));
}
