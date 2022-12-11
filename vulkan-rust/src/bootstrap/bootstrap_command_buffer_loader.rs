use super::{BootstrapLoader, BootstrapSwapchainLoader};

use std::sync::{Arc};
use anyhow::{Result};
use winit::window::{Window};
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    app_data::{AppData, VulkanQueueInfo},
    bootstrap_loader
};

#[derive(Debug)]
pub struct CommandPoolsInfo {
    queue_info: Arc<VulkanQueueInfo>,

    pub command_pool: vk::CommandPool,

    pub transient_command_pool: vk::CommandPool,
    pub command_buffers: Vec<vk::CommandBuffer>
}

impl CommandPoolsInfo {
    pub fn new(queue_info: Arc<VulkanQueueInfo>) -> Self {
        Self {
            queue_info,

            command_pool: Default::default(),
            transient_command_pool: Default::default(),
            command_buffers: Default::default()
        }
    }

    fn submit_command_transient<TRet>(&self, device: &Device, command_pool: &vk::CommandPool, submit_queue: &vk::Queue, command: impl FnOnce(&vk::CommandBuffer) -> Result<TRet>) -> Result<TRet> {
        let cmd_buff_info = vk::CommandBufferAllocateInfo::builder()
            .level(vk::CommandBufferLevel::PRIMARY)
            .command_pool(*command_pool)
            .command_buffer_count(1);

        let command_buffer: vk::CommandBuffer;
        unsafe {
            command_buffer = device.allocate_command_buffers(&cmd_buff_info)?[0];
        }

        let begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);

        let retval: TRet;
        unsafe {
            device.begin_command_buffer(command_buffer, &begin_info)?;

            retval = command(&command_buffer)?;

            device.end_command_buffer(command_buffer)?;
        }

        let command_buffers = &[command_buffer];
        let submit_info = vk::SubmitInfo::builder()
            .command_buffers(command_buffers);

        unsafe {
            device.queue_submit(*submit_queue, &[submit_info], vk::Fence::null())?;

            device.queue_wait_idle(*submit_queue)?;

            device.free_command_buffers(*command_pool, command_buffers);
        }

        Ok(retval)
    }

    pub fn submit_command_transient_sync(&self, device: &Device, command: impl FnOnce(&vk::CommandBuffer) -> Result<()>) -> Result<()> {
        self.submit_command_transient(device, &self.transient_command_pool, &self.queue_info.graphics_queue, command)
    }

    fn submit_command_graphics(&self, device: &Device, command_buffer: &vk::CommandBuffer, submit_queue: &vk::Queue, wait_semaphores: &[vk::Semaphore], wait_dst_stage_mask: &[vk::PipelineStageFlags], signal_semaphores: &[vk::Semaphore], fence: &vk::Fence, command: impl FnOnce(&vk::CommandBuffer) -> Result<()>) -> Result<()> {
        unsafe {
            device.reset_command_buffer(*command_buffer, vk::CommandBufferResetFlags::empty())?;
        }

        let inheritance = vk::CommandBufferInheritanceInfo::builder();

        let begin_info = vk::CommandBufferBeginInfo::builder()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT)
            .inheritance_info(&inheritance);

        unsafe {
            device.begin_command_buffer(*command_buffer, &begin_info)?;

            command(command_buffer)?;

            device.end_command_buffer(*command_buffer)?;
        }

        let command_buffers = &[*command_buffer];
        let submit_info = vk::SubmitInfo::builder()
            .command_buffers(command_buffers)
            .wait_semaphores(wait_semaphores)
            .wait_dst_stage_mask(wait_dst_stage_mask)
            .signal_semaphores(signal_semaphores);

        unsafe {
            if !fence.is_null() {
                device.reset_fences(&[*fence])?;
            }
            device.queue_submit(*submit_queue, &[submit_info], *fence)?;
        }

        Ok(())
    }

    pub fn submit_command_async(&self, device: &Device, command_buffer: &vk::CommandBuffer, wait_semaphores: &[vk::Semaphore], wait_dst_stage_mask: &[vk::PipelineStageFlags], signal_semaphores: &[vk::Semaphore], fence: &vk::Fence, command: impl FnOnce(&vk::CommandBuffer) -> Result<()>) -> Result<()> {
        self.submit_command_graphics(device, command_buffer, &self.queue_info.graphics_queue, wait_semaphores, wait_dst_stage_mask, signal_semaphores, fence, command)
    }
}

bootstrap_loader! {
    pub struct BootstrapCommandBufferLoader {
        depends_on(BootstrapSwapchainLoader);
    }
}

impl BootstrapCommandBufferLoader {
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

        let queue_info = app_data.queue_info.as_ref().unwrap();

        let graphics_queue_family = queue_info.graphics_queue_family;
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
        let mut command_pools_info = CommandPoolsInfo::new(app_data.queue_info.as_ref().unwrap().clone());
        self.create_command_pools(device, &mut command_pools_info, app_data)?;
        self.create_command_buffers(device, &mut command_pools_info, app_data)?;
        app_data.command_pools = Some(command_pools_info);

        Ok(())
    }

    fn before_destroy_logical_device(&self, _inst: &Instance, device: &Device, app_data: &mut AppData) -> () {
        if let Some(mut command_pools_info) = app_data.command_pools.take() {
            self.destroy_command_buffers(device, &mut command_pools_info);
            self.destroy_command_pools(device, &mut command_pools_info);
        }
    }

    fn recreate_swapchain(&self, inst: &Instance, device: &Device, window: &Window, app_data: &mut AppData, next: &dyn Fn(&Instance, &Device, &Window, &mut AppData) -> Result<()>) -> Result<()> {
        trace!("Recreating command buffers (but not command pool) in recreate_swapchain");

        let mut command_pools_info = app_data.command_pools.take().unwrap();

        self.destroy_command_buffers(device, &mut command_pools_info);
        next(inst, device, window, app_data)?;
        self.create_command_buffers(device, &mut command_pools_info, app_data)?;

        app_data.command_pools = Some(command_pools_info);

        Ok(())
    }
}
