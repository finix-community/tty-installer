/// Languages the installer UI itself can be displayed in.
pub fn installer_languages() -> &'static [(&'static str, &'static str)] {
    &[
        ("pl", "Polski"),
        ("en-US", "English (US)"),
        ("en-GB", "English (UK)"),
    ]
}
