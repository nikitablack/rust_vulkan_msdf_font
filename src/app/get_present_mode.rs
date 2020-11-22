use ash::vk;

pub fn get_present_mode(
    physical_device: vk::PhysicalDevice,
    surface: vk::SurfaceKHR,
    surface_loader: &ash::extensions::khr::Surface,
) -> Result<vk::PresentModeKHR, String> {
    let modes = match unsafe {
        surface_loader.get_physical_device_surface_present_modes(physical_device, surface)
    } {
        Ok(formats) => formats,
        Err(_) => {
            return Err(String::from(
                "failed to get physical device surface present modes",
            ));
        }
    };

    if modes.is_empty() {
        return Err(String::from(
            "failed to get physical device surface present modes",
        ));
    }

    if modes.contains(&vk::PresentModeKHR::MAILBOX) {
        return Ok(vk::PresentModeKHR::MAILBOX);
    }

    if modes.contains(&vk::PresentModeKHR::IMMEDIATE) {
        return Ok(vk::PresentModeKHR::IMMEDIATE);
    }

    Ok(vk::PresentModeKHR::FIFO)
}
