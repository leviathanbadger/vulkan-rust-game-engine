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
pub struct BootstrapFramebufferLoader { }

impl BootstrapFramebufferLoader {
    pub fn new() -> Self {
        Self::default()
    }

    fn create_framebuffers(&self, device: &Device, app_data: &mut AppData) -> Result<()> {
        debug!("Creating framebuffers for {} image views...", app_data.swapchain_image_views.len());

        let depth_image_view = unsafe { app_data.depth_buffer.unwrap().image.raw_image_view().unwrap() };

        let framebuffers = app_data.swapchain_image_views.iter()
            .map(|iv| {
                let attachments = &[*iv, depth_image_view];
                let extent = app_data.swapchain_extent.unwrap();
                let framebuffer_info = vk::FramebufferCreateInfo::builder()
                    .render_pass(app_data.render_pass.unwrap())
                    .attachments(attachments)
                    .width(extent.width)
                    .height(extent.height)
                    .layers(1);

                unsafe {
                    device.create_framebuffer(&framebuffer_info, None)
                }
            })
            .collect::<Result<Vec<_>, _>>()?;

        app_data.framebuffers = framebuffers;
        debug!("Framebuffers created: {:?}", app_data.framebuffers);

        Ok(())
    }

    fn destroy_framebuffers(&self, device: &Device, app_data: &mut AppData) -> () {
        debug!("Destroying framebuffers...");
        unsafe {
            for framebuffer in app_data.framebuffers.iter() {
                device.destroy_framebuffer(*framebuffer, None);
            }
        }
        app_data.framebuffers.clear();
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
