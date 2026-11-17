//! Value formatting for the country info block.

/// Placeholder shown when the API has no value for a field.
pub const MISSING: &str = "unknown";

/// Thousands-separate a non-negative integer: 1234567 -> "1,234,567".
pub fn thousands(n: u64) -> String {
    let digits = n.to_string();
    let mut out = String::with_capacity(digits.len() + digits.len() / 3);
    for (i, d) in digits.chars().enumerate() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(d);
    }
    out
}

/// "lat/lng" with two decimals, or the placeholder when coordinates
/// are missing.
pub fn latlng(coords: &[f64]) -> String {
    match coords {
        [lat, lng, ..] => format!("{lat:.2}/{lng:.2}"),
        _ => MISSING.into(),
    }
}

/// Join displayable items with `sep`, or the placeholder when empty.
pub fn join_or_missing<I, T>(items: I, sep: &str) -> String
where
    I: IntoIterator<Item = T>,
    T: std::fmt::Display,
{
    let mut parts = items.into_iter().peekable();
    if parts.peek().is_none() {
        return MISSING.into();
    }
    parts
        .map(|p| p.to_string())
        .collect::<Vec<_>>()
        .join(sep)
}

/// An optional-ish string field, or the placeholder when empty.
pub fn or_missing(value: &str) -> String {
    if value.is_empty() {
        MISSING.into()
    } else {
        value.into()
    }
}
