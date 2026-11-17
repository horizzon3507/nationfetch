# Contributing

## Environment

Rust 1.85 or newer (edition 2024). `cargo build` and you're set.

## Before committing

```sh
cargo fmt --check
cargo clippy -- -D warnings
cargo build
cargo test
```

All four need to pass. There are few tests today — new logic in
`country.rs`/`fmt.rs` should come with unit tests where it makes sense.

## Commits

One commit per logical change, breakable-commits style:

- Prefixes `feat:`, `fix:`, `refactor:`, `test:`, `docs:`, `chore:`.
- Every commit is small, reviewable, and leaves the project building.
- Imperative subjects ≤ 72 chars, like the history — `feat: render
  country flags as colored ascii`, `chore: switch license to apache 2.0`.
- No `wip`, no empty commits, no two subjects in one commit.
- The cache (`~/.cache/nationfetch`) and any secrets never get committed.

## Branching

Work happens on `master`, straight history. PRs welcome; keep them
single-purpose like the commits.

## Layout

| File | Holds |
|---|---|
| `src/cli.rs` | clap argument definitions + help examples |
| `src/country.rs` | `Country` model, `info_lines`, `field_value`, lookups |
| `src/display.rs` | flag+info layout, ANSI-aware padding |
| `src/ascii.rs` | PNG → truecolor ASCII art |
| `src/api.rs` | dataset + flag PNG fetching |
| `src/cache.rs` | `~/.cache/nationfetch`: dataset + flag art |
| `src/fmt.rs` | number/date-ish formatting helpers |

## Add a CLI flag

1. Field on `Cli` in `src/cli.rs` — `#[arg(long)]` plus a doc comment
   (that becomes `--help` text). Kebab-case the flag name, or set
   `long = "..."` to match the original's spelling.
2. Dispatch in `run()` in `src/main.rs`.
3. Line in the `EXAMPLES` const if it's a headline feature.
4. Bullet in the development version of `CHANGELOG.md`.
5. Row in the usage table of `README.md`.

## Surface a dataset field

The cache stores the upstream JSON verbatim, so new fields appear with
no migration:

1. Field on `Country` in `src/country.rs` — `#[serde(default)]` so
   entries that lack it parse anyway; `Option<T>` when it's genuinely
   optional.
2. `push()` line in `info_lines` (skip the line entirely when the field
   has no data) and/or an arm in `field_value`.
3. Bullet in `CHANGELOG.md`; mention it in the README info-block list.

## Releases

Hory-style versioning — see `CHANGELOG.md` for the format. Bumps touch
`Cargo.toml`'s `version`, a changelog entry, and a `vX.Y.Z-stable` tag.
