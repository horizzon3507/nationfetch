//! Render flag PNGs as truecolor ANSI art.

use std::fmt::Write;

use anyhow::{Context, Result};
use image::{DynamicImage, imageops::FilterType};

/// Output dimensions in terminal characters.
pub struct FlagSize {
    pub width: u16,
    pub height: u16,
}

/// Darkest → brightest glyph ramp.
const RAMP: &[u8] = b" .:-=+*#%@";

/// Convert a PNG flag into colored ASCII art: every pixel becomes a
/// brightness-mapped glyph painted with the pixel's truecolor ANSI
/// escape. A single trailing `[0m` resets each line.
///
/// The escapes are emitted unconditionally — the art is meant for
/// terminals and is stored in the cache as-is.
pub fn png_to_ascii(png: &[u8], size: &FlagSize) -> Result<String> {
    let img = image::load_from_memory_with_format(png, image::ImageFormat::Png)
        .context("could not decode flag PNG")?;
    Ok(image_to_ascii(&img, size))
}

fn image_to_ascii(img: &DynamicImage, size: &FlagSize) -> String {
    let img = img
        .resize_exact(size.width as u32, size.height as u32, FilterType::Triangle)
        .to_rgba8();

    // Per cell: 24 (fg) + 24 (bg) escape bytes + 2 chars + a newline's
    // worth of slack.
    let mut out = String::with_capacity(img.width() as usize * img.height() as usize * 50);
    let mut prev: Option<[u8; 3]> = None;
    for y in 0..img.height() {
        for x in 0..img.width() {
            let p = img.get_pixel(x, y);
            let [r, g, b] = [p[0], p[1], p[2]];
            let rgb = [r, g, b];
            if prev != Some(rgb) {
                let _ = write!(out, "\x1b[38;2;{r};{g};{b}m");
                prev = Some(rgb);
            }
            // Rec. 601 luma, un-premultiplied over black.
            let luma = (299.0 * r as f32 + 587.0 * g as f32 + 114.0 * b as f32) / (255.0 * 1000.0)
                * p[3] as f32
                / 255.0;
            let idx = ((luma * (RAMP.len() - 1) as f32).round() as usize).min(RAMP.len() - 1);
            out.push(RAMP[idx] as char);
        }
        out.push_str("\x1b[0m\n");
        prev = None;
    }
    out
}
