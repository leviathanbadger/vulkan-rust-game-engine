use super::{BootstrapLoader, BootstrapUniformLoader, BootstrapCommandBufferLoader};
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
        uniform_buffer_object::UniformBufferObject
    },
    buffer::{Image2D},
    bootstrap_loader
};

#[derive(Debug, Default)]
pub struct DescriptorSetInfo {
    pub image: Image2D,
    pub descriptor_sets: Vec<vk::DescriptorSet>
}

bootstrap_loader! {
    pub struct BootstrapDescriptorSetLoader {
        depends_on(BootstrapUniformLoader, BootstrapCommandBufferLoader);
    }
}

impl BootstrapDescriptorSetLoader {
    fn load_image<P: AsRef<Path>>(&self, path: P, device: &Device, descriptor_sets_info: &mut DescriptorSetInfo, app_data: &AppData) -> Result<()> {
        let image_file = File::open(path)?;

        let decoder = png::Decoder::new(image_file);
        let mut reader = decoder.read_info()?;

        let mut image = Image2D::new();
        image.create_from_png(&mut reader, device, &app_data.memory_properties, app_data.command_pools.as_ref().unwrap())?;
        descriptor_sets_info.image = image;

        Ok(())
    }

    fn destroy_image(&self, device: &Device, descriptor_sets_info: &mut DescriptorSetInfo) -> () {
        descriptor_sets_info.image.destroy(device);
    }

    fn create_descriptor_sets(&self, device: &Device, descriptor_sets_info: &mut DescriptorSetInfo, app_data: &AppData) -> Result<()> {
        let image_count = app_data.swapchain.as_ref().unwrap().image_count as usize;
        let uniforms_unfo = app_data.uniforms.as_ref().unwrap();

        let layouts = vec![uniforms_unfo.descriptor_set_layout; image_count];
        let desc_set_info = vk::DescriptorSetAllocateInfo::builder()
            .descriptor_pool(uniforms_unfo.descriptor_pool)
            .set_layouts(&layouts);

        let desc_sets: Vec<vk::DescriptorSet>;
        unsafe {
            debug!("Allocating descriptor sets...");
            desc_sets = device.allocate_descriptor_sets(&desc_set_info)?;
        }

        for (q, desc_set) in desc_sets.iter().enumerate() {
            let buffer = unsafe { uniforms_unfo.uniform_buffers[q].raw_buffer().unwrap() };
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

            let img_info = descriptor_sets_info.image.get_descriptor_image_info();
            let image_info = &[img_info];
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

        debug!("Descriptor sets allocated: {:?}", desc_sets);
        descriptor_sets_info.descriptor_sets = desc_sets;

        Ok(())
    }
}

impl BootstrapLoader for BootstrapDescriptorSetLoader {
    fn after_create_logical_device(&self, _inst: &Instance, device: &Device, _window: &Window, app_data: &mut AppData) -> Result<()> {
        let mut descriptor_sets_info = DescriptorSetInfo::default();
        self.load_image("resources/images/crate.png", device, &mut descriptor_sets_info, app_data)?;
        self.create_descriptor_sets(device, &mut descriptor_sets_info, app_data)?;
        app_data.descriptor_sets = Some(descriptor_sets_info);

        Ok(())
    }

    fn before_destroy_logical_device(&self, _inst: &Instance, device: &Device, app_data: &mut AppData) -> () {
        if let Some(mut descriptor_sets_info) = app_data.descriptor_sets.take() {
            descriptor_sets_info.descriptor_sets.clear(); //No need to clean these up, apparently
            self.destroy_image(device, &mut descriptor_sets_info);
        }
    }

    fn recreate_swapchain(&self, inst: &Instance, device: &Device, window: &Window, app_data: &mut AppData, next: &dyn Fn(&Instance, &Device, &Window, &mut AppData) -> Result<()>) -> Result<()> {
        trace!("Recreating descriptor sets (but not image) in recreate_swapchain");

        let mut descriptor_sets_info = app_data.descriptor_sets.take().unwrap();

        descriptor_sets_info.descriptor_sets.clear(); //No need to clean these up, apparently
        next(inst, device, window, app_data)?;
        self.create_descriptor_sets(device, &mut descriptor_sets_info, app_data)?;

        app_data.descriptor_sets = Some(descriptor_sets_info);

        Ok(())
    }
}
