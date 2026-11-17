# countryfetch

A CLI tool for fetching information about countries, written in Rust (edition
2024). Inspired by [pridont/countryfetch](https://github.com/pridont/countryfetch)
(Go).

The original tool talked to the public restcountries.com v3.1 API, which has
since been deprecated. Instead, `countryfetch --sync` downloads the same frozen
v3.1 dataset from the [archived project
repository](https://github.com/restcountries/restcountries) — no API key
needed. Flag images come from [flagcdn.com](https://flagcdn.com).

## Installation

```bash
cargo install --path .
```

Run `countryfetch --sync` once to populate the cache (`~/.cache/countryfetch`
on Linux, the platform cache dir elsewhere). Add `--flag` to also render and
store ASCII art for every country's flag.

## Usage

```
USAGE:
  countryfetch [OPTIONS] [NAME]

ARGS:
  <NAME>          Find a country by name (exact match first, then fuzzy)

OPTIONS:
  --sync                    Fetch and save country data to the cache
  --capital <CAPITAL>       Find a country by its capital
  -r, --random              Print a random country
  -f, --flag                Include the flag: stores art with --sync,
                            prints cached art with a lookup
  --flagonly                Print the flag only; needs a country name
  --flagremote              Render the flag from the remote image URL
  --width <WIDTH>           Flag width in characters [default: 40]
  --height <HEIGHT>         Flag height in characters [default: 12]
  -h, --help                Print help
  -V, --version             Print version
```

## Examples

```bash
countryfetch italy
        Fetch information about Italy.
countryfetch italy --flag
        Fetch information about Italy, including its flag.
countryfetch --sync --flag
        Store information of all countries in the cache, including
        generated flag ASCII art.
countryfetch --capital "kuala lumpur"
        Fetch information about the country of the given capital.
countryfetch --flagonly "united states"
        Fetch just the flag of the USA.
```

## Data sources

- Country data: [REST Countries v3.1](https://github.com/restcountries/restcountries)
  dataset (built on [mledoze/world-countries](https://github.com/mledoze/countries)),
  cached as `countries.json`.
- Flag images: [flagcdn.com](https://flagcdn.com), rendered to truecolor ANSI
  ASCII art and cached under `flags/`.
