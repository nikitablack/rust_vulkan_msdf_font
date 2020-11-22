use ash::version::DeviceV1_0;
use ash::vk;

pub fn allocate_position_uv_font_descriptor_set(
    device: &ash::Device,
    desc_pool: vk::DescriptorPool,
    desc_set_layout: vk::DescriptorSetLayout,
) -> Result<vk::DescriptorSet, String> {
    let alloc_info = vk::DescriptorSetAllocateInfo::builder()
        .descriptor_pool(desc_pool)
        .set_layouts(&[desc_set_layout])
        .build();

    let sets = match unsafe { device.allocate_descriptor_sets(&alloc_info) } {
        Ok(sets) => sets,
        Err(_) => return Err(String::from("failed to allocate uv descriptor set")),
    };

    Ok(sets[0])
}
