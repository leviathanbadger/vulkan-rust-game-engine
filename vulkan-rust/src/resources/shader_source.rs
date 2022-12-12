use std::{
    fs::{File},
    io::{Read}
};
use anyhow::{anyhow, Result};
use vulkanalia::{
    prelude::v1_0::*
};

#[derive(Debug)]
pub enum ShaderSource {
    Source(Box<[u8]>, &'static str),
    SourcePath(String, &'static str)
}

impl ShaderSource {
    pub fn flatten(self) -> Result<Self> {
        match self {
            Self::Source(..) => Ok(self),
            Self::SourcePath(path, name) => {
                let mut file = File::open(path)?;
                let mut bytes = Vec::with_capacity(file.metadata()?.len() as usize);
                file.read_to_end(&mut bytes)?;
                Ok(ShaderSource::Source(bytes.into_boxed_slice(), name))
            }
        }
    }

    pub fn get_source(&self) -> Result<(&[u8], &'static str)> {
        match self {
            Self::Source(source, name) => Ok((source, *name)),
            _ => Err(anyhow!("Can't get source from SourcePath. Flatten the ShaderSource first."))
        }
    }
}

#[derive(Debug)]
pub struct ShaderSources {
    pub vertex: ShaderSource,
    pub fragment: ShaderSource
}

#[derive(Debug, Copy, Clone)]
pub struct AttachmentDescriptor {
    pub format: vk::Format,
    pub load_op: vk::AttachmentLoadOp,
    pub store_op: vk::AttachmentStoreOp,
    pub stencil_load_op: Option<vk::AttachmentLoadOp>,
    pub stencil_store_op: Option<vk::AttachmentStoreOp>,
    pub initial_layout: vk::ImageLayout,
    pub final_layout: vk::ImageLayout
}

impl Default for AttachmentDescriptor {
    fn default() -> Self {
        Self {
            format: vk::Format::R8G8B8A8_SINT,
            load_op: vk::AttachmentLoadOp::CLEAR,
            store_op: vk::AttachmentStoreOp::STORE,
            stencil_load_op: None,
            stencil_store_op: None,
            initial_layout: vk::ImageLayout::UNDEFINED,
            final_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DepthBufferUsageMode {
    DontUse,
    WriteIfLess,
    WriteIfEqual
}

#[derive(Debug, Copy, Clone, Default)]
pub struct SubpassAttachmentDescriptor {
    pub attached: bool,
    pub ref_layout: vk::ImageLayout
}

#[derive(Debug)]
pub struct SubpassDescriptor {
    pub color_attachments: Vec<SubpassAttachmentDescriptor>,
    pub depth_attachment: SubpassAttachmentDescriptor
}

#[derive(Debug, Copy, Clone)]
pub struct BlendStateDescriptor {
    pub components: vk::ColorComponentFlags,

    pub enable_blend: bool,

    pub src_color_blend_factor: vk::BlendFactor,
    pub dst_color_blend_factor: vk::BlendFactor,
    pub color_blend_op: vk::BlendOp,
    pub src_alpha_blend_factor: vk::BlendFactor,
    pub dst_alpha_blend_factor: vk::BlendFactor,
    pub alpha_blend_op: vk::BlendOp
}

impl Default for BlendStateDescriptor {
    fn default() -> Self {
        Self {
            components: vk::ColorComponentFlags::all(),

            enable_blend: false,

            src_color_blend_factor: vk::BlendFactor::SRC_ALPHA,
            dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
            color_blend_op: vk::BlendOp::ADD,
            src_alpha_blend_factor: vk::BlendFactor::ONE,
            dst_alpha_blend_factor: vk::BlendFactor::ZERO,
            alpha_blend_op: vk::BlendOp::ADD
        }
    }
}

pub fn create_shader_sources(stage: &'static str, shader_name: &'static str, suffix: &'static str, shader_entry: &'static str) -> ShaderSources {
    let vertex_path_str = format!("shaders/{}{}{}/shader.vert.spv", stage, shader_name, suffix).to_owned();
    let fragment_path_str = format!("shaders/{}{}{}/shader.frag.spv", stage, shader_name, suffix).to_owned();

    ShaderSources {
        vertex: ShaderSource::SourcePath(vertex_path_str, shader_entry),
        fragment: ShaderSource::SourcePath(fragment_path_str, shader_entry)
    }
}

pub fn create_pipeline_layout(device: &Device, set_layouts: &[vk::DescriptorSetLayout], push_constant_ranges: &[vk::PushConstantRange]) -> Result<vk::PipelineLayout> {
    let layout_info = vk::PipelineLayoutCreateInfo::builder()
        .set_layouts(set_layouts)
        .push_constant_ranges(push_constant_ranges);

    unsafe {
        Ok(device.create_pipeline_layout(&layout_info, None)?)
    }
}

pub fn create_shader_module(device: &Device, bytecode: &[u8]) -> Result<vk::ShaderModule> {
    unsafe {
        let bytecode = Vec::<u8>::from(bytecode);
        let (prefix, code, suffix) = bytecode.align_to::<u32>();
        if !prefix.is_empty() || !suffix.is_empty() {
            return Err(anyhow!("Shader bytecode is not properly aligned"));
        }

        let shader_info = vk::ShaderModuleCreateInfo::builder()
            .code_size(bytecode.len())
            .code(code);

        let shader_module = device.create_shader_module(&shader_info, None)?;

        Ok(shader_module)
    }
}

pub fn create_pipeline(mut vertex_shader_source: ShaderSource, mut fragment_shader_source: ShaderSource, device: &Device, extent: Option<vk::Extent2D>, layout: vk::PipelineLayout, render_pass: vk::RenderPass, subpass_idx: u32, blend_state_descriptors: &[BlendStateDescriptor], depth_buffer_usage: DepthBufferUsageMode, binding_descriptions: &[impl vk::Cast<Target = vk::VertexInputBindingDescription>], attribute_descriptions: &[impl vk::Cast<Target = vk::VertexInputAttributeDescription>]) -> Result<vk::Pipeline> {
    vertex_shader_source = vertex_shader_source.flatten()?;
    let (vert, vert_entry_name) = vertex_shader_source.get_source()?;
    fragment_shader_source = fragment_shader_source.flatten()?;
    let (frag, frag_entry_name) = fragment_shader_source.get_source()?;

    let vert_module = create_shader_module(device, &*vert)?;
    let frag_module = create_shader_module(device, &*frag)?;

    let mut vert_entry_name = vert_entry_name.to_owned();
    vert_entry_name.push_str("\0");
    let mut frag_entry_name = frag_entry_name.to_owned();
    frag_entry_name.push_str("\0");

    let vert_stage = vk::PipelineShaderStageCreateInfo::builder()
        .stage(vk::ShaderStageFlags::VERTEX)
        .module(vert_module)
        .name(vert_entry_name.as_bytes());

    let frag_stage = vk::PipelineShaderStageCreateInfo::builder()
        .stage(vk::ShaderStageFlags::FRAGMENT)
        .module(frag_module)
        .name(frag_entry_name.as_bytes());

    let vertex_input_state = vk::PipelineVertexInputStateCreateInfo::builder()
        .vertex_binding_descriptions(binding_descriptions)
        .vertex_attribute_descriptions(attribute_descriptions);

    let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo::builder()
        .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
        .primitive_restart_enable(false);

    let viewport_extent = extent.unwrap_or(vk::Extent2D { width: 1920, height: 1080 }); //Default is only used as a placeholder, because it'll be set dynamically

    let viewport = vk::Viewport::builder()
        .x(0.0)
        .y(0.0)
        .width(viewport_extent.width as f32)
        .height(viewport_extent.height as f32)
        .min_depth(0.0)
        .max_depth(1.0);

    let scissor = vk::Rect2D::builder()
        .offset(vk::Offset2D { x: 0, y: 0 })
        .extent(viewport_extent);

    let viewports = &[viewport];
    let scissors = &[scissor];
    let viewport_state = vk::PipelineViewportStateCreateInfo::builder()
        .viewports(viewports)
        .scissors(scissors);

    let rasterization_state = vk::PipelineRasterizationStateCreateInfo::builder()
        .depth_clamp_enable(false)
        .rasterizer_discard_enable(false)
        .polygon_mode(vk::PolygonMode::FILL)
        .line_width(1.0)
        .cull_mode(vk::CullModeFlags::BACK)
        .front_face(vk::FrontFace::COUNTER_CLOCKWISE)
        .depth_bias_enable(false);

    let multisample_state = vk::PipelineMultisampleStateCreateInfo::builder()
        .sample_shading_enable(false)
        .rasterization_samples(vk::SampleCountFlags::_1);

    let mut blend_state_attachments = vec![];
    for blend_state_desc in blend_state_descriptors {
        let attachment = vk::PipelineColorBlendAttachmentState::builder()
            .color_write_mask(blend_state_desc.components)
            .blend_enable(blend_state_desc.enable_blend)
            .src_color_blend_factor(blend_state_desc.src_color_blend_factor)
            .dst_color_blend_factor(blend_state_desc.dst_color_blend_factor)
            .color_blend_op(blend_state_desc.color_blend_op)
            .src_alpha_blend_factor(blend_state_desc.src_alpha_blend_factor)
            .dst_alpha_blend_factor(blend_state_desc.dst_alpha_blend_factor)
            .alpha_blend_op(blend_state_desc.alpha_blend_op);

        blend_state_attachments.push(attachment);
    }

    let color_blend_state = vk::PipelineColorBlendStateCreateInfo::builder()
        .logic_op_enable(false)
        .logic_op(vk::LogicOp::COPY)
        .attachments(&blend_state_attachments[..])
        .blend_constants([0.0, 0.0, 0.0, 0.0]);

    let mut dynamic_states = vec![];
    if extent.is_none() {
        dynamic_states.push(vk::DynamicState::VIEWPORT);
        dynamic_states.push(vk::DynamicState::SCISSOR);
    }

    let dynamic_state = vk::PipelineDynamicStateCreateInfo::builder()
        .dynamic_states(&dynamic_states[..]);

    let stages = &[vert_stage, frag_stage];
    let mut pipeline_create_info = vk::GraphicsPipelineCreateInfo::builder()
        .stages(stages)
        .vertex_input_state(&vertex_input_state)
        .input_assembly_state(&input_assembly_state)
        .viewport_state(&viewport_state)
        .rasterization_state(&rasterization_state)
        .multisample_state(&multisample_state)
        .color_blend_state(&color_blend_state)
        .dynamic_state(&dynamic_state)
        .layout(layout)
        .render_pass(render_pass)
        .subpass(subpass_idx)
        .base_pipeline_handle(vk::Pipeline::null())
        .base_pipeline_index(-1);

    let mut depth_stencil_state: vk::PipelineDepthStencilStateCreateInfoBuilder;
    if depth_buffer_usage != DepthBufferUsageMode::DontUse {
        depth_stencil_state = vk::PipelineDepthStencilStateCreateInfo::builder()
            .depth_test_enable(true)
            .depth_bounds_test_enable(false)
            .min_depth_bounds(0.0)
            .max_depth_bounds(1.0)
            .stencil_test_enable(false);

        match depth_buffer_usage {
            DepthBufferUsageMode::WriteIfLess => {
                depth_stencil_state = depth_stencil_state
                    .depth_write_enable(true)
                    .depth_compare_op(vk::CompareOp::LESS);
            },
            DepthBufferUsageMode::WriteIfEqual => {
                depth_stencil_state = depth_stencil_state
                    .depth_write_enable(false)
                    .depth_compare_op(vk::CompareOp::EQUAL);
            },
            _ => return Err(anyhow!("Unrecognized or unsupported depth buffer usage mode: {:?}", depth_buffer_usage))
        }

        pipeline_create_info = pipeline_create_info.depth_stencil_state(&depth_stencil_state);
    }

    let pipeline_create_infos = &[pipeline_create_info];
    let pipeline: vk::Pipeline;
    unsafe {
        pipeline = device.create_graphics_pipelines(vk::PipelineCache::null(), pipeline_create_infos, None)?.0;
    }

    unsafe {
        device.destroy_shader_module(vert_module, None);
        device.destroy_shader_module(frag_module, None);
    }

    Ok(pipeline)
}
