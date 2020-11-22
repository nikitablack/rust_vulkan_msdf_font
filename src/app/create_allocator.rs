pub fn create_allocator(
    instance: &ash::Instance,
    device: &ash::Device,
    physical_device: ash::vk::PhysicalDevice,
) -> Result<vk_mem::Allocator, String> {
    let create_info = vk_mem::AllocatorCreateInfo {
        physical_device,
        device: device.clone(),
        instance: instance.clone(),
        flags: vk_mem::AllocatorCreateFlags::empty(),
        preferred_large_heap_block_size: 0,
        frame_in_use_count: 0,
        heap_size_limits: None,
    };

    let allocator = match vk_mem::Allocator::new(&create_info) {
        Ok(alloc) => alloc,
        Err(_) => return Err(String::from("failed to create allocator")),
    };

    Ok(allocator)

    /*let create_info = VulkanAllocatorCreateDesc {
        instance: instance.clone(),
        device: device.clone(),
        physical_device: physical_device,
        debug_settings: Default::default(),
    };

    VulkanAllocator::new(&create_info)*/
}
