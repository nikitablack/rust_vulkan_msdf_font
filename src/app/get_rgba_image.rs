pub fn get_rgba_image(path: &std::path::Path) -> Result<image::RgbaImage, String> {
    let image = match image::open(path) {
        Ok(img) => img,
        Err(_) => return Err(format!("failed to open image {:?}", path)),
    };

    Ok(image.to_rgba())
}
