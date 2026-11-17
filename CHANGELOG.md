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

## v0.1.0-stable · 17/11/2026

First public cut of nationfetch (then named countryfetch): cached lookups, truecolor flag art, and a parallel flag sync. This version was made for CLI with a stable release channel on 17/11/2026 (v0.1.0-stable).

- `--sync` pulls the frozen REST Countries v3.1 dataset (the live API now requires an API key) into the platform cache directory as `countries.json`.
- Lookups run fully offline after sync: country by name (exact match first, then fuzzy substring), by capital, or `--random`.
- Colored info block: population with thousands separators, languages, capital and its coordinates, region/subregion, timezones, TLD and currencies, with graceful fallbacks where the dataset has gaps.
- Flags render as truecolor ANSI ASCII art from flagcdn PNGs: `--flag` prints cached art, `--flagremote` renders live at `--width`/`--height`, `--flagonly` skips the info block.
- `--sync --flag` caches art for all 250 countries through a small worker pool — a few seconds, versus tens in the Go original.
- Falls back to the flagcdn `cca2` URL when a dataset `flags.png` link is dead (e.g. Afghanistan's Wikimedia thumbnail rejecting nonstandard sizes).
