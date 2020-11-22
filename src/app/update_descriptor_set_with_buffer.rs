use ash::version::DeviceV1_0;
use ash::vk;

pub fn update_descriptor_set_with_buffer(
    device: &ash::Device,
    desc_set: vk::DescriptorSet,
    buffer: vk::Buffer,
    binding: u32,
) {
    let buffer_info = vk::DescriptorBufferInfo {
        buffer: buffer,
        offset: 0,
        range: vk::WHOLE_SIZE,
    };

    let desc_write = vk::WriteDescriptorSet::builder()
        .dst_set(desc_set)
        .dst_binding(binding)
        .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
        .buffer_info(&[buffer_info])
        .build();

    unsafe {
        device.update_descriptor_sets(&[desc_write], &[]);
    }
}
