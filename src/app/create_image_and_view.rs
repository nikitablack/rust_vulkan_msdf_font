use crate::app;
use ash::version::DeviceV1_0;
use ash::vk;

pub fn create_image_and_view(
    allocator: &vk_mem::Allocator,
    image: &image::RgbaImage,
    device: &ash::Device,
    debug_utils_loader: &ash::extensions::ext::DebugUtils,
    name: &'static str,
) -> Result<app::MemImage, String> {
    let image_create_info = vk::ImageCreateInfo::builder()
        .image_type(vk::ImageType::TYPE_2D)
        .format(vk::Format::R8G8B8A8_UNORM)
        .extent(vk::Extent3D {
            width: image.width(),
            height: image.height(),
            depth: 1,
        })
        .mip_levels(1)
        .array_layers(1)
        .samples(vk::SampleCountFlags::TYPE_1)
        .tiling(vk::ImageTiling::OPTIMAL)
        .usage(vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST)
        .sharing_mode(vk::SharingMode::EXCLUSIVE)
        .initial_layout(vk::ImageLayout::UNDEFINED);

    let allocation_create_info = vk_mem::AllocationCreateInfo {
        usage: vk_mem::MemoryUsage::GpuOnly,
        ..Default::default()
    };

    let mut mem_image = match allocator.create_image(&image_create_info, &allocation_create_info) {
        Ok((img, allocation, allocation_info)) => app::MemImage {
            image: img,
            view: vk::ImageView::null(),
            allocation,
            allocation_info,
            extent: ash::vk::Extent3D {
                width: image.width(),
                height: image.height(),
                depth: 1,
            },
        },
        Err(_) => return Err(format!("failed to allocate image {}", name)),
    };

    app::set_debug_utils_object_name(
        debug_utils_loader,
        device.handle(),
        mem_image.image,
        name.to_owned(),
    );

    app::set_debug_utils_object_name(
        debug_utils_loader,
        device.handle(),
        mem_image.allocation_info.get_device_memory(),
        format!("device memory {}", name),
    );

    let create_info = vk::ImageViewCreateInfo {
        image: mem_image.image,
        view_type: vk::ImageViewType::TYPE_2D,
        format: vk::Format::R8G8B8A8_UNORM,
        components: vk::ComponentMapping {
            r: vk::ComponentSwizzle::R,
            g: vk::ComponentSwizzle::G,
            b: vk::ComponentSwizzle::B,
            a: vk::ComponentSwizzle::A,
        },
        subresource_range: vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        },
        ..Default::default()
    };

    mem_image.view = match unsafe { device.create_image_view(&create_info, None) } {
        Ok(v) => v,
        Err(_) => return Err(String::from("failed to create image view")),
    };

    Ok(mem_image)
}
