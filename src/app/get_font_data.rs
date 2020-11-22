use crate::app;
use std::collections::HashMap;

pub fn get_font_data(path: &std::path::Path) -> Result<HashMap<char, app::GlyphData>, String> {
    let data = match std::fs::read_to_string(path) {
        Ok(d) => d,
        Err(_) => return Err(format!("failed to read json {:?}", path)),
    };

    let value: app::FontData = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(_) => return Err(format!("failed to deserialize json {:?}", path)),
    };

    let glyph_by_unicode =
        value
            .glyphs
            .into_iter()
            .fold(std::collections::HashMap::new(), |mut acc, g| {
                acc.insert(std::char::from_u32(g.unicode).unwrap(), g);
                acc
            });

    Ok(glyph_by_unicode)
}
