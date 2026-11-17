use clap::Parser;

const EXAMPLES: &str = "\
EXAMPLES:
  nationfetch italy
        Fetch information about Italy.
  nationfetch italy --flag
        Flag art and info side by side.
  nationfetch --sync --flag
        Store all countries in the cache, including flag art.
  nationfetch --capital \"kuala lumpur\"
        Fetch the country of the given capital.
  nationfetch --field capital france
        Print just \"Paris\" — handy in scripts.
  nationfetch --list
        List all cached countries.";

/// Fetch information about countries in your terminal.
///
/// Lookups run against a local cache; create or refresh it with
/// `nationfetch --sync` (add `--flag` to also store flag art).
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

    /// Include the flag in the operation: with --sync it stores flag art
    /// in the cache, with a lookup it prints the cached flag
    #[arg(long, short = 'f')]
    pub flag: bool,

    /// Print the flag only; must be used together with a country name
    #[arg(long = "flagonly")]
    pub flag_only: bool,

    /// Render the flag from the remote image URL instead of the cache
    #[arg(long = "flagremote")]
    pub flag_remote: bool,

    /// Flag width in characters
    #[arg(long, default_value_t = 40, value_parser = clap::value_parser!(u16).range(1..))]
    pub width: u16,
    /// Flag height in characters
    #[arg(long, default_value_t = 20, value_parser = clap::value_parser!(u16).range(1..))]
    pub height: u16,
    /// List all cached country names
    #[arg(long, short = 'l')]
    pub list: bool,

    /// Print only one field's raw value (e.g. --field capital);
    /// see the error message for available names
    #[arg(long, value_name = "FIELD")]
    pub field: Option<String>,

    /// Print the resolved country's raw JSON
    #[arg(long)]
    pub json: bool,
}
