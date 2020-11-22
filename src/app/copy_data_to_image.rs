use crate::app;
use ash::version::DeviceV1_0;
use ash::vk;

fn allocate_command_buffer(
    device: &ash::Device,
    command_pool: vk::CommandPool,
) -> Result<vk::CommandBuffer, String> {
    let allocate_info = vk::CommandBufferAllocateInfo::builder()
        .command_pool(command_pool)
        .level(vk::CommandBufferLevel::PRIMARY)
        .command_buffer_count(1);

    let command_buffers = match unsafe { device.allocate_command_buffers(&allocate_info) } {
        Ok(buf) => buf,
        Err(_) => return Err(String::from("failed to allocate command buffer")),
    };

    Ok(command_buffers[0])
}

fn copy_buffer_to_image(
    device: &ash::Device,
    copy_queue: vk::Queue,
    command_buffer: vk::CommandBuffer,
    src_mem_buffer: &app::MemBuffer,
    dst_mem_image: &app::MemImage,
    dst_image_access_mask: vk::AccessFlags,
    dst_image_layout: vk::ImageLayout,
    dst_image_stage_mask: vk::PipelineStageFlags,
) -> Result<(), String> {
    let begin_info =
        vk::CommandBufferBeginInfo::builder().flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

    if let Err(_) = unsafe { device.begin_command_buffer(command_buffer, &begin_info) } {
        return Err(String::from(
            "failed to begin copy buffer to image command buffer",
        ));
    }

    let pre_copy_barrier = vk::ImageMemoryBarrier::builder()
        .image(dst_mem_image.image)
        .src_access_mask(vk::AccessFlags::empty())
        .dst_access_mask(vk::AccessFlags::TRANSFER_WRITE)
        .old_layout(vk::ImageLayout::UNDEFINED)
        .new_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .subresource_range(vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        })
        .build();

    unsafe {
        device.cmd_pipeline_barrier(
            command_buffer,
            vk::PipelineStageFlags::TOP_OF_PIPE,
            vk::PipelineStageFlags::TRANSFER,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &[pre_copy_barrier],
        );
    }

    let copy_region = vk::BufferImageCopy {
        buffer_offset: 0,
        buffer_row_length: 0,
        buffer_image_height: 0,
        image_subresource: vk::ImageSubresourceLayers {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            mip_level: 0,
            base_array_layer: 0,
            layer_count: 1,
        },
        image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
        image_extent: dst_mem_image.extent,
    };

    unsafe {
        device.cmd_copy_buffer_to_image(
            command_buffer,
            src_mem_buffer.buffer,
            dst_mem_image.image,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            &[copy_region],
        );
    }

    let post_copy_barrier = vk::ImageMemoryBarrier::builder()
        .image(dst_mem_image.image)
        .src_access_mask(vk::AccessFlags::TRANSFER_WRITE)
        .dst_access_mask(dst_image_access_mask)
        .old_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
        .new_layout(dst_image_layout)
        .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
        .subresource_range(vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::COLOR,
            base_mip_level: 0,
            level_count: 1,
            base_array_layer: 0,
            layer_count: 1,
        })
        .build();

    unsafe {
        device.cmd_pipeline_barrier(
            command_buffer,
            vk::PipelineStageFlags::TRANSFER,
            dst_image_stage_mask,
            vk::DependencyFlags::empty(),
            &[],
            &[],
            &[post_copy_barrier],
        );
    }

    if let Err(_) = unsafe { device.end_command_buffer(command_buffer) } {
        return Err(String::from(
            "failed to end copy buffer to image command buffer",
        ));
    }

    let submit_info = vk::SubmitInfo::builder()
        .command_buffers(&[command_buffer])
        .build();

    match unsafe { device.queue_submit(copy_queue, &[submit_info], vk::Fence::null()) } {
        Err(_) => return Err(String::from("failed to submit graphics command buffer")),
        _ => (),
    }

    if let Err(_) = unsafe { device.queue_wait_idle(copy_queue) } {
        return Err(String::from(
            "failed to wait queue idle on copy buffer to image",
        ));
    }

    Ok(())
}

pub fn copy_data_to_image(
    allocator: &vk_mem::Allocator,
    device: &ash::Device,
    copy_queue: vk::Queue,
    command_pool: vk::CommandPool,
    src_data: &[u8],
    dst_mem_image: &app::MemImage,
) -> Result<(), String> {
    let staging_mem_buffer = app::create_buffer(
        allocator,
        src_data.len() as vk::DeviceSize,
        vk::BufferUsageFlags::TRANSFER_SRC,
        vk_mem::MemoryUsage::CpuOnly,
        vk_mem::AllocationCreateFlags::MAPPED,
        None,
    )?;

    unsafe {
        std::ptr::copy_nonoverlapping(
            src_data.as_ptr(),
            staging_mem_buffer.allocation_info.get_mapped_data(),
            src_data.len(),
        )
    };

    let command_buffer = allocate_command_buffer(device, command_pool)?;

    copy_buffer_to_image(
        device,
        copy_queue,
        command_buffer,
        &staging_mem_buffer,
        dst_mem_image,
        vk::AccessFlags::SHADER_READ,
        vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        vk::PipelineStageFlags::FRAGMENT_SHADER,
    )?;

    let _ = allocator.destroy_buffer(staging_mem_buffer.buffer, &staging_mem_buffer.allocation);

    Ok(())
}
