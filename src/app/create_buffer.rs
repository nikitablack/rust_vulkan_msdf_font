use crate::app;
use ash::vk;

pub fn create_buffer(
    allocator: &vk_mem::Allocator,
    size: vk::DeviceSize,
    buffer_usage: vk::BufferUsageFlags,
    memory_usage: vk_mem::MemoryUsage,
    memory_flags: vk_mem::AllocationCreateFlags,
    debug_utils_data: Option<&app::DebugUtilsData>,
) -> Result<app::MemBuffer, String> {
    let buffer_create_info = vk::BufferCreateInfo::builder()
        .size(size)
        .usage(buffer_usage)
        .sharing_mode(vk::SharingMode::EXCLUSIVE);

    let allocation_create_info = vk_mem::AllocationCreateInfo {
        usage: memory_usage,
        flags: memory_flags,
        ..Default::default()
    };

    let mem_buffer = match allocator.create_buffer(&buffer_create_info, &allocation_create_info) {
        Ok((buffer, allocation, allocation_info)) => app::MemBuffer {
            buffer,
            allocation,
            allocation_info,
        },
        Err(_) => return Err(format!("failed to allocate buffer")),
    };

    if debug_utils_data.is_some() {
        let data = debug_utils_data.unwrap();

        app::set_debug_utils_object_name(
            data.debug_utils_loader,
            data.device,
            mem_buffer.buffer,
            data.name.to_owned(),
        );

        app::set_debug_utils_object_name(
            data.debug_utils_loader,
            data.device,
            mem_buffer.allocation_info.get_device_memory(),
            format!("device memory {}", data.name),
        );
    }

    Ok(mem_buffer)
}
