use ash::version::DeviceV1_0;
use ash::vk;

pub fn _create_pipeline(
    device: &ash::Device,
    vertex_shader_module: vk::ShaderModule,
    fragment_shader_module: vk::ShaderModule,
    pipeline_layout: vk::PipelineLayout,
    render_pass: vk::RenderPass,
) -> Result<vk::Pipeline, String> {
    let shader_entry_name = std::ffi::CString::new("main").unwrap();

    let vs_state = vk::PipelineShaderStageCreateInfo {
        stage: vk::ShaderStageFlags::VERTEX,
        module: vertex_shader_module,
        p_name: shader_entry_name.as_ptr(),
        ..Default::default()
    };

    let fs_state = vk::PipelineShaderStageCreateInfo {
        stage: vk::ShaderStageFlags::FRAGMENT,
        module: fragment_shader_module,
        p_name: shader_entry_name.as_ptr(),
        ..Default::default()
    };

    let ia_state = vk::PipelineInputAssemblyStateCreateInfo {
        topology: vk::PrimitiveTopology::TRIANGLE_LIST,
        ..Default::default()
    };

    let raster_state = vk::PipelineRasterizationStateCreateInfo {
        polygon_mode: vk::PolygonMode::FILL,
        cull_mode: vk::CullModeFlags::BACK,
        front_face: vk::FrontFace::CLOCKWISE,
        line_width: 1.0f32,
        ..Default::default()
    };

    let col_blend_attachment_state = vk::PipelineColorBlendAttachmentState {
        blend_enable: vk::TRUE,
        src_color_blend_factor: vk::BlendFactor::SRC_ALPHA,
        dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
        color_blend_op: vk::BlendOp::ADD,
        src_alpha_blend_factor: vk::BlendFactor::SRC_ALPHA,
        dst_alpha_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
        alpha_blend_op: vk::BlendOp::ADD,
        color_write_mask: vk::ColorComponentFlags::R
            | vk::ColorComponentFlags::G
            | vk::ColorComponentFlags::B
            | vk::ColorComponentFlags::A,
    };

    let col_blend_state = vk::PipelineColorBlendStateCreateInfo::builder()
        .logic_op(vk::LogicOp::CLEAR)
        .attachments(&[col_blend_attachment_state])
        .build();

    let dyn_state = vk::PipelineDynamicStateCreateInfo::builder()
        .dynamic_states(&[vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR])
        .build();

    let viewport = vk::Viewport {
        ..Default::default()
    };
    let scissor = vk::Rect2D {
        ..Default::default()
    };

    let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
        .viewports(&[viewport])
        .scissors(&[scissor])
        .build();

    let multisample_state = vk::PipelineMultisampleStateCreateInfo::builder()
        .rasterization_samples(vk::SampleCountFlags::TYPE_1);

    let vert_inp_state = vk::PipelineVertexInputStateCreateInfo {
        ..Default::default()
    };

    let create_info = vk::GraphicsPipelineCreateInfo::builder()
        .flags(vk::PipelineCreateFlags::empty())
        .stages(&[vs_state, fs_state])
        .input_assembly_state(&ia_state)
        .rasterization_state(&raster_state)
        .color_blend_state(&col_blend_state)
        .dynamic_state(&dyn_state)
        .layout(pipeline_layout)
        .render_pass(render_pass)
        .subpass(0)
        .viewport_state(&viewport_state)
        .multisample_state(&multisample_state)
        .vertex_input_state(&vert_inp_state)
        .build();

    let pipelines = match unsafe {
        device.create_graphics_pipelines(vk::PipelineCache::null(), &[create_info], None)
    } {
        Ok(p) => p,
        Err(_) => return Err(String::from("failed to create pipeline")),
    };

    Ok(pipelines[0])
}
