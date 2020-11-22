use crate::app;
use crate::utils;

pub fn update_vertex_uvs(
    dst_buffer: *mut u8,
    text: &str,
    glyph_by_unicode: &std::collections::HashMap<char, app::GlyphData>,
    atlas_width: f32,
    atlas_height: f32,
) {
    let mut uv_data = Vec::new();
    for x in text.chars() {
        let glyph = &glyph_by_unicode[&x];
        let atlas_bounds = glyph.atlasBounds.as_ref();

        let (left, right, top, bottom) = match atlas_bounds {
            Some(val) => (val.left, val.right, val.top, val.bottom),
            None => (0.0, 0.0, 0.0, 0.0),
        };

        let v1 = (left / atlas_width, (atlas_height - top) / atlas_height);
        let v2 = (right / atlas_width, v1.1);
        let v3 = (v2.0, (atlas_height - bottom) / atlas_height);
        let v4 = (v1.0, v3.1);

        uv_data.push(v1.0);
        uv_data.push(v1.1);
        uv_data.push(v2.0);
        uv_data.push(v2.1);
        uv_data.push(v3.0);
        uv_data.push(v3.1);
        uv_data.push(v3.0);
        uv_data.push(v3.1);
        uv_data.push(v4.0);
        uv_data.push(v4.1);
        uv_data.push(v1.0);
        uv_data.push(v1.1);
    }

    let uv_data_raw = utils::f32_to_u8(&uv_data);

    unsafe { std::ptr::copy_nonoverlapping(uv_data_raw.as_ptr(), dst_buffer, uv_data_raw.len()) };
}
