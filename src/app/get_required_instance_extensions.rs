pub fn get_required_instance_extensions(
    window: &winit::window::Window,
) -> Result<Vec<&'static std::ffi::CStr>, String> {
    let mut instance_extensions = match ash_window::enumerate_required_extensions(window) {
        Ok(extensions) => extensions,
        Err(_) => {
            return Err(String::from(
                "failed to enumerate required instance extensions",
            ))
        }
    };

    instance_extensions.push(ash::extensions::ext::DebugUtils::name());

    Ok(instance_extensions)
}
