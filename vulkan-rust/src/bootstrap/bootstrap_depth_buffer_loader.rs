use super::{BootstrapLoader, BootstrapSwapchainLoader, BootstrapCommandBufferLoader};

use anyhow::{Result};
use winit::window::{Window};
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    app_data::{AppData},
    buffer::{Image2D},
    bootstrap_loader
};

#[derive(Debug, Default)]
pub struct DepthBufferInfo {
    pub base_render_extent: vk::Extent2D,
    pub depth_stencil_buffers: Vec<Image2D>,
    pub base_render_images: Vec<Image2D>
}

impl DepthBufferInfo {
    pub fn depth_stencil_format(&self) -> vk::Format {
        self.depth_stencil_buffers[0].format().unwrap()
    }
}

bootstrap_loader! {
    pub struct BootstrapDepthBufferLoader {
        depends_on(BootstrapSwapchainLoader, BootstrapCommandBufferLoader);
    }
}

impl BootstrapDepthBufferLoader {
    fn create_depth_objects(&self, inst: &Instance, device: &Device, depth_buffer_info: &mut DepthBufferInfo, app_data: &AppData) -> Result<()> {
        debug!("Creating depth and stencil buffer images...");

        let swapchain_info = app_data.swapchain.as_ref().unwrap();
        let image_count = swapchain_info.image_count;
        let swapchain_extent = swapchain_info.extent;

        let command_pool_info = &app_data.command_pools.as_ref().unwrap();

        let depth_stencil_buffers = Image2D::new_and_create_depth_stencil_buffers(image_count, inst, device, app_data.physical_device.as_ref().unwrap(), &app_data.memory_properties, &swapchain_extent, command_pool_info)?;

        depth_buffer_info.base_render_extent = swapchain_extent;
        depth_buffer_info.depth_stencil_buffers = depth_stencil_buffers;

        Ok(())
    }

    fn destroy_depth_objects(&self, device: &Device, depth_buffer_info: &mut DepthBufferInfo) -> () {
        debug!("Destroying depth and stencil buffer images...");

        for depth_stencil_buffer in depth_buffer_info.depth_stencil_buffers.iter_mut() {
            depth_stencil_buffer.destroy(device);
        }
        depth_buffer_info.depth_stencil_buffers.clear();
    }

    fn create_render_images(&self, device: &Device, depth_buffer_info: &mut DepthBufferInfo, app_data: &AppData) -> Result<()> {
        debug!("Creating render images...");

        let swapchain_info = app_data.swapchain.as_ref().unwrap();
        let image_count = swapchain_info.image_count;
        let swapchain_format = swapchain_info.surface_format.format;
        let swapchain_extent = swapchain_info.extent;

        let command_pool_info = &app_data.command_pools.as_ref().unwrap();

        let render_images = Image2D::new_and_create_render_images(image_count, device, &app_data.memory_properties, swapchain_format, &swapchain_extent, true, command_pool_info)?;

        debug!("Render images created: {:?}", render_images);
        depth_buffer_info.base_render_images = render_images;

        Ok(())
    }

    fn destroy_render_images(&self, device: &Device, depth_buffer_info: &mut DepthBufferInfo) -> () {
        debug!("Destroying render images...");

        for render_image in depth_buffer_info.base_render_images.iter_mut() {
            render_image.destroy(device);
        }
        depth_buffer_info.base_render_images.clear();
    }
}

impl BootstrapLoader for BootstrapDepthBufferLoader {
    fn after_create_logical_device(&self, inst: &Instance, device: &Device, _window: &Window, app_data: &mut AppData) -> Result<()> {
        let mut depth_buffer_info = DepthBufferInfo::default();
        self.create_depth_objects(inst, device, &mut depth_buffer_info, app_data)?;
        self.create_render_images(device, &mut depth_buffer_info, app_data)?;
        app_data.depth_buffer = Some(depth_buffer_info);

        Ok(())
    }

    fn before_destroy_logical_device(&self, _inst: &Instance, device: &Device, app_data: &mut AppData) -> () {
        if let Some(mut depth_buffer_info) = app_data.depth_buffer.take() {
            self.destroy_render_images(device, &mut depth_buffer_info);
            self.destroy_depth_objects(device, &mut depth_buffer_info);
        }
    }
}
