use crate::app;
use ash::version::DeviceV1_0;
use ash::vk;

pub fn get_swapchain_image_views(
    device: &ash::Device,
    swapchain_images: &Vec<vk::Image>,
    surface_format: vk::SurfaceFormatKHR,
    debug_utils_loader: &ash::extensions::ext::DebugUtils,
) -> Result<Vec<vk::ImageView>, String> {
    let mut swapchain_image_views = Vec::with_capacity(swapchain_images.len());

    for (i, &image) in swapchain_images.iter().enumerate() {
        let create_info = vk::ImageViewCreateInfo::builder()
            .image(image)
            .view_type(vk::ImageViewType::TYPE_2D)
            .format(surface_format.format)
            .components(vk::ComponentMapping {
                r: vk::ComponentSwizzle::IDENTITY,
                g: vk::ComponentSwizzle::IDENTITY,
                b: vk::ComponentSwizzle::IDENTITY,
                a: vk::ComponentSwizzle::IDENTITY,
            })
            .subresource_range(vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            });

        let view = match unsafe { device.create_image_view(&create_info, None) } {
            Ok(view) => view,
            Err(_) => return Err(format!("failed to create image view {}", i)),
        };

        swapchain_image_views.push(view);

        app::set_debug_utils_object_name(
            &debug_utils_loader,
            device.handle(),
            view,
            format!("swapchain image view {}", i),
        );
    }

    Ok(swapchain_image_views)
}
