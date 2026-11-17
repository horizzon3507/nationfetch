//! Country dataset access.

use std::time::Duration;

use anyhow::{Context, Result};

use crate::country::Country;

/// The frozen REST Countries v3.1 dataset, from the archived project
/// repository (the live API now requires an API key).
const ALL_URL: &str = "https://raw.githubusercontent.com/restcountries/restcountries/master/src/main/resources/countriesV3.1.json";

/// Shared HTTP agent.
pub fn agent() -> ureq::Agent {
    ureq::Agent::config_builder()
        .timeout_global(Some(Duration::from_secs(60)))
        .build()
        .into()
}

/// Fetch the full dataset verbatim (kept raw in the cache so fields
/// added later don't need a re-sync).
pub fn fetch_dataset(agent: &ureq::Agent) -> Result<Vec<u8>> {
    fetch_bytes(agent, ALL_URL).context("failed to fetch the country dataset")
}

/// Fetch raw bytes (flag PNGs).
pub fn fetch_bytes(agent: &ureq::Agent, url: &str) -> Result<Vec<u8>> {
    use std::io::Read;
    let mut response = agent
        .get(url)
        .call()
        .with_context(|| format!("failed to fetch {url}"))?;
    let mut bytes = Vec::new();
    response
        .body_mut()
        .as_reader()
        .read_to_end(&mut bytes)
        .with_context(|| format!("failed to read response from {url}"))?;
    Ok(bytes)
}

/// Fetch a country's flag PNG, trying each candidate URL in order
/// (`flags.png`, then the flagcdn `cca2` URL).
pub fn fetch_flag_png(agent: &ureq::Agent, country: &Country) -> Result<Vec<u8>> {
    let mut last_err = anyhow::anyhow!("no flag URL available");
    for url in country.flag_urls() {
        match fetch_bytes(agent, &url) {
            Ok(png) => return Ok(png),
            Err(e) => last_err = e,
        }
    }
    Err(last_err.context(format!(
        "could not fetch a flag for {}",
        country.name.common
    )))
}
