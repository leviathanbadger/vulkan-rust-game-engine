use super::{BootstrapLoader};
use std::{
    mem::{size_of}
};
use anyhow::{Result};
use winit::window::{Window};
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    app_data::{AppData},
    shader_input::{
        simple::{Vertex, VERTICES},
        uniform_buffer_object::UniformBufferObject
    },
    buffer::{Buffer}
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

    fn create_vertex_buffers(&self, device: &Device, app_data: &mut AppData) -> Result<()> {
        debug!("Creating vertex buffer...");
        let mut buffer = Buffer::<Vertex>::new(vk::BufferUsageFlags::VERTEX_BUFFER, VERTICES.len());

        buffer.create(device, app_data.memory_properties)?;
        buffer.set_data(device, &*VERTICES)?;

        app_data.vertex_buffer = Some(buffer);

        Ok(())
    }

    fn destroy_vertex_buffers(&self, device: &Device, app_data: &mut AppData) -> () {
        debug!("Destroying vertex buffer...");

        if let Some(mut vertex_buffer) = app_data.vertex_buffer.take() {
            vertex_buffer.destroy(device);
        }
    }

    fn create_uniform_buffers(&self, device: &Device, app_data: &mut AppData) -> Result<()> {
        debug!("Creating uniform buffers...");
        let mut uniform_buffers = app_data.swapchain_images.iter()
            .map(|_| {
                Buffer::<UniformBufferObject>::new(vk::BufferUsageFlags::UNIFORM_BUFFER, 1)
            })
            .collect::<Vec<_>>();

        for buffer in uniform_buffers.iter_mut() {
            buffer.create(device, app_data.memory_properties)?;
        }

        app_data.uniform_buffers = uniform_buffers;
        debug!("Uniform buffers created: {:?}", app_data.uniform_buffers);

        Ok(())
    }

    fn destroy_uniform_buffers(&self, device: &Device, app_data: &mut AppData) -> () {
        debug!("Destroying uniform buffers...");

        for uniform_buffer in app_data.uniform_buffers.iter_mut() {
            uniform_buffer.destroy(device);
        }
        app_data.uniform_buffers.clear();
    }

    fn create_descriptor_pool(&self, device: &Device, app_data: &mut AppData) -> Result<()> {
        let max_sets = app_data.uniform_buffers.len() as u32;

        let ubo_size = vk::DescriptorPoolSize::builder()
            .type_(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(max_sets);

        let pool_sizes = &[ubo_size];
        let desc_pool_info = vk::DescriptorPoolCreateInfo::builder()
            .pool_sizes(pool_sizes)
            .max_sets(max_sets);

        let descriptor_pool: vk::DescriptorPool;
        unsafe {
            debug!("Creating descriptor pool...");
            descriptor_pool = device.create_descriptor_pool(&desc_pool_info, None)?;
        }

        app_data.descriptor_pool = Some(descriptor_pool);
        debug!("Descriptor pool created: {:?}", app_data.descriptor_pool);

        Ok(())
    }

    fn destroy_descriptor_pool(&self, device: &Device, app_data: &mut AppData) -> () {
        if let Some(descriptor_pool) = app_data.descriptor_pool.take() {
            debug!("Destroying descriptor pool...");
            unsafe {
                device.destroy_descriptor_pool(descriptor_pool, None);
            }
        }
    }

    fn create_descriptor_sets(&self, device: &Device, app_data: &mut AppData) -> Result<()> {
        let desc_pool = app_data.descriptor_pool.unwrap();
        let desc_set_layout = app_data.descriptor_set_layout.unwrap();

        let layouts = vec![desc_set_layout; app_data.swapchain_images.len()];
        let desc_set_info = vk::DescriptorSetAllocateInfo::builder()
            .descriptor_pool(desc_pool)
            .set_layouts(&layouts);

        let desc_sets: Vec<vk::DescriptorSet>;
        unsafe {
            debug!("Allocating descriptor sets...");
            desc_sets = device.allocate_descriptor_sets(&desc_set_info)?;
        }

        app_data.descriptor_sets = desc_sets;
        debug!("Descriptor sets allocated: {:?}", app_data.descriptor_sets);

        for (q, desc_set) in app_data.descriptor_sets.iter().enumerate() {
            let buffer = unsafe { app_data.uniform_buffers[q].raw_buffer().unwrap() };
            let info = vk::DescriptorBufferInfo::builder()
                .buffer(buffer)
                .offset(0)
                .range(size_of::<UniformBufferObject>() as u64);

            let buffer_info = &[info];
            let ubo_write = vk::WriteDescriptorSet::builder()
                .dst_set(*desc_set)
                .dst_binding(0)
                .dst_array_element(0)
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                .buffer_info(buffer_info);

            unsafe {
                device.update_descriptor_sets(&[ubo_write], &[] as &[vk::CopyDescriptorSet]);
            }
        }

        Ok(())
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
                    float32: [0.0, 0.0, 0.0, 1.0]
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

                let vertex_buffer = app_data.vertex_buffer.unwrap().raw_buffer().unwrap();
                device.cmd_bind_vertex_buffers(*command_buffer, 0, &[vertex_buffer], &[0]);
                device.cmd_bind_descriptor_sets(*command_buffer, vk::PipelineBindPoint::GRAPHICS, app_data.pipeline_layout.unwrap(), 0, &[app_data.descriptor_sets[q]], &[]);
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
        self.create_uniform_buffers(device, app_data)?;
        self.create_descriptor_pool(device, app_data)?;
        self.create_descriptor_sets(device, app_data)?;
        self.create_command_buffers(device, app_data)?;

        Ok(())
    }

    fn before_destroy_logical_device(&self, _inst: &Instance, device: &Device, app_data: &mut AppData) -> () {
        self.destroy_command_buffers(device, app_data);
        self.destroy_descriptor_pool(device, app_data);
        self.destroy_uniform_buffers(device, app_data);
        self.destroy_vertex_buffers(device, app_data);
        self.destroy_command_pool(device, app_data);
    }

    fn recreate_swapchain(&self, inst: &Instance, device: &Device, window: &Window, app_data: &mut AppData, next: &dyn Fn(&Instance, &Device, &Window, &mut AppData) -> Result<()>) -> Result<()> {
        trace!("Recreating command buffers, descriptor pool, and uniform buffers (but not command pool or vertex buffers) in recreate_swapchain");

        self.destroy_command_buffers(device, app_data);
        self.destroy_descriptor_pool(device, app_data);
        self.destroy_uniform_buffers(device, app_data);
        next(inst, device, window, app_data)?;
        self.create_uniform_buffers(device, app_data)?;
        self.create_descriptor_pool(device, app_data)?;
        self.create_descriptor_sets(device, app_data)?;
        self.create_command_buffers(device, app_data)?;

        Ok(())
    }
}
