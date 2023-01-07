use super::{BootstrapLoader, BootstrapSwapchainLoader, BootstrapRenderImagesLoader, BootstrapPipelineLoader};

use anyhow::{Result};
use winit::window::{Window};
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    app_data::{AppData},
    bootstrap_loader
};

#[derive(Debug, Default)]
pub struct FramebufferInfo {
    pub base_render_framebuffers: Vec<vk::Framebuffer>,
    pub postprocessing_framebuffers: Vec<vk::Framebuffer>
}

bootstrap_loader! {
    pub struct BootstrapFramebufferLoader {
        depends_on(BootstrapSwapchainLoader, BootstrapRenderImagesLoader, BootstrapPipelineLoader);
    }
}

impl BootstrapFramebufferLoader {
    fn create_framebuffers(&self, device: &Device, framebuffer_info: &mut FramebufferInfo, app_data: &AppData) -> Result<()> {
        let render_images_info = &app_data.render_images.as_ref().unwrap();
        let render_extent = render_images_info.base_render_extent;

        let pipeline_info = app_data.pipeline.as_ref().unwrap();

        let swapchain_info = app_data.swapchain.as_ref().unwrap();
        let image_count = swapchain_info.image_count;
        let swapchain_extent = swapchain_info.extent;

        debug!("Creating framebuffers for {} image views...", image_count);

        let base_render_framebuffers = (0..image_count)
            .map(|q| {
                let render_image_view = unsafe { render_images_info.base_render_images[q as usize].raw_image_view().unwrap() };
                let motion_vector_image_view = unsafe { render_images_info.motion_vector_buffers[q as usize].raw_image_view().unwrap() };
                let depth_stencil_image_view = unsafe { render_images_info.depth_stencil_buffers[q as usize].raw_image_view().unwrap() };
                let attachments = &[render_image_view, motion_vector_image_view, depth_stencil_image_view];
                let extent = render_extent;
                let framebuffer_info = vk::FramebufferCreateInfo::builder()
                    .render_pass(pipeline_info.base_render_pass)
                    .attachments(attachments)
                    .width(extent.width)
                    .height(extent.height)
                    .layers(1);

                unsafe {
                    device.create_framebuffer(&framebuffer_info, None)
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        let postprocessing_framebuffers = (0..image_count)
            .map(|q| {
                let swapchain_image_view = unsafe { swapchain_info.images[q as usize].raw_image_view().unwrap() };
                let attachments = &[swapchain_image_view];
                let extent = swapchain_extent;
                let framebuffer_info = vk::FramebufferCreateInfo::builder()
                    .render_pass(pipeline_info.postprocessing_render_pass)
                    .attachments(attachments)
                    .width(extent.width)
                    .height(extent.height)
                    .layers(1);

                unsafe {
                    device.create_framebuffer(&framebuffer_info, None)
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        debug!("Framebuffers created: {:?}; {:?}", base_render_framebuffers, postprocessing_framebuffers);
        framebuffer_info.base_render_framebuffers = base_render_framebuffers;
        framebuffer_info.postprocessing_framebuffers = postprocessing_framebuffers;

        Ok(())
    }

    fn destroy_framebuffers(&self, device: &Device, framebuffer_info: &mut FramebufferInfo) -> () {
        debug!("Destroying framebuffers...");

        unsafe {
            for framebuffer in framebuffer_info.base_render_framebuffers.iter() {
                device.destroy_framebuffer(*framebuffer, None);
            }
        }
        framebuffer_info.base_render_framebuffers.clear();

        unsafe {
            for framebuffer in framebuffer_info.postprocessing_framebuffers.iter() {
                device.destroy_framebuffer(*framebuffer, None);
            }
        }
        framebuffer_info.postprocessing_framebuffers.clear();
    }
}

impl BootstrapLoader for BootstrapFramebufferLoader {
    fn after_create_logical_device(&self, _inst: &Instance, device: &Device, _window: &Window, app_data: &mut AppData) -> Result<()> {
        let mut framebuffer_info = FramebufferInfo::default();
        self.create_framebuffers(device, &mut framebuffer_info, app_data)?;
        app_data.framebuffer = Some(framebuffer_info);

        Ok(())
    }

    fn before_destroy_logical_device(&self, _inst: &Instance, device: &Device, app_data: &mut AppData) -> () {
        if let Some(mut framebuffer_info) = app_data.framebuffer.take() {
            self.destroy_framebuffers(device, &mut framebuffer_info);
        }
    }
}
