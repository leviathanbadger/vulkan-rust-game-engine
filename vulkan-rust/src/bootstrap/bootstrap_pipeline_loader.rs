use super::{BootstrapLoader};

use std::mem::{size_of};
use anyhow::{anyhow, Result};
use winit::window::{Window};
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    app_data::{AppData},
    shader_input::{
        simple::{Vertex},
        push_constants::PushConstants
    }
};

#[derive(Debug, Copy, Clone, Default)]
pub struct PipelineInfo {
    pub render_pass: vk::RenderPass,
    pub layout: vk::PipelineLayout,
    pub pipeline: vk::Pipeline
}

#[derive(Debug, Default)]
pub struct BootstrapPipelineLoader { }

impl BootstrapPipelineLoader {
    pub fn new() -> Self {
        Self::default()
    }

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

    fn create_render_pass(&self, device: &Device, pipeline_info: &mut PipelineInfo, app_data: &AppData) -> Result<()> {
        let swapchain_format = app_data.swapchain.as_ref().unwrap().surface_format.format;

        let color_attachment = vk::AttachmentDescription::builder()
            .format(swapchain_format)
            .samples(vk::SampleCountFlags::_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::STORE)
            .stencil_load_op(vk::AttachmentLoadOp::CLEAR)
            .stencil_store_op(vk::AttachmentStoreOp::STORE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::PRESENT_SRC_KHR);

        let color_attachment_ref = vk::AttachmentReference::builder()
            .attachment(0)
            .layout(vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL);

        let depth_buffer_info = app_data.depth_buffer.as_ref().unwrap();

        let depth_attachment = vk::AttachmentDescription::builder()
            .format(depth_buffer_info.format())
            .samples(vk::SampleCountFlags::_1)
            .load_op(vk::AttachmentLoadOp::CLEAR)
            .store_op(vk::AttachmentStoreOp::DONT_CARE)
            .stencil_load_op(vk::AttachmentLoadOp::CLEAR)
            .stencil_store_op(vk::AttachmentStoreOp::DONT_CARE)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .final_layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL);

        let depth_attachment_ref = vk::AttachmentReference::builder()
            .attachment(1)
            .layout(vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL);

        let color_attachments = &[color_attachment_ref];
        let subpass = vk::SubpassDescription::builder()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(color_attachments)
            .depth_stencil_attachment(&depth_attachment_ref);

