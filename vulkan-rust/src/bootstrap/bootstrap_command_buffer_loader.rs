use std::{
    mem::{size_of},
    ptr::{copy_nonoverlapping as memcpy}
};

use super::{BootstrapLoader};

use anyhow::{anyhow, Result};
use winit::window::{Window};
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    app_data::{AppData},
    shader_input::static_screen_space::{Vertex, VERTICES}
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

    fn get_memory_type_index(&self, app_data: &AppData, properties: vk::MemoryPropertyFlags, requirements: vk::MemoryRequirements) -> Result<u32> {
        let memory = app_data.memory_properties;
        (0..memory.memory_type_count)
            .find(|i| {
                let is_suitable = (requirements.memory_type_bits & (1 << i)) != 0;
                let memory_type = memory.memory_types[*i as usize];
                is_suitable && memory_type.property_flags.contains(properties)
            })
            .ok_or_else(|| anyhow!("Failed to find suitable memory type for buffer"))
    }

    fn create_vertex_buffers(&self, device: &Device, app_data: &mut AppData) -> Result<()> {
        let buffer_info = vk::BufferCreateInfo::builder()
            .size((size_of::<Vertex>() * VERTICES.len()) as u64)
            .usage(vk::BufferUsageFlags::VERTEX_BUFFER)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let vertex_buffer: vk::Buffer;
        let requirements: vk::MemoryRequirements;
        unsafe {
            debug!("Creating vertex buffer...");
            vertex_buffer = device.create_buffer(&buffer_info, None)?;
            requirements = device.get_buffer_memory_requirements(vertex_buffer);
        }
        app_data.vertex_buffer = Some(vertex_buffer);

        let memory_type_index = self.get_memory_type_index(app_data, vk::MemoryPropertyFlags::HOST_COHERENT | vk::MemoryPropertyFlags::HOST_VISIBLE, requirements)?;
        let memory_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(requirements.size)
            .memory_type_index(memory_type_index);

        let vertex_buffer_memory: vk::DeviceMemory;
        unsafe {
            vertex_buffer_memory = device.allocate_memory(&memory_info, None)?;
            device.bind_buffer_memory(vertex_buffer, vertex_buffer_memory, 0)?;
        }
        app_data.vertex_buffer_memory = Some(vertex_buffer_memory);

        unsafe {
            let memory = device.map_memory(vertex_buffer_memory, 0, buffer_info.size, vk::MemoryMapFlags::empty())?;
            memcpy(VERTICES.as_ptr(), memory.cast(), VERTICES.len());
            device.unmap_memory(vertex_buffer_memory);
        }

        Ok(())
    }

    fn destroy_vertex_buffers(&self, device: &Device, app_data: &mut AppData) -> () {
        debug!("Destroying vertex buffer...");

        if let Some(vertex_buffer) = app_data.vertex_buffer.take() {
            unsafe {
                device.destroy_buffer(vertex_buffer, None);
            }
        }

        if let Some(vertex_buffer_memory) = app_data.vertex_buffer_memory.take() {
            unsafe {
                device.free_memory(vertex_buffer_memory, None);
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

                let vertex_buffer = app_data.vertex_buffer.unwrap();
                device.cmd_bind_vertex_buffers(*command_buffer, 0, &[vertex_buffer], &[0]);
                device.cmd_draw(*command_buffer, VERTICES.len() as u32, 1, 0, 0);

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
        self.create_vertex_buffers(device, app_data)?;
        self.create_command_buffers(device, app_data)?;

        Ok(())
    }

    fn before_destroy_logical_device(&self, _inst: &Instance, device: &Device, app_data: &mut AppData) -> () {
        self.destroy_command_buffers(device, app_data);
        self.destroy_vertex_buffers(device, app_data);
        self.destroy_command_pool(device, app_data);
    }

    fn recreate_swapchain(&self, inst: &Instance, device: &Device, window: &Window, app_data: &mut AppData, next: &dyn Fn(&Instance, &Device, &Window, &mut AppData) -> Result<()>) -> Result<()> {
        trace!("Recreating command buffers (but not command pool) in recreate_swapchain");

        self.destroy_command_buffers(device, app_data);
        next(inst, device, window, app_data)?;
        self.create_command_buffers(device, app_data)?;

        Ok(())
    }
}
