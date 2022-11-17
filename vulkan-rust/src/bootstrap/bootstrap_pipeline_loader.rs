use crate::app::{AppData};

use super::{BootstrapLoader};

use anyhow::{anyhow, Result};
use winit::window::{Window};
use vulkanalia::{
    prelude::v1_0::*
};

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

    fn create_render_pass(&self, device: &Device, app_data: &mut AppData) -> Result<()> {
        let color_attachment = vk::AttachmentDescription::builder()
            .format(app_data.swapchain_format.unwrap())
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

        let color_attachments = &[color_attachment_ref];
        let subpass = vk::SubpassDescription::builder()
            .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS)
            .color_attachments(color_attachments);

        let attachments = &[color_attachment];
        let subpasses = &[subpass];
        let render_pass_info = vk::RenderPassCreateInfo::builder()
            .attachments(attachments)
            .subpasses(subpasses);

        let render_pass: vk::RenderPass;
        unsafe {
            debug!("Creating render pass...");
            render_pass = device.create_render_pass(&render_pass_info, None)?;
            debug!("Render pass created: {:?}", render_pass);
        }
        app_data.render_pass = Some(render_pass);

        Ok(())
    }

    fn destroy_render_pass(&self, device: &Device, app_data: &mut AppData) -> () {
        if let Some(render_pass) = app_data.render_pass.take() {
            debug!("Destroying render pass...");
            unsafe {
                device.destroy_render_pass(render_pass, None);
            }
        }
    }

    fn create_pipeline(&self, device: &Device, app_data: &mut AppData) -> Result<()> {
        let vert = include_bytes!("../../shaders/static_tri/shader.vert.spv");
        let frag = include_bytes!("../../shaders/static_tri/shader.frag.spv");

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

        let vertex_input_state = vk::PipelineVertexInputStateCreateInfo::builder();

        let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo::builder()
            .topology(vk::PrimitiveTopology::TRIANGLE_LIST)
            .primitive_restart_enable(false);

        let extent = app_data.swapchain_extent.unwrap();
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

        let layout_info = vk::PipelineLayoutCreateInfo::builder();
        let pipeline_layout: vk::PipelineLayout;
        unsafe {
            debug!("Creating pipeline layout...");
            pipeline_layout = device.create_pipeline_layout(&layout_info, None)?;
            debug!("Pipeline layout created: {:?}", pipeline_layout);
        }
        app_data.pipeline_layout = Some(pipeline_layout);

        let render_pass = app_data.render_pass.unwrap();

        let stages = &[vert_stage, frag_stage];
        let pipeline_info = vk::GraphicsPipelineCreateInfo::builder()
            .stages(stages)
            .vertex_input_state(&vertex_input_state)
            .input_assembly_state(&input_assembly_state)
            .viewport_state(&viewport_state)
            .rasterization_state(&rasterization_state)
            .multisample_state(&multisample_state)
            .color_blend_state(&color_blend_state)
            .layout(pipeline_layout)
            .render_pass(render_pass)
            .subpass(0)
            .base_pipeline_handle(vk::Pipeline::null())
            .base_pipeline_index(-1);

        let pipeline_infos = &[pipeline_info];
        let pipeline: vk::Pipeline;
        unsafe {
            debug!("Creating pipeline...");
            pipeline = device.create_graphics_pipelines(vk::PipelineCache::null(), pipeline_infos, None)?.0;
            debug!("Pipeline created: {:?}", pipeline);
        }
        app_data.pipeline = Some(pipeline);

        unsafe {
            device.destroy_shader_module(vert_module, None);
            device.destroy_shader_module(frag_module, None);
        }

        Ok(())
    }

    fn destroy_pipeline(&self, device: &Device, app_data: &mut AppData) -> () {
        if let Some(pipeline) = app_data.pipeline.take() {
            debug!("Destroying pipeline...");
            unsafe {
                device.destroy_pipeline(pipeline, None);
            }
        }

        if let Some(pipeline_layout) = app_data.pipeline_layout.take() {
            debug!("Destroying pipeline layout...");
            unsafe {
                device.destroy_pipeline_layout(pipeline_layout, None);
            }
        }
    }
}

impl BootstrapLoader for BootstrapPipelineLoader {
    fn after_create_logical_device(&self, _inst: &Instance, device: &Device, _window: &Window, app_data: &mut AppData) -> Result<()> {
        self.create_render_pass(device, app_data)?;
        self.create_pipeline(device, app_data)?;

        Ok(())
    }

    fn before_destroy_logical_device(&self, _inst: &Instance, device: &Device, app_data: &mut AppData) -> () {
        self.destroy_pipeline(device, app_data);
        self.destroy_render_pass(device, app_data);
    }
}