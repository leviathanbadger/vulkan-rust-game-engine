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
pub struct UniformsInfo {
    pub descriptor_set_layout: vk::DescriptorSetLayout,
    pub uniform_buffers: Vec<Buffer::<UniformBufferObject>>,
    pub descriptor_pool: vk::DescriptorPool,
    pub descriptor_sets: Vec<vk::DescriptorSet>,
}

#[derive(Debug, Default)]
pub struct BootstrapUniformLoader { }

impl BootstrapUniformLoader {
    pub fn new() -> Self {
        Self::default()
    }

    fn create_descriptor_set_layout(&self, device: &Device, uniforms_info: &mut UniformsInfo) -> Result<()> {
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

        uniforms_info.descriptor_set_layout = dsl;

        Ok(())
    }

    fn destroy_descriptor_set_layout(&self, device: &Device, uniforms_info: &mut UniformsInfo) -> () {
        debug!("Destroying descriptor set layout...");
        unsafe {
            device.destroy_descriptor_set_layout(uniforms_info.descriptor_set_layout, None);
        }
        uniforms_info.descriptor_set_layout = vk::DescriptorSetLayout::null();
    }

    fn create_uniform_buffers(&self, device: &Device, uniforms_info: &mut UniformsInfo, app_data: &AppData) -> Result<()> {
        debug!("Creating uniform buffers...");
        let image_count = app_data.swapchain.as_ref().unwrap().image_count;
        let mut uniform_buffers = (0..image_count)
            .map(|_| {
                Buffer::<UniformBufferObject>::new(vk::BufferUsageFlags::UNIFORM_BUFFER, 1, false)
            })
            .collect::<Vec<_>>();

        for buffer in uniform_buffers.iter_mut() {
            buffer.create(device, app_data.memory_properties)?;
        }

        debug!("Uniform buffers created: {:?}", uniform_buffers);
        uniforms_info.uniform_buffers = uniform_buffers;

        Ok(())
    }

    fn destroy_uniform_buffers(&self, device: &Device, uniforms_info: &mut UniformsInfo) -> () {
        debug!("Destroying uniform buffers...");

        for uniform_buffer in uniforms_info.uniform_buffers.iter_mut() {
            uniform_buffer.destroy(device);
        }
        uniforms_info.uniform_buffers.clear();
    }

    fn create_descriptor_pool(&self, device: &Device, uniforms_info: &mut UniformsInfo, app_data: &AppData) -> Result<()> {
        let max_sets = app_data.swapchain.as_ref().unwrap().image_count;

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

        debug!("Descriptor pool created: {:?}", descriptor_pool);
        uniforms_info.descriptor_pool = descriptor_pool;

        Ok(())
    }

    fn destroy_descriptor_pool(&self, device: &Device, uniforms_info: &mut UniformsInfo) -> () {
        debug!("Destroying descriptor pool...");
        unsafe {
            device.destroy_descriptor_pool(uniforms_info.descriptor_pool, None);
        }
        uniforms_info.descriptor_pool = vk::DescriptorPool::null();
    }

    fn create_descriptor_sets(&self, device: &Device, uniforms_info: &mut UniformsInfo, app_data: &AppData) -> Result<()> {
        let image_count = app_data.swapchain.as_ref().unwrap().image_count as usize;

        let layouts = vec![uniforms_info.descriptor_set_layout; image_count];
        let desc_set_info = vk::DescriptorSetAllocateInfo::builder()
            .descriptor_pool(uniforms_info.descriptor_pool)
            .set_layouts(&layouts);

        let desc_sets: Vec<vk::DescriptorSet>;
        unsafe {
            debug!("Allocating descriptor sets...");
            desc_sets = device.allocate_descriptor_sets(&desc_set_info)?;
        }

        for (q, desc_set) in desc_sets.iter().enumerate() {
            let buffer = unsafe { uniforms_info.uniform_buffers[q].raw_buffer().unwrap() };
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

        debug!("Descriptor sets allocated: {:?}", desc_sets);
        uniforms_info.descriptor_sets = desc_sets;

        Ok(())
    }
}

impl BootstrapLoader for BootstrapUniformLoader {
    fn after_create_logical_device(&self, _inst: &Instance, device: &Device, _window: &Window, app_data: &mut AppData) -> Result<()> {
        let mut uniforms_info = UniformsInfo::default();
        self.create_descriptor_set_layout(device, &mut uniforms_info)?;
        self.create_uniform_buffers(device, &mut uniforms_info, app_data)?;
        self.create_descriptor_pool(device, &mut uniforms_info, app_data)?;
        self.create_descriptor_sets(device, &mut uniforms_info, app_data)?;
        app_data.uniforms = Some(uniforms_info);

        Ok(())
    }

    fn before_destroy_logical_device(&self, _inst: &Instance, device: &Device, app_data: &mut AppData) -> () {
        if let Some(mut uniforms_info) = app_data.uniforms.take() {
            uniforms_info.descriptor_sets.clear(); //No need to clean these up, apparently
            self.destroy_descriptor_pool(device, &mut uniforms_info);
            self.destroy_uniform_buffers(device, &mut uniforms_info);
            self.destroy_descriptor_set_layout(device, &mut uniforms_info);
        }
    }

    fn recreate_swapchain(&self, inst: &Instance, device: &Device, window: &Window, app_data: &mut AppData, next: &dyn Fn(&Instance, &Device, &Window, &mut AppData) -> Result<()>) -> Result<()> {
        trace!("Recreating descriptor sets, descriptor pool, and uniform buffers (but not descriptor set layout) in recreate_swapchain");

        let mut uniforms_info = app_data.uniforms.take().unwrap();

        uniforms_info.descriptor_sets.clear(); //No need to clean these up, apparently
        self.destroy_descriptor_pool(device, &mut uniforms_info);
        self.destroy_uniform_buffers(device, &mut uniforms_info);
        next(inst, device, window, app_data)?;
        self.create_uniform_buffers(device, &mut uniforms_info, app_data)?;
        self.create_descriptor_pool(device, &mut uniforms_info, app_data)?;
        self.create_descriptor_sets(device, &mut uniforms_info, app_data)?;

        app_data.uniforms = Some(uniforms_info);

        Ok(())
    }
}
