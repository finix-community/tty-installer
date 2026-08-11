use installer_core::SystemInfo;

#[test]
fn detect_never_panics() {
    let info = SystemInfo::detect();
    assert!(!info.installer_version.is_empty());
}
