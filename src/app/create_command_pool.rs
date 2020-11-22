use crate::app;
use ash::version::DeviceV1_0;
use ash::vk;

pub fn create_command_pool(
    device: &ash::Device,
    queue_family: u32,
    debug_utils_loader: &ash::extensions::ext::DebugUtils,
) -> Result<vk::CommandPool, String> {
    let create_info = vk::CommandPoolCreateInfo::builder()
        .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
        .queue_family_index(queue_family);

    let command_pool = match unsafe { device.create_command_pool(&create_info, None) } {
        Ok(pool) => pool,
        Err(_) => return Err(String::from("failed to create command pool")),
    };

    app::set_debug_utils_object_name(
        debug_utils_loader,
        device.handle(),
        command_pool,
        String::from("command pool"),
    );

    Ok(command_pool)
}
