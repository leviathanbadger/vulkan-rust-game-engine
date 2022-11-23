use super::{BootstrapLoader};
use anyhow::{Result};
use winit::window::{Window};
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    app_data::{AppData},
    shader_input::{
        simple::{Vertex, CUBE_VERTICES, CUBE_INDICES}
    },
    buffer::{Model}
};

#[derive(Debug, Default)]
pub struct CommandPoolsInfo {
    pub command_pool: vk::CommandPool,
    pub cube_model: Option<Model<Vertex>>,

    pub transient_command_pool: vk::CommandPool,
    pub command_buffers: Vec<vk::CommandBuffer>
}

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

    fn create_command_pools(&self, device: &Device, command_pools_info: &mut CommandPoolsInfo, app_data: &AppData) -> Result<()> {
        debug!("Creating command pools...");

        let graphics_queue_family = app_data.graphics_queue_family.unwrap();
        command_pools_info.command_pool = self.create_command_pool(device, vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER, graphics_queue_family)?;

        let transfer_queue_family = graphics_queue_family; //TODO MAYBE: use separate queue for transient operations (memory transfer for example)
        command_pools_info.transient_command_pool = self.create_command_pool(device, vk::CommandPoolCreateFlags::TRANSIENT, transfer_queue_family)?;

        debug!("Command pools created. Standard: {:?}, transient: {:?}", command_pools_info.command_pool, command_pools_info.transient_command_pool);

        Ok(())
    }

    fn destroy_command_pools(&self, device: &Device, command_pools_info: &mut CommandPoolsInfo) -> () {
        debug!("Destroying command pools...");

        unsafe {
            device.destroy_command_pool(command_pools_info.command_pool, None);
        }
        command_pools_info.command_pool = vk::CommandPool::null();

        unsafe {
            device.destroy_command_pool(command_pools_info.transient_command_pool, None);
        }
        command_pools_info.transient_command_pool = vk::CommandPool::null();
    }

    fn create_cube_model(&self, device: &Device, command_pools_info: &mut CommandPoolsInfo, app_data: &AppData) -> Result<()> {
        debug!("Creating cube model...");
        let mut model = Model::<Vertex>::new(CUBE_VERTICES.len(), CUBE_INDICES.len(), true)?;

        model.create(device, app_data.memory_properties)?;
        model.set_data(device, &*CUBE_VERTICES, &*CUBE_INDICES)?;
        model.submit(device, &command_pools_info.transient_command_pool, &app_data.graphics_queue.unwrap())?;

        command_pools_info.cube_model = Some(model);

        Ok(())
    }

    fn destroy_cube_model(&self, device: &Device, command_pools_info: &mut CommandPoolsInfo) -> () {
        debug!("Destroying cube model...");

        if let Some(mut cube_model) = command_pools_info.cube_model.take() {
            cube_model.destroy(device);
        }
    }

    fn create_command_buffers(&self, device: &Device, command_pools_info: &mut CommandPoolsInfo, app_data: &AppData) -> Result<()> {
        let command_pool = command_pools_info.command_pool;
        let count = app_data.swapchain.as_ref().unwrap().image_count;
        let command_buffer_info = vk::CommandBufferAllocateInfo::builder()
            .command_pool(command_pool)
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_buffer_count(count);

        let command_buffers: Vec<vk::CommandBuffer>;
        unsafe {
            debug!("Creating command buffers for {} framebuffers...", count);
            command_buffers = device.allocate_command_buffers(&command_buffer_info)?;
            debug!("Command buffers created: {:?}", command_buffers);
        }
        command_pools_info.command_buffers = command_buffers;

        Ok(())
    }

    fn destroy_command_buffers(&self, device: &Device, command_pools_info: &mut CommandPoolsInfo) -> () {
        debug!("Destroying command buffers...");
        let command_buffers = &command_pools_info.command_buffers;
        unsafe {
            device.free_command_buffers(command_pools_info.command_pool, &command_buffers[..]);
        }
        command_pools_info.command_buffers.clear();
    }
}

impl BootstrapLoader for BootstrapCommandBufferLoader {
    fn after_create_logical_device(&self, _inst: &Instance, device: &Device, _window: &Window, app_data: &mut AppData) -> Result<()> {
        let mut command_pools_info = CommandPoolsInfo::default();
        self.create_command_pools(device, &mut command_pools_info, app_data)?;
        self.create_cube_model(device, &mut command_pools_info, app_data)?;
        self.create_command_buffers(device, &mut command_pools_info, app_data)?;
        app_data.command_pools = Some(command_pools_info);

        Ok(())
    }

    fn before_destroy_logical_device(&self, _inst: &Instance, device: &Device, app_data: &mut AppData) -> () {

        if let Some(mut command_pools_info) = app_data.command_pools.take() {
            self.destroy_command_buffers(device, &mut command_pools_info);
            self.destroy_cube_model(device, &mut command_pools_info);
            self.destroy_command_pools(device, &mut command_pools_info);
        }
    }

    fn recreate_swapchain(&self, inst: &Instance, device: &Device, window: &Window, app_data: &mut AppData, next: &dyn Fn(&Instance, &Device, &Window, &mut AppData) -> Result<()>) -> Result<()> {
        trace!("Recreating command buffers (but not command pool or cube model) in recreate_swapchain");

        let mut command_pools_info = app_data.command_pools.take().unwrap();

        self.destroy_command_buffers(device, &mut command_pools_info);
        next(inst, device, window, app_data)?;
        self.create_command_buffers(device, &mut command_pools_info, app_data)?;

        app_data.command_pools = Some(command_pools_info);

        Ok(())
    }
}
