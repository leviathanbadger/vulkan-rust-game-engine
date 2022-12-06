use super::{BootstrapLoader, BootstrapSwapchainLoader, BootstrapRenderImagesLoader, BootstrapUniformLoader};

use std::{
    mem::{size_of},
    path::{Path},
    fs::{File},
    io::{Read}
};
use anyhow::{anyhow, Result};
use winit::window::{Window};
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    app_data::{AppData},
    shader_input::{
        {simple, marble, motion_blur},
        push_constants::{DepthMotionPushConstants, BaseRenderPushConstants},
        vertex_attribute_builder::{HasVertexAttributeBindings}
    },
    bootstrap_loader
};

#[derive(Debug)]
pub enum ShaderSource<'a> {
    Source(Box<[u8]>, &'static str),
    SourcePath(&'a Path, &'static str)
}

impl<'a> ShaderSource<'a> {
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

#[derive(Debug, Copy, Clone)]
pub struct AttachmentDescriptor {
    format: vk::Format,
    load_op: vk::AttachmentLoadOp,
    store_op: vk::AttachmentStoreOp,
    stencil_load_op: Option<vk::AttachmentLoadOp>,
    stencil_store_op: Option<vk::AttachmentStoreOp>,
    initial_layout: vk::ImageLayout,
    final_layout: vk::ImageLayout
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
    attached: bool,
    ref_layout: vk::ImageLayout
}

#[derive(Debug)]
pub struct SubpassDescriptor {
    color_attachments: Vec<SubpassAttachmentDescriptor>,
    depth_attachment: SubpassAttachmentDescriptor
}

#[derive(Debug, Copy, Clone)]
pub struct BlendStateDescriptor {
    components: vk::ColorComponentFlags,

    enable_blend: bool,

    src_color_blend_factor: vk::BlendFactor,
    dst_color_blend_factor: vk::BlendFactor,
    color_blend_op: vk::BlendOp,
    src_alpha_blend_factor: vk::BlendFactor,
    dst_alpha_blend_factor: vk::BlendFactor,
    alpha_blend_op: vk::BlendOp
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

#[derive(Debug, Copy, Clone, Default)]
pub struct PipelineInfo {
    pub base_render_pass: vk::RenderPass,
    pub postprocessing_render_pass: vk::RenderPass,

