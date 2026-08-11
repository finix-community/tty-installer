use std::fs;

pub mod system_lists;

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub kernel_version: String,
    pub tty_name: String,
    pub installer_version: String,
}

impl SystemInfo {
    pub fn detect() -> Self {
        Self {
            kernel_version: detect_kernel_version().unwrap_or_else(|| "unknown".to_string()),
            tty_name: detect_tty_name().unwrap_or_else(|| "tty1".to_string()),
            installer_version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }
}

fn detect_kernel_version() -> Option<String> {
    let raw = fs::read_to_string("/proc/version").ok()?;
    raw.split_whitespace().nth(2).map(str::to_string)
}

#[cfg(unix)]
fn detect_tty_name() -> Option<String> {
    let link = fs::read_link("/proc/self/fd/0").ok()?;
    let name = link.file_name()?.to_str()?.to_string();
    if name.is_empty() { None } else { Some(name) }
}

#[cfg(not(unix))]
fn detect_tty_name() -> Option<String> {
    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WelcomeAction {
    Install,
    Reboot,
    Exit,
}
