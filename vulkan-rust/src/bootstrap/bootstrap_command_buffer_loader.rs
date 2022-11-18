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
pub struct BootstrapCommandBufferLoader { }

impl BootstrapCommandBufferLoader {
    pub fn new() -> Self {
        Self::default()
    }

    fn create_command_pool(&self, device: &Device, app_data: &mut AppData) -> Result<()> {
        let command_pool_info = vk::CommandPoolCreateInfo::builder()
            .flags(vk::CommandPoolCreateFlags::empty())
            .queue_family_index(app_data.graphics_queue_family.unwrap());

        let command_pool: vk::CommandPool;
        unsafe {
            debug!("Creating command pool...");
            command_pool = device.create_command_pool(&command_pool_info, None)?;
            debug!("Command pool created: {:?}", command_pool);
        }
        app_data.command_pool = Some(command_pool);

        Ok(())
    }

    fn destroy_command_pool(&self, device: &Device, app_data: &mut AppData) -> () {
        if let Some(command_pool) = app_data.command_pool.take() {
            debug!("Destroying command pool...");
            unsafe {
                device.destroy_command_pool(command_pool, None);
            }
        }
    }

    fn create_command_buffers(&self, device: &Device, app_data: &mut AppData) -> Result<()> {
        let command_pool = app_data.command_pool.unwrap();
        let count = app_data.framebuffers.len();
        let command_buffer_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(count as u32);

        let command_buffers: Vec<vk::CommandBuffer>;
        unsafe {
            debug!("Creating command buffers for {} framebuffers...", count);
            command_buffers = device.allocate_command_buffers(&command_buffer_info)?;
            debug!("Command buffers created: {:?}", command_buffers);
        }
        app_data.command_buffers = command_buffers;

        let extent = app_data.swapchain_extent.unwrap();
        for (q, command_buffer) in app_data.command_buffers.iter().enumerate() {
            let inheritance = vk::CommandBufferInheritanceInfo::builder();

            let begin_info = vk::CommandBufferBeginInfo::builder()
                .flags(vk::CommandBufferUsageFlags::empty())
                .inheritance_info(&inheritance);

            unsafe {
                device.begin_command_buffer(*command_buffer, &begin_info)?;
            }

            let render_area = vk::Rect2D::builder()
                .offset(vk::Offset2D::default())
                .extent(extent);

            let color_clear_value = vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [0.0, 0.0, 1.0, 1.0]
                }
            };

            let clear_values = &[color_clear_value];
            let render_pass_info = vk::RenderPassBeginInfo::builder()
                .render_pass(app_data.render_pass.unwrap())
                .framebuffer(app_data.framebuffers[q])
                .render_area(render_area)
                .clear_values(clear_values);

            unsafe {
                device.cmd_begin_render_pass(*command_buffer, &render_pass_info, vk::SubpassContents::INLINE);

                let pipeline = app_data.pipeline.unwrap();
                device.cmd_bind_pipeline(*command_buffer, vk::PipelineBindPoint::GRAPHICS, pipeline);

                device.cmd_draw(*command_buffer, 3, 1, 0, 0);

                device.cmd_end_render_pass(*command_buffer);

                device.end_command_buffer(*command_buffer)?;
            }
        }

        Ok(())
    }

    fn destroy_command_buffers(&self, device: &Device, app_data: &mut AppData) -> () {
        if let Some(command_pool) = app_data.command_pool {
            debug!("Destroying command buffers...");
            let command_buffers = &app_data.command_buffers;
            unsafe {
                device.free_command_buffers(command_pool, &command_buffers[..]);
            }
            app_data.command_buffers.clear();
        }
    }
}

impl BootstrapLoader for BootstrapCommandBufferLoader {
    fn after_create_logical_device(&self, _inst: &Instance, device: &Device, _window: &Window, app_data: &mut AppData) -> Result<()> {
        self.create_command_pool(device, app_data)?;
        self.create_command_buffers(device, app_data)?;

        Ok(())
    }

    fn before_destroy_logical_device(&self, _inst: &Instance, device: &Device, app_data: &mut AppData) -> () {
        self.destroy_command_buffers(device, app_data);
        self.destroy_command_pool(device, app_data);
    }
}
