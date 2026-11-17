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

/// Fetch the full country dataset.
pub fn fetch_countries(agent: &ureq::Agent) -> Result<Vec<Country>> {
    let mut response = agent
        .get(ALL_URL)
        .call()
        .context("failed to fetch the country dataset")?;
    response
        .body_mut()
        .read_json()
        .context("invalid country dataset")
}
