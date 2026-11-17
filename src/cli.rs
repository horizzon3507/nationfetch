use clap::Parser;

const EXAMPLES: &str = "\
EXAMPLES:
  countryfetch italy
        Fetch information about Italy.
  countryfetch --capital \"kuala lumpur\"
        Fetch information about the country of the given capital.
  countryfetch --sync
        Store information of all countries in the cache.";

/// Fetch information about countries in your terminal.
///
/// Lookups run against a local cache; create or refresh it with
/// `countryfetch --sync`.
#[derive(Parser)]
#[command(version, about, long_about, after_help = EXAMPLES)]
pub struct Cli {
    /// Find a country by name (exact match first, then fuzzy)
    #[arg(value_name = "NAME")]
    pub name: Option<String>,

    /// Fetch and save country data to the cache
    #[arg(long)]
    pub sync: bool,

    /// Find a country by its capital
    #[arg(long, value_name = "CAPITAL")]
    pub capital: Option<String>,

    /// Print a random country
    #[arg(long, short = 'r')]
    pub random: bool,
}
