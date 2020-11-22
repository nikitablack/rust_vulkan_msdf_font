use ash::version::InstanceV1_0;
use ash::vk;

pub fn get_queue_family(
    instance: &ash::Instance,
    physical_device: vk::PhysicalDevice,
    surface: vk::SurfaceKHR,
    surface_loader: &ash::extensions::khr::Surface,
) -> Result<u32, String> {
    let props = unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

    for (ind, p) in props.iter().enumerate() {
        if p.queue_count > 0 && p.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
            let present_supported = match unsafe {
                surface_loader.get_physical_device_surface_support(
                    physical_device,
                    ind as u32,
                    surface,
                )
            } {
                Ok(result) => result,
                Err(_) => {
                    return Err(String::from(
                        "failed to get physical device surface_support",
                    ))
                }
            };

            if present_supported {
                return Ok(ind as u32);
            }
        }
    }

    Err(String::from(
        "failed to find graphics queue with present support",
    ))
}
