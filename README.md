# nationfetch

Fetch country information in your terminal, in Rust. Flag rendered as colored ASCII art, neofetch-style, next to the data — inspired by [pridont/countryfetch](https://github.com/pridont/countryfetch) (Go).

```
##################################   Japan 🇯🇵
##################################   --------
###############    ###############   Official: Japan
############  ###   ###############   Capital: Tokyo
############        ##############   Capital Lat/Lng: 35.68/139.69
############        ##############   Region: Asia (Eastern Asia)
############  ###   ###############   Population: 124,730,000
###############    ###############   Area: 377,930 km²
##################################   Languages: Japanese
##################################   Currencies: Japanese yen [JPY][¥]
##################################   TLD: .jp
##################################   Borders: none
##################################   Independent: yes
##################################   UN Member: yes
```

## Install

Needs Rust 1.85+ (edition 2024).

```sh
cargo install --git https://github.com/horizzon3507/nationfetch
```

Or from a clone:

```sh
git clone https://github.com/horizzon3507/nationfetch
cd nationfetch
cargo install --path .
```

## Setup

The dataset lives in a local cache so lookups are instant and offline.
Sync once, and again whenever you want fresh data:

```sh
nationfetch --sync          # country data (~1.5 MB)
nationfetch --sync --flag   # also render and cache flag art for all 250 countries
```

The flag sync runs a small worker pool — all flags in a few seconds.

## Usage

```sh
nationfetch [OPTIONS] [NAME]
```

| Command | Arguments | Does |
|---|---|---|
| `nationfetch` | `<name>` | Exact name match, then fuzzy; prints the info block |
| `nationfetch` | `<name> --flag` | Flag art and info side by side |
| `nationfetch` | `<name> --flagonly` | Just the cached flag art |
| `nationfetch` | `<name> --flagremote` | Renders the flag live from the remote PNG (no cache needed) |
| `nationfetch` | `--sync [--flag]` | Downloads the dataset to the cache |
| `nationfetch` | `--capital <name>` | Finds a country by its capital |
| `nationfetch` | `--random` | Picks a random country |
| `nationfetch` | `--list` | Lists all cached country names |
| `nationfetch` | `<name> --field <f>` | One raw value, for scripts |
| `nationfetch` | `<name> --json` | The country's full JSON object |
| `nationfetch` | `--width <n> --height <n>` | Flag art size in characters (40×20 default) |

`--field` accepts: `name`, `official`, `capital`, `capital-latlng`, `region`,
`subregion`, `population`, `area`, `languages`, `currencies`, `timezones`,
`tld`, `latlng`, `borders`, `independent`, `un-member`, `fifa`,
`driving-side`, `calling`, `gini`, `maps`, `cca2`, `cca3`, `emoji`.
Fields with no data print `-`.

`--list`, `--field` and `--json` also work on `--capital` and `--random`
picks. Examples:

```sh
nationfetch italy --flag              # flag + info, side by side
nationfetch brazil --field capital    # Brasília
nationfetch --capital "kuala lumpur"  # Malaysia's info block
nationfetch --flagonly "united states"
nationfetch chile --json | jq .cca3   # "CHL"
```

## Info block

Official name, capital and its coordinates, region (subregion),
population, area, languages, currencies, timezones, TLD, coordinates,
borders resolved to country names, independence, UN membership, FIFA
code, driving side, calling code, gini coefficient and a maps link.
Missing data shows `unknown` instead of failing.

## Data

- Country data: the frozen [REST Countries v3.1](https://github.com/restcountries/restcountries)
  dataset (built on [mledoze/world-countries](https://github.com/mledoze/countries)).
  The live API moved to v5 and requires a key; the archived dataset has the
  same fields the original tool used.
- Flag images: [flagcdn.com](https://flagcdn.com), rendered to truecolor
  ANSI art. Falls back to the `cca2` CDN URL when the dataset link is dead.
- Cache: `~/.cache/nationfetch` on Linux (platform cache dir elsewhere).
  `countries.json` holds the upstream JSON verbatim; `flags/` holds the art.
  An old `countryfetch` cache migrates automatically.

## Roadmap

- Embed the dataset in the binary so `--sync` works fully offline.
- Shell completions.

## More

- [CHANGELOG.md](CHANGELOG.md): versions in Hory style, `v0.2.0-stable` current.
- [LICENSE](LICENSE): Apache-2.0.
