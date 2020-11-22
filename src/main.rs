mod app;
mod utils;
use ash::extensions::ext;
use ash::extensions::khr;
use ash::version::DeviceV1_0;
use ash::version::InstanceV1_0;
use ash::vk;

pub enum FontType {
    Bitmap,
    Sdf,
    Msdf,
}

const NUM_RESOURCES_IN_FLIGHT: u32 = 2u32;
const FONT_TYPE: FontType = FontType::Msdf;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("Vulkan SDF Font")
        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
        .build(&event_loop)
        .unwrap();

    let entry = match ash::Entry::new() {
        Ok(entry) => entry,
        Err(err) => panic!(err),
    };

    let device_extensions = vec![ash::extensions::khr::Swapchain::name()];

    let _ = app::check_instance_version(&entry).unwrap();
    let instance_extensions = app::get_required_instance_extensions(&window).unwrap();
    let _ = app::check_required_instance_extensions(&entry, &instance_extensions).unwrap();
    let instance = app::create_instance(&entry, &instance_extensions).unwrap();
    let debug_utils_loader = ext::DebugUtils::new(&entry, &instance);
    let surface_loader = khr::Surface::new(&entry, &instance);
    let surface = app::create_surface(&entry, &instance, &window).unwrap();
    let physical_device = app::get_physical_device(&instance, &device_extensions).unwrap();
    let surface_format =
        app::get_surface_format(physical_device, surface, &surface_loader).unwrap();
    let present_mode = app::get_present_mode(physical_device, surface, &surface_loader).unwrap();
    let queue_family =
        app::get_queue_family(&instance, physical_device, surface, &surface_loader).unwrap();
    let _depth_format = app::get_depth_format(&instance, physical_device).unwrap();
    let _device_properties = unsafe { instance.get_physical_device_properties(physical_device) };
    let device =
        app::create_logical_device(&instance, physical_device, queue_family, &device_extensions)
            .unwrap();
    let queue = app::get_device_queue(&device, queue_family);
    let swapchain_loader = khr::Swapchain::new(&instance, &device);
    let (swapchain, surface_extent) = app::create_swapchain(
        &window,
        &instance,
        &device,
        physical_device,
        surface,
        surface_format,
        present_mode,
        &surface_loader,
        vk::SwapchainKHR::null(),
    )
    .unwrap();
    let swapchain_images =
        app::get_swapchain_images(&device, swapchain, &swapchain_loader, &debug_utils_loader)
            .unwrap();
    let swapchain_image_views = app::get_swapchain_image_views(
        &device,
        &swapchain_images,
        surface_format,
        &debug_utils_loader,
    )
    .unwrap();
    let render_pass =
        app::create_render_pass(&device, surface_format, &debug_utils_loader).unwrap();
    let (image_available_semaphore, rendering_complete_semaphore) =
        app::create_semaphores(&device, &debug_utils_loader).unwrap();
    let framebuffers = app::create_framebuffers(
        &device,
        render_pass,
        surface_extent,
        &swapchain_image_views,
        &debug_utils_loader,
    )
    .unwrap();
    let command_pool =
        app::create_command_pool(&device, queue_family, &debug_utils_loader).unwrap();
    let fences = app::create_fences(&device, &debug_utils_loader).unwrap();
    let mut allocator = app::create_allocator(&instance, &device, physical_device).unwrap();

    use std::path::Path;
    let image_path: &Path;
    let json_path: &Path;
    let fragment_shader_module: &Path;
    match FONT_TYPE {
        FontType::Bitmap => {
            image_path = Path::new("resources/arial.png");
            json_path = Path::new("resources/arial.json");
            fragment_shader_module = Path::new("resources/fragment_shader.frag.spv");
        }
        FontType::Sdf => {
            image_path = Path::new("resources/arial_sdf.png");
            json_path = Path::new("resources/arial_sdf.json");
            fragment_shader_module = Path::new("resources/fragment_shader_sdf.frag.spv");
        }
        FontType::Msdf => {
            image_path = Path::new("resources/arial_msdf.png");
            json_path = Path::new("resources/arial_msdf.json");
            fragment_shader_module = Path::new("resources/fragment_shader_msdf.frag.spv");
        }
    }

    let font_image = app::get_rgba_image(image_path).unwrap();
    let font_mem_image = app::create_image_and_view(
        &allocator,
        &font_image,
        &device,
        &debug_utils_loader,
        "font image",
    )
    .unwrap();

    app::copy_data_to_image(
        &allocator,
        &device,
        queue,
        command_pool,
        font_image.as_raw(),
        &font_mem_image,
    )
    .unwrap();

    let font_data = app::get_font_data(json_path).unwrap();

    let vertex_pos_mem_buffer = app::create_buffer(
        &allocator,
        1000,
        vk::BufferUsageFlags::STORAGE_BUFFER,
        vk_mem::MemoryUsage::CpuToGpu,
        vk_mem::AllocationCreateFlags::MAPPED,
        Some(&app::DebugUtilsData {
            debug_utils_loader: &debug_utils_loader,
            device: device.handle(),
            name: "vertex position buffer",
        }),
    )
    .unwrap();

    let text = "ABCDE, world!";
    app::update_vertex_positions(
        vertex_pos_mem_buffer.allocation_info.get_mapped_data(),
        text,
        &font_data,
        32.0,
    );

    let vertex_uv_mem_buffer = app::create_buffer(
        &allocator,
        1000,
        vk::BufferUsageFlags::STORAGE_BUFFER,
        vk_mem::MemoryUsage::CpuToGpu,
        vk_mem::AllocationCreateFlags::MAPPED,
        Some(&app::DebugUtilsData {
            debug_utils_loader: &debug_utils_loader,
            device: device.handle(),
            name: "vertex uv buffer",
        }),
    )
    .unwrap();

    app::update_vertex_uvs(
        vertex_uv_mem_buffer.allocation_info.get_mapped_data(),
        text,
        &font_data,
        font_image.width() as f32,
        font_image.height() as f32,
    );

    let vertex_shader_module =
        app::create_shader_module(&device, Path::new("resources/vertex_shader.vert.spv")).unwrap();
    let fragment_shader_module =
        app::create_shader_module(&device, fragment_shader_module).unwrap();

    let sampler = app::create_sampler(&device).unwrap();

    let desc_set_layout = app::create_descriptor_set_layout(&device, sampler).unwrap();

    let pipeline_layout = app::create_pipeline_layout(&device, desc_set_layout).unwrap();
    let pipeline = app::_create_pipeline(
        &device,
        vertex_shader_module,
        fragment_shader_module,
        pipeline_layout,
        render_pass,
    )
    .unwrap();

    let desc_pool = app::create_descriptor_pool(&device).unwrap();
    let pos_uv_font_desc_set =
        app::allocate_position_uv_font_descriptor_set(&device, desc_pool, desc_set_layout).unwrap();

    app::update_descriptor_set_with_buffer(
        &device,
        pos_uv_font_desc_set,
        vertex_pos_mem_buffer.buffer,
        0,
    );
    app::update_descriptor_set_with_buffer(
        &device,
        pos_uv_font_desc_set,
        vertex_uv_mem_buffer.buffer,
        1,
    );
    app::update_descriptor_set_with_combined_image_sampler(
        &device,
        pos_uv_font_desc_set,
        font_mem_image.view,
        2,
    );

    let mut active_resource_index = 0u32;

    let mut command_buffers = Vec::with_capacity(NUM_RESOURCES_IN_FLIGHT as usize);
    command_buffers.resize(
        NUM_RESOURCES_IN_FLIGHT as usize,
        ash::vk::CommandBuffer::null(),
    );

    let mut app_exit = false;
    let mut scale = 10;

    use winit::event::Event;
    use winit::event::MouseScrollDelta;
    use winit::event::WindowEvent;
    use winit::event_loop::ControlFlow;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;

                unsafe {
                    let _ = device.device_wait_idle();

                    for &view in &swapchain_image_views {
                        device.destroy_image_view(view, None);
                    }

                    for &framebuffer in &framebuffers {
                        device.destroy_framebuffer(framebuffer, None);
                    }

                    for &fence in &fences {
                        device.destroy_fence(fence, None);
                    }

                    device.destroy_semaphore(image_available_semaphore, None);
                    device.destroy_semaphore(rendering_complete_semaphore, None);
                    device.destroy_render_pass(render_pass, None);
                    device.destroy_command_pool(command_pool, None);

                    allocator
                        .destroy_image(font_mem_image.image, &font_mem_image.allocation)
                        .unwrap();
                    device.destroy_image_view(font_mem_image.view, None);
                    allocator
                        .destroy_buffer(
                            vertex_pos_mem_buffer.buffer,
                            &vertex_pos_mem_buffer.allocation,
                        )
                        .unwrap();
                    allocator
                        .destroy_buffer(
                            vertex_uv_mem_buffer.buffer,
                            &vertex_uv_mem_buffer.allocation,
                        )
                        .unwrap();

                    device.destroy_shader_module(vertex_shader_module, None);
                    device.destroy_shader_module(fragment_shader_module, None);
                    device.destroy_descriptor_set_layout(desc_set_layout, None);
                    device.destroy_pipeline_layout(pipeline_layout, None);
                    device.destroy_pipeline(pipeline, None);
                    device.destroy_descriptor_pool(desc_pool, None);
                    device.destroy_sampler(sampler, None);

                    allocator.destroy();
                    device.destroy_device(None)
                }

                app_exit = true;
            }

            winit::event::Event::MainEventsCleared => {
                if app_exit {
                    return;
                }

                app::draw(
                    &device,
                    swapchain,
                    queue,
                    active_resource_index,
                    command_pool,
                    &mut command_buffers,
                    pipeline,
                    pipeline_layout,
                    pos_uv_font_desc_set,
                    render_pass,
                    &framebuffers,
                    surface_extent,
                    image_available_semaphore,
                    rendering_complete_semaphore,
                    &fences,
                    &swapchain_loader,
                    scale,
                )
                .unwrap();

                active_resource_index += 1;
                active_resource_index %= NUM_RESOURCES_IN_FLIGHT;
            }

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized { .. } => todo!(),

                WindowEvent::MouseWheel {
                    delta: MouseScrollDelta::LineDelta(_, v_lines),
                    ..
                } => {
                    if v_lines < 0.0 {
                        scale = std::cmp::max(scale - 1, 0);
                    } else {
                        scale += 1;
                    }
                }

                _ => (),
            },

            _ => (),
        }
    });
}
