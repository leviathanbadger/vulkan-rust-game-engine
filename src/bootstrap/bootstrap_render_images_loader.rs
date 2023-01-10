use super::{BootstrapLoader, BootstrapSwapchainLoader, BootstrapCommandBufferLoader};

use anyhow::{Result};
use winit::window::{Window};
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    app_data::{AppData},
    resources::{Image2D},
    bootstrap_loader
};

#[derive(Debug, Default)]
pub struct RenderImagesInfo {
    pub base_render_extent: vk::Extent2D,

    pub base_render_images: Vec<Image2D>,
    pub depth_stencil_buffers: Vec<Image2D>,
    pub motion_vector_buffers: Vec<Image2D>
}

impl RenderImagesInfo {
    pub fn base_render_format(&self) -> vk::Format {
        self.base_render_images[0].format().unwrap()
    }

    pub fn depth_stencil_format(&self) -> vk::Format {
        self.depth_stencil_buffers[0].format().unwrap()
    }

    pub fn motion_vector_format(&self) -> vk::Format {
        self.motion_vector_buffers[0].format().unwrap()
    }
}

bootstrap_loader! {
    pub struct BootstrapRenderImagesLoader {
        depends_on(BootstrapSwapchainLoader, BootstrapCommandBufferLoader);
    }
}

impl BootstrapRenderImagesLoader {
    fn select_base_render_extent(&self, render_images_info: &mut RenderImagesInfo, app_data: &AppData) -> Result<()> {
        let swapchain_info = app_data.swapchain.as_ref().unwrap();
        let swapchain_extent = swapchain_info.extent;

        let base_render_extent = vk::Extent2D {
            width: swapchain_extent.width / 2,
            height: swapchain_extent.height / 2
        };

        render_images_info.base_render_extent = base_render_extent;

        Ok(())
    }

    fn create_depth_objects(&self, inst: &Instance, device: &Device, render_images_info: &mut RenderImagesInfo, app_data: &AppData) -> Result<()> {
        debug!("Creating depth and stencil buffer images...");

        let swapchain_info = app_data.swapchain.as_ref().unwrap();
        let image_count = swapchain_info.image_count;

        let command_pool_info = &app_data.command_pools.as_ref().unwrap();

        let depth_stencil_buffers = Image2D::new_and_create_depth_stencil_buffers(image_count, inst, device, app_data.physical_device.as_ref().unwrap(), &app_data.memory_properties, &render_images_info.base_render_extent, false, command_pool_info)?;

        render_images_info.depth_stencil_buffers = depth_stencil_buffers;

        Ok(())
    }

    fn destroy_depth_objects(&self, device: &Device, render_images_info: &mut RenderImagesInfo) -> () {
        debug!("Destroying depth and stencil buffer images...");

        for depth_stencil_buffer in render_images_info.depth_stencil_buffers.iter_mut() {
            depth_stencil_buffer.destroy(device);
        }
        render_images_info.depth_stencil_buffers.clear();
    }

    fn create_render_images(&self, inst: &Instance, device: &Device, render_images_info: &mut RenderImagesInfo, app_data: &AppData) -> Result<()> {
        debug!("Creating render images...");

        let swapchain_info = app_data.swapchain.as_ref().unwrap();
        let image_count = swapchain_info.image_count;

        let command_pool_info = &app_data.command_pools.as_ref().unwrap();

        let base_render_images = Image2D::new_and_create_render_images(image_count, inst, device, app_data.physical_device.as_ref().unwrap(), &app_data.memory_properties, &render_images_info.base_render_extent, true, command_pool_info)?;

        debug!("Render images created: {:?}", base_render_images);
        render_images_info.base_render_images = base_render_images;

        Ok(())
    }

    fn destroy_render_images(&self, device: &Device, render_images_info: &mut RenderImagesInfo) -> () {
        debug!("Destroying render images...");

        for render_image in render_images_info.base_render_images.iter_mut() {
            render_image.destroy(device);
        }
        render_images_info.base_render_images.clear();
    }

    fn create_motion_vector_buffers(&self, inst: &Instance, device: &Device, render_images_info: &mut RenderImagesInfo, app_data: &AppData) -> Result<()> {
        debug!("Creating motion vector buffers...");

        let swapchain_info = app_data.swapchain.as_ref().unwrap();
        let image_count = swapchain_info.image_count;

        let command_pool_info = &app_data.command_pools.as_ref().unwrap();

        let motion_vector_buffers = Image2D::new_and_create_motion_vector_buffers(image_count, inst, device, app_data.physical_device.as_ref().unwrap(), &app_data.memory_properties, &render_images_info.base_render_extent, true, command_pool_info)?;

        debug!("Motion vector buffers created: {:?}", motion_vector_buffers);
        render_images_info.motion_vector_buffers = motion_vector_buffers;

        Ok(())
    }

    fn destroy_motion_vector_buffers(&self, device: &Device, render_images_info: &mut RenderImagesInfo) -> () {
        debug!("Destroying motion vector buffers...");

        for motion_vector_buffer in render_images_info.motion_vector_buffers.iter_mut() {
            motion_vector_buffer.destroy(device);
        }
        render_images_info.motion_vector_buffers.clear();
    }
}

impl BootstrapLoader for BootstrapRenderImagesLoader {
    fn after_create_logical_device(&self, inst: &Instance, device: &Device, _window: &Window, app_data: &mut AppData) -> Result<()> {
        let mut render_images_info = RenderImagesInfo::default();
        self.select_base_render_extent(&mut render_images_info, app_data);
        self.create_depth_objects(inst, device, &mut render_images_info, app_data)?;
        self.create_render_images(inst, device, &mut render_images_info, app_data)?;
        self.create_motion_vector_buffers(inst, device, &mut render_images_info, app_data)?;
        app_data.render_images = Some(render_images_info);

        Ok(())
    }

    fn before_destroy_logical_device(&self, _inst: &Instance, device: &Device, app_data: &mut AppData) -> () {
        if let Some(mut render_images_info) = app_data.render_images.take() {
            self.destroy_motion_vector_buffers(device, &mut render_images_info);
            self.destroy_render_images(device, &mut render_images_info);
            self.destroy_depth_objects(device, &mut render_images_info);
        }
    }
}
