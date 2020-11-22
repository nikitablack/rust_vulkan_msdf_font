use ash::version::DeviceV1_0;
use ash::vk;

pub fn create_sampler(device: &ash::Device) -> Result<vk::Sampler, String> {
    let create_info = vk::SamplerCreateInfo {
        mag_filter: vk::Filter::LINEAR,
        min_filter: vk::Filter::LINEAR,
        mipmap_mode: vk::SamplerMipmapMode::LINEAR,
        address_mode_u: vk::SamplerAddressMode::REPEAT,
        address_mode_v: vk::SamplerAddressMode::REPEAT,
        address_mode_w: vk::SamplerAddressMode::REPEAT,
        mip_lod_bias: 0.0f32,
        anisotropy_enable: vk::FALSE,
        max_anisotropy: 0.0f32,
        compare_enable: vk::FALSE,
        compare_op: vk::CompareOp::NEVER,
        min_lod: 0.0f32,
        max_lod: 0.0f32,
        border_color: vk::BorderColor::FLOAT_OPAQUE_WHITE,
        unnormalized_coordinates: vk::FALSE,
        ..Default::default()
    };

    let sampler = match unsafe { device.create_sampler(&create_info, None) } {
        Ok(s) => s,
        Err(_) => return Err(String::from("failed to create sampler")),
    };

    Ok(sampler)
}
