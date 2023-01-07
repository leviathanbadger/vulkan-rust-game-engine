use std::io::{Read};

use super::{get_memory_type_index, Buffer};
use anyhow::{anyhow, Result};
use png::ColorType;
use vulkanalia::{
    prelude::v1_0::*, vk::{PhysicalDeviceMemoryProperties}
};

use crate::{
    bootstrap::{CommandPoolsInfo}
};

pub enum AttachmentKind {
    Color,
    Depth
}

#[derive(Debug, Copy, Clone)]
pub struct Image2D {
    pub format: Option<vk::Format>,
    pub size: Option<vk::Extent2D>,
    pub image: Option<vk::Image>,
    pub image_memory: Option<vk::DeviceMemory>,
    pub image_view: Option<vk::ImageView>,
    pub image_sampler: Option<vk::Sampler>,
    initialized: bool,
    owns_image: bool
}

impl Default for Image2D {
    fn default() -> Self {
        Self {
            format: Default::default(),
            size: Default::default(),
            image: Default::default(),
            image_memory: Default::default(),
            image_view: Default::default(),
            image_sampler: Default::default(),
            initialized: false,
            owns_image: true
        }
    }
}

impl Image2D {
    pub fn new() -> Self {
        Self::default()
    }
    fn new_with_image(image: vk::Image, format: vk::Format, size: vk::Extent2D) -> Self {
        Self {
            format: Some(format),
            size: Some(size),
            image: Some(image),
            owns_image: false,
            ..Self::default()
        }
    }

    unsafe fn get_supported_format(inst: &Instance, physical_device: &vk::PhysicalDevice, candidates: &[vk::Format], tiling: vk::ImageTiling, features: vk::FormatFeatureFlags) -> Result<vk::Format> {
        for format in candidates.iter() {
            let properties = inst.get_physical_device_format_properties(*physical_device, *format);
            let supported = match tiling {
                vk::ImageTiling::LINEAR => properties.linear_tiling_features.contains(features),
                vk::ImageTiling::OPTIMAL => properties.optimal_tiling_features.contains(features),
                _ => false
            };
            if supported {
                return Ok(*format);
            }
        }

        Err(anyhow!("Failed to find a supported format"))
    }

    unsafe fn choose_render_image_format(inst: &Instance, physical_device: &vk::PhysicalDevice) -> Result<vk::Format> {
        let candidates = &[
            vk::Format::R16G16B16A16_SFLOAT
        ];

        Self::get_supported_format(inst, physical_device, candidates, vk::ImageTiling::OPTIMAL, vk::FormatFeatureFlags::COLOR_ATTACHMENT)
    }
    unsafe fn choose_depth_stencil_format(inst: &Instance, physical_device: &vk::PhysicalDevice) -> Result<vk::Format> {
        let candidates = &[
            vk::Format::D32_SFLOAT_S8_UINT,
            vk::Format::D24_UNORM_S8_UINT,
            vk::Format::D32_SFLOAT
        ];

        Self::get_supported_format(inst, physical_device, candidates, vk::ImageTiling::OPTIMAL, vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT)
    }
    unsafe fn choose_motion_vector_format(inst: &Instance, physical_device: &vk::PhysicalDevice) -> Result<vk::Format> {
        let candidates = &[
            vk::Format::R16G16_SFLOAT,
            vk::Format::R16G16B16A16_SFLOAT,
            // vk::Format::R32G32_SFLOAT,
            // vk::Format::R32G32B32A32_SFLOAT
        ];

        Self::get_supported_format(inst, physical_device, candidates, vk::ImageTiling::OPTIMAL, vk::FormatFeatureFlags::COLOR_ATTACHMENT)
    }

