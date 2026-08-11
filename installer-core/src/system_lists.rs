use std::env;
use std::fmt;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub enum SystemListError {
    MissingEnvVar(&'static str),
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
}

impl fmt::Display for SystemListError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingEnvVar(name) => write!(
                f,
                "{name} is not set - run via `nix develop` or the built package, \
                 not a bare `cargo run` outside that environment (see flake.nix)"
            ),
            Self::Io { path, source } => {
                write!(f, "failed to read {}: {source}", path.display())
            }
        }
    }
}

impl std::error::Error for SystemListError {}

fn read_env_path(var: &'static str) -> Result<PathBuf, SystemListError> {
    env::var(var)
        .map(PathBuf::from)
        .map_err(|_| SystemListError::MissingEnvVar(var))
}

fn read_file(path: PathBuf) -> Result<String, SystemListError> {
    fs::read_to_string(&path).map_err(|source| SystemListError::Io { path, source })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimezoneEntry {
    pub name: String,
    pub country_code: String,
}

/// Reads tzdata's `zone1970.tab` from `FINIX_TZDATA_ZONE1970`.
pub fn list_timezones() -> Result<Vec<TimezoneEntry>, SystemListError> {
    let path = read_env_path("FINIX_TZDATA_ZONE1970")?;
    Ok(parse_zone1970(&read_file(path)?))
}

pub fn parse_zone1970(content: &str) -> Vec<TimezoneEntry> {
    content
        .lines()
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
        .filter_map(|line| {
            let mut cols = line.split('\t');
            let codes = cols.next()?;
            let _coordinates = cols.next()?;
            let name = cols.next()?;
            let country_code = codes.split(',').next().unwrap_or(codes).to_string();
            Some(TimezoneEntry {
                name: name.to_string(),
                country_code,
            })
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocaleEntry {
    pub name: String,
    pub charmap: String,
}

/// Reads glibc's `SUPPORTED` file from `FINIX_LOCALE_SUPPORTED`.
pub fn list_locales() -> Result<Vec<LocaleEntry>, SystemListError> {
    let path = read_env_path("FINIX_LOCALE_SUPPORTED")?;
    Ok(parse_supported_locales(&read_file(path)?))
}

pub fn parse_supported_locales(content: &str) -> Vec<LocaleEntry> {
    content
        .lines()
        .filter_map(|raw_line| {
            let mut line = raw_line.trim();
            line = line.trim_start_matches("SUPPORTED-LOCALES=");
            line = line.trim_end_matches('\\').trim();
            if line.is_empty() || line.starts_with('#') {
                return None;
            }
            let (name, charmap) = if let Some((n, c)) = line.split_once('/') {
                (n, c)
            } else if let Some((n, c)) = line.split_once(char::is_whitespace) {
                (n, c.trim())
            } else {
                (line, "")
            };
            if name.is_empty() {
                return None;
            }
            Some(LocaleEntry {
                name: name.to_string(),
                charmap: charmap.to_string(),
            })
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeyboardLayoutEntry {
    pub code: String,
    pub description: String,
}

/// Reads the `! layout` section of `$XKB_CONFIG_ROOT/rules/base.lst`.
pub fn list_keyboard_layouts() -> Result<Vec<KeyboardLayoutEntry>, SystemListError> {
    let root = read_env_path("XKB_CONFIG_ROOT")?;
    let path = root.join("rules").join("base.lst");
    Ok(parse_xkb_layouts(&read_file(path)?))
}

pub fn parse_xkb_layouts(content: &str) -> Vec<KeyboardLayoutEntry> {
    let mut out = Vec::new();
    let mut in_layout_section = false;

    for raw_line in content.lines() {
        let line = raw_line.trim();
        if let Some(section) = line.strip_prefix('!') {
            in_layout_section = section.trim() == "layout";
            continue;
        }
        if !in_layout_section || line.is_empty() {
            continue;
        }
        if let Some((code, description)) = line.split_once(char::is_whitespace) {
            out.push(KeyboardLayoutEntry {
                code: code.trim().to_string(),
                description: description.trim().to_string(),
            });
        }
    }

    out
}
