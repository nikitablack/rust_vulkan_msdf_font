pub fn create_surface(
    entry: &ash::Entry,
    instance: &ash::Instance,
    window: &winit::window::Window,
) -> Result<ash::vk::SurfaceKHR, String> {
    let surface = match unsafe { ash_window::create_surface(entry, instance, window, None) } {
        Ok(surface) => surface,
        Err(_) => return Err(String::from("failed to create surface")),
    };

    Ok(surface)
}
