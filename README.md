# nationfetch

A CLI tool for fetching information about countries, written in Rust (edition
2024). Inspired by [pridont/countryfetch](https://github.com/pridont/countryfetch)
(Go).

The original tool talked to the public restcountries.com v3.1 API, which has
since been deprecated. Instead, `nationfetch --sync` downloads the same frozen
v3.1 dataset from the [archived project
repository](https://github.com/restcountries/restcountries) — no API key
needed. Flag images come from [flagcdn.com](https://flagcdn.com).

## Installation

```bash
cargo install --git https://github.com/horizzon3507/nationfetch
```

Run `nationfetch --sync` once to populate the cache (`~/.cache/nationfetch`
on Linux, the platform cache dir elsewhere). Add `--flag` to also render and
store ASCII art for every country's flag.

## Usage

```
USAGE:
  nationfetch [OPTIONS] [NAME]

ARGS:
  <NAME>          Find a country by name (exact match first, then fuzzy)

OPTIONS:
  --sync                    Fetch and save country data to the cache
  --capital <CAPITAL>       Find a country by its capital
  -r, --random              Print a random country
  -f, --flag                Include the flag: stores art with --sync,
                            prints a neofetch-style flag+info layout
  --flagonly                Print the flag only; needs a country name
  --flagremote              Render the flag from the remote image URL
  --width <WIDTH>           Flag width in characters [default: 40]
  --height <HEIGHT>         Flag height in characters [default: 20]
  -l, --list                List all cached country names
  --field <FIELD>           Print one raw value (capital, borders,
                            calling, gini, cca3, ...) for scripts
  --json                    Print the resolved country's raw JSON
  -h, --help                Print help
  -V, --version             Print version
```

## Examples

```bash
nationfetch italy
        Fetch information about Italy.
nationfetch italy --flag
        Fetch information about Italy, including its flag side by side.
nationfetch --sync --flag
        Store information of all countries in the cache, including
        generated flag ASCII art.
nationfetch --capital "kuala lumpur"
        Fetch information about the country of the given capital.
nationfetch --flagonly "united states"
        Fetch just the flag of the USA.
nationfetch --field capital france
        Print just "Paris" — handy in scripts.
nationfetch --list
        List all cached countries.
```

## Data sources

- Country data: [REST Countries v3.1](https://github.com/restcountries/restcountries)
  dataset (built on [mledoze/world-countries](https://github.com/mledoze/countries)),
  cached as `countries.json`.
- Flag images: [flagcdn.com](https://flagcdn.com), rendered to truecolor ANSI
  ASCII art and cached under `flags/`.
