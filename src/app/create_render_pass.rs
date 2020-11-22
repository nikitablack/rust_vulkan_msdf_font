use crate::app;
use ash::version::DeviceV1_0;
use ash::vk;

pub fn create_render_pass(
    device: &ash::Device,
    surface_format: vk::SurfaceFormatKHR,
    debug_utils_loader: &ash::extensions::ext::DebugUtils,
) -> Result<vk::RenderPass, String> {
    let mut attachment_descriptions = Vec::new();

    attachment_descriptions.push(
        vk::AttachmentDescription::builder()
            .format(surface_format.format)
            .samples(vk::SampleCountFlags::TYPE_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::DONT_CARE)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR)
            .build(),
    );

    let col_attachment_ref = vk::AttachmentReference::builder()
        .attachment(0)
        .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
        .build();

    let references = [col_attachment_ref];

    let mut subpass_descriptions = Vec::new();

    subpass_descriptions.push(
        vk::SubpassDescription::builder()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(&references)
            .build(),
    );

    let create_info = vk::RenderPassCreateInfo::builder()
        .attachments(&attachment_descriptions)
        .subpasses(&subpass_descriptions);

    let render_pass = match unsafe { device.create_render_pass(&create_info, None) } {
        Ok(rp) => rp,
        Err(_) => return Err(String::from("failed to create render pass")),
    };

    app::set_debug_utils_object_name(
        debug_utils_loader,
        device.handle(),
        render_pass,
        String::from("render pass"),
    );

    Ok(render_pass)
}
