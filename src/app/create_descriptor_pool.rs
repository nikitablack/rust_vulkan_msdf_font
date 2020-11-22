use ash::version::DeviceV1_0;
use ash::vk;

pub fn create_descriptor_pool(device: &ash::Device) -> Result<vk::DescriptorPool, String> {
    let pool_size_1 = vk::DescriptorPoolSize {
        ty: vk::DescriptorType::STORAGE_BUFFER,
        descriptor_count: 1,
    };

    let pool_size_2 = vk::DescriptorPoolSize {
        ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
        descriptor_count: 1,
    };

    let create_info = vk::DescriptorPoolCreateInfo::builder()
        .max_sets(10)
        .pool_sizes(&[pool_size_1, pool_size_2])
        .build();

    let pool = match unsafe { device.create_descriptor_pool(&create_info, None) } {
        Ok(p) => p,
        Err(_) => return Err(String::from("failed to create descriptor pool")),
    };

    Ok(pool)
}
