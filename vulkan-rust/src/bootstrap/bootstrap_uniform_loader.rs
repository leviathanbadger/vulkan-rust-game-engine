use super::{BootstrapLoader, BootstrapSwapchainLoader};

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
    resources::{Buffer},
    bootstrap_loader
};

#[derive(Debug, Default)]
pub struct UniformsInfo {
    pub base_descriptor_set_layout: vk::DescriptorSetLayout,
    pub postprocessing_descriptor_set_layout: vk::DescriptorSetLayout,
    pub uniform_buffers: Vec<Buffer::<UniformBufferObject>>,
    pub base_descriptor_pool: vk::DescriptorPool,
    pub postprocessing_descriptor_pool: vk::DescriptorPool
}

bootstrap_loader! {
    pub struct BootstrapUniformLoader {
        depends_on(BootstrapSwapchainLoader);
    }
}

impl BootstrapUniformLoader {
    fn create_base_descriptor_set_layout(&self, device: &Device) -> Result<vk::DescriptorSetLayout> {
        let ubo_binding = vk::DescriptorSetLayoutBinding::builder()
            .binding(0)
            .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::ALL_GRAPHICS);

        let sampler_binding = vk::DescriptorSetLayoutBinding::builder()
            .binding(1)
            .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(3)
            .stage_flags(vk::ShaderStageFlags::ALL_GRAPHICS);

        let bindings = &[ubo_binding, sampler_binding];
        let dsl_info = vk::DescriptorSetLayoutCreateInfo::builder()
            .bindings(bindings);

