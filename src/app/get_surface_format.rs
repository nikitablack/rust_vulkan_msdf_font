use ash::vk;

pub fn get_surface_format(
    physical_device: vk::PhysicalDevice,
    surface: vk::SurfaceKHR,
    surface_loader: &ash::extensions::khr::Surface,
) -> Result<vk::SurfaceFormatKHR, String> {
    let formats = match unsafe {
        surface_loader.get_physical_device_surface_formats(physical_device, surface)
    } {
        Ok(formats) => formats,
        Err(_) => {
            return Err(String::from(
                "failed to get physical device surface formats",
            ));
        }
    };

    if formats.is_empty() {
        return Err(String::from(
            "failed to get physical device surface formats",
        ));
    }

    if formats.len() == 1 && formats[0].format == vk::Format::UNDEFINED {
        return Ok(vk::SurfaceFormatKHR {
            format: vk::Format::B8G8R8A8_UNORM,
            color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR,
        });
    }

    for f in &formats {
        if f.format == vk::Format::B8G8R8A8_UNORM
            && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
        {
            return Ok(vk::SurfaceFormatKHR {
                format: vk::Format::B8G8R8A8_UNORM,
                color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR,
            });
        }
    }

    Ok(formats[0])
}
