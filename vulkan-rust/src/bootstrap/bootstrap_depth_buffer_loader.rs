use super::{BootstrapLoader};

use anyhow::{Result};
use winit::window::{Window};
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    app_data::{AppData},
    buffer::{Image2D}
};

#[derive(Debug, Copy, Clone, Default)]
pub struct DepthBufferInfo {
    pub image: Image2D
}

impl DepthBufferInfo {
    pub fn format(&self) -> vk::Format {
        self.image.format().unwrap()
    }
}

#[derive(Debug, Default)]
pub struct BootstrapDepthBufferLoader { }

impl BootstrapDepthBufferLoader {
    pub fn new() -> Self {
        Self::default()
    }

    fn create_depth_objects(&self, inst: &Instance, device: &Device, app_data: &mut AppData) -> Result<()> {
        debug!("Creating depth and stencil buffer objects...");

        let mut image = Image2D::new();
        image.create_depth_stencil_buffer(inst, device, app_data)?;

        let mut depth_buffer_info = DepthBufferInfo::default();
        depth_buffer_info.image = image;
        app_data.depth_buffer = Some(depth_buffer_info);

        Ok(())
    }

    fn destroy_depth_objects(&self, device: &Device, app_data: &mut AppData) -> () {
        debug!("Destroying depth and stencil buffer objects...");
        if let Some(mut depth_buffer_info) = app_data.depth_buffer.take() {
            depth_buffer_info.image.destroy(device);
        }
    }
}

impl BootstrapLoader for BootstrapDepthBufferLoader {
    fn after_create_logical_device(&self, inst: &Instance, device: &Device, _window: &Window, app_data: &mut AppData) -> Result<()> {
        self.create_depth_objects(inst, device, app_data)?;

        Ok(())
    }

    fn before_destroy_logical_device(&self, _inst: &Instance, device: &Device, app_data: &mut AppData) -> () {
        self.destroy_depth_objects(device, app_data);
    }
}
