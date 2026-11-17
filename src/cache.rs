//! Local cache under the platform cache dir (`~/.cache/nationfetch`
//! on Linux).

use std::fs;
use std::io::{ErrorKind, IsTerminal, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use anyhow::{Context, Result};

use crate::ascii::FlagSize;
use crate::{api, country::Country};

pub const COUNTRIES_FILE: &str = "countries.json";

/// `<cache>/nationfetch` — e.g. `~/.cache/nationfetch`.
///
/// Migrates the old `countryfetch` cache directory on first run so a
/// previous `--sync` isn't wasted.
pub fn dir() -> Result<PathBuf> {
    let base = dirs::cache_dir().context("could not determine the user cache directory")?;
    let new = base.join("nationfetch");
    let old = base.join("countryfetch");
    if !new.exists() && old.is_dir() {
        let _ = fs::rename(&old, &new);
    }
    Ok(new)
}

/// Persist the raw upstream dataset verbatim.
pub fn save_dataset(raw: &[u8], dir: &Path) -> Result<()> {
    fs::create_dir_all(dir).with_context(|| format!("could not create {}", dir.display()))?;
    fs::write(dir.join(COUNTRIES_FILE), raw)
        .with_context(|| format!("could not write {}", dir.join(COUNTRIES_FILE).display()))
}

/// Load and parse the cached dataset; tells the user to `--sync` when
/// missing.
pub fn read_countries(dir: &Path) -> Result<Vec<Country>> {
    let path = dir.join(COUNTRIES_FILE);
    let raw = match fs::read_to_string(&path) {
        Ok(raw) => raw,
        Err(e) if e.kind() == ErrorKind::NotFound => {
            anyhow::bail!(
                "cache not found at {} — run `nationfetch --sync` first",
                path.display()
            )
        }
        Err(e) => return Err(e).with_context(|| format!("could not read {}", path.display())),
    };
    serde_json::from_str(&raw).with_context(|| format!("could not parse {}", path.display()))
}

fn flags_dir(dir: &Path) -> PathBuf {
    dir.join("flags")
}

/// Render every country's flag from its remote PNG and store the art
/// under `<cache>/flags`. Failures are reported but don't abort the run.
pub fn cache_flags(countries: &[Country], dir: &Path, size: &FlagSize) -> Result<()> {
    let flags_dir = flags_dir(dir);
    fs::create_dir_all(&flags_dir)
        .with_context(|| format!("could not create {}", flags_dir.display()))?;

    let agent = api::agent();
    let tty = std::io::stdout().is_terminal();
    let next = AtomicUsize::new(0);
    let failed = AtomicUsize::new(0);
    let workers = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
        .min(8)
        .min(countries.len().max(1));

    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| {
                let total = countries.len();
                loop {
                    let i = next.fetch_add(1, Ordering::Relaxed);
                    let Some(country) = countries.get(i) else {
                        return;
                    };
                    let label = format!("({}/{}) {}", i + 1, total, country.name.common);
                    if tty {
                        let mut out = std::io::stdout().lock();
                        let _ = write!(out, "\r{label:<60}");
                        let _ = out.flush();
                    } else {
                        println!("{label}");
                    }
                    let art = api::fetch_flag_png(&agent, country)
                        .and_then(|png| crate::ascii::png_to_ascii(&png, size))
                        .and_then(|art| {
                            fs::write(flags_dir.join(country.flag_file_name()), &art)
                                .map_err(anyhow::Error::new)
                        });
                    if let Err(e) = art {
                        eprintln!("\nwarning: {}: {e}", country.name.common);
                        failed.fetch_add(1, Ordering::Relaxed);
                    }
                }
            });
        }
    });

    if tty {
        println!("\r{:<60}\r", "");
    }
    let failed = failed.load(Ordering::Relaxed);
    if failed > 0 {
        eprintln!("warning: {failed} flag(s) could not be generated");
    }
    Ok(())
}

/// Read a country's cached flag art.
pub fn read_flag(country: &Country, dir: &Path) -> Result<String> {
    let path = flags_dir(dir).join(country.flag_file_name());
    fs::read_to_string(&path).map_err(|e| {
        if e.kind() == ErrorKind::NotFound {
            anyhow::anyhow!(
                "flag not found at {} — run `nationfetch --sync --flag` first",
                path.display()
            )
        } else {
            anyhow::Error::new(e).context(format!("could not read {}", path.display()))
        }
    })
}