        unsafe {
            Ok(device.create_descriptor_set_layout(&dsl_info, None)?)
        }
    }
    fn create_postprocessing_descriptor_set_layout(&self, device: &Device) -> Result<vk::DescriptorSetLayout> {
        let sampler_binding = vk::DescriptorSetLayoutBinding::builder()
            .binding(0)
            .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::ALL_GRAPHICS);

        let motion_vector_sampler_binding = vk::DescriptorSetLayoutBinding::builder()
            .binding(1)
            .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(1)
            .stage_flags(vk::ShaderStageFlags::ALL_GRAPHICS);

        let bindings = &[sampler_binding, motion_vector_sampler_binding];
        let dsl_info = vk::DescriptorSetLayoutCreateInfo::builder()
            .bindings(bindings);

        unsafe {
            Ok(device.create_descriptor_set_layout(&dsl_info, None)?)
        }
    }
    fn create_descriptor_set_layouts(&self, device: &Device, uniforms_info: &mut UniformsInfo) -> Result<()> {
        debug!("Creating descriptor set layouts...");
        uniforms_info.base_descriptor_set_layout = self.create_base_descriptor_set_layout(device)?;
        uniforms_info.postprocessing_descriptor_set_layout = self.create_postprocessing_descriptor_set_layout(device)?;
        debug!("Descriptor set layouts created: {:?}, {:?}", uniforms_info.base_descriptor_set_layout, uniforms_info.postprocessing_descriptor_set_layout);

        Ok(())
    }

    fn destroy_descriptor_set_layouts(&self, device: &Device, uniforms_info: &mut UniformsInfo) -> () {
        debug!("Destroying descriptor set layout...");

        unsafe {
            device.destroy_descriptor_set_layout(uniforms_info.base_descriptor_set_layout, None);
        }
        uniforms_info.base_descriptor_set_layout = vk::DescriptorSetLayout::null();

        unsafe {
            device.destroy_descriptor_set_layout(uniforms_info.postprocessing_descriptor_set_layout, None);
        }
        uniforms_info.postprocessing_descriptor_set_layout = vk::DescriptorSetLayout::null();
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

    fn create_base_descriptor_pool(&self, device: &Device, image_count: u32) -> Result<vk::DescriptorPool> {
        let ubo_size = vk::DescriptorPoolSize::builder()
            .type_(vk::DescriptorType::UNIFORM_BUFFER)
            .descriptor_count(image_count);

        let sampler_size = vk::DescriptorPoolSize::builder()
            .type_(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(image_count);

        let pool_sizes = &[ubo_size, sampler_size];
        let desc_pool_info = vk::DescriptorPoolCreateInfo::builder()
            .pool_sizes(pool_sizes)
            .max_sets(image_count);

        unsafe {
            Ok(device.create_descriptor_pool(&desc_pool_info, None)?)
        }
    }
    fn create_postprocessing_descriptor_pool(&self, device: &Device, image_count: u32) -> Result<vk::DescriptorPool> {
        let sampler_size = vk::DescriptorPoolSize::builder()
            .type_(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(image_count);

        let motion_vector_sampler_size = vk::DescriptorPoolSize::builder()
            .type_(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
            .descriptor_count(image_count);

        let pool_sizes = &[sampler_size, motion_vector_sampler_size];
        let desc_pool_info = vk::DescriptorPoolCreateInfo::builder()
            .pool_sizes(pool_sizes)
            .max_sets(image_count);

        unsafe {
            Ok(device.create_descriptor_pool(&desc_pool_info, None)?)
        }
    }
    fn create_descriptor_pools(&self, device: &Device, uniforms_info: &mut UniformsInfo, app_data: &AppData) -> Result<()> {
        let image_count = app_data.swapchain.as_ref().unwrap().image_count;

        debug!("Creating descriptor pools...");
        uniforms_info.base_descriptor_pool = self.create_base_descriptor_pool(device, image_count)?;
        uniforms_info.postprocessing_descriptor_pool = self.create_postprocessing_descriptor_pool(device, image_count)?;
        debug!("Descriptor pool created: {:?}, {:?}", uniforms_info.base_descriptor_pool, uniforms_info.postprocessing_descriptor_pool);

        Ok(())
    }

    fn destroy_descriptor_pools(&self, device: &Device, uniforms_info: &mut UniformsInfo) -> () {
        debug!("Destroying descriptor pools...");

        unsafe {
            device.destroy_descriptor_pool(uniforms_info.base_descriptor_pool, None);
        }
        uniforms_info.base_descriptor_pool = vk::DescriptorPool::null();

        unsafe {
            device.destroy_descriptor_pool(uniforms_info.postprocessing_descriptor_pool, None);
        }
        uniforms_info.postprocessing_descriptor_pool = vk::DescriptorPool::null();
    }
}

impl BootstrapLoader for BootstrapUniformLoader {
    fn after_create_logical_device(&self, _inst: &Instance, device: &Device, _window: &Window, app_data: &mut AppData) -> Result<()> {
        let mut uniforms_info = UniformsInfo::default();
        self.create_descriptor_set_layouts(device, &mut uniforms_info)?;
        self.create_uniform_buffers(device, &mut uniforms_info, app_data)?;
        self.create_descriptor_pools(device, &mut uniforms_info, app_data)?;
        app_data.uniforms = Some(uniforms_info);

        Ok(())
    }

    fn before_destroy_logical_device(&self, _inst: &Instance, device: &Device, app_data: &mut AppData) -> () {
        if let Some(mut uniforms_info) = app_data.uniforms.take() {
            self.destroy_descriptor_pools(device, &mut uniforms_info);
            self.destroy_uniform_buffers(device, &mut uniforms_info);
            self.destroy_descriptor_set_layouts(device, &mut uniforms_info);
        }
    }

    fn recreate_swapchain(&self, inst: &Instance, device: &Device, window: &Window, app_data: &mut AppData, next: &dyn Fn(&Instance, &Device, &Window, &mut AppData) -> Result<()>) -> Result<()> {
        trace!("Recreating descriptor pool, and uniform buffers (but not descriptor set layout) in recreate_swapchain");

        let mut uniforms_info = app_data.uniforms.take().unwrap();

        self.destroy_descriptor_pools(device, &mut uniforms_info);
        self.destroy_uniform_buffers(device, &mut uniforms_info);
        next(inst, device, window, app_data)?;
        self.create_uniform_buffers(device, &mut uniforms_info, app_data)?;
        self.create_descriptor_pools(device, &mut uniforms_info, app_data)?;

        app_data.uniforms = Some(uniforms_info);

        Ok(())
    }
}
