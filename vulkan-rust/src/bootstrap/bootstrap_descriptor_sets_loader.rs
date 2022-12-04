use super::{BootstrapLoader, BootstrapUniformLoader, BootstrapCommandBufferLoader, CommandPoolsInfo};
use std::{
    mem::{size_of},
    path::{Path},
    fs::{File}
};
use anyhow::{Result};
use winit::window::{Window};
use vulkanalia::{
    prelude::v1_0::*
};

use crate::{
    app_data::{AppData},
    shader_input::{
        {motion_blur},
        uniform_buffer_object::{UniformBufferObject}
    },
    buffer::{Image2D, Buffer},
    bootstrap_loader
};

#[derive(Debug, Default)]
pub struct DescriptorSetInfo {
    pub diffuse: Image2D,
    pub occlusion_roughness_metallic: Image2D,
    pub base_descriptor_sets: Vec<vk::DescriptorSet>,
    pub postprocessing_descriptor_sets: Vec<vk::DescriptorSet>,

    pub postprocessing_vertex_buffer: Buffer<motion_blur::Vertex>
}

bootstrap_loader! {
    pub struct BootstrapDescriptorSetLoader {
        depends_on(BootstrapUniformLoader, BootstrapCommandBufferLoader);
    }
}

impl BootstrapDescriptorSetLoader {
    fn load_image<P: AsRef<Path>>(&self, path: P, device: &Device, memory_properties: &vk::PhysicalDeviceMemoryProperties, command_pools_info: &CommandPoolsInfo) -> Result<Image2D> {
        let image_file = File::open(path)?;

        let mut decoder = png::Decoder::new(image_file);
        decoder.set_ignore_text_chunk(true);
        let mut reader = decoder.read_info()?;

        let mut image = Image2D::new();
        image.create_from_png(&mut reader, device, memory_properties, command_pools_info)?;

        Ok(image)
    }

    fn load_images(&self, device: &Device, descriptor_sets_info: &mut DescriptorSetInfo, app_data: &AppData) -> Result<()> {
        let memory_properties = &app_data.memory_properties;
        let command_pools_info = app_data.command_pools.as_ref().unwrap();

        descriptor_sets_info.diffuse = self.load_image("resources/models/die/die_DefaultMaterial_BaseColor.png", device, memory_properties, command_pools_info)?;
        // descriptor_sets_info.diffuse = self.load_image("resources/models/viking-room/viking-room.png", device, memory_properties, command_pools_info)?;
        // descriptor_sets_info.diffuse = self.load_image("resources/models/sphere/sphere_DefaultMaterial_BaseColor.png", device, memory_properties, command_pools_info)?;
        descriptor_sets_info.occlusion_roughness_metallic = self.load_image("resources/models/die/die_DefaultMaterial_OcclusionRoughnessMetallic.png", device, memory_properties, command_pools_info)?;

        Ok(())
    }

    fn destroy_images(&self, device: &Device, descriptor_sets_info: &mut DescriptorSetInfo) -> () {
        descriptor_sets_info.diffuse.destroy(device);
        descriptor_sets_info.occlusion_roughness_metallic.destroy(device);
    }