        let dependency = vk::SubpassDependency::builder()
            .src_subpass(vk::SUBPASS_EXTERNAL)
            .dst_subpass(0)
            .src_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS)
            .src_access_mask(vk::AccessFlags::empty())
            .dst_stage_mask(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT | vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS)
            .dst_access_mask(vk::AccessFlags::COLOR_ATTACHMENT_WRITE | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE);

        let attachments = &[color_attachment, depth_attachment];
        let subpasses = &[subpass];
        let dependencies = &[dependency];
        let render_pass_info = vk::RenderPassCreateInfo::builder()
            .attachments(attachments)
            .subpasses(subpasses)
            .dependencies(dependencies);

        let render_pass: vk::RenderPass;
        unsafe {
            debug!("Creating render pass...");
            render_pass = device.create_render_pass(&render_pass_info, None)?;
            debug!("Render pass created: {:?}", render_pass);
        }
        pipeline_info.render_pass = render_pass;

        Ok(())
    }

    fn destroy_render_pass(&self, device: &Device, pipeline_info: &mut PipelineInfo) -> () {
        debug!("Destroying render pass...");
        unsafe {
            device.destroy_render_pass(pipeline_info.render_pass, None);
        }
        pipeline_info.render_pass = vk::RenderPass::null();
    }

    fn create_pipeline(&self, device: &Device, pipeline_info: &mut PipelineInfo, app_data: &AppData) -> Result<()> {
        let vert = include_bytes!("../../shaders/simple/shader.vert.spv");
        let frag = include_bytes!("../../shaders/simple/shader.frag.spv");

        debug!("Creating vertex and fragment shader modules...");
        let vert_module = self.create_shader_module(device, &vert[..])?;
        let frag_module = self.create_shader_module(device, &frag[..])?;

        let vert_stage = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::VERTEX)
            .module(vert_module)
            .name(b"main\0");

        let frag_stage = vk::PipelineShaderStageCreateInfo::builder()
            .stage(vk::ShaderStageFlags::FRAGMENT)
            .module(frag_module)
            .name(b"main\0");

        let binding_descriptions = &[Vertex::binding_description()];
        let attribute_descriptions = &Vertex::attribute_descriptions();
        let vertex_input_state = vk::PipelineVertexInputStateCreateInfo::builder()
            .vertex_binding_descriptions(binding_descriptions)
            .vertex_attribute_descriptions(attribute_descriptions);

        let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);

        let extent = app_data.swapchain.as_ref().unwrap().extent;
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
            .front_face(vk::FrontFace::CLOCKWISE)
            .depth_bias_enable(false);

        let multisample_state = vk::PipelineMultisampleStateCreateInfo::builder()
            .sample_shading_enable(false)
            .rasterization_samples(vk::SampleCountFlags::_1);

        let depth_stencil_state = vk::PipelineDepthStencilStateCreateInfo::builder()
            .depth_test_enable(true)
            .depth_write_enable(true)
            .depth_compare_op(vk::CompareOp::LESS)
            .depth_bounds_test_enable(false)
            .min_depth_bounds(0.0)
            .max_depth_bounds(1.0)
            .stencil_test_enable(false);

        let attachment = vk::PipelineColorBlendAttachmentState::builder()
            .color_write_mask(vk::ColorComponentFlags::all())
            .blend_enable(false)
            .src_color_blend_factor(vk::BlendFactor::ONE)
            .dst_color_blend_factor(vk::BlendFactor::ZERO)
            .color_blend_op(vk::BlendOp::ADD)
            .src_alpha_blend_factor(vk::BlendFactor::ONE)
            .dst_alpha_blend_factor(vk::BlendFactor::ZERO)
            .alpha_blend_op(vk::BlendOp::ADD);

        let attachments = &[attachment];
        let color_blend_state = vk::PipelineColorBlendStateCreateInfo::builder()
            .logic_op_enable(false)
            .logic_op(vk::LogicOp::COPY)
            .attachments(attachments)
            .blend_constants([0.0, 0.0, 0.0, 0.0]);

        let desc_set_layout = app_data.uniforms.as_ref().unwrap().descriptor_set_layout;
        let set_layouts = &[desc_set_layout];

        let vert_push_constant_range = vk::PushConstantRange::builder()
            .stage_flags(vk::ShaderStageFlags::ALL_GRAPHICS)
            .offset(0)
            .size(size_of::<PushConstants>() as u32);
        let push_constant_ranges = &[vert_push_constant_range];

        let layout_info = vk::PipelineLayoutCreateInfo::builder()
            .set_layouts(set_layouts)
            .push_constant_ranges(push_constant_ranges);
        let pipeline_layout: vk::PipelineLayout;
        unsafe {
            debug!("Creating pipeline layout...");
            pipeline_layout = device.create_pipeline_layout(&layout_info, None)?;
            debug!("Pipeline layout created: {:?}", pipeline_layout);
        }
        pipeline_info.layout = pipeline_layout;

        let render_pass = pipeline_info.render_pass;

        let stages = &[vert_stage, frag_stage];
        let pipeline_create_info = vk::GraphicsPipelineCreateInfo::builder()
            .stages(stages)
            .vertex_input_state(&vertex_input_state)
            .input_assembly_state(&input_assembly_state)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterization_state)
            .multisample_state(&multisample_state)
            .depth_stencil_state(&depth_stencil_state)
            .color_blend_state(&color_blend_state)
            .layout(pipeline_layout)
            .render_pass(render_pass)
            .subpass(0)
            .base_pipeline_handle(vk::Pipeline::null())
            .base_pipeline_index(-1);

        let pipeline_create_infos = &[pipeline_create_info];
        let pipeline: vk::Pipeline;
        unsafe {
            debug!("Creating pipeline...");
            pipeline = device.create_graphics_pipelines(vk::PipelineCache::null(), pipeline_create_infos, None)?.0;
            debug!("Pipeline created: {:?}", pipeline);
        }
        pipeline_info.pipeline = pipeline;

        unsafe {
            device.destroy_shader_module(vert_module, None);
            device.destroy_shader_module(frag_module, None);
        }

        Ok(())
    }

    fn destroy_pipeline(&self, device: &Device, pipeline_info: &mut PipelineInfo) -> () {
        debug!("Destroying pipeline...");
        unsafe {
            device.destroy_pipeline(pipeline_info.pipeline, None);
        }
        pipeline_info.pipeline = vk::Pipeline::null();

        debug!("Destroying pipeline layout...");
        unsafe {
            device.destroy_pipeline_layout(pipeline_info.layout, None);
        }
        pipeline_info.layout = vk::PipelineLayout::null();
    }
}

impl BootstrapLoader for BootstrapPipelineLoader {
    fn after_create_logical_device(&self, _inst: &Instance, device: &Device, _window: &Window, app_data: &mut AppData) -> Result<()> {
        let mut pipeline_info = PipelineInfo::default();
        self.create_render_pass(device, &mut pipeline_info, app_data)?;
        self.create_pipeline(device, &mut pipeline_info, app_data)?;
        app_data.pipeline = Some(pipeline_info);

        Ok(())
    }

    fn before_destroy_logical_device(&self, _inst: &Instance, device: &Device, app_data: &mut AppData) -> () {
        if let Some(mut pipeline_info) = app_data.pipeline.take() {
            self.destroy_pipeline(device, &mut pipeline_info);
            self.destroy_render_pass(device, &mut pipeline_info);
        }
    }
}
