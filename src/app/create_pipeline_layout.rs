use ash::version::DeviceV1_0;
use ash::vk;

pub fn create_pipeline_layout(
    device: &ash::Device,
    desc_set_layout: vk::DescriptorSetLayout,
) -> Result<vk::PipelineLayout, String> {
    let pc_range = vk::PushConstantRange {
        stage_flags: vk::ShaderStageFlags::VERTEX,
        offset: 0,
        size: 16 * 4,
    };

    let create_info = vk::PipelineLayoutCreateInfo::builder()
        .set_layouts(&[desc_set_layout])
        .push_constant_ranges(&[pc_range])
        .build();

    let pipeline_layout = match unsafe { device.create_pipeline_layout(&create_info, None) } {
        Ok(pl) => pl,
        Err(_) => return Err(String::from("failed to create pipeline layout")),
    };

    Ok(pipeline_layout)
}
