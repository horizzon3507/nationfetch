# Changelog

We follow [Semantic Versioning](https://semver.org/) and [Keep a Changelog](https://keepachangelog.com/). nationfetch is a single CLI surface.

<details>
<summary>To see more about versioning, expand this.</summary>

Every version string starts with `v` (required), e.g. `v0.1.0-stable`.

Here the installable surface is **CLI** (`nationfetch`). Other apps swap in their own names the same way — e.g. **GNOME**, **Desktop**, **Web** — whatever you actually ship.

| Part | What you install | Example |
| --- | --- | --- |
| **CLI** | `nationfetch` in the terminal | `v0.1.0-stable` |

With one surface there is no `m` in the tag and no per-surface sections — just the version notes.

Each release heading is the version and date (`## v0.1.0-stable · 17/11/2026`); under it, a short summary ends with a plain sentence like: "This version was made for CLI with a stable release channel on 17/11/2026 (v0.1.0-stable)."

### What the channel suffix means

| Suffix | In plain words |
| --- | --- |
| **-alpha** | Very early. Expect missing pieces and lots of bugs. |
| **-beta** | Mostly there, but still rough. Fine to try; not the "official" install. |
| **-stable** | Ready for daily use. This is what we put on GitHub Releases. |

We only call something **stable** when we mean it.

</details>


## v0.3.0-stable · 17/11/2026

Docs pass: README rewritten in the ace style (usage table, info-block spec, data section, roadmap) and `--help` examples now cover the scripting flags. This version was made for CLI with a stable release channel on 17/11/2026 (v0.3.0-stable).

- README rewritten: real usage table, full `--field` name list, info-block spec, data/cache details and a short roadmap.
- `--help` examples updated to show `--field` and `--list`.
- License switched to Apache-2.0.

## v0.2.0-stable · 17/11/2026

Renamed to nationfetch, neofetch-style side-by-side output, a much richer info block, and scripting-friendly raw outputs. This version was made for CLI with a stable release channel on 17/11/2026 (v0.2.0-stable).

- Renamed the binary and cache directory from `countryfetch` to `nationfetch`; the old cache migrates automatically on first run.
- With `--flag`, output is now flag art on the left and the info block on the right (vertically centered), like neofetch/fastfetch; default flag height goes to 20 to match.
- The info block doubles: official name, capital coordinates, region (subregion), population, area in km², languages, currencies, timezones, TLD, borders resolved to country names, independence and UN membership, FIFA code, driving side, calling code, gini and a maps link.
- `--list` prints every cached country name; `--field <name>` prints one raw value (capital, borders, calling, cca3 and ~20 more) for scripts; `--json` emits the resolved country's full object — all three also work on `--capital` and `--random` picks.
- The cache stores the upstream dataset verbatim, so newly surfaced fields never need a re-sync.

## v0.1.0-stable · 17/11/2026

First public cut of nationfetch (then named countryfetch): cached lookups, truecolor flag art, and a parallel flag sync. This version was made for CLI with a stable release channel on 17/11/2026 (v0.1.0-stable).

- `--sync` pulls the frozen REST Countries v3.1 dataset (the live API now requires an API key) into the platform cache directory as `countries.json`.
- Lookups run fully offline after sync: country by name (exact match first, then fuzzy substring), by capital, or `--random`.
- Colored info block: population with thousands separators, languages, capital and its coordinates, region/subregion, timezones, TLD and currencies, with graceful fallbacks where the dataset has gaps.
- Flags render as truecolor ANSI ASCII art from flagcdn PNGs: `--flag` prints cached art, `--flagremote` renders live at `--width`/`--height`, `--flagonly` skips the info block.
- `--sync --flag` caches art for all 250 countries through a small worker pool — a few seconds, versus tens in the Go original.
- Falls back to the flagcdn `cca2` URL when a dataset `flags.png` link is dead (e.g. Afghanistan's Wikimedia thumbnail rejecting nonstandard sizes).
