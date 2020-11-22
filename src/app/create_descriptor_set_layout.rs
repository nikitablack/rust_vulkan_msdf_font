use ash::version::DeviceV1_0;
use ash::vk;

pub fn create_descriptor_set_layout(
    device: &ash::Device,
    sampler: vk::Sampler,
) -> Result<vk::DescriptorSetLayout, String> {
    let vertex_pos_binding = vk::DescriptorSetLayoutBinding {
        binding: 0,
        descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
        descriptor_count: 1,
        stage_flags: vk::ShaderStageFlags::VERTEX,
        ..Default::default()
    };

    let vertex_uv_binding = vk::DescriptorSetLayoutBinding {
        binding: 1,
        descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
        descriptor_count: 1,
        stage_flags: vk::ShaderStageFlags::VERTEX,
        ..Default::default()
    };

    let vertex_font_binding = vk::DescriptorSetLayoutBinding::builder()
        .binding(2)
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::FRAGMENT)
        .immutable_samplers(&[sampler])
        .build();

    let create_info = vk::DescriptorSetLayoutCreateInfo::builder()
        .bindings(&[vertex_pos_binding, vertex_uv_binding, vertex_font_binding])
        .build();

    let set_layout = match unsafe { device.create_descriptor_set_layout(&create_info, None) } {
        Ok(layout) => layout,
        Err(_) => return Err(String::from("failed to create descriptor set layout")),
    };

    Ok(set_layout)
}
