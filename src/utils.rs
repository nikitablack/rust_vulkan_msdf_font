pub fn f32_to_u8(v: &[f32]) -> &[u8] {
    unsafe { std::slice::from_raw_parts(v.as_ptr() as *const u8, v.len() * 4) }
}

pub fn get_ortho_projection_matrix(
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
    z_near: f32,
    z_far: f32,
) -> Vec<f32> {
    vec![
        2.0 / (right - left),
        0.0,
        0.0,
        0.0,
        0.0,
        2.0 / (bottom - top),
        0.0,
        0.0,
        0.0,
        0.0,
        1.0 / (z_near - z_far),
        0.0,
        -(left + right) / (right - left),
        -(top + bottom) / (bottom - top),
        z_near / (z_near - z_far),
        1.0,
    ]
}
