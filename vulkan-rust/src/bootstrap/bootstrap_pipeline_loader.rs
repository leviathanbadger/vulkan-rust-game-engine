use super::{BootstrapLoader, BootstrapSwapchainLoader, BootstrapRenderImagesLoader, BootstrapUniformLoader};

use std::{
    mem::{size_of}
};
use anyhow::{anyhow, Result};
use winit::window::{Window};
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    app_data::{AppData},
    shader_input::{
        {motion_blur},
        vertex_attribute_builder::{HasVertexAttributeBindings},
        push_constants::{DepthMotionPushConstants, BaseRenderPushConstants}
    },
    bootstrap_loader,
    resources::shader_source::{AttachmentDescriptor, SubpassDescriptor, SubpassAttachmentDescriptor, ShaderSource, BlendStateDescriptor, DepthBufferUsageMode, create_pipeline_layout, create_pipeline}
};

#[derive(Debug, Copy, Clone, Default)]
pub struct PipelineInfo {
    pub base_render_pass: vk::RenderPass,
    pub postprocessing_render_pass: vk::RenderPass,

    pub depth_motion_layout: vk::PipelineLayout,
    pub base_render_layout: vk::PipelineLayout,
    pub postprocessing_layout: vk::PipelineLayout,

    pub postprocessing_pipeline: vk::Pipeline
}

bootstrap_loader! {
    pub struct BootstrapPipelineLoader {
        depends_on(BootstrapSwapchainLoader, BootstrapRenderImagesLoader, BootstrapUniformLoader);
    }
}

impl BootstrapPipelineLoader {
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

    fn create_depth_motion_pipeline_layout(&self, device: &Device, pipeline_info: &mut PipelineInfo, descriptor_set_layout: vk::DescriptorSetLayout) -> Result<()> {
        let set_layouts = &[descriptor_set_layout][..];

        let vert_push_constant_range = vk::PushConstantRange::builder()
            .stage_flags(vk::ShaderStageFlags::ALL_GRAPHICS)
            .offset(0)
            .size(size_of::<DepthMotionPushConstants>() as u32)
            .build();
        let push_constant_ranges = &[vert_push_constant_range][..];

        let pipeline_layout = create_pipeline_layout(device, set_layouts, push_constant_ranges)?;
        pipeline_info.depth_motion_layout = pipeline_layout;

        Ok(())
    }
    fn create_base_render_pipeline_layout(&self, device: &Device, pipeline_info: &mut PipelineInfo, descriptor_set_layout: vk::DescriptorSetLayout) -> Result<()> {
        let set_layouts = &[descriptor_set_layout][..];

        let vert_push_constant_range = vk::PushConstantRange::builder()
            .stage_flags(vk::ShaderStageFlags::ALL_GRAPHICS)
            .offset(0)
            .size(size_of::<BaseRenderPushConstants>() as u32)
            .build();
        let push_constant_ranges = &[vert_push_constant_range][..];

        let pipeline_layout = create_pipeline_layout(device, set_layouts, push_constant_ranges)?;
        pipeline_info.base_render_layout = pipeline_layout;

        Ok(())
    }
    fn create_postprocessing_pipeline(&self, device: &Device, pipeline_info: &mut PipelineInfo, extent: vk::Extent2D, descriptor_set_layout: vk::DescriptorSetLayout, render_pass: vk::RenderPass) -> Result<()> {
        debug!("Creating postprocessing pipeline layout and pipeline...");

        let vert_source = ShaderSource::SourcePath("shaders/motion_blur/shader.vert.spv".to_owned(), "main");
        let frag_source = ShaderSource::SourcePath("shaders/motion_blur/shader.frag.spv".to_owned(), "main");

        let set_layouts = &[descriptor_set_layout][..];

        let push_constant_ranges = &[][..] as &[vk::PushConstantRange];

        let pipeline_layout = create_pipeline_layout(device, set_layouts, push_constant_ranges)?;

        let blend_state = &[
            BlendStateDescriptor::default()
        ][..];
        let binding_descriptions = &[motion_blur::Vertex::binding_descriptions()].concat()[..];
        let attribute_descriptions = &[motion_blur::Vertex::attribute_descriptions()].concat()[..];
        let pipeline = create_pipeline(vert_source, frag_source, device, extent, pipeline_layout, render_pass, 0, blend_state, DepthBufferUsageMode::DontUse, binding_descriptions, attribute_descriptions)?;

        debug!("Postprocessing pipeline layout ({:?}) and pipeline ({:?}) created.", pipeline_layout, pipeline);

        pipeline_info.postprocessing_layout = pipeline_layout;
        pipeline_info.postprocessing_pipeline = pipeline;

        Ok(())
    }

    fn create_pipelines(&self, device: &Device, pipeline_info: &mut PipelineInfo, app_data: &AppData) -> Result<()> {
        let extent = app_data.swapchain.as_ref().unwrap().extent;
        let uniforms_info = app_data.uniforms.as_ref().unwrap();

        self.create_depth_motion_pipeline_layout(device, pipeline_info, uniforms_info.base_descriptor_set_layout)?;
        self.create_base_render_pipeline_layout(device, pipeline_info, uniforms_info.base_descriptor_set_layout)?;
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
            device.destroy_pipeline_layout(pipeline_info.base_render_layout, None);
        }
        pipeline_info.base_render_layout = vk::PipelineLayout::null();

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
