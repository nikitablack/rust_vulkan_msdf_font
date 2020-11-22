use crate::app;
use ash::version::DeviceV1_0;
use ash::vk;

pub fn create_semaphores(
    device: &ash::Device,
    debug_utils_loader: &ash::extensions::ext::DebugUtils,
) -> Result<(vk::Semaphore, vk::Semaphore), String> {
    let create_info = vk::SemaphoreCreateInfo {
        ..Default::default()
    };

    let image_available_semaphore = match unsafe { device.create_semaphore(&create_info, None) } {
        Ok(semaphore) => semaphore,
        Err(_) => return Err(String::from("failed to create image available semaphore")),
    };

    app::set_debug_utils_object_name(
        debug_utils_loader,
        device.handle(),
        image_available_semaphore,
        String::from("image availbale semaphore"),
    );

    let graphics_finished_semaphore = match unsafe { device.create_semaphore(&create_info, None) } {
        Ok(semaphore) => semaphore,
        Err(_) => return Err(String::from("failed to create graphics finished semaphore")),
    };

    app::set_debug_utils_object_name(
        debug_utils_loader,
        device.handle(),
        graphics_finished_semaphore,
        String::from("graphics finished semaphore"),
    );

    Ok((image_available_semaphore, graphics_finished_semaphore))
}
