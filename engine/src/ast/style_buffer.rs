use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct StyleBuffer {
    /// Each style attribute maps to a 4-byte RGBA representation [R, G, B, A]
    pub entries: HashMap<String, [u8; 4]>,
    /// Raw opacity values (0.0 – 1.0) stored separately for blending
    pub opacities: HashMap<String, f32>,
}

impl StyleBuffer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Parse a CSS hex color string (#RRGGBB or #RGB) and store it.
    pub fn set_color(&mut self, key: &str, hex: &str) {
        if let Some(rgba) = parse_hex_color(hex) {
            self.entries.insert(key.to_string(), rgba);
        }
    }

    /// Store a raw RGBA value directly.
    pub fn set_rgba(&mut self, key: &str, rgba: [u8; 4]) {
        self.entries.insert(key.to_string(), rgba);
    }

    /// Retrieve a color as a CSS `rgba()` string, applying stored opacity.
    pub fn get_css(&self, key: &str) -> Option<String> {
        let [r, g, b, _] = *self.entries.get(key)?;
        let alpha = self.opacities.get(key).copied().unwrap_or(1.0);
        Some(format!("rgba({},{},{},{:.2})", r, g, b, alpha))
    }

    /// Retrieve the raw hex string (e.g. `#1a2b3c`) for SVG attributes.
    pub fn get_hex(&self, key: &str) -> Option<String> {
        let [r, g, b, _] = *self.entries.get(key)?;
        Some(format!("#{:02x}{:02x}{:02x}", r, g, b))
    }
}

/// Parse `#RRGGBB`, `#RGB`, or `#RRGGBBAA` into an RGBA byte array.
fn parse_hex_color(hex: &str) -> Option<[u8; 4]> {
    let hex = hex.trim_start_matches('#');
    match hex.len() {
        3 => {
            let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
            let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
            let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
            Some([r, g, b, 255])
        }
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some([r, g, b, 255])
        }
        8 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            let a = u8::from_str_radix(&hex[6..8], 16).ok()?;
            Some([r, g, b, a])
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_color_parsing() {
        let mut buf = StyleBuffer::new();
        buf.set_color("fill", "#007acc");
        assert_eq!(buf.get_hex("fill"), Some("#007acc".to_string()));
    }

    #[test]
    fn test_short_hex() {
        let mut buf = StyleBuffer::new();
        buf.set_color("stroke", "#fff");
        assert_eq!(buf.get_hex("stroke"), Some("#ffffff".to_string()));
    }
}
