use crate::app;
use ash::version::DeviceV1_0;
use ash::vk;

pub fn create_framebuffers(
    device: &ash::Device,
    render_pass: vk::RenderPass,
    surface_extent: vk::Extent2D,
    swapchain_image_views: &Vec<vk::ImageView>,
    debug_utils_loader: &ash::extensions::ext::DebugUtils,
) -> Result<Vec<vk::Framebuffer>, String> {
    let mut framebuffers = Vec::with_capacity(swapchain_image_views.len());

    for (i, &view) in swapchain_image_views.iter().enumerate() {
        let attachments = [view];

        let create_info = vk::FramebufferCreateInfo::builder()
            .render_pass(render_pass)
            .attachments(&attachments)
            .width(surface_extent.width)
            .height(surface_extent.height)
            .layers(1);

        let framebuffer = match unsafe { device.create_framebuffer(&create_info, None) } {
            Ok(fb) => fb,
            Err(_) => return Err(format!("failed to create framebuffer {}", i)),
        };

        framebuffers.push(framebuffer);

        app::set_debug_utils_object_name(
            &debug_utils_loader,
            device.handle(),
            framebuffer,
            format!("framebuffer {}", i),
        );
    }

    Ok(framebuffers)
}
