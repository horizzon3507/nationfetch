//! Local cache under the platform cache dir (`~/.cache/countryfetch`
//! on Linux).

use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::country::Country;

pub const COUNTRIES_FILE: &str = "countries.json";

/// `<cache>/countryfetch` — e.g. `~/.cache/countryfetch`.
pub fn dir() -> Result<PathBuf> {
    Ok(dirs::cache_dir()
        .context("could not determine the user cache directory")?
        .join("countryfetch"))
}

/// Persist the full dataset as pretty-printed JSON.
pub fn save_countries(countries: &[Country], dir: &Path) -> Result<()> {
    fs::create_dir_all(dir)
        .with_context(|| format!("could not create {}", dir.display()))?;
    let json = serde_json::to_string_pretty(countries)?;
    fs::write(dir.join(COUNTRIES_FILE), json)
        .with_context(|| format!("could not write {}", dir.join(COUNTRIES_FILE).display()))
}

/// Load the cached dataset; tells the user to `--sync` when missing.
pub fn read_countries(dir: &Path) -> Result<Vec<Country>> {
    let path = dir.join(COUNTRIES_FILE);
    let raw = match fs::read_to_string(&path) {
        Ok(raw) => raw,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            anyhow::bail!(
                "cache not found at {} — run `countryfetch --sync` first",
                path.display()
            )
        }
        Err(e) => return Err(e).with_context(|| format!("could not read {}", path.display())),
    };
    serde_json::from_str(&raw).with_context(|| format!("could not parse {}", path.display()))
}
