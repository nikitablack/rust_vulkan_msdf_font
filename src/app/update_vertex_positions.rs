use crate::app;
use crate::utils;

pub fn update_vertex_positions(
    dst_buffer: *mut u8,
    text: &str,
    glyph_by_unicode: &std::collections::HashMap<char, app::GlyphData>,
    font_size: f32,
) {
    let mut cursor = (0.0f32, 100.0f32);

    let mut pos_data = Vec::new();

    for x in text.chars() {
        let glyph = &glyph_by_unicode[&x];
        let plane_bounds = glyph.planeBounds.as_ref();

        let (left, right, top, bottom) = match plane_bounds {
            Some(val) => (val.left, val.right, val.top, val.bottom),
            None => (0.0, 0.0, 0.0, 0.0),
        };

        let v1 = (cursor.0 + left * font_size, cursor.1 - top * font_size);
        let v2 = (cursor.0 + right * font_size, v1.1);
        let v3 = (v2.0, cursor.1 - bottom * font_size);
        let v4 = (v1.0, v3.1);

        pos_data.push(v1.0);
        pos_data.push(v1.1);
        pos_data.push(v2.0);
        pos_data.push(v2.1);
        pos_data.push(v3.0);
        pos_data.push(v3.1);
        pos_data.push(v3.0);
        pos_data.push(v3.1);
        pos_data.push(v4.0);
        pos_data.push(v4.1);
        pos_data.push(v1.0);
        pos_data.push(v1.1);

        cursor.0 += glyph.advance * 32.0;
    }

    let pos_data_raw = utils::f32_to_u8(&pos_data);

    unsafe { std::ptr::copy_nonoverlapping(pos_data_raw.as_ptr(), dst_buffer, pos_data_raw.len()) };
}
