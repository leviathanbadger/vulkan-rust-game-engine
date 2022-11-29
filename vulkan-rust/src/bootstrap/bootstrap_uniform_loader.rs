use super::{BootstrapLoader};

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
    pub descriptor_pool: vk::DescriptorPool
}

#[derive(Debug, Default)]
pub struct BootstrapUniformLoader { }

//Depends on swapchain
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

        let sampler_binding = vk::DescriptorSetLayoutBinding::builder()
            .binding(1)
            .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::ALL_GRAPHICS);

        let bindings = &[ubo_binding, sampler_binding];
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
            buffer.create(device, &app_data.memory_properties)?;
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

        let sampler_size = vk::DescriptorPoolSize::builder()
            .type_(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(max_sets);

        let pool_sizes = &[ubo_size, sampler_size];
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
}

impl BootstrapLoader for BootstrapUniformLoader {
    fn after_create_logical_device(&self, _inst: &Instance, device: &Device, _window: &Window, app_data: &mut AppData) -> Result<()> {
        let mut uniforms_info = UniformsInfo::default();
        self.create_descriptor_set_layout(device, &mut uniforms_info)?;
        self.create_uniform_buffers(device, &mut uniforms_info, app_data)?;
        self.create_descriptor_pool(device, &mut uniforms_info, app_data)?;
        app_data.uniforms = Some(uniforms_info);

        Ok(())
    }

    fn before_destroy_logical_device(&self, _inst: &Instance, device: &Device, app_data: &mut AppData) -> () {
        if let Some(mut uniforms_info) = app_data.uniforms.take() {
            self.destroy_descriptor_pool(device, &mut uniforms_info);
            self.destroy_uniform_buffers(device, &mut uniforms_info);
            self.destroy_descriptor_set_layout(device, &mut uniforms_info);
        }
    }

    fn recreate_swapchain(&self, inst: &Instance, device: &Device, window: &Window, app_data: &mut AppData, next: &dyn Fn(&Instance, &Device, &Window, &mut AppData) -> Result<()>) -> Result<()> {
        trace!("Recreating descriptor pool, and uniform buffers (but not descriptor set layout) in recreate_swapchain");

        let mut uniforms_info = app_data.uniforms.take().unwrap();

        self.destroy_descriptor_pool(device, &mut uniforms_info);
        self.destroy_uniform_buffers(device, &mut uniforms_info);
        next(inst, device, window, app_data)?;
        self.create_uniform_buffers(device, &mut uniforms_info, app_data)?;
        self.create_descriptor_pool(device, &mut uniforms_info, app_data)?;

        app_data.uniforms = Some(uniforms_info);

        Ok(())
    }
}
