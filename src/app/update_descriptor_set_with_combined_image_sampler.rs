use ash::version::DeviceV1_0;
use ash::vk;

pub fn update_descriptor_set_with_combined_image_sampler(
    device: &ash::Device,
    desc_set: vk::DescriptorSet,
    image_view: vk::ImageView,
    binding: u32,
) {
    let image_info = vk::DescriptorImageInfo {
        sampler: vk::Sampler::null(),
        image_view: image_view,
        image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
    };

    let desc_write = vk::WriteDescriptorSet::builder()
        .dst_set(desc_set)
        .dst_binding(binding)
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .image_info(&[image_info])
        .build();

    unsafe {
        device.update_descriptor_sets(&[desc_write], &[]);
    }
}
