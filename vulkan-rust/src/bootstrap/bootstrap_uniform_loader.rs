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
        uniform_buffer_object::UniformBufferObject
    },
    buffer::{Buffer}
};

#[derive(Debug, Default)]
pub struct BootstrapUniformLoader { }

impl BootstrapUniformLoader {
    pub fn new() -> Self {
        Self::default()
    }

    fn create_descriptor_set_layout(&self, device: &Device, app_data: &mut AppData) -> Result<()> {
        let ubo_binding = vk::DescriptorSetLayoutBinding::builder()
            .binding(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::ALL_GRAPHICS);

        let bindings = &[ubo_binding];
        let dsl_info = vk::DescriptorSetLayoutCreateInfo::builder()
            .bindings(bindings);

        let dsl: vk::DescriptorSetLayout;
        unsafe {
            debug!("Creating descriptor set layout...");
            dsl = device.create_descriptor_set_layout(&dsl_info, None)?;
        }
        app_data.descriptor_set_layout = Some(dsl);

        Ok(())
    }

    fn destroy_descriptor_set_layout(&self, device: &Device, app_data: &mut AppData) -> () {
        if let Some(dsl) = app_data.descriptor_set_layout.take() {
            debug!("Destroying descriptor set layout...");
            unsafe {
                device.destroy_descriptor_set_layout(dsl, None);
            }
        }
    }

    fn create_uniform_buffers(&self, device: &Device, app_data: &mut AppData) -> Result<()> {
        debug!("Creating uniform buffers...");
        let mut uniform_buffers = app_data.swapchain_images.iter()
            .map(|_| {
                Buffer::<UniformBufferObject>::new(vk::BufferUsageFlags::UNIFORM_BUFFER, 1, false)
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
}

impl BootstrapLoader for BootstrapUniformLoader {
    fn after_create_logical_device(&self, _inst: &Instance, device: &Device, _window: &Window, app_data: &mut AppData) -> Result<()> {
        self.create_descriptor_set_layout(device, app_data)?;
        self.create_uniform_buffers(device, app_data)?;
        self.create_descriptor_pool(device, app_data)?;
        self.create_descriptor_sets(device, app_data)?;

        Ok(())
    }

    fn before_destroy_logical_device(&self, _inst: &Instance, device: &Device, app_data: &mut AppData) -> () {
        self.destroy_descriptor_pool(device, app_data);
        self.destroy_uniform_buffers(device, app_data);
        self.destroy_descriptor_set_layout(device, app_data);
    }

    fn recreate_swapchain(&self, inst: &Instance, device: &Device, window: &Window, app_data: &mut AppData, next: &dyn Fn(&Instance, &Device, &Window, &mut AppData) -> Result<()>) -> Result<()> {
        trace!("Recreating descriptor sets, descriptor pool, and uniform buffers (but not descriptor set layout) in recreate_swapchain");

        self.destroy_descriptor_pool(device, app_data);
        self.destroy_uniform_buffers(device, app_data);
        next(inst, device, window, app_data)?;
        self.create_uniform_buffers(device, app_data)?;
        self.create_descriptor_pool(device, app_data)?;
        self.create_descriptor_sets(device, app_data)?;

        Ok(())
    }
}
