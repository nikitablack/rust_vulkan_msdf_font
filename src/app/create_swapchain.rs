use ash::extensions::khr;
use ash::vk;

pub fn create_swapchain(
    window: &winit::window::Window,
    instance: &ash::Instance,
    device: &ash::Device,
    physical_device: vk::PhysicalDevice,
    surface: vk::SurfaceKHR,
    surface_format: vk::SurfaceFormatKHR,
    present_mode: vk::PresentModeKHR,
    surface_loader: &khr::Surface,
    old_swapchain: vk::SwapchainKHR,
) -> Result<(vk::SwapchainKHR, vk::Extent2D), String> {
    let capabilities = match unsafe {
        surface_loader.get_physical_device_surface_capabilities(physical_device, surface)
    } {
        Ok(capabilities) => capabilities,
        Err(_) => {
            return Err(String::from(
                "failed to get physical device surface capabilities",
            ))
        }
    };

    let window_size = window.inner_size();

    let mut surface_extent = vk::Extent2D::default();
    if capabilities.current_extent.width == u32::MAX {
        surface_extent.width = std::cmp::max(
            capabilities.min_image_extent.width,
            std::cmp::min(capabilities.max_image_extent.width, window_size.width),
        );
        surface_extent.height = std::cmp::max(
            capabilities.min_image_extent.height,
            std::cmp::min(capabilities.max_image_extent.height, window_size.height),
        );
    } else {
        surface_extent = capabilities.current_extent;
    }

    let image_count = std::cmp::min(
        std::cmp::max(capabilities.min_image_count, 3),
        capabilities.max_image_count,
    );

    let create_info = vk::SwapchainCreateInfoKHR::builder()
        .surface(surface)
        .min_image_count(image_count)
        .image_format(surface_format.format)
        .image_color_space(surface_format.color_space)
        .image_extent(surface_extent)
        .image_array_layers(1)
        .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
        .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
        .pre_transform(capabilities.current_transform)
        .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
        .present_mode(present_mode)
        .clipped(true)
        .old_swapchain(old_swapchain);

    let swapchain_loader = khr::Swapchain::new(instance, device);

    let swapchain = match unsafe { swapchain_loader.create_swapchain(&create_info, None) } {
        Ok(swapchain) => swapchain,
        Err(_) => return Err(String::from("failed to create swapchain")),
    };

    if old_swapchain == vk::SwapchainKHR::null() {
        unsafe { swapchain_loader.destroy_swapchain(old_swapchain, None) };
    }

    Ok((swapchain, surface_extent))
}
