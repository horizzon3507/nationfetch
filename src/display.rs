//! Terminal output: neofetch-style flag + info side by side.

use colored::Colorize;

use crate::country::Country;

const GAP: &str = "   ";

/// Print the title (`Name 🇮🇹` + dashes) followed by the info lines —
/// the plain, flagless layout.
pub fn print(country: &Country, all: &[Country]) {
    println!();
    for line in title_lines(country)
        .into_iter()
        .chain(country.info_lines(all))
    {
        println!("{line}");
    }
}

/// Print flag art on the left, the info block on the right, like
/// `neofetch`/`fastfetch`. When the art is shorter than the info it's
/// vertically centered against it.
pub fn print_side_by_side(art: &str, country: &Country, all: &[Country]) {
    let art_lines: Vec<&str> = art.lines().collect();
    let art_width = art_lines.iter().map(|l| visible_len(l)).max().unwrap_or(0);
    let info = {
        let mut v = title_lines(country);
        v.extend(country.info_lines(all));
        v
    };

    // Center the shorter column vertically against the taller one.
    let offset = info.len().saturating_sub(art_lines.len()) / 2;
    let rows = (art_lines.len() + offset).max(info.len());
    for i in 0..rows {
        let left = i
            .checked_sub(offset)
            .and_then(|j| art_lines.get(j))
            .copied()
            .unwrap_or_default();
        let pad = art_width.saturating_sub(visible_len(left));
        let right = info.get(i).map(String::as_str).unwrap_or_default();
        if art_width == 0 {
            println!("{right}");
        } else {
            println!("{left}{:pad$}{GAP}{right}", "", pad = pad);
        }
    }
}

fn title_lines(country: &Country) -> Vec<String> {
    let title = format!("{} {}", country.name.common, country.flag).bold();
    let dashes = "-".repeat(title_text_len(&country.name.common, &country.flag));
    vec![title.to_string(), dashes]
}

/// Title character count without color escapes; emoji count as one
/// (close enough for the dash separator).
fn title_text_len(common: &str, flag: &str) -> usize {
    common.chars().count() + 1 + flag.chars().count()
}

/// Length of a string ignoring ANSI escape sequences (`ESC[…m`).
pub fn visible_len(s: &str) -> usize {
    let mut len = 0;
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c == '\u{1b}' {
            for next in chars.by_ref() {
                if next == 'm' {
                    break;
                }
            }
        } else {
            len += 1;
        }
    }
    len
}
