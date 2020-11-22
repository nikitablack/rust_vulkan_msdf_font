use ash::version::InstanceV1_0;
use ash::vk;

pub fn get_depth_format(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
) -> Result<vk::Format, String> {
    let format_candidates = [
        vk::Format::D24_UNORM_S8_UINT,
        vk::Format::D32_SFLOAT_S8_UINT,
        vk::Format::D16_UNORM_S8_UINT,
    ];

    for &format in &format_candidates {
        let props =
            unsafe { instance.get_physical_device_format_properties(physical_device, format) };

        if props
            .optimal_tiling_features
            .contains(vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT)
        {
            return Ok(format);
        }
    }

    Err(String::from("failed to find depth format"))
}
