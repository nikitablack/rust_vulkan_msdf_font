use crate::app;
use ash::vk;

pub fn get_swapchain_images(
    device: &ash::Device,
    swapchain: vk::SwapchainKHR,
    swapchain_loader: &ash::extensions::khr::Swapchain,
    debug_utils_loader: &ash::extensions::ext::DebugUtils,
) -> Result<Vec<vk::Image>, String> {
    let swapchain_images = match unsafe { swapchain_loader.get_swapchain_images(swapchain) } {
        Ok(images) => images,
        Err(_) => return Err(String::from("failed to get swapchain images")),
    };

    for (i, &image) in swapchain_images.iter().enumerate() {
        app::set_debug_utils_object_name(
            &debug_utils_loader,
            device.handle(),
            image,
            format!("swapchain image {}", i),
        );
    }

    Ok(swapchain_images)
}
