/// Embedded proportional character-width table derived from standard monospace metrics.
/// Allows node boundary calculation without DOM access, enabling pure-Wasm layout.

/// Average character width ratios relative to font-size for a proportional sans-serif font.
/// Values are in units of `font_size * ratio`.
const CHAR_WIDTHS: &[(char, f32)] = &[
    (' ', 0.28),
    ('!', 0.30),
    ('"', 0.38),
    ('#', 0.62),
    ('$', 0.56),
    ('%', 0.74),
    ('&', 0.68),
    ('\'', 0.22),
    ('(', 0.32),
    (')', 0.32),
    ('*', 0.44),
    ('+', 0.62),
    (',', 0.28),
    ('-', 0.34),
    ('.', 0.28),
    ('/', 0.34),
    ('0', 0.56),
    ('1', 0.56),
    ('2', 0.56),
    ('3', 0.56),
    ('4', 0.56),
    ('5', 0.56),
    ('6', 0.56),
    ('7', 0.56),
    ('8', 0.56),
    ('9', 0.56),
    (':', 0.28),
    (';', 0.28),
    ('<', 0.62),
    ('=', 0.62),
    ('>', 0.62),
    ('?', 0.50),
    ('@', 0.90),
    ('A', 0.62),
    ('B', 0.62),
    ('C', 0.62),
    ('D', 0.68),
    ('E', 0.56),
    ('F', 0.52),
    ('G', 0.68),
    ('H', 0.68),
    ('I', 0.24),
    ('J', 0.40),
    ('K', 0.62),
    ('L', 0.52),
    ('M', 0.78),
    ('N', 0.68),
    ('O', 0.72),
    ('P', 0.58),
    ('Q', 0.72),
    ('R', 0.62),
    ('S', 0.56),
    ('T', 0.54),
    ('U', 0.68),
    ('V', 0.62),
    ('W', 0.88),
    ('X', 0.60),
    ('Y', 0.58),
    ('Z', 0.56),
    ('[', 0.32),
    ('\\', 0.34),
    (']', 0.32),
    ('^', 0.62),
    ('_', 0.50),
    ('`', 0.36),
    ('a', 0.54),
    ('b', 0.56),
    ('c', 0.48),
    ('d', 0.56),
    ('e', 0.54),
    ('f', 0.30),
    ('g', 0.56),
    ('h', 0.56),
    ('i', 0.22),
    ('j', 0.22),
    ('k', 0.52),
    ('l', 0.22),
    ('m', 0.84),
    ('n', 0.56),
    ('o', 0.56),
    ('p', 0.56),
    ('q', 0.56),
    ('r', 0.34),
    ('s', 0.46),
    ('t', 0.34),
    ('u', 0.56),
    ('v', 0.52),
    ('w', 0.74),
    ('x', 0.52),
    ('y', 0.52),
    ('z', 0.46),
    ('{', 0.34),
    ('|', 0.24),
    ('}', 0.34),
    ('~', 0.62),
];

/// Default width ratio for characters not found in the table.
const DEFAULT_RATIO: f32 = 0.56;

/// Measure the rendered pixel width of a string at the given font size.
/// Uses baked-in proportional metrics — no DOM access required.
pub fn measure_text(text: &str, font_size: f32) -> f32 {
    text.chars().map(|c| char_width(c) * font_size).sum()
}

/// Get the width ratio for a single character.
fn char_width(c: char) -> f32 {
    // Binary search would be faster but linear scan is fine for this table size.
    CHAR_WIDTHS
        .iter()
        .find(|(ch, _)| *ch == c)
        .map(|(_, w)| *w)
        .unwrap_or(DEFAULT_RATIO)
}

/// Calculate the minimum bounding box width for a node label with padding.
pub fn node_width_for_label(label: &str, font_size: f32, padding: f32) -> f32 {
    let text_w = measure_text(label, font_size);
    (text_w + padding * 2.0).max(80.0) // ensure minimum width
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measure_text() {
        let w = measure_text("Hello", 12.0);
        assert!(w > 0.0, "width should be positive");
    }

    #[test]
    fn test_node_width_minimum() {
        let w = node_width_for_label("A", 12.0, 8.0);
        assert!(w >= 80.0);
    }
}