    fn create_image(&mut self, device: &Device, memory_properties: &PhysicalDeviceMemoryProperties, size: vk::Extent2D, format: vk::Format, tiling: vk::ImageTiling, usage_flags: vk::ImageUsageFlags, memory_flags: vk::MemoryPropertyFlags) -> Result<()> {
        let image_info = vk::ImageCreateInfo::builder()
            .image_type(vk::ImageType::_2D)
            .extent(vk::Extent3D { width: size.width, height: size.height, depth: 1 })
            .mip_levels(1)
            .array_layers(1)
            .format(format)
            .tiling(tiling)
            .initial_layout(vk::ImageLayout::UNDEFINED)
            .usage(usage_flags)
            .samples(vk::SampleCountFlags::_1)
            .sharing_mode(vk::SharingMode::EXCLUSIVE);

        let image: vk::Image;
        let requirements: vk::MemoryRequirements;
        unsafe {
            image = device.create_image(&image_info, None)?;
            requirements = device.get_image_memory_requirements(image);
        }

        let memory_type_index = get_memory_type_index(memory_properties, memory_flags, requirements)?;
        let alloc_info = vk::MemoryAllocateInfo::builder()
            .allocation_size(requirements.size)
            .memory_type_index(memory_type_index);

        let image_memory: vk::DeviceMemory;
        unsafe {
            image_memory = device.allocate_memory(&alloc_info, None)?;
            device.bind_image_memory(image, image_memory, 0)?;
        }

        self.image = Some(image);
        self.image_memory = Some(image_memory);

        Ok(())
    }

    fn create_image_view(&mut self, device: &Device, aspect_flags: vk::ImageAspectFlags) -> Result<()> {
        let components = vk::ComponentMapping::builder()
            .r(vk::ComponentSwizzle::IDENTITY)
            .g(vk::ComponentSwizzle::IDENTITY)
            .b(vk::ComponentSwizzle::IDENTITY)
            .a(vk::ComponentSwizzle::IDENTITY);

        let subresource_range = vk::ImageSubresourceRange::builder()
            .aspect_mask(aspect_flags)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1);

        let image_view_info = vk::ImageViewCreateInfo::builder()
            .image(self.image.unwrap())
            .view_type(vk::ImageViewType::_2D)
            .format(self.format.unwrap())
            .components(components)
            .subresource_range(subresource_range);

        let image_view: vk::ImageView;
        unsafe {
            image_view = device.create_image_view(&image_view_info, None)?;
        }

        self.image_view = Some(image_view);

