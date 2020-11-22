use crate::app;
use crate::NUM_RESOURCES_IN_FLIGHT;
use ash::version::DeviceV1_0;
use ash::vk;

pub fn create_fences(
    device: &ash::Device,
    debug_utils_loader: &ash::extensions::ext::DebugUtils,
) -> Result<Vec<vk::Fence>, String> {
    let create_info = vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED);

    let mut fences = Vec::with_capacity(NUM_RESOURCES_IN_FLIGHT as usize);

    for i in 0..NUM_RESOURCES_IN_FLIGHT {
        let fence = match unsafe { device.create_fence(&create_info, None) } {
            Ok(fence) => fence,
            Err(_) => return Err(format!("failed to create fence {}", i)),
        };

        app::set_debug_utils_object_name(
            debug_utils_loader,
            device.handle(),
            fence,
            format!("fence {}", i),
        );

        fences.push(fence);
    }

    Ok(fences)
}
