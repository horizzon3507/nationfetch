//! Country model and lookups against the cached dataset.

use std::collections::BTreeMap;

use anyhow::{Context, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct CountryName {
    pub common: String,
    pub official: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrencyInfo {
    pub name: String,
    pub symbol: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FlagUrls {
    pub png: String,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CapitalInfo {
    #[serde(default)]
    pub latlng: Vec<f64>,
}

/// A country as returned by restcountries.com. The API omits keys on
/// some entries (missing capitals, empty TLDs, …), so every
/// collection and string falls back instead of failing to parse.
#[derive(Debug, Deserialize, Serialize)]
pub struct Country {
    pub name: CountryName,
    #[serde(default)]
    pub cca2: String,
    #[serde(default)]
    pub currencies: BTreeMap<String, CurrencyInfo>,
    #[serde(default)]
    pub capital: Vec<String>,
    #[serde(default)]
    pub flag: String,
    #[serde(default)]
    pub population: u64,
    #[serde(default)]
    pub languages: BTreeMap<String, String>,
    #[serde(default)]
    pub region: String,
    #[serde(default)]
    pub subregion: String,
    #[serde(default)]
    pub timezones: Vec<String>,
    #[serde(default)]
    pub latlng: Vec<f64>,
    #[serde(default, rename = "capitalInfo")]
    pub capital_info: CapitalInfo,
    #[serde(default)]
    pub tld: Vec<String>,
    pub flags: FlagUrls,
}

impl Country {
    /// Cache file name for this country's flag art.
    pub fn flag_file_name(&self) -> String {
        self.name.common.replace(' ', "_")
    }

    /// Flag PNG URLs to try, in order. `flags.png` usually points at
    /// flagcdn, but some entries link elsewhere (e.g. Wikimedia thumbs
    /// that reject the requested size) — fall back to the cca2 CDN URL.
    pub fn flag_urls(&self) -> Vec<String> {
        let mut urls = vec![self.flags.png.clone()];
        if !self.cca2.is_empty() {
            let cdn = format!(
                "https://flagcdn.com/w320/{}.png",
                self.cca2.to_lowercase()
            );
            if !urls.contains(&cdn) {
                urls.push(cdn);
            }
        }
        urls
    }

    fn currencies(&self) -> impl Iterator<Item = String> + '_ {
        self.currencies.iter().map(|(abbr, info)| match &info.symbol {
            Some(symbol) => format!("{} [{abbr}][{symbol}]", info.name),
            None => format!("{} [{abbr}]", info.name),
        })
    }

    /// Print the formatted info block to stdout.
    pub fn print(&self) {
        let value = |s: String| s.cyan();
        println!(
            "Name: {} {}\n\
             Lat/Lng: {}\n\
             Population: {}\n\
             Languages: {}\n\
             Capital: {}\n\
             Capital Lat/Lng: {}\n\
             Region: {}\n\
             Subregion: {}\n\
             Timezones: {}\n\
             Top Level Domain: {}\n\
             Currencies: {}",
            value(self.name.common.clone()),
            self.flag,
            value(fmt::latlng(&self.latlng)),
            value(fmt::thousands(self.population)),
            value(fmt::join_or_missing(self.languages.values(), " | ")),
            value(fmt::join_or_missing(&self.capital, " | ")),
            value(fmt::latlng(&self.capital_info.latlng)),
            value(fmt::or_missing(&self.region)),
            value(fmt::or_missing(&self.subregion)),
            value(fmt::join_or_missing(&self.timezones, " | ")),
            value(fmt::join_or_missing(&self.tld, " | ")),
            value(fmt::join_or_missing(self.currencies(), " | ")),
        );
    }
}

/// Exact common-name match first, then substring match.
pub fn find_by_name<'a>(countries: &'a [Country], exp: &str) -> Result<&'a Country> {
    let needle = exp.to_lowercase();
    countries
        .iter()
        .find(|c| c.name.common.to_lowercase() == needle)
        .or_else(|| {
            countries
                .iter()
                .find(|c| c.name.common.to_lowercase().contains(&needle))
        })
        .with_context(|| format!("could not find a country matching '{exp}'"))
}

/// Substring match against any of a country's capitals.
pub fn find_by_capital<'a>(countries: &'a [Country], exp: &str) -> Result<&'a Country> {
    let needle = exp.to_lowercase();
    countries
        .iter()
        .find(|c| {
            c.capital
                .iter()
                .any(|cap| cap.to_lowercase().contains(&needle))
        })
        .with_context(|| format!("could not find a country with capital matching '{exp}'"))
}

pub fn random(countries: &[Country]) -> Result<&Country> {
    fastrand::choice(countries).context("the country list is empty")
}