    pub depth_motion_layout: vk::PipelineLayout,
    pub depth_motion_pipeline: vk::Pipeline,
    pub base_render_layout: vk::PipelineLayout,
    pub base_render_pipeline: vk::Pipeline,
    pub postprocessing_layout: vk::PipelineLayout,
    pub postprocessing_pipeline: vk::Pipeline
}

bootstrap_loader! {
    pub struct BootstrapPipelineLoader {
        depends_on(BootstrapSwapchainLoader, BootstrapRenderImagesLoader, BootstrapUniformLoader);
    }
}

impl BootstrapPipelineLoader {
    fn create_shader_module(&self, device: &Device, bytecode: &[u8]) -> Result<vk::ShaderModule> {
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

    fn is_depth_stencil_image_layout(layout: vk::ImageLayout) -> bool {
        match layout {
            vk::ImageLayout::DEPTH_ATTACHMENT_OPTIMAL => true,
            vk::ImageLayout::DEPTH_ATTACHMENT_STENCIL_READ_ONLY_OPTIMAL => true,
            vk::ImageLayout::DEPTH_READ_ONLY_OPTIMAL => true,
            vk::ImageLayout::DEPTH_READ_ONLY_STENCIL_ATTACHMENT_OPTIMAL => true,
            vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL => true,
            vk::ImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL => true,
            vk::ImageLayout::STENCIL_ATTACHMENT_OPTIMAL => true,
            vk::ImageLayout::STENCIL_READ_ONLY_OPTIMAL => true,
            _ => false
        }
    }

    fn create_render_pass(&self, device: &Device, attach_descs: &[AttachmentDescriptor], depth_attach_desc: Option<&AttachmentDescriptor>, subpass_descs: &[SubpassDescriptor], subpass_dependencies: &[vk::SubpassDependency]) -> Result<vk::RenderPass> {
        let mut render_pass_attachments = vec![];

        for attach_desc in attach_descs {
            match attach_desc.initial_layout {
                vk::ImageLayout::UNDEFINED => {
                    let is_stencil_load = attach_desc.stencil_load_op.unwrap_or(attach_desc.load_op) == vk::AttachmentLoadOp::LOAD;
                    if attach_desc.load_op == vk::AttachmentLoadOp::LOAD || is_stencil_load {
                        warn!("Attachment load op or stencil load op is configured as 'LOAD' but the initial image layout is 'UNDEFINED'. Attachment state will not be loaded.");
                    }
                },
                _ => { }
            }

            let attachment = vk::AttachmentDescription::builder()
                .format(attach_desc.format)
                .samples(vk::SampleCountFlags::_1)
                .load_op(attach_desc.load_op)
                .store_op(attach_desc.store_op)
                .stencil_load_op(attach_desc.stencil_load_op.unwrap_or(attach_desc.load_op))
                .stencil_store_op(attach_desc.stencil_store_op.unwrap_or(attach_desc.store_op))
                .initial_layout(attach_desc.initial_layout)
                .final_layout(attach_desc.final_layout);

            if Self::is_depth_stencil_image_layout(attach_desc.final_layout) {
                return Err(anyhow!("The depth/stencil attachment should be specified as a separate parameter to #create_render_pass."));
            }
            render_pass_attachments.push(attachment);
        }

        let mut depth_attach_idx = None;
        if let Some(attach_desc) = depth_attach_desc {
            match attach_desc.initial_layout {
                vk::ImageLayout::UNDEFINED => {
                    let is_stencil_load = attach_desc.stencil_load_op.unwrap_or(attach_desc.load_op) == vk::AttachmentLoadOp::LOAD;
                    if attach_desc.load_op == vk::AttachmentLoadOp::LOAD || is_stencil_load {
                        warn!("Depth attachment load op or stencil load op is configured as 'LOAD' but the initial image layout is 'UNDEFINED'. Attachment state will not be loaded.");
                    }
                },
                _ => { }
            }

            let attachment = vk::AttachmentDescription::builder()
                .format(attach_desc.format)
                .samples(vk::SampleCountFlags::_1)
                .load_op(attach_desc.load_op)
                .store_op(attach_desc.store_op)
                .stencil_load_op(attach_desc.stencil_load_op.unwrap_or(attach_desc.load_op))
                .stencil_store_op(attach_desc.stencil_store_op.unwrap_or(attach_desc.store_op))
                .initial_layout(attach_desc.initial_layout)
                .final_layout(attach_desc.final_layout);

            depth_attach_idx = Some(render_pass_attachments.len() as u32);
            render_pass_attachments.push(attachment);
        }

        let mut subpasses = vec![];
        let mut subpass_color_attachments = vec![];
        let mut subpass_depth_attachments = vec![];

        for subpass_desc in subpass_descs {
            let mut this_subpass_color_attachments = vec![];
            for (q, color_attachment) in subpass_desc.color_attachments.iter().enumerate() {
                if !color_attachment.attached { continue; }

                let ref_layout = color_attachment.ref_layout;
                let attachment_ref = vk::AttachmentReference::builder()
                    .attachment(q as u32)
                    .layout(ref_layout);
                this_subpass_color_attachments.push(attachment_ref);
            }
            subpass_color_attachments.push(this_subpass_color_attachments);

            if subpass_desc.depth_attachment.attached {
                if let Some(idx) = depth_attach_idx {
                    let ref_layout = subpass_desc.depth_attachment.ref_layout;
                    let depth_attach_ref = vk::AttachmentReference::builder()
                        .attachment(idx)
                        .layout(ref_layout);
                    subpass_depth_attachments.push(Some(depth_attach_ref));
                } else {
                    return Err(anyhow!("Subpass requires depth buffer be attached, but none was provided to the pipeline"));
                }
            } else {
                subpass_depth_attachments.push(None);
            }
        }

        for (idx, _subpass_desc) in subpass_descs.iter().enumerate() {
            let color_attachment_refs = &subpass_color_attachments[idx][..];
            let mut subpass = vk::SubpassDescription::builder()
                .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
                .color_attachments(color_attachment_refs);

            if let Some(depth_attachment_ref) = subpass_depth_attachments[idx].as_ref() {
                subpass = subpass.depth_stencil_attachment(depth_attachment_ref);
            }
            subpasses.push(subpass);
        }

        let mut src_stage_mask = vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT;
        let src_access_mask = vk::AccessFlags::empty();
        let mut dst_stage_mask = vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT;
        let mut dst_access_mask = vk::AccessFlags::COLOR_ATTACHMENT_WRITE;

        if let Some(_) = depth_attach_idx {
            src_stage_mask |= vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS;
            dst_stage_mask |= vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS;
            dst_access_mask |= vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE;
        }

        let mut dependencies = vec![];

        let external_dependency = vk::SubpassDependency::builder()
            .src_subpass(vk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(src_stage_mask)
            .src_access_mask(src_access_mask)
            .dst_stage_mask(dst_stage_mask)
            .dst_access_mask(dst_access_mask)
            .build();
        dependencies.push(external_dependency);
        for dep in subpass_dependencies {
            dependencies.push(*dep);
        }

        let render_pass_info = vk::RenderPassCreateInfo::builder()
            .attachments(&render_pass_attachments[..])
            .subpasses(&subpasses[..])
            .dependencies(&dependencies[..]);

        unsafe {
            Ok(device.create_render_pass(&render_pass_info, None)?)
        }
    }

    fn create_render_passes(&self, device: &Device, pipeline_info: &mut PipelineInfo, app_data: &AppData) -> Result<()> {
        debug!("Creating render passes...");

        let swapchain_format = app_data.swapchain.as_ref().unwrap().surface_format.format;
        let render_images_info = &app_data.render_images.as_ref().unwrap();
        let depth_buffer_format = render_images_info.depth_stencil_format();
        let motion_vector_format = render_images_info.motion_vector_format();

        let color_attachments = &[
            AttachmentDescriptor {
                format: swapchain_format,
                final_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                ..Default::default()
            },
            AttachmentDescriptor {
                format: motion_vector_format,
                final_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                ..Default::default()
            }
        ][..];
        let depth_attachment = AttachmentDescriptor {
            format: depth_buffer_format,
            store_op: vk::AttachmentStoreOp::DONT_CARE,
            final_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
            ..Default::default()
        };
        let subpasses = &[
            SubpassDescriptor {
                color_attachments: vec![
                    SubpassAttachmentDescriptor {
                        attached: false,
                        ..Default::default()
                    },
                    SubpassAttachmentDescriptor {
                        attached: true,
                        ref_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL
                    }
                ],
                depth_attachment: SubpassAttachmentDescriptor {
                    attached: true,
                    ref_layout: vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL
                }
            },
            SubpassDescriptor {
                color_attachments: vec![
                    SubpassAttachmentDescriptor {
                        attached: true,
                        ref_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL
                    },
                    SubpassAttachmentDescriptor {
                        attached: false,
                        ..Default::default()
                    }
                ],
                depth_attachment: SubpassAttachmentDescriptor {
                    attached: true,
                    ref_layout: vk::ImageLayout::DEPTH_STENCIL_READ_ONLY_OPTIMAL
                }
            }
        ][..];
        let subpass_dependencies = &[
            vk::SubpassDependency::builder()
                .src_subpass(0)
                .dst_subpass(1)
                .src_stage_mask(vk::PipelineStageFlags::LATE_FRAGMENT_TESTS)
                .src_access_mask(vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE)
                .dst_stage_mask(vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS)
                .dst_access_mask(vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ)
                .build()
        ][..];
        let base_render_pass = self.create_render_pass(device, color_attachments, Some(&depth_attachment), subpasses, subpass_dependencies)?;

        let color_attachments = &[
            AttachmentDescriptor {
                format: swapchain_format,
                final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
                ..Default::default()
            }
        ][..];
        let subpasses = &[
            SubpassDescriptor {
                color_attachments: vec![
                    SubpassAttachmentDescriptor {
                        attached: true,
                        ref_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL
                    }
                ],
                depth_attachment: SubpassAttachmentDescriptor {
                    attached: false,
                    ..Default::default()
                }
            }
        ][..];
        let subpass_dependencies = &[][..];
        let motion_blur_render_pass = self.create_render_pass(device, color_attachments, None, subpasses, subpass_dependencies)?;

        debug!("Render passes created: {:?}, {:?}", base_render_pass, motion_blur_render_pass);
        pipeline_info.base_render_pass = base_render_pass;
        pipeline_info.postprocessing_render_pass = motion_blur_render_pass;

        Ok(())
    }

    fn destroy_render_passes(&self, device: &Device, pipeline_info: &mut PipelineInfo) -> () {
        debug!("Destroying render passes...");

        unsafe {
            device.destroy_render_pass(pipeline_info.base_render_pass, None);
        }
        pipeline_info.base_render_pass = vk::RenderPass::null();

        unsafe {
            device.destroy_render_pass(pipeline_info.postprocessing_render_pass, None);
        }
        pipeline_info.postprocessing_render_pass = vk::RenderPass::null();
    }

    fn create_pipeline_layout(&self, device: &Device, set_layouts: &[vk::DescriptorSetLayout], push_constant_ranges: &[vk::PushConstantRange]) -> Result<vk::PipelineLayout> {
        let layout_info = vk::PipelineLayoutCreateInfo::builder()
            .set_layouts(set_layouts)
            .push_constant_ranges(push_constant_ranges);

        unsafe {
            Ok(device.create_pipeline_layout(&layout_info, None)?)
        }
    }

    fn create_pipeline<TVert: HasVertexAttributeBindings, TInstVert: HasVertexAttributeBindings>(&self, mut vertex_shader_source: ShaderSource, mut fragment_shader_source: ShaderSource, device: &Device, extent: vk::Extent2D, layout: vk::PipelineLayout, render_pass: vk::RenderPass, subpass_idx: u32, blend_state_descriptors: &[BlendStateDescriptor], depth_buffer_usage: DepthBufferUsageMode) -> Result<vk::Pipeline> {
        vertex_shader_source = vertex_shader_source.flatten()?;
        let (vert, vert_entry_name) = vertex_shader_source.get_source()?;
        fragment_shader_source = fragment_shader_source.flatten()?;
        let (frag, frag_entry_name) = fragment_shader_source.get_source()?;

        let vert_module = self.create_shader_module(device, &*vert)?;
        let frag_module = self.create_shader_module(device, &*frag)?;

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

        let binding_descriptions = &[TVert::binding_descriptions(), TInstVert::binding_descriptions()].concat()[..];
        let attribute_descriptions = &[TVert::attribute_descriptions(), TInstVert::attribute_descriptions()].concat()[..];
        let vertex_input_state = vk::PipelineVertexInputStateCreateInfo::builder()
            .vertex_binding_descriptions(binding_descriptions)
            .vertex_attribute_descriptions(attribute_descriptions);

        let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);

        let viewport = vk::Viewport::builder()
            .x(0.0)
            .y(0.0)
            .width(extent.width as f32)
            .height(extent.height as f32)
            .min_depth(0.0)
            .max_depth(1.0);

        let scissor = vk::Rect2D::builder()
            .offset(vk::Offset2D { x: 0, y: 0 })
            .extent(extent);

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

        let stages = &[vert_stage, frag_stage];
        let mut pipeline_create_info = vk::GraphicsPipelineCreateInfo::builder()
            .stages(stages)
            .vertex_input_state(&vertex_input_state)
            .input_assembly_state(&input_assembly_state)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterization_state)
            .multisample_state(&multisample_state)
            .color_blend_state(&color_blend_state)
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

    fn create_depth_motion_pipeline(&self, device: &Device, pipeline_info: &mut PipelineInfo, extent: vk::Extent2D, descriptor_set_layout: vk::DescriptorSetLayout, render_pass: vk::RenderPass) -> Result<()> {
        debug!("Creating depth+motion pipeline layout and pipeline...");

        let vert_source = ShaderSource::SourcePath(&Path::new("shaders/depth_and_motion_marble/shader.vert.spv"), "main");
        let frag_source = ShaderSource::SourcePath(&Path::new("shaders/depth_and_motion_marble/shader.frag.spv"), "main");

        let set_layouts = &[descriptor_set_layout][..];

        let vert_push_constant_range = vk::PushConstantRange::builder()
            .stage_flags(vk::ShaderStageFlags::ALL_GRAPHICS)
            .offset(0)
            .size(size_of::<DepthMotionPushConstants>() as u32)
            .build();
        let push_constant_ranges = &[vert_push_constant_range][..];

        let pipeline_layout = self.create_pipeline_layout(device, set_layouts, push_constant_ranges)?;

        let blend_state = &[
            BlendStateDescriptor {
                components: vk::ColorComponentFlags::R | vk::ColorComponentFlags::G,
                ..Default::default()
            }
        ][..];
        let pipeline = self.create_pipeline::<marble::Vertex, marble::MarbleInstance>(vert_source, frag_source, device, extent, pipeline_layout, render_pass, 0, blend_state, DepthBufferUsageMode::WriteIfLess)?;

        debug!("Depth+motion pipeline layout ({:?}) and pipeline ({:?}) created.", pipeline_layout, pipeline);

        pipeline_info.depth_motion_layout = pipeline_layout;
        pipeline_info.depth_motion_pipeline = pipeline;

        Ok(())
    }
    fn create_base_render_pipeline(&self, device: &Device, pipeline_info: &mut PipelineInfo, extent: vk::Extent2D, descriptor_set_layout: vk::DescriptorSetLayout, render_pass: vk::RenderPass) -> Result<()> {
        debug!("Creating base render pipeline layout and pipeline...");

        let vert_source = ShaderSource::SourcePath(&Path::new("shaders/marble/shader.vert.spv"), "main");
        let frag_source = ShaderSource::SourcePath(&Path::new("shaders/marble/shader.frag.spv"), "main");

        let set_layouts = &[descriptor_set_layout][..];

        let vert_push_constant_range = vk::PushConstantRange::builder()
            .stage_flags(vk::ShaderStageFlags::ALL_GRAPHICS)
            .offset(0)
            .size(size_of::<BaseRenderPushConstants>() as u32)
            .build();
        let push_constant_ranges = &[vert_push_constant_range][..];

        let pipeline_layout = self.create_pipeline_layout(device, set_layouts, push_constant_ranges)?;

        let blend_state = &[
            BlendStateDescriptor::default()
        ][..];
        let pipeline = self.create_pipeline::<marble::Vertex, marble::MarbleInstance>(vert_source, frag_source, device, extent, pipeline_layout, render_pass, 1, blend_state, DepthBufferUsageMode::WriteIfEqual)?;

        debug!("Base render pipeline layout ({:?}) and pipeline ({:?}) created.", pipeline_layout, pipeline);

        pipeline_info.base_render_layout = pipeline_layout;
        pipeline_info.base_render_pipeline = pipeline;

        Ok(())
    }
    fn create_postprocessing_pipeline(&self, device: &Device, pipeline_info: &mut PipelineInfo, extent: vk::Extent2D, descriptor_set_layout: vk::DescriptorSetLayout, render_pass: vk::RenderPass) -> Result<()> {
        debug!("Creating postprocessing pipeline layout and pipeline...");

        let vert_source = ShaderSource::SourcePath(&Path::new("shaders/motion_blur/shader.vert.spv"), "main");
        let frag_source = ShaderSource::SourcePath(&Path::new("shaders/motion_blur/shader.frag.spv"), "main");

        let set_layouts = &[descriptor_set_layout][..];

        let push_constant_ranges = &[][..] as &[vk::PushConstantRange];

        let pipeline_layout = self.create_pipeline_layout(device, set_layouts, push_constant_ranges)?;

        let blend_state = &[
            BlendStateDescriptor::default()
        ][..];
        let pipeline = self.create_pipeline::<motion_blur::Vertex, ()>(vert_source, frag_source, device, extent, pipeline_layout, render_pass, 0, blend_state, DepthBufferUsageMode::DontUse)?;

        debug!("Postprocessing pipeline layout ({:?}) and pipeline ({:?}) created.", pipeline_layout, pipeline);

        pipeline_info.postprocessing_layout = pipeline_layout;
        pipeline_info.postprocessing_pipeline = pipeline;

        Ok(())
    }

    fn create_pipelines(&self, device: &Device, pipeline_info: &mut PipelineInfo, app_data: &AppData) -> Result<()> {
        let extent = app_data.swapchain.as_ref().unwrap().extent;
        let uniforms_info = app_data.uniforms.as_ref().unwrap();

        self.create_depth_motion_pipeline(device, pipeline_info, extent, uniforms_info.base_descriptor_set_layout, pipeline_info.base_render_pass)?;
        self.create_base_render_pipeline(device, pipeline_info, extent, uniforms_info.base_descriptor_set_layout, pipeline_info.base_render_pass)?;
        self.create_postprocessing_pipeline(device, pipeline_info, extent, uniforms_info.postprocessing_descriptor_set_layout, pipeline_info.postprocessing_render_pass)?;

        Ok(())
    }

    fn destroy_pipelines(&self, device: &Device, pipeline_info: &mut PipelineInfo) -> () {
        debug!("Destroying pipelines and pipeline layouts...");

        unsafe {
            device.destroy_pipeline(pipeline_info.postprocessing_pipeline, None);
        }
        pipeline_info.postprocessing_pipeline = vk::Pipeline::null();

        unsafe {
            device.destroy_pipeline_layout(pipeline_info.postprocessing_layout, None);
        }
        pipeline_info.postprocessing_layout = vk::PipelineLayout::null();

        unsafe {
            device.destroy_pipeline(pipeline_info.base_render_pipeline, None);
        }
        pipeline_info.base_render_pipeline = vk::Pipeline::null();

        unsafe {
            device.destroy_pipeline_layout(pipeline_info.base_render_layout, None);
        }
        pipeline_info.base_render_layout = vk::PipelineLayout::null();

        unsafe {
            device.destroy_pipeline(pipeline_info.depth_motion_pipeline, None);
        }
        pipeline_info.depth_motion_pipeline = vk::Pipeline::null();

        unsafe {
            device.destroy_pipeline_layout(pipeline_info.depth_motion_layout, None);
        }
        pipeline_info.depth_motion_layout = vk::PipelineLayout::null();
    }
}

impl BootstrapLoader for BootstrapPipelineLoader {
    fn after_create_logical_device(&self, _inst: &Instance, device: &Device, _window: &Window, app_data: &mut AppData) -> Result<()> {
        let mut pipeline_info = PipelineInfo::default();
        self.create_render_passes(device, &mut pipeline_info, app_data)?;
        self.create_pipelines(device, &mut pipeline_info, app_data)?;
        app_data.pipeline = Some(pipeline_info);

        Ok(())
    }

    fn before_destroy_logical_device(&self, _inst: &Instance, device: &Device, app_data: &mut AppData) -> () {
        if let Some(mut pipeline_info) = app_data.pipeline.take() {
            self.destroy_pipelines(device, &mut pipeline_info);
            self.destroy_render_passes(device, &mut pipeline_info);
        }
    }
}
