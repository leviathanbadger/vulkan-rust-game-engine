use super::{BootstrapLoader};

use anyhow::{Result};
use winit::window::{Window};
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    app_data::{AppData}
};

#[derive(Debug, Default)]
pub struct FramebufferInfo {
    pub framebuffers: Vec<vk::Framebuffer>
}

#[derive(Debug, Default)]
pub struct BootstrapFramebufferLoader { }

impl BootstrapFramebufferLoader {
    pub fn new() -> Self {
        Self::default()
    }

    fn create_framebuffers(&self, device: &Device, app_data: &mut AppData) -> Result<()> {
        let swapchain_info = app_data.swapchain.as_ref().unwrap();
        let pipeline_info = app_data.pipeline.as_ref().unwrap();
        let image_count = swapchain_info.image_count;
        let swapchain_extent = swapchain_info.extent;

        debug!("Creating framebuffers for {} image views...", image_count);

        let depth_image_view = unsafe { app_data.depth_buffer.as_ref().unwrap().image.raw_image_view().unwrap() };

        let framebuffers = (0..image_count)
            .map(|q| {
                let swapchain_image_view = unsafe { swapchain_info.images[q as usize].raw_image_view().unwrap() };
                let attachments = &[swapchain_image_view, depth_image_view];
                let extent = swapchain_extent;
                let framebuffer_info = vk::FramebufferCreateInfo::builder()
                    .render_pass(pipeline_info.render_pass)
                    .attachments(attachments)
                    .width(extent.width)
                    .height(extent.height)
                    .layers(1);

                unsafe {
                    device.create_framebuffer(&framebuffer_info, None)
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        debug!("Framebuffers created: {:?}", framebuffers);
        app_data.framebuffer = Some(FramebufferInfo { framebuffers });

        Ok(())
    }

    fn destroy_framebuffers(&self, device: &Device, app_data: &mut AppData) -> () {
        if let Some(framebuffer_info) = app_data.framebuffer.take() {
            debug!("Destroying framebuffers...");
            unsafe {
                for framebuffer in framebuffer_info.framebuffers.iter() {
                    device.destroy_framebuffer(*framebuffer, None);
                }
            }
        }
    }
}

impl BootstrapLoader for BootstrapFramebufferLoader {
    fn after_create_logical_device(&self, _inst: &Instance, device: &Device, _window: &Window, app_data: &mut AppData) -> Result<()> {
        self.create_framebuffers(device, app_data)?;

        Ok(())
    }

    fn before_destroy_logical_device(&self, _inst: &Instance, device: &Device, app_data: &mut AppData) -> () {
        self.destroy_framebuffers(device, app_data);
    }
}
