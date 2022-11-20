use super::{BootstrapLoader};
use anyhow::{Result};
use winit::window::{Window};
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    app_data::{AppData},
    shader_input::{
        simple::{Vertex, VERTICES}
    },
    buffer::{Buffer}
};

#[derive(Debug, Default)]
pub struct BootstrapCommandBufferLoader { }

impl BootstrapCommandBufferLoader {
    pub fn new() -> Self {
        Self::default()
    }

    fn create_command_pool(&self, device: &Device, flags: vk::CommandPoolCreateFlags, queue_family: u32) -> Result<vk::CommandPool> {
        let command_pool_info = vk::CommandPoolCreateInfo::builder()
            .flags(flags)
            .queue_family_index(queue_family);

        unsafe {
            let command_pool = device.create_command_pool(&command_pool_info, None)?;

            Ok(command_pool)
        }
    }

    fn create_command_pools(&self, device: &Device, app_data: &mut AppData) -> Result<()> {
        debug!("Creating command pools...");

        let graphics_queue_family = app_data.graphics_queue_family.unwrap();
        app_data.command_pool = Some(self.create_command_pool(device, vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER, graphics_queue_family)?);

        let transfer_queue_family = graphics_queue_family; //TODO MAYBE: use separate queue for transient operations (memory transfer for example)
        app_data.transient_command_pool = Some(self.create_command_pool(device, vk::CommandPoolCreateFlags::TRANSIENT, transfer_queue_family)?);

        debug!("Command pools created. Standard: {:?}, transient: {:?}", app_data.command_pool.unwrap(), app_data.transient_command_pool.unwrap());

        Ok(())
    }

    fn destroy_command_pools(&self, device: &Device, app_data: &mut AppData) -> () {
        debug!("Destroying command pools...");

        if let Some(command_pool) = app_data.command_pool.take() {
            unsafe {
                device.destroy_command_pool(command_pool, None);
            }
        }

        if let Some(command_pool) = app_data.transient_command_pool.take() {
            unsafe {
                device.destroy_command_pool(command_pool, None);
            }
        }
    }

    fn create_vertex_buffers(&self, device: &Device, app_data: &mut AppData) -> Result<()> {
        debug!("Creating vertex buffer...");
        let mut buffer = Buffer::<Vertex>::new(vk::BufferUsageFlags::VERTEX_BUFFER, VERTICES.len(), true);

        buffer.create(device, app_data.memory_properties)?;
        buffer.set_data(device, &*VERTICES)?;
        buffer.submit(device, &app_data.transient_command_pool.unwrap(), &app_data.graphics_queue.unwrap())?;

        app_data.vertex_buffer = Some(buffer);

        Ok(())
    }

    fn destroy_vertex_buffers(&self, device: &Device, app_data: &mut AppData) -> () {
        debug!("Destroying vertex buffer...");

        if let Some(mut vertex_buffer) = app_data.vertex_buffer.take() {
            vertex_buffer.destroy(device);
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
        self.create_command_pools(device, app_data)?;
        self.create_vertex_buffers(device, app_data)?;
        self.create_command_buffers(device, app_data)?;

        Ok(())
    }

    fn before_destroy_logical_device(&self, _inst: &Instance, device: &Device, app_data: &mut AppData) -> () {
        self.destroy_command_buffers(device, app_data);
        self.destroy_vertex_buffers(device, app_data);
        self.destroy_command_pools(device, app_data);
    }

    fn recreate_swapchain(&self, inst: &Instance, device: &Device, window: &Window, app_data: &mut AppData, next: &dyn Fn(&Instance, &Device, &Window, &mut AppData) -> Result<()>) -> Result<()> {
        trace!("Recreating command buffers (but not command pool or vertex buffers) in recreate_swapchain");

        self.destroy_command_buffers(device, app_data);
        next(inst, device, window, app_data)?;
        self.create_command_buffers(device, app_data)?;

        Ok(())
    }
}