        Ok(())
    }

    fn create_image_sampler(&mut self, device: &Device) -> Result<()> {
        let sampler_info = vk::SamplerCreateInfo::builder()
            .mag_filter(vk::Filter::LINEAR)
            .min_filter(vk::Filter::LINEAR)
            .address_mode_u(vk::SamplerAddressMode::REPEAT)
            .address_mode_v(vk::SamplerAddressMode::REPEAT)
            .address_mode_w(vk::SamplerAddressMode::CLAMP_TO_BORDER)
            .anisotropy_enable(true)
            .max_anisotropy(16.0)
            .border_color(vk::BorderColor::INT_OPAQUE_BLACK)
            .unnormalized_coordinates(false)
            .compare_enable(false)
            .compare_op(vk::CompareOp::ALWAYS)
            .mipmap_mode(vk::SamplerMipmapMode::LINEAR)
            .mip_lod_bias(0.0)
            .min_lod(0.0)
            .max_lod(0.0);

        let sampler: vk::Sampler;
        unsafe {
            sampler = device.create_sampler(&sampler_info, None)?;
        }

        self.image_sampler = Some(sampler);

        Ok(())
    }

    fn transition_image_layout(&self, device: &Device, old_layout: vk::ImageLayout, new_layout: vk::ImageLayout, command_buffer: &vk::CommandBuffer) -> Result<()> {
        let format = self.format.unwrap();
        let aspect_mask: vk::ImageAspectFlags = match new_layout {
            vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL => {
                match format {
                    vk::Format::D32_SFLOAT_S8_UINT | vk::Format::D24_UNORM_S8_UINT => vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL,
                    _ => vk::ImageAspectFlags::COLOR
                }
            },
            _ => vk::ImageAspectFlags::COLOR
        };

        let subresource = vk::ImageSubresourceRange::builder()
            .aspect_mask(aspect_mask)
            .base_mip_level(0)
            .level_count(1)
            .base_array_layer(0)
            .layer_count(1);

        let (src_access_mask, dst_access_mask, src_stage_mask, dst_stage_mask) = match (old_layout, new_layout) {
            (vk::ImageLayout::UNDEFINED, vk::ImageLayout::TRANSFER_DST_OPTIMAL) => (vk::AccessFlags::empty(), vk::AccessFlags::TRANSFER_WRITE, vk::PipelineStageFlags::TOP_OF_PIPE, vk::PipelineStageFlags::TRANSFER),
            (vk::ImageLayout::TRANSFER_DST_OPTIMAL, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL) => (vk::AccessFlags::TRANSFER_WRITE, vk::AccessFlags::SHADER_READ, vk::PipelineStageFlags::TRANSFER, vk::PipelineStageFlags::FRAGMENT_SHADER),
            (vk::ImageLayout::UNDEFINED, vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL) => (vk::AccessFlags::empty(), vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE, vk::PipelineStageFlags::TOP_OF_PIPE, vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS),
            (vk::ImageLayout::UNDEFINED, vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL) => (vk::AccessFlags::empty(), vk::AccessFlags::COLOR_ATTACHMENT_READ | vk::AccessFlags::COLOR_ATTACHMENT_WRITE, vk::PipelineStageFlags::TOP_OF_PIPE, vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT),
            _ => return Err(anyhow!("Unsupported image layout transition in Image2D::transition_image_layout"))
        };

        let barrier = vk::ImageMemoryBarrier::builder()
            .old_layout(old_layout)
            .new_layout(new_layout)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .image(self.image.unwrap())
            .subresource_range(subresource)
            .src_access_mask(src_access_mask)
            .dst_access_mask(dst_access_mask);

        unsafe {
            device.cmd_pipeline_barrier(
                *command_buffer,
                src_stage_mask,
                dst_stage_mask,
                vk::DependencyFlags::empty(),
                &[] as &[vk::MemoryBarrier],
                &[] as &[vk::BufferMemoryBarrier],
                &[barrier]
            );
        }

        Ok(())
    }

    fn copy_buffer_to_image(&self, device: &Device, buffer: &Buffer::<u8>, command_buffer: &vk::CommandBuffer) -> Result<()> {
        let subresource = vk::ImageSubresourceLayers::builder()
            .aspect_mask(vk::ImageAspectFlags::COLOR)
            .mip_level(0)
            .base_array_layer(0)
            .layer_count(1);

        let extent = self.size.unwrap();

        let region = vk::BufferImageCopy::builder()
            .buffer_offset(0)
            .buffer_row_length(0)
            .buffer_image_height(0)
            .image_subresource(subresource)
            .image_offset(vk::Offset3D { x: 0, y: 0, z: 0 })
            .image_extent(vk::Extent3D { width: extent.width, height: extent.height, depth: 1 });

        unsafe {
            device.cmd_copy_buffer_to_image(*command_buffer, buffer.raw_buffer().unwrap(), self.image.unwrap(), vk::ImageLayout::TRANSFER_DST_OPTIMAL, &[region]);
        }

        Ok(())
    }

    pub fn format(&self) -> Option<vk::Format> {
        self.format
    }

    #[allow(unused)]
    pub fn size(&self) -> Option<vk::Extent2D> {
        self.size
    }

    #[allow(unused)]
    pub unsafe fn raw_image(&self) -> Option<vk::Image> {
        self.image
    }

    #[allow(unused)]
    pub unsafe fn raw_image_memory(&self) -> Option<vk::DeviceMemory> {
        self.image_memory
    }

    #[allow(unused)]
    pub unsafe fn raw_image_view(&self) -> Option<vk::ImageView> {
        self.image_view
    }

    fn create_attachment_buffer(&mut self, device: &Device, memory_properties: &PhysicalDeviceMemoryProperties, format: vk::Format, extent: &vk::Extent2D, attachment_kind: AttachmentKind, sampled: bool) -> Result<()> {
        if self.initialized {
            return Err(anyhow!("This image has already been initialized. It can't be created again!"));
        }
        if !self.owns_image {
            return Err(anyhow!("This Image2D was constructed with a passed-in vk::Image. It can't be created in a way that creates a new vk::Image."));
        }

        self.format = Some(format);

        let size = *extent;
        self.size = Some(size);

        let mut usage_flags: vk::ImageUsageFlags;
        let aspect_flags: vk::ImageAspectFlags;
        (usage_flags, aspect_flags) = match attachment_kind {
            AttachmentKind::Color => (vk::ImageUsageFlags::COLOR_ATTACHMENT, vk::ImageAspectFlags::COLOR),
            AttachmentKind::Depth => (vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT, vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL)
        };
        if sampled {
            usage_flags |= vk::ImageUsageFlags::SAMPLED;
        }

        self.create_image(device, memory_properties, size, format, vk::ImageTiling::OPTIMAL, usage_flags, vk::MemoryPropertyFlags::DEVICE_LOCAL)?;
        self.create_image_view(device, aspect_flags)?;

        if sampled {
            self.create_image_sampler(device)?;
        }

        self.initialized = true;

        Ok(())
    }

    pub fn new_and_create_depth_stencil_buffers(image_count: u32, inst: &Instance, device: &Device, physical_device: &vk::PhysicalDevice, memory_properties: &PhysicalDeviceMemoryProperties, extent: &vk::Extent2D, sampled: bool, command_pool_info: &CommandPoolsInfo) -> Result<Vec<Self>> {
        let format = unsafe { Self::choose_depth_stencil_format(inst, physical_device)? };

        let depth_stencil_buffers = (0..image_count)
            .map(|_| -> Result<Self> {
                let mut image = Image2D::new();
                image.create_attachment_buffer(device, memory_properties, format, extent, AttachmentKind::Depth, sampled)?;

                Ok(image)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let depth_buffers_ref = &depth_stencil_buffers;
        command_pool_info.submit_command_transient_sync(device, |command_buffer| {
            for depth_stencil_buffer in depth_buffers_ref {
                depth_stencil_buffer.transition_image_layout(device, vk::ImageLayout::UNDEFINED, vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL, command_buffer)?;
            }

            Ok(())
        })?;

        Ok(depth_stencil_buffers)
    }
    pub fn new_and_create_render_images(image_count: u32, inst: &Instance, device: &Device, physical_device: &vk::PhysicalDevice, memory_properties: &PhysicalDeviceMemoryProperties, extent: &vk::Extent2D, sampled: bool, command_pool_info: &CommandPoolsInfo) -> Result<Vec<Self>> {
        let format = unsafe { Self::choose_render_image_format(inst, physical_device)? };

        let render_images = (0..image_count)
            .map(|_| -> Result<Self> {
                let mut image = Image2D::new();
                image.create_attachment_buffer(device, memory_properties, format, extent, AttachmentKind::Color, sampled)?;

                Ok(image)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let render_images_ref = &render_images;
        command_pool_info.submit_command_transient_sync(device, |command_buffer| {
            for render_image in render_images_ref {
                render_image.transition_image_layout(device, vk::ImageLayout::UNDEFINED, vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL, command_buffer)?;
            }

            Ok(())
        })?;

        Ok(render_images)
    }
    pub fn new_and_create_motion_vector_buffers(image_count: u32, inst: &Instance, device: &Device, physical_device: &vk::PhysicalDevice, memory_properties: &PhysicalDeviceMemoryProperties, extent: &vk::Extent2D, sampled: bool, command_pool_info: &CommandPoolsInfo) -> Result<Vec<Self>> {
        let format = unsafe { Self::choose_motion_vector_format(inst, physical_device)? };

        let motion_vector_buffers = (0..image_count)
            .map(|_| -> Result<Self> {
                let mut image = Image2D::new();
                image.create_attachment_buffer(device, memory_properties, format, extent, AttachmentKind::Color, sampled)?;

                Ok(image)
            })
            .collect::<Result<Vec<_>, _>>()?;

        let motion_buffers_ref = &motion_vector_buffers;
        command_pool_info.submit_command_transient_sync(device, |command_buffer| {
            for depth_stencil_buffer in motion_buffers_ref {
                depth_stencil_buffer.transition_image_layout(device, vk::ImageLayout::UNDEFINED, vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL, command_buffer)?;
            }

            Ok(())
        })?;

        Ok(motion_vector_buffers)
    }

    pub fn create_from_swapchain_images(swapchain_images: &[vk::Image], format: vk::Format, size: vk::Extent2D, device: &Device) -> Result<Vec<Image2D>> {
        swapchain_images.iter()
            .map(|i| {
                let mut image = Self::new_with_image(*i, format, size);
                image.create_from_swapchain_image(device)?;
                Ok(image)
            })
            .collect::<Result<Vec<_>, _>>()
    }
    pub fn create_from_swapchain_image(&mut self, device: &Device) -> Result<()> {
        if self.initialized {
            return Err(anyhow!("This image has already been initialized. It can't be created again!"));
        }
        if self.owns_image {
            return Err(anyhow!("This Image2D was constructed without a passed-in vk::Image. It can't be created in a way that requires a preexisting vk::Image."));
        }

        self.create_image_view(device, vk::ImageAspectFlags::COLOR)?;

        self.initialized = true;

        Ok(())
    }

    pub fn create_from_png<R: Read>(&mut self, reader: &mut png::Reader<R>, device: &Device, memory_properties: &vk::PhysicalDeviceMemoryProperties, command_pool_info: &CommandPoolsInfo, is_srgb: bool) -> Result<()> {
        let buff_size = reader.info().raw_bytes();
        let color_type = reader.info().color_type;
        let mut pixels = vec![0; buff_size];
        reader.next_frame(&mut pixels)?;

        let format = if is_srgb { vk::Format::R8G8B8A8_SRGB } else { vk::Format::R8G8B8A8_UNORM };
        self.format = Some(format);

        let (width, height) = reader.info().size();
        let size = vk::Extent2D { width, height };
        self.size = Some(size);

        let mut buffer: Buffer::<u8>;
        match color_type {
            ColorType::Rgba => { },
            ColorType::Rgb => {
                let expected_pixel_count = size.width * size.height;
                let mut new_pixels = vec![0; (expected_pixel_count * 4) as usize];
                self.convert_rgb_to_rgba(&pixels, &mut new_pixels, expected_pixel_count as usize)?;
                pixels = new_pixels;
            },
            _ => return Err(anyhow!("Unsupported color type when loading PNG: {:?}", color_type))
        }

        buffer = Buffer::new(vk::BufferUsageFlags::TRANSFER_SRC, pixels.len(), false);
        buffer.create(device, memory_properties)?;
        buffer.set_data(device, &pixels)?;

        self.create_image(device, memory_properties, size, format, vk::ImageTiling::OPTIMAL, vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST, vk::MemoryPropertyFlags::DEVICE_LOCAL)?;

        command_pool_info.submit_command_transient_sync(device, |command_buffer| {
            self.transition_image_layout(device, vk::ImageLayout::UNDEFINED, vk::ImageLayout::TRANSFER_DST_OPTIMAL, command_buffer)?;
            self.copy_buffer_to_image(device, &buffer, command_buffer)?;
            self.transition_image_layout(device, vk::ImageLayout::TRANSFER_DST_OPTIMAL, vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL, command_buffer)?;

            Ok(())
        })?;

        buffer.destroy(device);

        self.create_image_view(device, vk::ImageAspectFlags::COLOR)?;
        self.create_image_sampler(device)?;

        Ok({})
    }
    pub fn convert_rgb_to_rgba(&self, pixels: &Vec<u8>, new_pixels: &mut Vec<u8>, pixel_count: usize) -> Result<()> {
        for q in 0..pixel_count {
            let from = q * 3;
            let to = q * 4;
            new_pixels[to + 0] = pixels[from + 0];
            new_pixels[to + 1] = pixels[from + 1];
            new_pixels[to + 2] = pixels[from + 2];
            new_pixels[to + 3] = 255;
        }

        Ok(())
    }

    pub fn destroy(&mut self, device: &Device) {
        if let Some(sampler) = self.image_sampler.take() {
            unsafe {
                device.destroy_sampler(sampler, None);
            }
        }

        if let Some(image_view) = self.image_view.take() {
            unsafe {
                device.destroy_image_view(image_view, None);
            }
        }

        if self.owns_image {
            if let Some(image) = self.image.take() {
                unsafe {
                    device.destroy_image(image, None);
                }
            }

            if let Some(image_memory) = self.image_memory.take() {
                unsafe {
                    device.free_memory(image_memory, None);
                }
            }

            self.format = None;
            self.size = None;
        }

        self.initialized = false;
    }

    pub(crate) fn get_descriptor_image_info(&self) -> vk::DescriptorImageInfoBuilder {
        vk::DescriptorImageInfo::builder()
            .image_layout(vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
            .image_view(self.image_view.unwrap())
            .sampler(self.image_sampler.unwrap())
    }
}
