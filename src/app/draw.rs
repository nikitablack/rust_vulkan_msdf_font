use crate::utils;
use ash::extensions::khr;
use ash::version::DeviceV1_0;
use ash::vk;

pub fn draw(
    device: &ash::Device,
    swapchain: vk::SwapchainKHR,
    queue: vk::Queue,
    active_resource_index: u32,
    command_pool: vk::CommandPool,
    active_command_buffers: &mut Vec<vk::CommandBuffer>,
    pipeline: vk::Pipeline,
    pipeline_layout: vk::PipelineLayout,
    pos_uv_desc_set: vk::DescriptorSet,
    render_pass: vk::RenderPass,
    framebuffers: &Vec<vk::Framebuffer>,
    surface_extent: vk::Extent2D,
    image_available_semaphore: vk::Semaphore,
    rendering_complete_semaphore: vk::Semaphore,
    fences: &Vec<vk::Fence>,
    swapchain_loader: &khr::Swapchain,
    scale: i32,
) -> Result<(), String> {
    let (image_index, _) = match unsafe {
        swapchain_loader.acquire_next_image(
            swapchain,
            u64::MAX,
            image_available_semaphore,
            vk::Fence::null(),
        )
    } {
        Ok(index) => index,
        Err(_) => return Err(String::from("failed to acquire next image")),
    };

    unsafe {
        let graphics_fence = fences[active_resource_index as usize];
        let fences = [graphics_fence];

        match device.wait_for_fences(&fences, true, u64::MAX) {
            Err(_) => {
                return Err(format!(
                    "failed to wait for graphics fence {}",
                    active_resource_index
                ))
            }
            _ => (),
        }

        let fences = [graphics_fence];

        match device.reset_fences(&fences) {
            Err(_) => {
                return Err(format!(
                    "failed to reset graphics fence {}",
                    active_resource_index
                ))
            }
            _ => (),
        }

        {
            let command_buffer = active_command_buffers[active_resource_index as usize];

            let buffers = [command_buffer];

            device.free_command_buffers(command_pool, &buffers);

            let allocate_info = vk::CommandBufferAllocateInfo::builder()
                .command_pool(command_pool)
                .level(vk::CommandBufferLevel::PRIMARY)
                .command_buffer_count(1);

            let command_buffers = match device.allocate_command_buffers(&allocate_info) {
                Ok(buf) => buf,
                Err(_) => {
                    return Err(format!(
                        "failed to allocate command buffer fot active resource index {}",
                        active_resource_index
                    ))
                }
            };

            active_command_buffers[active_resource_index as usize] = command_buffers[0];
        }

        let command_buffer = active_command_buffers[active_resource_index as usize];

        let begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        match device.begin_command_buffer(command_buffer, &begin_info) {
            Err(_) => return Err(String::from("failed to begin graphics command buffer")),
            _ => (),
        }

        let clear_color = vk::ClearColorValue {
            float32: [0.5f32, 0.1f32, 0.1f32, 0.1f32],
        };
        let clear_values = vec![vk::ClearValue { color: clear_color }];

        let render_pass_begin_info = vk::RenderPassBeginInfo::builder()
            .render_pass(render_pass)
            .framebuffer(framebuffers[image_index as usize])
            .render_area(vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: surface_extent,
            })
            .clear_values(&clear_values);

        device.cmd_begin_render_pass(
            command_buffer,
            &render_pass_begin_info,
            vk::SubpassContents::INLINE,
        );

        let mut proj_matrix = utils::get_ortho_projection_matrix(
            0.0,
            surface_extent.width as f32,
            0.0,
            surface_extent.height as f32,
            0.0,
            1.0,
        );
        proj_matrix[0] *= (scale as f32) * 0.1;
        proj_matrix[5] *= (scale as f32) * 0.1;
        let proj_matrix_raw = utils::f32_to_u8(&proj_matrix);

        device.cmd_push_constants(
            command_buffer,
            pipeline_layout,
            vk::ShaderStageFlags::VERTEX,
            0,
            proj_matrix_raw,
        );

        let viewport = vk::Viewport {
            x: 0.0f32,
            y: 0.0f32,
            width: surface_extent.width as f32,
            height: surface_extent.height as f32,
            min_depth: 0.0f32,
            max_depth: 1.0f32,
        };
        device.cmd_set_viewport(command_buffer, 0, &[viewport]);

        let scissor = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: vk::Extent2D {
                width: surface_extent.width,
                height: surface_extent.height,
            },
        };
        device.cmd_set_scissor(command_buffer, 0, &[scissor]);

        device.cmd_bind_descriptor_sets(
            command_buffer,
            vk::PipelineBindPoint::GRAPHICS,
            pipeline_layout,
            0,
            &[pos_uv_desc_set],
            &[],
        );
        device.cmd_bind_pipeline(command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);
        device.cmd_draw(command_buffer, 78, 1, 0, 0);
        device.cmd_end_render_pass(command_buffer);

        match device.end_command_buffer(command_buffer) {
            Err(_) => {
                return Err(format!(
                    "failed to end command buffer fot active resource index {}",
                    active_resource_index
                ))
            }
            _ => (),
        }

        let wait_semaphores = [image_available_semaphore];
        let masks = [vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT];
        let buffers = [command_buffer];
        let signal_semaphores = [rendering_complete_semaphore];

        let submit_info = vk::SubmitInfo::builder()
            .wait_semaphores(&wait_semaphores)
            .wait_dst_stage_mask(&masks)
            .command_buffers(&buffers)
            .signal_semaphores(&signal_semaphores)
            .build();

        let infos = [submit_info];

        match device.queue_submit(queue, &infos, graphics_fence) {
            Err(_) => return Err(String::from("failed to submit graphics command buffer")),
            _ => (),
        }

        let wait_semaphores = [rendering_complete_semaphore];
        let swapchains = [swapchain];
        let indices = [image_index];

        let present_info = vk::PresentInfoKHR::builder()
            .wait_semaphores(&wait_semaphores)
            .swapchains(&swapchains)
            .image_indices(&indices);

        if let Err(err) = swapchain_loader.queue_present(queue, &present_info) {
            if err == vk::Result::SUBOPTIMAL_KHR || err == vk::Result::ERROR_OUT_OF_DATE_KHR {
                panic!("swapchain resized");
            } else {
                return Err(String::from("failed to present"));
            }
        }
    }

    Ok(())
}