    fn allocate_descriptor_sets(&self, device: &Device, count: u32, layout: vk::DescriptorSetLayout, desc_pool: vk::DescriptorPool) -> Result<Vec<vk::DescriptorSet>> {
        let layouts = vec![layout; count as usize];
        let desc_set_info = vk::DescriptorSetAllocateInfo::builder()
            .descriptor_pool(desc_pool)
            .set_layouts(&layouts);

        unsafe {
            Ok(device.allocate_descriptor_sets(&desc_set_info)?)
        }
    }
    fn create_base_descriptor_sets(&self, device: &Device, count: u32, layout: vk::DescriptorSetLayout, desc_pool: vk::DescriptorPool, uniform_buffers: &Vec<Buffer<UniformBufferObject>>, descriptor_sets_info: &DescriptorSetInfo) -> Result<Vec<vk::DescriptorSet>> {
        let desc_sets = self.allocate_descriptor_sets(device, count, layout, desc_pool)?;

        for (q, desc_set) in desc_sets.iter().enumerate() {
            let buffer = unsafe { uniform_buffers[q].raw_buffer().unwrap() };
            let buff_info = vk::DescriptorBufferInfo::builder()
                .buffer(buffer)
                .offset(0)
                .range(size_of::<UniformBufferObject>() as u64);

            let buffer_info = &[buff_info];
            let ubo_write = vk::WriteDescriptorSet::builder()
                .dst_set(*desc_set)
                .dst_binding(0)
                .dst_array_element(0)
                .descriptor_type(vk::DescriptorType::UNIFORM_BUFFER)
                .buffer_info(buffer_info);

            let image_info = &[
                descriptor_sets_info.diffuse.get_descriptor_image_info(),
                descriptor_sets_info.occlusion_roughness_metallic.get_descriptor_image_info()
            ];
            let sampler_write = vk::WriteDescriptorSet::builder()
                .dst_set(*desc_set)
                .dst_binding(1)
                .dst_array_element(0)
                .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .image_info(image_info);

            unsafe {
                device.update_descriptor_sets(&[ubo_write, sampler_write], &[] as &[vk::CopyDescriptorSet]);
            }
        }

        Ok(desc_sets)
    }
    fn create_postprocessing_descriptor_sets(&self, device: &Device, count: u32, layout: vk::DescriptorSetLayout, desc_pool: vk::DescriptorPool, base_render_images: &Vec<Image2D>, motion_vector_images: &Vec<Image2D>) -> Result<Vec<vk::DescriptorSet>> {
        let desc_sets = self.allocate_descriptor_sets(device, count, layout, desc_pool)?;

        for (q, desc_set) in desc_sets.iter().enumerate() {
            let image_info = &[
                base_render_images[q].get_descriptor_image_info(),
                motion_vector_images[q].get_descriptor_image_info()
            ];
            let sampler_write = vk::WriteDescriptorSet::builder()
                .dst_set(*desc_set)
                .dst_binding(0)
                .dst_array_element(0)
                .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
                .image_info(image_info);

            unsafe {
                device.update_descriptor_sets(&[sampler_write], &[] as &[vk::CopyDescriptorSet]);
            }
        }

        Ok(desc_sets)
    }
    fn create_descriptor_sets(&self, device: &Device, descriptor_sets_info: &mut DescriptorSetInfo, app_data: &AppData) -> Result<()> {
        let image_count = app_data.swapchain.as_ref().unwrap().image_count;
        let uniforms_info = app_data.uniforms.as_ref().unwrap();
        let depth_buffer_info = app_data.depth_buffer.as_ref().unwrap();

        debug!("Allocating descriptor sets...");
        descriptor_sets_info.base_descriptor_sets = self.create_base_descriptor_sets(device, image_count, uniforms_info.base_descriptor_set_layout, uniforms_info.base_descriptor_pool, &uniforms_info.uniform_buffers, descriptor_sets_info)?;
        descriptor_sets_info.postprocessing_descriptor_sets = self.create_postprocessing_descriptor_sets(device, image_count, uniforms_info.postprocessing_descriptor_set_layout, uniforms_info.postprocessing_descriptor_pool, &depth_buffer_info.base_render_images, &depth_buffer_info.motion_vector_buffers)?;
        debug!("Descriptor sets allocated: {:?}", descriptor_sets_info.base_descriptor_sets);

        Ok(())
    }

    fn create_postprocessing_vertex_buffer(&self, device: &Device, descriptor_sets_info: &mut DescriptorSetInfo, app_data: &AppData) -> Result<()> {
        let command_pools_info = &app_data.command_pools.as_ref().unwrap();

        let vertices = &*motion_blur::VERTICES;
        let mut buffer = Buffer::<motion_blur::Vertex>::new(vk::BufferUsageFlags::VERTEX_BUFFER, vertices.len(), true);
        buffer.create(device, &app_data.memory_properties)?;
        buffer.set_data(device, vertices)?;
        buffer.submit(device, command_pools_info)?;

        descriptor_sets_info.postprocessing_vertex_buffer = buffer;

        Ok(())
    }

    fn destroy_postprocessing_vertex_buffer(&self, device: &Device, descriptor_sets_info: &mut DescriptorSetInfo) -> () {
        descriptor_sets_info.postprocessing_vertex_buffer.destroy(device);
    }
}

impl BootstrapLoader for BootstrapDescriptorSetLoader {
    fn after_create_logical_device(&self, _inst: &Instance, device: &Device, _window: &Window, app_data: &mut AppData) -> Result<()> {
        let mut descriptor_sets_info = DescriptorSetInfo::default();
        self.load_images(device, &mut descriptor_sets_info, app_data)?;
        self.create_postprocessing_vertex_buffer(device, &mut descriptor_sets_info, app_data)?;
        self.create_descriptor_sets(device, &mut descriptor_sets_info, app_data)?;
        app_data.descriptor_sets = Some(descriptor_sets_info);

        Ok(())
    }

    fn before_destroy_logical_device(&self, _inst: &Instance, device: &Device, app_data: &mut AppData) -> () {
        if let Some(mut descriptor_sets_info) = app_data.descriptor_sets.take() {
            descriptor_sets_info.base_descriptor_sets.clear(); //No need to clean these up, apparently
            descriptor_sets_info.postprocessing_descriptor_sets.clear(); //No need to clean these up, apparently
            self.destroy_postprocessing_vertex_buffer(device, &mut descriptor_sets_info);
            self.destroy_images(device, &mut descriptor_sets_info);
        }
    }

    fn recreate_swapchain(&self, inst: &Instance, device: &Device, window: &Window, app_data: &mut AppData, next: &dyn Fn(&Instance, &Device, &Window, &mut AppData) -> Result<()>) -> Result<()> {
        trace!("Recreating descriptor sets (but not images or postprocessing model) in recreate_swapchain");

        let mut descriptor_sets_info = app_data.descriptor_sets.take().unwrap();

        descriptor_sets_info.base_descriptor_sets.clear(); //No need to clean these up, apparently
        descriptor_sets_info.postprocessing_descriptor_sets.clear(); //No need to clean these up, apparently
        next(inst, device, window, app_data)?;
        self.create_descriptor_sets(device, &mut descriptor_sets_info, app_data)?;

        app_data.descriptor_sets = Some(descriptor_sets_info);

        Ok(())
    }
}
