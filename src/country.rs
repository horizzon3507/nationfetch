//! Country model and lookups against the cached dataset.

use std::collections::BTreeMap;

use anyhow::{Context, Result};
use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::fmt;

fn yes_no(value: bool) -> String {
    if value { "yes" } else { "no" }.into()
}

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

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Idd {
    pub root: Option<String>,
    #[serde(default)]
    pub suffixes: Vec<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Car {
    pub side: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Maps {
    #[serde(rename = "googleMaps")]
    pub google_maps: Option<String>,
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
    pub cca3: String,
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
    #[serde(default)]
    pub area: f64,
    #[serde(default)]
    pub borders: Vec<String>,
    #[serde(default)]
    pub independent: Option<bool>,
    #[serde(default, rename = "unMember")]
    pub un_member: Option<bool>,
    #[serde(default)]
    pub fifa: Option<String>,
    #[serde(default)]
    pub idd: Idd,
    #[serde(default)]
    pub car: Car,
    #[serde(default)]
    pub maps: Maps,
    #[serde(default)]
    pub gini: BTreeMap<String, f64>,
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

    /// Colored `label: value` lines for the info block, like
    /// neofetch's fields. `all` resolves border codes into names.
    pub fn info_lines(&self, all: &[Country]) -> Vec<String> {
        let mut lines = Vec::new();
        let mut push = |label: &str, value: String| {
            lines.push(format!("{}: {}", label.cyan(), value));
        };

        push("Official", fmt::or_missing(&self.name.official));
        push("Capital", fmt::join_or_missing(&self.capital, " | "));
        push(
            "Capital Lat/Lng",
            fmt::latlng(&self.capital_info.latlng),
        );
        push("Region", self.region_line());
        push("Population", fmt::thousands(self.population));
        if self.area > 0.0 {
            push("Area", format!("{} km²", fmt::thousands(self.area as u64)));
        }
        push("Languages", self.languages_line());
        push("Currencies", fmt::join_or_missing(self.currencies(), " | "));
        push("Timezones", fmt::join_or_missing(&self.timezones, " | "));
        push("TLD", fmt::join_or_missing(&self.tld, " | "));
        push("Lat/Lng", fmt::latlng(&self.latlng));
        push("Borders", self.borders_line(all));
        if let Some(independent) = self.independent {
            push("Independent", yes_no(independent));
        }
        if let Some(un_member) = self.un_member {
            push("UN Member", yes_no(un_member));
        }
        if let Some(fifa) = &self.fifa {
            push("FIFA", fifa.clone());
        }
        if let Some(side) = &self.car.side {
            push("Driving side", side.clone());
        }
        if let Some(calling) = self.calling_code() {
            push("Calling code", calling);
        }
        if let Some((year, value)) = self.gini.iter().next_back() {
            push("Gini", format!("{value} ({year})"));
        }
        if let Some(maps) = &self.maps.google_maps {
            push("Maps", maps.clone());
        }
        lines
    }

    pub fn region_line(&self) -> String {
        match (self.region.is_empty(), self.subregion.is_empty()) {
            (true, true) => fmt::MISSING.into(),
            (true, false) => self.subregion.clone(),
            (false, true) => self.region.clone(),
            (false, false) => format!("{} ({})", self.region, self.subregion),
        }
    }

    pub fn languages_line(&self) -> String {
        fmt::join_or_missing(self.languages.values(), " | ")
    }

    pub fn borders_line(&self, all: &[Country]) -> String {
        if self.borders.is_empty() {
            return "none".into();
        }
        self.borders
            .iter()
            .map(|code| {
                all.iter()
                    .find(|c| &c.cca3 == code)
                    .map(|c| c.name.common.clone())
                    .unwrap_or_else(|| code.clone())
            })
            .collect::<Vec<_>>()
            .join(" | ")
    }

    /// "+39" style dialing code; multi-suffix countries (US: +1207…)
    /// show the root only.
    pub fn calling_code(&self) -> Option<String> {
        let root = self.idd.root.as_deref().filter(|r| !r.is_empty())?;
        Some(match self.idd.suffixes.as_slice() {
            [suffix] => format!("{root}{suffix}"),
            _ => root.to_string(),
        })
    }

    /// A single field's plain (uncolored) value for `--field`.
    /// `Ok(None)` when the field exists but this country has no data.
    pub fn field_value(&self, name: &str, all: &[Country]) -> Result<Option<String>> {
        const FIELDS: &[&str] = &[
            "name", "official", "capital", "capital-latlng", "region",
            "subregion", "population", "area", "languages", "currencies",
            "timezones", "tld", "latlng", "borders", "independent",
            "un-member", "fifa", "driving-side", "calling", "gini",
            "maps", "cca2", "cca3", "emoji",
        ];
        let value = match name {
            "name" => Some(self.name.common.clone()),
            "official" => Some(self.name.official.clone()),
            "capital" => (!self.capital.is_empty())
                .then(|| self.capital.join(" | ")),
            "capital-latlng" => (self.capital_info.latlng.len() >= 2)
                .then(|| fmt::latlng(&self.capital_info.latlng)),
            "region" => (!self.region.is_empty()).then(|| self.region.clone()),
            "subregion" => {
                (!self.subregion.is_empty()).then(|| self.subregion.clone())
            }
            "population" => Some(fmt::thousands(self.population)),
            "area" => (self.area > 0.0)
                .then(|| fmt::thousands(self.area as u64)),
            "languages" => (!self.languages.is_empty())
                .then(|| self.languages_line()),
            "currencies" => (!self.currencies.is_empty())
                .then(|| fmt::join_or_missing(self.currencies(), " | ")),
            "timezones" => (!self.timezones.is_empty())
                .then(|| self.timezones.join(" | ")),
            "tld" => (!self.tld.is_empty()).then(|| self.tld.join(" | ")),
            "latlng" => (self.latlng.len() >= 2)
                .then(|| fmt::latlng(&self.latlng)),
            "borders" => (!self.borders.is_empty())
                .then(|| self.borders_line(all)),
            "independent" => self.independent.map(yes_no),
            "un-member" => self.un_member.map(yes_no),
            "fifa" => self.fifa.clone(),
            "driving-side" => self.car.side.clone(),
            "calling" => self.calling_code(),
            "gini" => self
                .gini
                .iter()
                .next_back()
                .map(|(year, v)| format!("{v} ({year})")),
            "maps" => self.maps.google_maps.clone(),
            "cca2" => Some(self.cca2.clone()),
            "cca3" => Some(self.cca3.clone()),
            "emoji" => Some(self.flag.clone()),
            _ => anyhow::bail!(
                "unknown field '{name}' — available: {}",
                FIELDS.join(", ")
            ),
        };
        Ok(value)
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
